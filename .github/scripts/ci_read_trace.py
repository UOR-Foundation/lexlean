#!/usr/bin/env python3
"""Record which workspace files a CI job opens (CI #603, phase 1; advisory).

A change-scoped gate may only skip a job whose inputs a change cannot touch,
and LexLean's tests read arbitrary repository files at run time, so the inputs
are measured, not declared. On Linux, `record` bind-mounts the workspace onto
itself and marks that mount with fanotify, so every open of a file or listing
of a directory under it, by any process of the job (cargo, rustc, Lean, the
tests), is reported by the kernel. `stop` ends the recording and writes the
inventory.

What it cannot see, and what a consumer must therefore treat as unknown:
`stat`-only probes (a test that checks whether a path exists without opening
it), non-file inputs (environment, network, clock), and opens that happened
before the mark. A queue overflow makes the whole trace invalid. This file
never decides what runs; it only records. Python 3.9+, standard library only.
"""

import argparse
import ctypes
import json
import os
from pathlib import Path
import select
import signal
import struct
import subprocess
import sys
import time

SCHEMA = "lexlean/ci-read-trace/1"

# <linux/fanotify.h>
FAN_CLASS_NOTIF = 0x00000000
FAN_CLOEXEC = 0x00000001
FAN_UNLIMITED_QUEUE = 0x00000010
FAN_UNLIMITED_MARKS = 0x00000020
FAN_MARK_ADD = 0x00000001
FAN_MARK_MOUNT = 0x00000010
FAN_OPEN = 0x00000020
FAN_OPEN_EXEC = 0x00001000
FAN_Q_OVERFLOW = 0x00004000
FAN_ONDIR = 0x40000000
FAN_NOFD = -1
AT_FDCWD = -100
O_RDONLY = 0o0
O_LARGEFILE = 0o100000
# struct fanotify_event_metadata: u32 event_len, u8 vers, u8 reserved,
# u16 metadata_len, u64 mask (aligned), s32 fd, s32 pid.
METADATA = struct.Struct("=IBBHQii")
FANOTIFY_METADATA_VERSION = 3

# Build output is derived from the inputs that produced it; its reads would
# bury the inputs. `.git` is kept apart: a job that reads it depends on the
# commit itself, which every change alters.
DERIVED = ("target",)
GIT = ".git"


class Invalid(Exception):
    """The trace cannot be a complete record of the job's opens."""


def parse_events(buffer):
    """The (mask, fd, pid) of each event in one read of a fanotify descriptor."""
    events, offset = [], 0
    while offset < len(buffer):
        if len(buffer) - offset < METADATA.size:
            raise Invalid("truncated fanotify event")
        length, version, _, header, mask, fd, pid = METADATA.unpack_from(buffer, offset)
        if version != FANOTIFY_METADATA_VERSION:
            raise Invalid(f"fanotify metadata version {version}")
        if length < header or header < METADATA.size or offset + length > len(buffer):
            raise Invalid("malformed fanotify event")
        events.append((mask, fd, pid))
        offset += length
    return events


def classify(path, root):
    """('input' | 'git' | 'derived' | 'outside', relative path) for an opened path."""
    root = root.rstrip("/")
    if path == root:
        return "input", "."
    if not path.startswith(root + "/"):
        return "outside", path
    relative = path[len(root) + 1:]
    head = relative.split("/", 1)[0]
    if head == GIT:
        return "git", relative
    if head in DERIVED:
        return "derived", relative
    return "input", relative


class Inventory:
    """What one recording saw, kept as sets and counts."""

    def __init__(self, root):
        self.root = root
        self.files, self.dirs, self.git = set(), set(), set()
        self.counts = {"events": 0, "input": 0, "git": 0, "derived": 0, "outside": 0, "deleted": 0}
        self.overflow = False

    def add(self, mask, path):
        self.counts["events"] += 1
        deleted = path.endswith(" (deleted)")
        if deleted:
            self.counts["deleted"] += 1
            path = path[: -len(" (deleted)")]
        kind, relative = classify(path, self.root)
        self.counts[kind] += 1
        if kind == "input":
            (self.dirs if mask & FAN_ONDIR else self.files).add(relative)
        elif kind == "git":
            self.git.add(relative)

    def report(self, started, stopped, identity):
        return {
            "schema": SCHEMA,
            "root": self.root,
            "complete": not self.overflow,
            "overflow": self.overflow,
            "started": started,
            "stopped": stopped,
            "identity": identity,
            "counts": self.counts,
            "files": sorted(self.files),
            "dirs": sorted(self.dirs),
            "git": sorted(self.git),
            "unobserved": ["stat-only probes", "environment", "network", "clock",
                           "opens before the mark"],
        }


def identity():
    environ = os.environ
    return {key: environ.get(key, "") for key in (
        "GITHUB_REPOSITORY", "GITHUB_RUN_ID", "GITHUB_RUN_ATTEMPT", "GITHUB_JOB",
        "GITHUB_SHA", "RUNNER_OS", "RUNNER_ARCH", "ImageOS", "ImageVersion")}


def record(root, output, ready):
    """Mark the workspace mount and record until SIGTERM (run as root)."""
    root = str(Path(root).resolve())
    libc = ctypes.CDLL(None, use_errno=True)
    libc.fanotify_mark.argtypes = [ctypes.c_int, ctypes.c_uint, ctypes.c_uint64,
                                   ctypes.c_int, ctypes.c_char_p]
    # Its own mount, so that the mark sees this tree and nothing else.
    subprocess.run(["mount", "--bind", root, root], check=True)
    # The kernel's default bounded queue, not an unlimited one: the Atlas jobs
    # run near their memory budget, and a recorder that falls behind must cost
    # its own trace (an overflow, recorded as incomplete), never the job.
    fd = libc.fanotify_init(FAN_CLASS_NOTIF | FAN_CLOEXEC | FAN_UNLIMITED_MARKS,
                            O_RDONLY | O_LARGEFILE)
    if fd < 0:
        raise Invalid(f"fanotify_init: {os.strerror(ctypes.get_errno())}")
    if libc.fanotify_mark(fd, FAN_MARK_ADD | FAN_MARK_MOUNT, FAN_OPEN | FAN_OPEN_EXEC | FAN_ONDIR,
                          AT_FDCWD, root.encode()) != 0:
        raise Invalid(f"fanotify_mark: {os.strerror(ctypes.get_errno())}")
    inventory, started = Inventory(root), time.time()
    stopping = []
    signal.signal(signal.SIGTERM, lambda *_: stopping.append(True))
    Path(ready).write_text(f"{os.getpid()}\n")
    os.set_blocking(fd, False)

    def drain():
        """Read every queued event; whether any were read."""
        any_read = False
        while True:
            try:
                buffer = os.read(fd, 1 << 20)
            except (BlockingIOError, InterruptedError):
                return any_read
            if not buffer:
                return any_read
            any_read = True
            for mask, event_fd, _pid in parse_events(buffer):
                if mask & FAN_Q_OVERFLOW:
                    inventory.overflow = True
                if event_fd == FAN_NOFD:
                    continue
                try:
                    inventory.add(mask, os.readlink(f"/proc/self/fd/{event_fd}"))
                finally:
                    os.close(event_fd)

    while not stopping:
        try:
            select.select([fd], [], [], 0.5)
        except InterruptedError:
            pass
        drain()
    # What the kernel queued before the signal.
    drain()
    os.close(fd)
    subprocess.run(["umount", "-l", root], check=False)
    Path(output).write_text(json.dumps(inventory.report(started, time.time(), identity()),
                                       indent=1, sort_keys=True) + "\n")


def stop(ready, output, timeout=120):
    """Signal the recorder named by READY and wait for its inventory."""
    pid = int(Path(ready).read_text().split()[0])
    os.kill(pid, signal.SIGTERM)
    deadline = time.time() + timeout
    while time.time() < deadline:
        if Path(output).exists():
            try:
                json.loads(Path(output).read_text())
                return
            except ValueError:
                pass
        time.sleep(0.5)
    raise Invalid("the recorder wrote no inventory")


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="command", required=True)
    one = sub.add_parser("record")
    one.add_argument("--root", required=True)
    one.add_argument("--output", required=True)
    one.add_argument("--ready", required=True)
    two = sub.add_parser("stop")
    two.add_argument("--ready", required=True)
    two.add_argument("--output", required=True)
    arguments = parser.parse_args(argv)
    try:
        if arguments.command == "record":
            record(arguments.root, arguments.output, arguments.ready)
        else:
            stop(arguments.ready, arguments.output)
    except (Invalid, OSError, subprocess.CalledProcessError) as error:
        print(f"ci-read-trace: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
