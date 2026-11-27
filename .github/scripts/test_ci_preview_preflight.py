"""Execute the exact draft-preview ref-preflight Bash with fake bounded curl.

The preflight does not qualify a merge. Its only job is to refuse a stale PR
base before the expensive checkout, without a hanging unauthenticated Git fetch.
"""
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import textwrap
import unittest


WORKFLOW = Path(__file__).resolve().parents[1] / "workflows" / "development-feedback.yml"
PRECHECK = "      - name: Bounded stale-base preflight before checkout\n"
CHECKOUT = "      - uses: actions/checkout@"
SHA = "a" * 40


def actual_script():
    body = WORKFLOW.read_text(encoding="utf-8")
    if body.count(PRECHECK) != 1 or body.count(CHECKOUT) != 1:
        raise ValueError("precheck/checkout missing or duplicated")
    left, right = body.split(PRECHECK, 1)
    if CHECKOUT in left:
        raise ValueError("precheck must precede checkout")
    before_checkout = right.split(CHECKOUT, 1)[0]
    if before_checkout.count("        run: |\n") != 1:
        raise ValueError("precheck shell run block missing")
    script = textwrap.dedent(before_checkout.split("        run: |\n", 1)[1])
    if not script.startswith("set -euo pipefail\n"):
        raise ValueError("precheck must fail closed")
    return script


@unittest.skipUnless(shutil.which("bash"), "requires Bash")
class EarlyRefCheck(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.script = actual_script()

    def setUp(self):
        tmp = tempfile.TemporaryDirectory()
        self.addCleanup(tmp.cleanup)
        self.root = Path(tmp.name)
        executable = self.root / "curl"
        executable.write_text(
            "#!/bin/bash\n"
            "set -euo pipefail\n"
            "args=\" $* \"\n"
            "for required in '--connect-timeout 2' '--max-time 5' "
            "'--retry 0' '--max-filesize 65536'; do\n"
            "  [[ \"$args\" == *\"$required\"* ]] || exit 82\n"
            "done\n"
            "printf 'invoked\\n' >> \"$MOCK_LOG\"\n"
            "if [[ \"$MOCK_FAIL\" == yes ]]; then exit 28; fi\n"
            "if [[ \"$MOCK_BAD_JSON\" == yes ]]; then printf 'not-json'; exit 0; fi\n"
            "printf '{\"ref\":\"%s\",\"object\":{\"type\":\"%s\",\"sha\":\"%s\"}}' "
            "\"$MOCK_REF\" \"$MOCK_TYPE\" \"$MOCK_SHA\"\n",
            encoding="utf-8",
        )
        executable.chmod(0o755)
        self.env = dict(os.environ)
        self.env.update({
            "PATH": str(self.root) + os.pathsep + os.environ.get("PATH", ""),
            "GITHUB_EVENT_NAME": "pull_request",
            "GITHUB_REPOSITORY": "UOR-Foundation/lexlean",
            "GITHUB_TOKEN": "local-test-only",
            "BASE_REF": "release/v0.4",
            "EXPECTED_BASE": SHA,
            "MOCK_REF": "refs/heads/release/v0.4",
            "MOCK_TYPE": "commit",
            "MOCK_SHA": SHA,
            "MOCK_FAIL": "no",
            "MOCK_BAD_JSON": "no",
            "MOCK_LOG": str(self.root / "requests"),
        })

    def run_command(self, **updates):
        env = self.env.copy()
        env.update(updates)
        result = subprocess.run(
            ["bash", "-c", self.script], cwd=self.root, env=env,
            check=False, capture_output=True, text=True, timeout=8,
        )
        return result, (self.root / "requests").read_text().count("invoked") if (
            self.root / "requests").exists() else 0

    def test_valid_live_commit_allows_checkout(self):
        r, n = self.run_command()
        self.assertEqual(r.returncode, 0, r.stderr + r.stdout)
        self.assertEqual(n, 1)

    def test_moved_live_commit_refuses_before_checkout(self):
        r, n = self.run_command(MOCK_SHA="b" * 40)
        self.assertNotEqual(r.returncode, 0)
        self.assertIn("Base moved", r.stdout)
        self.assertEqual(n, 1)

    def test_timeout_or_network_failure_refuses(self):
        r, n = self.run_command(MOCK_FAIL="yes")
        self.assertNotEqual(r.returncode, 0)
        self.assertIn("lookup failed", r.stdout)
        self.assertEqual(n, 1)

    def test_malformed_json_refuses(self):
        r, n = self.run_command(MOCK_BAD_JSON="yes")
        self.assertNotEqual(r.returncode, 0)
        self.assertIn("Malformed base-ref", r.stdout)
        self.assertEqual(n, 1)

    def test_response_for_other_ref_refuses(self):
        r, n = self.run_command(MOCK_REF="refs/heads/main")
        self.assertNotEqual(r.returncode, 0)
        self.assertEqual(n, 1)

    def test_response_not_commit_refuses(self):
        r, n = self.run_command(MOCK_TYPE="tag")
        self.assertNotEqual(r.returncode, 0)
        self.assertEqual(n, 1)

    def test_malformed_sha_refuses(self):
        r, n = self.run_command(MOCK_SHA="invalid")
        self.assertNotEqual(r.returncode, 0)
        self.assertEqual(n, 1)

    def test_invalid_base_input_never_calls_network(self):
        r, n = self.run_command(BASE_REF="release/v0.4; rm -rf /")
        self.assertNotEqual(r.returncode, 0)
        self.assertEqual(n, 0)

    def test_invalid_expected_sha_never_calls_network(self):
        r, n = self.run_command(EXPECTED_BASE="not-a-sha")
        self.assertNotEqual(r.returncode, 0)
        self.assertEqual(n, 0)

    def test_invalid_repository_never_calls_network(self):
        r, n = self.run_command(GITHUB_REPOSITORY="foo/bar;touch wrong")
        self.assertNotEqual(r.returncode, 0)
        self.assertEqual(n, 0)

    def test_dispatch_defers_to_postcheckout_authority(self):
        r, n = self.run_command(GITHUB_EVENT_NAME="workflow_dispatch",
                                EXPECTED_BASE="")
        self.assertEqual(r.returncode, 0)
        self.assertEqual(n, 0)

    def test_original_exact_head_and_ancestry_checks_stay_intact(self):
        body = WORKFLOW.read_text(encoding="utf-8")
        for expected in (
            "git merge-base --is-ancestor", "Base branch moved since the opted-in event",
            "test \"$(git rev-parse HEAD)\" = \"$EXPECTED_HEAD\"",
            "fetch-depth: 0", "filter: blob:none", "timeout-minutes: 20",
        ):
            self.assertIn(expected, body)
        self.assertLess(body.index(PRECHECK), body.index(CHECKOUT))

    def test_preflight_curl_is_bounded_and_not_git_ls_remote(self):
        self.assertIn("--connect-timeout 2 --max-time 5", self.script)
        self.assertIn("--retry 0", self.script)
        self.assertNotIn("git ls-remote", self.script)


if __name__ == "__main__":
    unittest.main()
