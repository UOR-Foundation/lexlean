"""Tests of the phase-1 read recorder (CI #603). The kernel part runs only as root on Linux."""
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import time
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parent))
import ci_read_trace as trace  # noqa: E402


def event(mask, fd=7, pid=42, version=trace.FANOTIFY_METADATA_VERSION, extra=b""):
    length = trace.METADATA.size + len(extra)
    return trace.METADATA.pack(length, version, 0, trace.METADATA.size, mask, fd, pid) + extra


class Parse(unittest.TestCase):
    def test_events_in_one_buffer(self):
        buffer = event(trace.FAN_OPEN, 5) + event(trace.FAN_OPEN | trace.FAN_ONDIR, 6, extra=b"\0" * 8)
        self.assertEqual(trace.parse_events(buffer),
                         [(trace.FAN_OPEN, 5, 42), (trace.FAN_OPEN | trace.FAN_ONDIR, 6, 42)])

    def test_truncated_and_malformed_buffers_are_refused(self):
        whole = event(trace.FAN_OPEN)
        for bad in (whole[:-1], whole + whole[:5], event(trace.FAN_OPEN, version=2)):
            with self.subTest(length=len(bad)):
                with self.assertRaises(trace.Invalid):
                    trace.parse_events(bad)

    def test_an_event_longer_than_the_buffer_is_refused(self):
        lying = trace.METADATA.pack(trace.METADATA.size * 4, trace.FANOTIFY_METADATA_VERSION, 0,
                                    trace.METADATA.size, trace.FAN_OPEN, 3, 1)
        with self.assertRaises(trace.Invalid):
            trace.parse_events(lying)


class Classify(unittest.TestCase):
    def test_kinds(self):
        root = "/w/repo"
        cases = {
            "/w/repo": ("input", "."),
            "/w/repo/SPEC.md": ("input", "SPEC.md"),
            "/w/repo/crates/a.rs": ("input", "crates/a.rs"),
            "/w/repo/.git/HEAD": ("git", ".git/HEAD"),
            "/w/repo/target/debug/x": ("derived", "target/debug/x"),
            "/w/repo-other/file": ("outside", "/w/repo-other/file"),
            "/w/repo/.github/x": ("input", ".github/x"),
            "/w/repo/targets/x": ("input", "targets/x"),
        }
        for path, expected in cases.items():
            with self.subTest(path=path):
                self.assertEqual(trace.classify(path, root + "/"), expected)


class Inventory(unittest.TestCase):
    def test_files_dirs_git_and_counts(self):
        inv = trace.Inventory("/w/r")
        inv.add(trace.FAN_OPEN, "/w/r/a.txt")
        inv.add(trace.FAN_OPEN, "/w/r/a.txt")
        inv.add(trace.FAN_OPEN | trace.FAN_ONDIR, "/w/r/crates")
        inv.add(trace.FAN_OPEN, "/w/r/.git/index")
        inv.add(trace.FAN_OPEN_EXEC, "/w/r/target/debug/t")
        inv.add(trace.FAN_OPEN, "/w/r/gone.txt (deleted)")
        report = inv.report(1.0, 2.0, {})
        self.assertEqual(report["files"], ["a.txt", "gone.txt"])
        self.assertEqual(report["dirs"], ["crates"])
        self.assertEqual(report["git"], [".git/index"])
        self.assertEqual(report["counts"], {"events": 6, "input": 4, "git": 1, "derived": 1,
                                            "outside": 0, "deleted": 1})
        self.assertTrue(report["complete"])

    def test_overflow_makes_the_trace_incomplete(self):
        inv = trace.Inventory("/w/r")
        inv.overflow = True
        report = inv.report(1.0, 2.0, {})
        self.assertFalse(report["complete"])
        self.assertIn("stat-only probes", report["unobserved"])


@unittest.skipUnless(sys.platform.startswith("linux") and hasattr(os, "geteuid") and os.geteuid() == 0,
                     "needs root on Linux for fanotify")
class Kernel(unittest.TestCase):
    def test_records_opens_of_a_child_process_and_nothing_before_the_mark(self):
        with tempfile.TemporaryDirectory() as work, tempfile.TemporaryDirectory() as out:
            root = Path(work)
            (root / "before.txt").write_text("b")
            (root / "read.txt").write_text("r")
            (root / "sub").mkdir()
            (root / "sub" / "x").write_text("x")
            (root / "target").mkdir()
            (root / "target" / "built").write_text("t")
            subprocess.run(["cat", str(root / "before.txt")], check=True, capture_output=True)
            ready, output = Path(out) / "ready", Path(out) / "reads.json"
            recorder = subprocess.Popen([sys.executable, trace.__file__, "record", "--root", str(root),
                                         "--output", str(output), "--ready", str(ready)])
            deadline = time.time() + 30
            while not ready.exists() and time.time() < deadline:
                time.sleep(0.05)
            self.assertTrue(ready.exists(), "the recorder never marked the mount")
            subprocess.run(["cat", str(root / "read.txt")], check=True, capture_output=True)
            subprocess.run(["ls", str(root / "sub")], check=True, capture_output=True)
            subprocess.run(["cat", str(root / "target" / "built")], check=True, capture_output=True)
            trace.stop(ready, output)
            recorder.wait(timeout=30)
            report = json.loads(output.read_text())
            self.assertTrue(report["complete"])
            self.assertIn("read.txt", report["files"])
            self.assertNotIn("before.txt", report["files"])
            self.assertIn("sub", report["dirs"])
            self.assertNotIn("target/built", report["files"])
            self.assertGreaterEqual(report["counts"]["derived"], 1)


if __name__ == "__main__":
    unittest.main()
