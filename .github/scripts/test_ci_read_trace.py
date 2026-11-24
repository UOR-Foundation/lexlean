"""Tests of the phase-1 read recorder (CI #603).

Every failure path is driven through an injected kernel, so the negative
controls run on any host; only `Kernel` itself needs root on Linux.
"""
import io
import json
import os
from pathlib import Path
import shutil
import signal
import subprocess
import sys
import tempfile
import textwrap
import time
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parent))
import ci_read_trace as trace  # noqa: E402

HERE = Path(__file__).resolve().parent


def event(mask, fd=7, pid=42, version=trace.FANOTIFY_METADATA_VERSION, extra=b""):
    length = trace.METADATA.size + len(extra)
    return trace.METADATA.pack(length, version, 0, trace.METADATA.size, mask, fd, pid) + extra


class FakeKernel:
    """Scripted kernel: BUFFERS are returned one per read, PATHS by event fd."""

    def __init__(self, root, buffers=(), paths=None, fail=None, mounted=False):
        self.root, self.buffers, self.paths = root, list(buffers), paths or {}
        self.fail, self.mounted, self.calls = fail or {}, mounted, []

    def _step(self, name):
        self.calls.append(name)
        if name in self.fail:
            raise self.fail[name]

    def confine(self):
        self._step("confine")

    def is_mount_point(self, root):
        self.calls.append("is_mount_point")
        return self.mounted

    def bind(self, root):
        self._step("bind")
        self.mounted = True

    def unbind(self, root):
        self.calls.append("unbind")
        self.mounted = False

    def init(self):
        self._step("init")
        return 99

    def mark(self, fd, root):
        self._step("mark")

    def wait(self, fd, timeout):
        pass

    def read(self, fd):
        self._step("read")
        return self.buffers.pop(0) if self.buffers else None

    def path_of(self, event_fd):
        return self.paths[event_fd]

    def close(self, fd):
        self.calls.append("close")


def prepare(directory, nonce="n1"):
    (Path(directory) / "provenance.json").write_text(json.dumps({"nonce": nonce, "job": "j"}))


def stop_after(turns):
    count = [0]

    def stopping():
        count[0] += 1
        return count[0] > turns
    return stopping


class Parse(unittest.TestCase):
    def test_events_in_one_buffer(self):
        buffer = event(trace.FAN_OPEN, 5) + event(trace.FAN_OPEN | trace.FAN_ONDIR, 6, extra=b"\0" * 8)
        self.assertEqual(trace.parse_events(buffer),
                         [(trace.FAN_OPEN, 5, 42), (trace.FAN_OPEN | trace.FAN_ONDIR, 6, 42)])

    def test_truncated_and_malformed_buffers_are_refused(self):
        whole = event(trace.FAN_OPEN)
        lying = trace.METADATA.pack(trace.METADATA.size * 4, trace.FANOTIFY_METADATA_VERSION, 0,
                                    trace.METADATA.size, trace.FAN_OPEN, 3, 1)
        for bad in (whole[:-1], whole + whole[:5], event(trace.FAN_OPEN, version=2), lying):
            with self.subTest(length=len(bad)):
                with self.assertRaises(trace.Invalid):
                    trace.parse_events(bad)


class Classify(unittest.TestCase):
    def test_kinds(self):
        cases = {
            "/w/repo": ("input", "."),
            "/w/repo/SPEC.md": ("input", "SPEC.md"),
            "/w/repo/.git/HEAD": ("git", ".git/HEAD"),
            "/w/repo/target/debug/x": ("derived", "target/debug/x"),
            "/w/repo-other/file": ("outside", "/w/repo-other/file"),
            "/w/repo/targets/x": ("input", "targets/x"),
        }
        for path, expected in cases.items():
            with self.subTest(path=path):
                self.assertEqual(trace.classify(path, "/w/repo/"), expected)


class Inventory(unittest.TestCase):
    def test_files_dirs_git_and_counts(self):
        inv = trace.Inventory("/w/r")
        for mask, path in ((trace.FAN_OPEN, "/w/r/a.txt"), (trace.FAN_OPEN, "/w/r/a.txt"),
                           (trace.FAN_OPEN | trace.FAN_ONDIR, "/w/r/crates"),
                           (trace.FAN_OPEN, "/w/r/.git/index"),
                           (trace.FAN_OPEN_EXEC, "/w/r/target/debug/t"),
                           (trace.FAN_OPEN, "/w/r/gone.txt (deleted)")):
            inv.add(mask, path)
        self.assertEqual(sorted(inv.files), ["a.txt", "gone.txt"])
        self.assertEqual(sorted(inv.dirs), ["crates"])
        self.assertEqual(inv.counts["derived"], 1)
        self.assertEqual(inv.counts["deleted"], 1)

    def test_the_path_sets_stop_at_the_cap_and_say_so(self):
        inv = trace.Inventory("/w/r", cap=3)
        for index in range(10):
            inv.add(trace.FAN_OPEN, f"/w/r/f{index}")
        inv.add(trace.FAN_OPEN, "/w/r/f0")  # already held: not dropped
        self.assertEqual(inv.size(), 3)
        self.assertTrue(inv.capped)
        self.assertEqual(inv.counts["dropped"], 7)
        rep = trace.report(inv, "stopped", [], {"nonce": "n"}, 0, 1)
        self.assertFalse(rep["usable"])
        self.assertTrue(rep["path_cap_reached"])


class Record(unittest.TestCase):
    def setUp(self):
        self.dir = Path(tempfile.mkdtemp())
        prepare(self.dir)
        self.root = str(Path(self.dir).resolve())

    def tearDown(self):
        shutil.rmtree(self.dir, ignore_errors=True)

    def read_report(self):
        return json.loads((self.dir / "reads.json").read_text())

    def test_a_clean_recording_is_usable_and_unmounts(self):
        kernel = FakeKernel(self.root, [event(trace.FAN_OPEN, 5)], {5: self.root + "/SPEC.md"})
        self.assertEqual(trace.record(self.root, self.dir, kernel, stop_after(2)), "stopped")
        rep = self.read_report()
        self.assertEqual((rep["state"], rep["usable"], rep["files"]), ("stopped", True, ["SPEC.md"]))
        self.assertEqual(rep["authority"], trace.AUTHORITY)
        self.assertEqual(rep["nonce"], "n1")
        self.assertIn("resources", rep)
        self.assertFalse(kernel.mounted)
        self.assertEqual(kernel.calls[-2:], ["close", "unbind"])

    def test_every_injected_failure_ends_in_an_unusable_report_and_no_mount(self):
        failures = {
            "confine": OSError("setrlimit"),
            "bind": subprocess.CalledProcessError(32, "mount"),
            "init": trace.Invalid("fanotify_init: Operation not permitted"),
            "mark": trace.Invalid("fanotify_mark: No such device"),
            "read": OSError("EIO"),
        }
        for step, error in failures.items():
            with self.subTest(step=step):
                (self.dir / "reads.json").unlink(missing_ok=True)
                (self.dir / "ready.json").unlink(missing_ok=True)
                kernel = FakeKernel(self.root, fail={step: error})
                self.assertEqual(trace.record(self.root, self.dir, kernel, stop_after(3)), "failed")
                rep = self.read_report()
                self.assertEqual(rep["state"], "failed")
                self.assertFalse(rep["usable"])
                self.assertTrue(rep["errors"])
                self.assertFalse(kernel.mounted, "the workspace mount was left behind")
                ready = (self.dir / "ready.json").exists()
                self.assertEqual(ready, step == "read", "ready before the mark held")

    def test_a_malformed_event_stream_fails_the_trace_not_the_job(self):
        kernel = FakeKernel(self.root, [event(trace.FAN_OPEN)[:-3]])
        self.assertEqual(trace.record(self.root, self.dir, kernel, stop_after(2)), "failed")
        self.assertIn("truncated", self.read_report()["errors"][0])
        self.assertFalse(kernel.mounted)

    def test_a_queue_overflow_makes_the_trace_unusable(self):
        kernel = FakeKernel(self.root, [event(trace.FAN_Q_OVERFLOW, trace.FAN_NOFD)])
        trace.record(self.root, self.dir, kernel, stop_after(2))
        rep = self.read_report()
        self.assertTrue(rep["queue_overflow"])
        self.assertFalse(rep["usable"])

    def test_memory_exhaustion_is_reported_not_raised(self):
        kernel = FakeKernel(self.root, fail={"read": MemoryError()})
        self.assertEqual(trace.record(self.root, self.dir, kernel, stop_after(2)), "failed")
        self.assertEqual(self.read_report()["errors"], ["memory cap reached"])
        self.assertFalse(kernel.mounted)

    def test_an_existing_mount_point_is_refused_and_left_alone(self):
        kernel = FakeKernel(self.root, mounted=True)
        self.assertEqual(trace.record(self.root, self.dir, kernel, stop_after(1)), "failed")
        self.assertNotIn("bind", kernel.calls)
        self.assertNotIn("unbind", kernel.calls)
        self.assertTrue(kernel.mounted)

    def test_one_turn_reads_a_bounded_amount_so_a_stop_is_seen(self):
        busy = [event(trace.FAN_OPEN, 5)] * (trace.READS_PER_TURN * 3)
        kernel = FakeKernel(self.root, busy, {5: self.root + "/x"})
        inventory = trace.Inventory(self.root)
        trace.drain(kernel, 99, inventory)
        self.assertEqual(kernel.calls.count("read"), trace.READS_PER_TURN)


class Stop(unittest.TestCase):
    def setUp(self):
        self.dir = Path(tempfile.mkdtemp())
        prepare(self.dir)
        self.signals = []

    def tearDown(self):
        shutil.rmtree(self.dir, ignore_errors=True)

    def ready(self, nonce="n1", start="100", bound=True):
        (self.dir / "ready.json").write_text(json.dumps(
            {"nonce": nonce, "pid": 4242, "start": start, "root": "/w/r", "bound": bound}))

    def final(self, nonce="n1", state="stopped"):
        inv = trace.Inventory("/w/r")
        trace.write_atomic(self.dir / "reads.json", json.dumps(
            trace.report(inv, state, [], {"nonce": nonce}, 0, 1)))

    def stop(self, start_of, on_kill=None, kernel=None, timeout=2):
        clock = [0.0]

        def kill(pid, sig):
            self.signals.append((pid, sig))
            if on_kill:
                on_kill(sig)
        return trace.stop(self.dir, "n1", kernel=kernel or FakeKernel("/w/r"), timeout=timeout,
                          kill=kill, start_of=start_of,
                          sleep=lambda s: clock.__setitem__(0, clock[0] + s),
                          clock=lambda: clock[0])

    def test_signals_the_recorder_and_accepts_only_its_report(self):
        self.ready()
        outcome = self.stop(lambda pid: "100",
                            on_kill=lambda sig: self.final() if sig == signal.SIGTERM else None)
        self.assertEqual(outcome, "stopped")
        self.assertEqual(self.signals, [(4242, signal.SIGTERM)])

    def test_a_recycled_pid_is_never_signalled(self):
        self.ready(start="100")
        kernel = FakeKernel("/w/r", mounted=True)
        self.assertEqual(self.stop(lambda pid: "777", kernel=kernel), "recorder-gone")
        self.assertEqual(self.signals, [])
        self.assertFalse(kernel.mounted, "a dead recorder's mount is removed")

    def test_a_stale_ready_file_is_refused(self):
        self.ready(nonce="old")
        self.assertEqual(self.stop(lambda pid: "100"), "stale-ready")
        self.assertEqual(self.signals, [])

    def test_a_stale_or_partial_report_is_not_accepted(self):
        self.ready()
        (self.dir / "reads.json").write_text('{"schema": "lexlean/ci-read-trace/2", "nonce": "n1"')
        kernel = FakeKernel("/w/r", mounted=True)
        self.assertEqual(self.stop(lambda pid: "100", kernel=kernel), "timed-out")
        self.final(nonce="other")
        self.assertIsNone(trace.load_report(self.dir, "n1"))

    def test_a_hung_recorder_is_killed_and_its_mount_removed(self):
        self.ready()
        kernel = FakeKernel("/w/r", mounted=True)
        self.assertEqual(self.stop(lambda pid: "100", kernel=kernel), "timed-out")
        self.assertEqual([sig for _, sig in self.signals], [signal.SIGTERM, getattr(signal, "SIGKILL", 9)])
        self.assertFalse(kernel.mounted)

    def test_a_recorder_that_never_mounted_leaves_mounts_alone(self):
        self.ready(bound=False)
        kernel = FakeKernel("/w/r", mounted=True)
        self.stop(lambda pid: None, kernel=kernel)
        self.assertTrue(kernel.mounted)

    def test_no_ready_file_and_an_early_failure(self):
        self.assertEqual(self.stop(lambda pid: "100"), "never-ready")
        self.final(state="failed")
        self.assertEqual(self.stop(lambda pid: "100"), "ended-early")
        self.assertEqual(self.signals, [])


class Delivery(unittest.TestCase):
    def setUp(self):
        self.dir = Path(tempfile.mkdtemp())
        prepare(self.dir)

    def tearDown(self):
        shutil.rmtree(self.dir, ignore_errors=True)

    def emitted(self):
        out = io.StringIO()
        trace.emit(self.dir, "n1", out=out, width=64)
        return out.getvalue()

    def test_round_trip_through_a_timestamped_log(self):
        inv = trace.Inventory("/w/r")
        inv.add(trace.FAN_OPEN, "/w/r/SPEC.md")
        trace.write_atomic(self.dir / "reads.json", json.dumps(
            trace.report(inv, "stopped", [], {"nonce": "n1"}, 0, 1)))
        log = "".join(f"2026-11-24T10:00:00.0000000Z {line}\n" for line in self.emitted().splitlines())
        data = trace.decode(log)
        self.assertEqual(data["files"], ["SPEC.md"])
        self.assertTrue(data["usable"])
        self.assertIn("ci_read_trace.py", data["recorder_at_stop"])

    def test_a_missing_report_is_emitted_as_missing(self):
        data = trace.decode(self.emitted())
        self.assertEqual(data["state"], "missing")
        self.assertFalse(data["usable"])

    def test_tampered_duplicated_and_truncated_blocks_are_refused(self):
        good = self.emitted()
        lines = good.splitlines()
        tampered = "\n".join([lines[0], lines[1][:-1] + ("A" if lines[1][-1] != "A" else "B"), *lines[2:]])
        for name, log in (("tampered", tampered), ("twice", good + good),
                          ("truncated", "\n".join(lines[:-2])), ("absent", "nothing here\n")):
            with self.subTest(name):
                with self.assertRaises(trace.Invalid):
                    trace.decode(log)


class Provenance(unittest.TestCase):
    def test_binds_the_checkout_matrix_and_recorder(self):
        answers = {("rev-parse", "HEAD"): "c" * 40, ("rev-parse", "HEAD^{tree}"): "t" * 40,
                   ("status", "--porcelain", "--untracked-files=no"): "",
                   ("ls-files", "-s"): "100644 aaa 0\tSPEC.md\n"}

        def run(command, **_):
            return subprocess.CompletedProcess(command, 0, answers[tuple(command[3:])], "")
        prov = trace.provenance("/w/r", "vv", '{"part": "bdd-verification"}', "3",
                                environ={"GITHUB_RUN_ID": "1", "GITHUB_RUN_ATTEMPT": "2"}, run=run)
        self.assertEqual((prov["head"], prov["tree"], prov["dirty"]), ("c" * 40, "t" * 40, False))
        self.assertEqual(prov["matrix"], {"part": "bdd-verification"})
        self.assertEqual(prov["job_index"], "3")
        self.assertEqual(prov["github"]["GITHUB_RUN_ATTEMPT"], "2")
        self.assertEqual(len(prov["nonce"]), 32)
        self.assertEqual(set(prov["recorder"]), {"ci_read_trace.py", "read-trace-start.sh",
                                                 "read-trace-stop.sh", "read-trace-start/action.yml",
                                                 "read-trace-stop/action.yml"})

    def test_a_failing_git_leaves_the_identity_empty_not_invented(self):
        def run(command, **_):
            return subprocess.CompletedProcess(command, 128, "", "fatal")
        prov = trace.provenance("/w/r", "vv", "", "", environ={}, run=run)
        self.assertEqual((prov["head"], prov["tree"], prov["matrix"]), ("", "", None))


@unittest.skipUnless(shutil.which("bash"), "needs bash")
class Scripts(unittest.TestCase):
    """The action scripts end every path with status 0, whatever their tools do."""

    def setUp(self):
        self.tmp = Path(tempfile.mkdtemp())
        self.workspace = self.tmp / "ws"
        (self.workspace / ".github").mkdir(parents=True)
        shutil.copytree(HERE, self.workspace / ".github" / "scripts",
                        ignore=shutil.ignore_patterns("__pycache__"))
        self.bin = self.tmp / "bin"
        self.bin.mkdir()
        self.env_file = self.tmp / "github_env"
        self.env_file.write_text("")

    def tearDown(self):
        shutil.rmtree(self.tmp, ignore_errors=True)

    def tool(self, name, body):
        path = self.bin / name
        path.write_text("#!/usr/bin/env bash\n" + textwrap.dedent(body))
        path.chmod(0o755)

    def run_script(self, name, extra=None):
        python = Path(sys.executable).as_posix()
        self.tool("python3", f'exec "{python}" "$@"\n')
        env = dict(os.environ, GITHUB_WORKSPACE=str(self.workspace), RUNNER_TEMP=str(self.tmp),
                   GITHUB_ENV=str(self.env_file), READ_TRACE_JOB="vv",
                   PATH=f"{self.bin}{os.pathsep}{os.environ.get('PATH', '')}")
        env.update(extra or {})
        script = self.workspace / ".github" / "scripts" / name
        return subprocess.run(["bash", "--noprofile", "--norc", str(script)], env=env,
                              capture_output=True, text=True, timeout=120)

    def test_start_without_sudo_warns_and_succeeds(self):
        self.tool("sudo", "exit 1\n")
        self.tool("seq", "echo 1; echo 2\n")
        done = self.run_script("read-trace-start.sh")
        self.assertEqual(done.returncode, 0, done.stderr)
        self.assertIn("did not start", done.stdout)
        self.assertIn("LEXLEAN_READ_TRACE_NONCE=", self.env_file.read_text())

    def test_start_with_an_unwritable_temp_succeeds(self):
        done = self.run_script("read-trace-start.sh", {"RUNNER_TEMP": str(self.tmp / "absent" / "x")})
        self.assertEqual(done.returncode, 0, done.stderr)
        self.assertIn("could not be prepared", done.stdout)

    def test_stop_without_a_recording_succeeds(self):
        done = self.run_script("read-trace-stop.sh")
        self.assertEqual(done.returncode, 0, done.stderr)
        self.assertIn("no read recorder ran", done.stdout)

    def test_stop_with_failing_tools_still_prints_a_decodable_missing_trace(self):
        directory = self.tmp / "read-trace.x"
        directory.mkdir()
        prepare(directory)
        self.tool("sudo", "exit 1\n")
        done = self.run_script("read-trace-stop.sh", {"LEXLEAN_READ_TRACE_DIR": str(directory),
                                                      "LEXLEAN_READ_TRACE_NONCE": "n1"})
        self.assertEqual(done.returncode, 0, done.stderr)
        self.assertIn("did not stop cleanly", done.stdout)
        self.assertEqual(trace.decode(done.stdout)["state"], "missing")

    def test_stop_with_a_broken_python_succeeds(self):
        directory = self.tmp / "read-trace.y"
        directory.mkdir()
        self.tool("sudo", "exit 1\n")
        self.tool("timeout", "exit 137\n")
        done = self.run_script("read-trace-stop.sh", {"LEXLEAN_READ_TRACE_DIR": str(directory),
                                                      "LEXLEAN_READ_TRACE_NONCE": "n1"})
        self.assertEqual(done.returncode, 0, done.stderr)
        self.assertIn("could not be printed", done.stdout)


@unittest.skipUnless(sys.platform.startswith("linux") and hasattr(os, "geteuid") and os.geteuid() == 0,
                     "needs root on Linux for fanotify")
class KernelRecording(unittest.TestCase):
    def test_records_opens_of_a_child_process_and_nothing_before_the_mark(self):
        with tempfile.TemporaryDirectory() as work, tempfile.TemporaryDirectory() as out:
            root, directory = Path(work), Path(out)
            (root / "before.txt").write_text("b")
            (root / "read.txt").write_text("r")
            (root / "sub").mkdir()
            (root / "target").mkdir()
            (root / "target" / "built").write_text("t")
            subprocess.run(["cat", str(root / "before.txt")], check=True, capture_output=True)
            prepare(directory)
            recorder = subprocess.Popen([sys.executable, trace.__file__, "record", "--root", str(root),
                                         "--dir", str(directory)])
            deadline = time.time() + 30
            while not (directory / "ready.json").exists() and time.time() < deadline:
                time.sleep(0.05)
            self.assertTrue((directory / "ready.json").exists(), "the recorder never marked the mount")
            subprocess.run(["cat", str(root / "read.txt")], check=True, capture_output=True)
            subprocess.run(["ls", str(root / "sub")], check=True, capture_output=True)
            subprocess.run(["cat", str(root / "target" / "built")], check=True, capture_output=True)
            self.assertEqual(trace.stop(directory, "n1"), "stopped")
            recorder.wait(timeout=30)
            report = trace.load_report(directory, "n1")
            self.assertTrue(report["usable"], report["errors"])
            self.assertIn("read.txt", report["files"])
            self.assertNotIn("before.txt", report["files"])
            self.assertIn("sub", report["dirs"])
            self.assertGreaterEqual(report["counts"]["derived"], 1)
            self.assertGreater(report["resources"]["max_rss_kib"], 0)
            self.assertNotIn(str(root.resolve()), trace.mount_points(
                Path("/proc/self/mountinfo").read_text()), "the bind mount outlived the recorder")


if __name__ == "__main__":
    unittest.main()
