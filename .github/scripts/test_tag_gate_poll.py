"""Fail-closed tests for tag.yml's *actual* shell wait step (#603).

The workflow can observe a successful GitHub Actions run containing skipped
jobs. It must ask release-gate.sh whether all source-bound statuses and
receipts actually qualify before it allows the tag step to execute.
"""
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import textwrap
import unittest


WORKFLOW = Path(__file__).resolve().parents[1] / "workflows" / "tag.yml"
WAIT = "      - name: Wait for the gate of this commit's tree"
TAG = "      - name: Tag, release, and carry the line to main"


def real_wait_script(text):
    if text.count(WAIT) != 1 or text.count(TAG) != 1:
        raise ValueError("release gate step missing or duplicated")
    before, rest = text.split(WAIT, 1)
    step, _after = rest.split(TAG, 1)
    if "        run: |\n" not in step or "shell: bash" not in step:
        raise ValueError("tag gate shell step changed")
    script = textwrap.dedent(step.split("        run: |\n", 1)[1])
    if not script.startswith("set -euo pipefail\n"):
        raise ValueError("tag gate must fail closed")
    return script


@unittest.skipUnless(shutil.which("bash"), "requires local bash for actual shell semantics")
class TagGateProof(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        (self.root / ".github").mkdir()
        (self.root / "bin").mkdir()
        gate = self.root / ".github" / "release-gate.sh"
        gate.write_text(
            '#!/usr/bin/env bash\n'
            'count_file="$RUNNER_TEMP/receipt-calls"\n'
            'count=$(cat "$count_file" 2>/dev/null || echo 0)\n'
            'count=$((count+1))\n'
            'echo "$count" > "$count_file"\n'
            'case "$RECEIPT_MODE" in\n'
            '  pass) exit 0 ;;\n'
            '  refuse) exit 1 ;;\n'
            '  delayed) if [ "$count" -eq 1 ]; then exit 1; else exit 0; fi ;;\n'
            '  *) exit 2 ;;\n'
            'esac\n'
        )
        gate.chmod(0o755)
        gh = self.root / "bin" / "gh"
        gh.write_text('#!/usr/bin/env bash\n'
                      'echo "$POLL_STATE"\n')
        gh.chmod(0o755)
        self.source = WORKFLOW.read_text(encoding="utf-8")
        self.script = real_wait_script(self.source)

    def run_wait(self, receipt_mode, state="completed success https://example.invalid/run",
                 called="false", script=None):
        env = dict(os.environ)
        env.update(
            REPO="UOR-Foundation/lexlean", COMMIT="a" * 40,
            RUNNER_TEMP=str(self.root), CALLED=called,
            RECEIPT_MODE=receipt_mode, POLL_STATE=state,
            PATH=str(self.root / "bin") + os.pathsep + env.get("PATH", ""),
        )
        (self.root / "receipt-calls").unlink(missing_ok=True)
        result = subprocess.run(
            ["bash", "-c", script or self.script], cwd=self.root, env=env,
            capture_output=True, text=True, timeout=5, check=False,
        )
        count = self.root / "receipt-calls"
        return result, int(count.read_text()) if count.exists() else 0

    def test_initial_exact_tree_receipt_pass_allows_tag(self):
        proc, count = self.run_wait("pass")
        self.assertEqual(proc.returncode, 0, proc.stderr)
        self.assertEqual(count, 1)

    def test_success_conclusion_without_receipts_is_not_qualification(self):
        proc, count = self.run_wait("refuse")
        self.assertNotEqual(proc.returncode, 0)
        self.assertIn("no complete exact-tree gate evidence", proc.stdout)
        self.assertEqual(count, 2, "must check again after provider claims success")

    def test_late_provider_success_requires_fresh_qualifying_receipts(self):
        proc, count = self.run_wait("delayed")
        self.assertEqual(proc.returncode, 0, proc.stderr)
        self.assertIn("qualified gate passed", proc.stdout)
        self.assertEqual(count, 2)

    def test_provider_failure_is_rejected(self):
        proc, count = self.run_wait("refuse", state="completed failure https://example.invalid/run")
        self.assertNotEqual(proc.returncode, 0)
        self.assertEqual(count, 1)

    def test_called_workflow_never_waits_for_unqualified_receipts(self):
        proc, count = self.run_wait("refuse", called="true")
        self.assertNotEqual(proc.returncode, 0)
        self.assertIn("no run of ci.yml verified", proc.stdout)
        self.assertEqual(count, 1)

    def test_regression_does_not_accept_success_without_gate_recheck(self):
        self.assertLess(self.source.index(WAIT), self.source.index(TAG))
        self.assertEqual(self.script.count(".github/release-gate.sh"), 2)
        self.assertIn("if SHA=\"$COMMIT\"", self.script)
        self.assertIn("exit 1", self.script)


if __name__ == "__main__":
    unittest.main()
