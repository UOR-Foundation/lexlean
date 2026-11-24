#!/usr/bin/env python3
"""Record which workspace files a CI job opens (CI #603, phase 1; advisory).

A change-scoped gate may only skip a job whose inputs a change cannot touch,
and LexLean's tests read arbitrary repository files at run time, so the inputs
are measured, not declared. On Linux, `record` bind-mounts the workspace onto
itself and marks that mount with fanotify, so every successful open of a file
or directory under it, by any process of the job, is reported by the kernel.

A trace is an observation of one run, never an authority: it cannot see a
`stat`-only probe, a failed open (a dependency on a file's absence), a file
reached through a symlink that leaves the workspace, a descriptor opened before
the mark, or any non-file input, and one run's opens say nothing about another
control flow. A consumer may use it only to *add* re-execution, or to narrow a
skip that something else already authorizes; `authority` says so in every
report. This file never decides what runs.

Delivery is the job log (`emit`), not an artifact: printing cannot fail a job,
and the log is already bound by the provider to the exact job, run and attempt.
Python 3.9+, standard library only.
"""

import argparse
import base64
import ctypes
import gzip
import hashlib
import json
import os
from pathlib import Path
import secrets
import select
import signal
import struct
import subprocess
import sys
import time

SCHEMA = "lexlean/ci-read-trace/2"
AUTHORITY = "advisory: observed opens of one run; never sufficient to skip an obligation"
UNOBSERVED = (
    "stat-only probes",
    "failed opens (dependencies on absence)",
    "files reached through symlinks that leave the workspace",
    "descriptors opened before the mark",
    "environment, network and clock",
    "other control flows than this run's",
)

# <linux/fanotify.h>
FAN_CLASS_NOTIF = 0x00000000
FAN_CLOEXEC = 0x00000001
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

# The recorder must cost its own trace, never the job: the kernel queue stays
# bounded (an overflow marks the trace), the path sets stop growing at a cap
# (marked too), its address space is capped, the OOM killer takes it first,
# and one turn of the loop reads a bounded amount so a stop is always seen.
PATH_CAP = 100_000
MEMORY_CAP = 1 << 30
READS_PER_TURN = 64
READ_SIZE = 1 << 20
STOP_TIMEOUT = 120

# Build output is derived from the inputs that produced it; its reads would
# bury the inputs. `.git` is kept apart: a job that reads it depends on the
# commit itself, which every change alters.
DERIVED = ("target",)
GIT = ".git"
BEGIN, END = "lexlean-read-trace-begin", "lexlean-read-trace-end"
HERE = Path(__file__).resolve()
RECORDER_FILES = (
    HERE,
    HERE.parent / "read-trace-start.sh",
    HERE.parent / "read-trace-stop.sh",
    HERE.parents[1] / "actions" / "read-trace-start" / "action.yml",
    HERE.parents[1] / "actions" / "read-trace-stop" / "action.yml",
)


class Invalid(Exception):
    """The trace cannot be a usable record of the job's opens."""


def digest_files(paths):
    """sha256 of each recorder file, by name; a missing file is recorded as such."""
    out = {}
    for path in paths:
        key = f"{path.parent.name}/{path.name}" if path.suffix == ".yml" else path.name
        try:
            out[key] = hashlib.sha256(path.read_bytes()).hexdigest()
        except OSError:
            out[key] = "missing"
    return out


def write_atomic(path, text):
    """Write TEXT so that PATH is either absent, its old content, or all of TEXT."""
    path = Path(path)
    temporary = path.with_name(f".{path.name}.{os.getpid()}.tmp")
    with open(temporary, "w", encoding="utf-8") as handle:
        handle.write(text)
        handle.flush()
        os.fsync(handle.fileno())
    os.replace(temporary, path)


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
    """What one recording saw, kept as capped sets and counts."""

    def __init__(self, root, cap=PATH_CAP):
        self.root, self.cap = root, cap
        self.files, self.dirs, self.git = set(), set(), set()
        self.counts = {"events": 0, "input": 0, "git": 0, "derived": 0, "outside": 0,
                       "deleted": 0, "dropped": 0}
        self.overflow = False
        self.capped = False

    def size(self):
        return len(self.files) + len(self.dirs) + len(self.git)

    def add(self, mask, path):
        self.counts["events"] += 1
        deleted = path.endswith(" (deleted)")
        if deleted:
            self.counts["deleted"] += 1
            path = path[: -len(" (deleted)")]
        kind, relative = classify(path, self.root)
        self.counts[kind] += 1
        if kind == "input":
            target = self.dirs if mask & FAN_ONDIR else self.files
        elif kind == "git":
            target = self.git
        else:
            return
        if relative in target:
            return
        if self.size() >= self.cap:
            self.capped = True
            self.counts["dropped"] += 1
            return
        target.add(relative)


def report(inventory, state, errors, provenance, started, stopped):
    usable = (state == "stopped" and not errors and not inventory.overflow
              and not inventory.capped)
    return {
        "schema": SCHEMA,
        "authority": AUTHORITY,
        "nonce": provenance.get("nonce", ""),
        "state": state,
        "usable": usable,
        "queue_overflow": inventory.overflow,
        "path_cap_reached": inventory.capped,
        "errors": errors,
        "provenance": provenance,
        "started": started,
        "stopped": stopped,
        "root": inventory.root,
        "counts": inventory.counts,
        "files": sorted(inventory.files),
        "dirs": sorted(inventory.dirs),
        "git": sorted(inventory.git),
        "unobserved": list(UNOBSERVED),
    }


def provenance(root, job, matrix, job_index, environ=None, run=subprocess.run):
    """Who this trace is about, taken before the mark so none of it is traced."""
    environ = os.environ if environ is None else environ

    def git(*arguments):
        done = run(["git", "-C", root, *arguments], capture_output=True, text=True, check=False)
        return done.stdout.strip() if done.returncode == 0 else ""

    symlinks = {}
    for line in git("ls-files", "-s").splitlines():
        meta, _, name = line.partition("\t")
        if meta.startswith("120000 "):
            try:
                symlinks[name] = os.readlink(os.path.join(root, name))
            except OSError:
                symlinks[name] = "unreadable"
    try:
        parsed_matrix = json.loads(matrix) if matrix else None
    except ValueError:
        parsed_matrix = {"unparsed": matrix}
    return {
        "nonce": secrets.token_hex(16),
        "job": job,
        "matrix": parsed_matrix,
        "job_index": job_index,
        "head": git("rev-parse", "HEAD"),
        "tree": git("rev-parse", "HEAD^{tree}"),
        "dirty": bool(git("status", "--porcelain", "--untracked-files=no")),
        "symlinks": symlinks,
        "recorder": digest_files(RECORDER_FILES),
        "github": {key: environ.get(key, "") for key in (
            "GITHUB_REPOSITORY", "GITHUB_WORKFLOW_REF", "GITHUB_WORKFLOW_SHA", "GITHUB_RUN_ID",
            "GITHUB_RUN_ATTEMPT", "GITHUB_JOB", "GITHUB_SHA", "GITHUB_REF", "GITHUB_EVENT_NAME",
            "RUNNER_OS", "RUNNER_ARCH", "RUNNER_NAME", "ImageOS", "ImageVersion")},
    }


def mount_points(text):
    """The mount points named in a /proc/self/mountinfo text, unescaped."""
    points = set()
    for line in text.splitlines():
        fields = line.split(" ")
        if len(fields) > 4:
            points.add(fields[4].encode().decode("unicode_escape"))
    return points


def process_start(pid):
    """The start time of PID in clock ticks (/proc/PID/stat field 22), or None."""
    try:
        stat = Path(f"/proc/{pid}/stat").read_text()
    except OSError:
        return None
    return stat[stat.rindex(")") + 2:].split(" ")[19]


class Kernel:
    """The Linux calls the recorder makes; tests replace it to inject failures."""

    def __init__(self):
        self.libc = ctypes.CDLL(None, use_errno=True)
        self.libc.fanotify_mark.argtypes = [ctypes.c_int, ctypes.c_uint, ctypes.c_uint64,
                                            ctypes.c_int, ctypes.c_char_p]

    def is_mount_point(self, root):
        return root in mount_points(Path("/proc/self/mountinfo").read_text())

    def bind(self, root):
        subprocess.run(["mount", "--bind", root, root], check=True)

    def unbind(self, root):
        subprocess.run(["umount", "-l", root], check=False)

    def confine(self):
        import resource
        resource.setrlimit(resource.RLIMIT_AS, (MEMORY_CAP, MEMORY_CAP))
        Path("/proc/self/oom_score_adj").write_text("1000\n")

    def init(self):
        # The kernel's default bounded queue, not an unlimited one.
        fd = self.libc.fanotify_init(FAN_CLASS_NOTIF | FAN_CLOEXEC | FAN_UNLIMITED_MARKS,
                                     O_RDONLY | O_LARGEFILE)
        if fd < 0:
            raise Invalid(f"fanotify_init: {os.strerror(ctypes.get_errno())}")
        os.set_blocking(fd, False)
        return fd

    def mark(self, fd, root):
        if self.libc.fanotify_mark(fd, FAN_MARK_ADD | FAN_MARK_MOUNT,
                                   FAN_OPEN | FAN_OPEN_EXEC | FAN_ONDIR,
                                   AT_FDCWD, root.encode()) != 0:
            raise Invalid(f"fanotify_mark: {os.strerror(ctypes.get_errno())}")

    def wait(self, fd, timeout):
        try:
            select.select([fd], [], [], timeout)
        except InterruptedError:
            pass

    def read(self, fd):
        """One buffer of events, or None when the queue is empty."""
        try:
            return os.read(fd, READ_SIZE) or None
        except (BlockingIOError, InterruptedError):
            return None

    def path_of(self, event_fd):
        try:
            return os.readlink(f"/proc/self/fd/{event_fd}")
        finally:
            os.close(event_fd)

    def close(self, fd):
        os.close(fd)


def drain(kernel, fd, inventory, budget=READS_PER_TURN):
    """Read at most BUDGET buffers of queued events into INVENTORY."""
    for _ in range(budget):
        buffer = kernel.read(fd)
        if buffer is None:
            return
        for mask, event_fd, _pid in parse_events(buffer):
            if mask & FAN_Q_OVERFLOW:
                inventory.overflow = True
            if event_fd != FAN_NOFD:
                inventory.add(mask, kernel.path_of(event_fd))


def record(root, directory, kernel=None, stopping=None, clock=time.time):
    """Mark the workspace mount and record until STOPPING (run as root).

    Every exit, whatever failed, leaves the workspace unmounted if this call
    mounted it and writes a final `reads.json` naming the state it ended in.
    """
    directory = Path(directory)
    root = str(Path(root).resolve())
    prov = json.loads((directory / "provenance.json").read_text())
    nonce = prov["nonce"]
    inventory, errors, state = Inventory(root), [], "failed"
    started = clock()
    bound, fd = False, None
    flag = []
    if stopping is None:
        signal.signal(signal.SIGTERM, lambda *_: flag.append(True))

        def stopping():
            return bool(flag)
    try:
        kernel = Kernel() if kernel is None else kernel
        kernel.confine()
        # A workspace that is already a mount point would make the fallback
        # unmount in `stop` remove something this recorder did not create.
        if kernel.is_mount_point(root):
            raise Invalid("the workspace is already a mount point")
        kernel.bind(root)
        bound = True
        fd = kernel.init()
        kernel.mark(fd, root)
        write_atomic(directory / "ready.json", json.dumps({
            "nonce": nonce, "pid": os.getpid(), "start": process_start(os.getpid()),
            "root": root, "bound": True}) + "\n")
        while not stopping():
            kernel.wait(fd, 0.5)
            drain(kernel, fd, inventory)
        # What the kernel queued before the stop.
        drain(kernel, fd, inventory, budget=1 << 20)
        state = "stopped"
    except MemoryError:
        inventory.files.clear(), inventory.dirs.clear(), inventory.git.clear()
        errors.append("memory cap reached")
    except Exception as error:  # noqa: BLE001 - every failure must end in a report
        errors.append(f"{type(error).__name__}: {error}")
    finally:
        if fd is not None:
            try:
                kernel.close(fd)
            except OSError as error:
                errors.append(f"close: {error}")
        if bound:
            kernel.unbind(root)
        write_atomic(directory / "reads.json", json.dumps(
            report(inventory, state, errors, prov, started, clock()),
            indent=1, sort_keys=True) + "\n")
    return state


def load_report(directory, nonce):
    """The final report in DIRECTORY if it is this recording's, else None."""
    try:
        data = json.loads((Path(directory) / "reads.json").read_text())
    except (OSError, ValueError):
        return None
    if data.get("schema") != SCHEMA or data.get("nonce") != nonce:
        return None
    if data.get("state") not in ("stopped", "failed"):
        return None
    return data


def stop(directory, nonce, kernel=None, timeout=STOP_TIMEOUT, kill=os.kill,
         start_of=process_start, sleep=time.sleep, clock=time.time):
    """End this job's recording; return a one-word outcome. Never raises for a
    recorder that is gone, stale, or stuck: those are outcomes, recorded."""
    directory = Path(directory)
    if load_report(directory, nonce) is not None:
        return "ended-early"
    try:
        ready = json.loads((directory / "ready.json").read_text())
    except (OSError, ValueError):
        return "never-ready"
    if ready.get("nonce") != nonce:
        return "stale-ready"
    pid, start = int(ready["pid"]), ready.get("start")
    # A recycled PID names another process: signal only the recorder itself.
    alive = start is not None and start_of(pid) == start
    if alive:
        kill(pid, signal.SIGTERM)
        deadline = clock() + timeout
        while clock() < deadline:
            if load_report(directory, nonce) is not None:
                return "stopped"
            sleep(0.5)
        if start_of(pid) == start:
            kill(pid, getattr(signal, "SIGKILL", 9))
    if load_report(directory, nonce) is not None:
        return "stopped"
    # The recorder died or hung after mounting: remove its mount ourselves.
    kernel = Kernel() if kernel is None else kernel
    root = ready.get("root", "")
    if ready.get("bound") and root and kernel.is_mount_point(root):
        kernel.unbind(root)
    return "timed-out" if alive else "recorder-gone"


def emit(directory, nonce, out=sys.stdout, width=4000):
    """Print the report into the job log as one verifiable block."""
    data = load_report(directory, nonce)
    if data is None:
        prov_path = Path(directory) / "provenance.json"
        try:
            prov = json.loads(prov_path.read_text())
        except (OSError, ValueError):
            prov = {"nonce": nonce}
        data = report(Inventory(""), "missing", ["no report from the recorder"], prov, 0, 0)
    data["recorder_at_stop"] = digest_files(RECORDER_FILES)
    payload = base64.b64encode(gzip.compress(
        json.dumps(data, sort_keys=True).encode(), mtime=0)).decode()
    digest = hashlib.sha256(payload.encode()).hexdigest()
    print(f"{BEGIN} {SCHEMA} {digest} {len(payload)}", file=out)
    for start in range(0, len(payload), width):
        print(payload[start:start + width], file=out)
    print(f"{END} {digest}", file=out)
    print(f"read trace: {data['state']}, usable={data['usable']}, {len(data['files'])} files, "
          f"{len(data['dirs'])} dirs, counts {data['counts']}", file=out)
    return data


def decode(log_text):
    """The one report a job log carries. Refuses zero, two, or a corrupt block."""
    blocks, current = [], None
    for raw in log_text.splitlines():
        # The provider prefixes each line with a timestamp and a space.
        line = raw.split(" ", 1)[1] if raw[:4].isdigit() and " " in raw else raw
        if line.startswith(BEGIN + " "):
            if current is not None:
                raise Invalid("nested read-trace block")
            _, schema, digest, length = line.split(" ")
            current = {"schema": schema, "digest": digest, "length": int(length), "body": []}
        elif line.startswith(END + " "):
            if current is None or line.split(" ")[1] != current["digest"]:
                raise Invalid("unmatched read-trace end")
            blocks.append(current)
            current = None
        elif current is not None:
            current["body"].append(line.strip())
    if current is not None:
        raise Invalid("unterminated read-trace block")
    if len(blocks) != 1:
        raise Invalid(f"{len(blocks)} read-trace blocks")
    block = blocks[0]
    payload = "".join(block["body"])
    if len(payload) != block["length"] or hashlib.sha256(payload.encode()).hexdigest() != block["digest"]:
        raise Invalid("read-trace block does not match its digest")
    data = json.loads(gzip.decompress(base64.b64decode(payload, validate=True)))
    if data.get("schema") != block["schema"] or data.get("schema") != SCHEMA:
        raise Invalid("read-trace schema mismatch")
    return data


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="command", required=True)
    one = sub.add_parser("provenance")
    one.add_argument("--root", required=True)
    one.add_argument("--job", required=True)
    one.add_argument("--matrix", default="")
    one.add_argument("--job-index", default="")
    one.add_argument("--output", required=True)
    two = sub.add_parser("record")
    two.add_argument("--root", required=True)
    two.add_argument("--dir", required=True)
    for name in ("stop", "emit"):
        three = sub.add_parser(name)
        three.add_argument("--dir", required=True)
        three.add_argument("--nonce", required=True)
    four = sub.add_parser("decode")
    four.add_argument("log")
    arguments = parser.parse_args(argv)
    try:
        if arguments.command == "provenance":
            write_atomic(arguments.output, json.dumps(provenance(
                arguments.root, arguments.job, arguments.matrix, arguments.job_index),
                indent=1, sort_keys=True) + "\n")
        elif arguments.command == "record":
            return 0 if record(arguments.root, arguments.dir) == "stopped" else 1
        elif arguments.command == "stop":
            outcome = stop(arguments.dir, arguments.nonce)
            print(f"read recorder: {outcome}")
            return 0 if outcome == "stopped" else 1
        elif arguments.command == "emit":
            emit(arguments.dir, arguments.nonce)
        else:
            print(json.dumps(decode(Path(arguments.log).read_text(errors="replace")),
                             indent=1, sort_keys=True))
    except (Invalid, OSError, ValueError, KeyError, subprocess.CalledProcessError) as error:
        print(f"ci-read-trace: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
