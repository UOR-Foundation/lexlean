"""Capture terminal provider evidence for #619 run 45316289312 and check DOD-4 joins.

Usage: python -I dod4_capture.py <lexlean-606 scripts dir> <lexlean-619 checkout> <out.json>
Read-only against GitHub (gh api). Exits nonzero on any refusal.
"""
import json, subprocess, sys, collections
from pathlib import Path

REPO = "UOR-Foundation/lexlean"
RUN, REPRO = 45316289312, 45316289414
SHA = "bd80fa3739c70396664b80b0f5ac2b289288f625"
TREE = "2f618ea9166797da981e62650e8ed3730b6beab0"
JOB_KEYS = ("id", "run_id", "run_attempt", "head_sha", "name", "status", "conclusion",
            "created_at", "started_at", "completed_at", "labels", "runner_id",
            "runner_name", "runner_group_name", "workflow_name", "head_branch", "html_url")
RUN_KEYS = ("id", "run_attempt", "head_sha", "head_branch", "workflow_id", "path", "event",
            "status", "conclusion", "created_at", "run_started_at", "updated_at", "html_url")


def api(path):
    import time
    for attempt in range(4):
        p = subprocess.run(["gh", "api", path], capture_output=True, text=True)
        if p.returncode == 0:
            return json.loads(p.stdout)
        print(f"retry {attempt + 1} {path}: {p.stderr.strip()[:120]}", file=sys.stderr)
        time.sleep(5 * (attempt + 1))
    sys.exit(f"REFUSE: provider read failed: {path}")


def pages(path):
    rows, page, total = [], 1, None
    while True:
        d = api(f"{path}{'&' if '?' in path else '?'}per_page=35&page={page}")
        if total is None:
            total = d["total_count"]
        if d["total_count"] != total:
            sys.exit("REFUSE: total_count changed during pagination")
        rows += d["jobs"]
        if not d["jobs"] or len(rows) >= total:
            break
        page += 1
    if len(rows) != total:
        sys.exit(f"REFUSE: paginated {len(rows)} != total {total}")
    return total, [{k: j.get(k) for k in JOB_KEYS} for j in rows]


def main():
    scripts, cand, out = Path(sys.argv[1]), Path(sys.argv[2]), Path(sys.argv[3])
    sys.path.insert(0, str(scripts))
    import ci_attempt_lineage as lineage

    run = {k: api(f"repos/{REPO}/actions/runs/{RUN}")[k] for k in RUN_KEYS}
    repro = {k: api(f"repos/{REPO}/actions/runs/{REPRO}")[k] for k in RUN_KEYS}
    problems = []
    def need(ok, msg):
        if not ok:
            problems.append(msg)
    need(run["status"] == "completed" and run["conclusion"] == "success", f"run not success: {run['status']}/{run['conclusion']}")
    need(run["head_sha"] == SHA and run["path"] == ".github/workflows/ci.yml", "run identity")
    need(repro["status"] == "completed" and repro["conclusion"] == "success" and repro["head_sha"] == SHA
         and repro["path"] == ".github/workflows/reproducibility.yml", "repro identity/verdict")
    attempts = []
    for n in range(1, run["run_attempt"] + 1):
        meta = {k: api(f"repos/{REPO}/actions/runs/{RUN}/attempts/{n}")[k] for k in RUN_KEYS}
        total, jobs = pages(f"repos/{REPO}/actions/runs/{RUN}/attempts/{n}/jobs")
        attempts.append({"run": meta, "run_id": RUN, "head_sha": SHA, "run_attempt": n,
                         "total_count": total, "jobs": jobs})
    ltotal, latest = pages(f"repos/{REPO}/actions/runs/{RUN}/jobs?filter=latest")
    names = [j["name"] for j in latest]
    need(len(names) == 70 and len(set(names)) == 70, f"latest roster {len(names)}/{len(set(names))}")
    for a in attempts:
        need(sorted(j["name"] for j in a["jobs"]) == sorted(names), f"attempt {a['run_attempt']} roster differs")
    need({j["id"] for j in latest} == {j["id"] for j in attempts[-1]["jobs"]}, "latest != final attempt ids")
    need(all(j["conclusion"] == "success" for j in latest), "non-success latest job")

    # Expected host labels from attempt 1 (original scheduling) cross-checked against the
    # candidate's ci.yml text: every label must be a runs-on literal in the workflow.
    wf = (cand / ".github/workflows/ci.yml").read_text(encoding="utf-8")
    expected = {}
    for j in attempts[0]["jobs"]:
        lab = [l for l in j["labels"] if l in wf]
        need(len(lab) >= 1, f"no workflow-literal label for {j['name']}: {j['labels']}")
        expected[j["name"]] = lab[0] if lab else "?"
    try:
        rec = lineage.reconcile(RUN, SHA, expected, attempts)
    except lineage.Hold as e:
        rec = None
        problems.append(f"lineage HOLD: {e}")

    # release-gate.sh / ci-plan.sh join semantics, applied to the latest listing.
    rows = json.loads((cand / ".github/vv-rows.json").read_text(encoding="utf-8"))
    status = {j["name"]: j["conclusion"] for j in latest}
    need(status.get("atlas (join)") == "success", "atlas join")
    hosts = ["build and non-Lean tests (aarch64-unknown-linux-gnu)",
             "build and non-Lean tests (x86_64-apple-darwin)",
             "build and non-Lean tests (aarch64-apple-darwin)",
             "build and non-Lean tests (x86_64-pc-windows-msvc)"]
    host_seen = collections.Counter(n for n in names if n.startswith("build and non-Lean tests ("))
    need(set(host_seen) == set(hosts) and all(v == 1 for v in host_seen.values()), f"host contexts {dict(host_seen)}")
    for r in rows:
        job = "honesty" if r["part"] == "honesty" else f"just vv ({r['part']})"
        need(status.get(job) == "success", f"vv row {job}")
    need(any("bootstrap" in n and status[n] == "success" for n in names), "bootstrap export job")

    fresh = [j for j in attempts[-1]["jobs"] if rec and rec["jobs"][j["name"]]["origin_attempt"] == len(attempts)]
    snap = {"schema": "lexlean/ci-phase-b-attempt-snapshot/1", "phase": "B_GENERIC_CI_PRE_CUTOVER_TERMINAL",
            "source": "GitHub REST provider observation, explicit attempt pages plus filter=latest",
            "repository": REPO, "candidate_tree": TREE, "observed_run": run, "independent_repro": repro,
            "attempts": [{"run": a["run"], "total_count": a["total_count"], "jobs": a["jobs"]} for a in attempts],
            "latest_total_count": ltotal, "lineage": rec,
            "vv_rows": [r["part"] for r in rows],
            "dod4_problems": problems, "dod4_joins_satisfied": not problems,
            "billed_minutes": None, "billed_cost": None}
    out.write_text(json.dumps(snap, indent=1, sort_keys=False) + "\n", encoding="utf-8")
    if rec:
        print(json.dumps({k: rec[k] for k in ("retained_successes", "completed_execution_minutes", "all_latest_jobs_success")}))
        print("fresh executions in final attempt:", [(j["name"], j["id"], j["started_at"], j["completed_at"]) for j in fresh])
    print("vv rows:", len(rows), "problems:", problems or "none")
    sys.exit(1 if problems else 0)


main()
