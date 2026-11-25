"""The release gate never accepts a jobs listing from a failed provider read (#603).

Before the backport, `release-gate.sh` read a run's jobs with
`gh api ... || true`. A request that printed a success-shaped partial listing
and then failed was taken as the run's whole roster, and could authorize a
tag. Reproduced on synthetic output: commit a752992f of advisory #606. These
tests run the real script against a stubbed `gh` on PATH.
"""
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import textwrap
import unittest

ROOT = Path(__file__).resolve().parents[2]
HOSTS = ("aarch64-unknown-linux-gnu", "x86_64-apple-darwin", "aarch64-apple-darwin",
         "x86_64-pc-windows-msvc")


def tools_present():
    """bash, and jq and git as bash finds them."""
    if not shutil.which("bash"):
        return False
    return subprocess.run(["bash", "-c", "command -v jq && command -v git"],
                          capture_output=True).returncode == 0


class Source(unittest.TestCase):
    def test_reads_are_bounded_and_atomic(self):
        text = (ROOT / ".github/release-gate.sh").read_text()
        self.assertIn('source "$(dirname "${BASH_SOURCE[0]}")/scripts/ci-provider-read.sh"', text)
        self.assertIn('ci_provider_read "repos/$repo/actions/runs/$1/jobs?per_page=50" --paginate', text)
        self.assertNotIn('gh api "repos/$repo/actions/', text)
        plan =(ROOT / ".github/ci-plan.sh").read_text()
        self.assertNotIn("for part in $(jq", plan, "an empty or failed legs read accepts any run")
        self.assertEqual(plan.count('[ -n "$parts" ] || return 1'), 2)


@unittest.skipUnless(tools_present(), "needs bash, jq and git")
class ReleaseGate(unittest.TestCase):
    def setUp(self):
        self.tmp = Path(tempfile.mkdtemp())
        self.bin = self.tmp / "bin"
        self.bin.mkdir()
        self.repo = self.tmp / "repo"
        self.repo.mkdir()
        subprocess.run(["git", "init", "-q", str(self.repo)], check=True)
        subprocess.run(["git", "-C", str(self.repo), "-c", "user.name=t", "-c", "user.email=t@t",
                        "commit", "-q", "--allow-empty", "-m", "c"], check=True)
        self.sha = subprocess.run(["git", "-C", str(self.repo), "rev-parse", "HEAD"],
                                  capture_output=True, text=True, check=True).stdout.strip()
        parts = [row["part"] for row in json.loads((ROOT / ".github/vv-rows.json").read_text())]
        roster = ["honesty\tsuccess\ts\te" if p == "honesty" else f"just vv ({p})\tsuccess\ts\te"
                  for p in parts]
        roster += ["atlas (join)\tsuccess\ts\te"]
        roster += [f"build and non-Lean tests ({h})\tsuccess\ts\te" for h in HOSTS]
        (self.tmp / "roster.txt").write_text("\n".join(roster) + "\n")
        self.tool("sleep", "exit 0\n")
        self.tool("gh", f"""
            count="{self.tmp}/jobs-reads"
            case "$*" in
              *"/jobs?"*)
                n=$(( $(cat "$count" 2>/dev/null || echo 0) + 1 )); echo "$n" > "$count"
                cat "{self.tmp}/roster.txt"
                case "$GH_MODE" in
                  partial) exit 1 ;;
                  transient) [ "$n" -ge 2 ] || exit 1 ;;
                  failed) printf 'planted auxiliary job\\tfailure\\ts\\te\\n' ;;
                esac
                exit 0 ;;
              *"/runs?"*) printf '1\\tpull_request\\tc\\tu\\thttps://example/run/1\\n'; exit 0 ;;
              "run download"*) exit 1 ;;
            esac
            exit 97
            """)

    def tearDown(self):
        shutil.rmtree(self.tmp, ignore_errors=True)

    def tool(self, name, body):
        path = self.bin / name
        path.write_text("#!/usr/bin/env bash\n" + textwrap.dedent(body))
        path.chmod(0o755)

    def gate(self, mode, rows=None):
        env = dict(os.environ, PATH=f"{self.bin}{os.pathsep}{os.environ['PATH']}", GH_MODE=mode,
                   REPO="example/repo", SHA=self.sha, OUT=str(self.tmp / "evidence.txt"))
        # The script runs in the stand-in repository; the legs are this one's.
        env["LEXLEAN_CI_ROWS"] = str(rows if rows is not None else ROOT / ".github/vv-rows.json")
        return subprocess.run(["bash", str(ROOT / ".github/release-gate.sh")], cwd=self.repo,
                              env=env, capture_output=True, text=True, timeout=120)

    def reads(self):
        return int((self.tmp / "jobs-reads").read_text())

    def test_a_complete_listing_is_accepted(self):
        done = self.gate("complete")
        self.assertEqual(done.returncode, 0, done.stderr)
        self.assertIn("gate run id 1", (self.tmp / "evidence.txt").read_text())

    def test_partial_output_of_a_failed_read_is_never_accepted(self):
        done = self.gate("partial")
        self.assertNotEqual(done.returncode, 0, done.stdout)
        self.assertIn("no run of ci.yml was the whole gate", done.stderr)
        self.assertEqual(self.reads(), 3, "the whole read is retried, then refused")

    def test_a_transient_failure_is_retried_whole(self):
        done = self.gate("transient")
        self.assertEqual(done.returncode, 0, done.stderr)
        self.assertEqual(self.reads(), 2)

    def test_a_visible_failed_job_is_refused(self):
        self.assertNotEqual(self.gate("failed").returncode, 0)

    def test_a_rows_file_with_no_leg_is_refused(self):
        empty = self.tmp / "rows.json"
        empty.write_text("[]")
        done = self.gate("complete", rows=empty)
        self.assertNotEqual(done.returncode, 0)
        self.assertIn("names no leg", done.stderr)



if __name__ == "__main__":
    unittest.main()
