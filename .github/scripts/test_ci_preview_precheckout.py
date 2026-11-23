"""Real shell probes for the nonqualifying stale-base guard (#603)."""
import os
import subprocess
import tempfile
import textwrap
import unittest
from pathlib import Path

WORKFLOW = Path(__file__).resolve().parents[1] / "workflows" / "development-feedback.yml"
START = "      - name: Reject stale preview base before checkout\n"
CHECKOUT = "      - uses: actions/checkout@"
GOOD_SHA = "a" * 40


def script():
    workflow = WORKFLOW.read_text(encoding="utf-8")
    if workflow.count(START) != 1:
        raise ValueError("precheckout stale-base step missing or duplicated")
    head, tail = workflow.split(START, 1)
    if CHECKOUT in head:
        raise ValueError("checkout runs before stale-base verification")
    if "        run: |\n" not in tail:
        raise ValueError("precheckout command not found")
    body = tail.split("        run: |\n", 1)[1].split(CHECKOUT, 1)[0]
    return textwrap.dedent(body)


class PrecheckoutGuard(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.command = script()

    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.folder = Path(self.tmp.name)
        git = self.folder / "git"
        git.write_text(
            "#!/bin/sh\n"
            "if [ \"$1\" != ls-remote ] || [ \"$2\" != --exit-code ] || "
            "[ \"$3\" != --heads ]; then exit 79; fi\n"
            "if [ \"$MOCK_FAIL\" = yes ]; then exit 128; fi\n"
            "printf '%s\\t%s\\n' \"$MOCK_SHA\" \"$MOCK_REF\"\n"
            "if [ \"$MOCK_DUPLICATE\" = yes ]; then "
            "printf '%s\\t%s\\n' \"$MOCK_SHA\" \"$MOCK_REF\"; fi\n",
            encoding="utf-8",
        )
        git.chmod(0o755)
        self.environment = dict(os.environ)
        self.environment.update({
            "PATH": str(self.folder) + os.pathsep + os.environ.get("PATH", ""),
            "BASE_REF": "release/v0.4",
            "EXPECTED_BASE": GOOD_SHA,
            "GITHUB_REPOSITORY": "UOR-Foundation/lexlean",
            "MOCK_SHA": GOOD_SHA,
            "MOCK_REF": "refs/heads/release/v0.4",
            "MOCK_FAIL": "no",
            "MOCK_DUPLICATE": "no",
        })

    def invoke(self, **changes):
        env = self.environment.copy()
        env.update(changes)
        return subprocess.run(
            ["bash", "-c", self.command], cwd=self.folder, env=env,
            capture_output=True, text=True, timeout=10, check=False,
        )

    def test_actual_shell_matches_event_base_without_checkout(self):
        result = self.invoke()
        self.assertEqual(result.returncode, 0, result.stderr + result.stdout)

    def test_changed_base_is_refused(self):
        result = self.invoke(MOCK_SHA="b" * 40)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("Base branch moved", result.stdout)

    def test_remote_ref_failure_is_refused(self):
        result = self.invoke(MOCK_FAIL="yes")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("Could not read current baseline", result.stdout)

    def test_other_branch_is_refused(self):
        result = self.invoke(MOCK_REF="refs/heads/main")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("Malformed baseline ref", result.stdout)

    def test_duplicate_remote_responses_fail_closed(self):
        result = self.invoke(MOCK_DUPLICATE="yes")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("Ambiguous baseline ref", result.stdout)

    def test_noncommit_sha_is_refused(self):
        result = self.invoke(MOCK_SHA="not-a-commit")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("Malformed baseline ref", result.stdout)

    def test_shell_injection_as_branch_is_not_executed(self):
        target = self.folder / "escaped"
        result = self.invoke(BASE_REF="release/v0.4; touch escaped")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("Preview baseline must", result.stdout)
        self.assertFalse(target.exists())

    def test_untrusted_repository_slug_is_refused(self):
        result = self.invoke(GITHUB_REPOSITORY="UOR-Foundation/lexlean;evil")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("Invalid repository identity", result.stdout)

    def test_forged_event_sha_is_refused(self):
        result = self.invoke(EXPECTED_BASE="abc")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("Invalid event base commit", result.stdout)

    def test_manual_dispatch_only_checks_live_ref(self):
        result = self.invoke(EXPECTED_BASE="", BASE_REF="main",
                             MOCK_REF="refs/heads/main")
        self.assertEqual(result.returncode, 0, result.stderr + result.stdout)

    def test_original_exact_tree_check_remains_after_checkout(self):
        source = WORKFLOW.read_text(encoding="utf-8")
        self.assertIn("git merge-base --is-ancestor", source)
        self.assertIn("Base branch moved since the opted-in event", source)
        self.assertLess(source.index(START), source.index(CHECKOUT))


if __name__ == "__main__":
    unittest.main()
