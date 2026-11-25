#!/usr/bin/env python3
"""DOD-7 Linux warm pilot. Explicit execution only, no acceptance authority."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import resource
import signal
import subprocess
import time

HEAD = "471237b3c00b6b6a78f8867de190c7cd9eaf4d3c"
TREE = "f7d2a53de569d4c18d7b148bbf5eeb52526e386c"
PART = "preservation-std"
CASES = ["conformance_sp_29", "conformance_sp_32", "conformance_sp_33"]
ORDER = [(1, None), (1, 2), (2, 2), (2, None), (3, None), (3, 2)]
BLOBS = {
    ".github/bdd-parts.json": "f385106a694199736ca81451587380976fa18e2b",
    "crates/conformance/src/bdd_parts.rs": "1d39d05e0afdf40bb0bb470e81fd21de6a73ae7d",
    "xtask/src/ci.rs": "862a176cec7530ff289b1645c34e7cdaaa3d8c39",
    ".cargo/config.toml": "db4afa6cf1a558b81b145129026cb6c61c0ec239",
    "Cargo.lock": "867b4732f9fb0202046e455f8b8ea7076e233614",
}

def require(ok, message):
    if not ok:
        raise ValueError(message)

def parse_log(text):
    rows = re.findall(r"(?m)^test (\S+) \.\.\. (.+)$", text)
    require(sorted(name for name, status in rows) == CASES, "missing, duplicate or foreign case")
    require(all(status == "ok" for name, status in rows), "nonpassing or ignored case")
    summaries = re.findall(r"(?m)^test result: (\w+)\. (\d+) passed; (\d+) failed; (\d+) ignored;", text)
    require(summaries and all(s[0] == "ok" and int(s[2]) == int(s[3]) == 0 for s in summaries), "bad summary")
    require(sum(int(s[1]) for s in summaries) == len(CASES), "summary count differs")
    return sorted(name for name, _ in rows)

def self_test():
    good = "\n".join("test " + name + " ... ok" for name in CASES) + "\ntest result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 100 filtered out\n"
    require(parse_log(good) == CASES, "positive")
    bad = [good.replace(CASES[0], CASES[1]), good.replace(CASES[0], "foreign"),
           good.replace(" ... ok", " ... ignored", 1), good.replace("3 passed", "2 passed"),
           good.replace("0 failed", "1 failed"), good.replace("0 ignored", "1 ignored"),
           good.split("test result:")[0], good.replace(" ... ok", " ... FAILED", 1),
           good + "\ntest conformance_sp_29 ... ok\n"]
    for item in bad:
        try:
            parse_log(item)
        except ValueError:
            continue
        raise AssertionError("negative control survived")
    print("10/10 parser controls passed; synthetic only")

def command(root, args, env=None):
    return subprocess.check_output(args, cwd=root, env=env, text=True, stderr=subprocess.STDOUT).strip()

def git(root, *args):
    return command(root, ["git", *args])

def check_source(root):
    require(git(root, "rev-parse", "HEAD") == HEAD, "wrong commit")
    require(git(root, "rev-parse", "HEAD^{tree}") == TREE, "wrong tree")
    require(not git(root, "status", "--porcelain", "--untracked-files=all"), "dirty checkout")
    for path, blob in BLOBS.items():
        require(git(root, "hash-object", path) == blob, "changed file: " + path)

def host_snapshot():
    def read(path):
        p = Path(path)
        return p.read_text().strip() if p.is_file() else None
    return {"uname": list(os.uname()), "affinity": sorted(os.sched_getaffinity(0)),
            "cpu_max": read("/sys/fs/cgroup/cpu.max"),
            "memory_max": read("/sys/fs/cgroup/memory.max"),
            "cgroup": read("/proc/self/cgroup"),
            "meminfo": read("/proc/meminfo"), "loadavg": read("/proc/loadavg")}

def group_rss_kib(pgid):
    # Sum current RSS in this process group. Shared pages can be double-counted;
    # detached children can be missed. Sampled proxy, NOT exact peak memory.
    rss = 0
    for proc in Path("/proc").iterdir():
        if not proc.name.isdigit():
            continue
        try:
            stat = (proc / "stat").read_text().rsplit(")", 1)[1].split()
            if int(stat[2]) == pgid:
                for line in (proc / "status").read_text().splitlines():
                    if line.startswith("VmRSS:"):
                        rss += int(line.split()[1])
                        break
        except (FileNotFoundError, ProcessLookupError, PermissionError, IndexError, ValueError):
            continue
    return rss

def digest(data):
    return hashlib.sha256(data).hexdigest()

def run_trial(root, output, label, threads, env, timeout):
    check_source(root)
    trial_env = dict(env)
    if threads is not None:
        trial_env["RUST_TEST_THREADS"] = str(threads)
    log = output / (label + ".log")
    samples = []
    started = time.monotonic()
    before = resource.getrusage(resource.RUSAGE_CHILDREN)
    timed_out = False
    with log.open("xb") as stream:
        proc = subprocess.Popen(["cargo", "xtask", "bdd"], cwd=root, env=trial_env,
                                stdin=subprocess.DEVNULL, stdout=stream, stderr=subprocess.STDOUT,
                                start_new_session=True)
        try:
            while proc.poll() is None:
                elapsed = time.monotonic() - started
                samples.append({"seconds": elapsed, "groupRssKiB": group_rss_kib(proc.pid)})
                if elapsed > timeout:
                    timed_out = True
                    os.killpg(proc.pid, signal.SIGKILL)
                    break
                time.sleep(0.2)
        finally:
            if proc.poll() is None:
                os.killpg(proc.pid, signal.SIGKILL)
            proc.wait()
    wall = time.monotonic() - started
    after = resource.getrusage(resource.RUSAGE_CHILDREN)
    raw = log.read_bytes()
    observation = {"label": label, "threads": threads, "exit": proc.returncode,
                   "timeout": timed_out, "wallSeconds": wall, "logSha256": digest(raw),
                   "peakSampledGroupRssKiB": max((s["groupRssKiB"] for s in samples), default=0),
                   "userCpuSeconds": after.ru_utime - before.ru_utime,
                   "systemCpuSeconds": after.ru_stime - before.ru_stime, "samples": samples,
                   "authority": False}
    receipt = output / (label + ".json")
    receipt.write_text(json.dumps(observation, indent=2))
    require(not timed_out and proc.returncode == 0, "trial failed; retained raw evidence: " + label)
    observation["executed"] = parse_log(raw.decode("utf-8"))
    check_source(root)
    receipt.write_text(json.dumps(observation, indent=2))
    return observation

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--self-test", action="store_true")
    parser.add_argument("--execute", action="store_true")
    parser.add_argument("--root", type=Path)
    parser.add_argument("--output", type=Path)
    parser.add_argument("--timeout", type=int, default=900)
    args = parser.parse_args()
    if args.self_test:
        self_test()
    if not args.execute:
        require(args.self_test, "explicit --execute required")
        return
    require(args.root and args.output and 1 <= args.timeout <= 1800, "invalid paths or timeout")
    root, output = args.root.resolve(), args.output.resolve()
    require(root != output and root not in output.parents, "output must be outside source checkout")
    check_source(root)
    require(not git(root, "clean", "-ndx"), "use a disposable clean checkout without ignored files")
    # Preserve host PATH/toolchain but reject overrides that can change the workload.
    forbidden = ("LEXLEAN_", "RUST_TEST_", "RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS",
                 "CARGO_BUILD_", "CARGO_PROFILE_", "CARGO_TARGET_", "CARGO_INCREMENTAL",
                 "RUSTC", "RUSTDOC", "RUSTUP_TOOLCHAIN")
    require(not any(k.startswith(forbidden) for k in os.environ), "unset ambient workload overrides")
    require(command(root, ["rustc", "--version"]).startswith("rustc 1.97.1 "), "wrong rustc")
    output.mkdir(parents=True, exist_ok=False)
    env = {**os.environ, "CARGO_INCREMENTAL": "0", "CARGO_NET_OFFLINE": "true",
           "CARGO_TERM_COLOR": "never", "LEXLEAN_BDD_PART": PART,
           "CARGO_TARGET_DIR": str(output / "target")}
    metadata = {"head": HEAD, "tree": TREE, "part": PART, "authority": False,
                "regime": "warm", "host": host_snapshot(), "rustc": command(root, ["rustc", "-vV"], env),
                "cargo": command(root, ["cargo", "-V"], env), "order": ORDER, "blobs": BLOBS,
                "limitations": ["offline dependencies must be preseeded", "sampled group RSS proxy",
                                "shared warm target; not cold timing", "no proof or native host parity"]}
    (output / "metadata.json").write_text(json.dumps(metadata, indent=2))
    run_trial(root, output, "warmup", None, env, args.timeout)
    for index, (pair, threads) in enumerate(ORDER, 1):
        run_trial(root, output, f"{index:02d}-pair{pair}-threads{threads}", threads, env, args.timeout)
    (output / "complete.json").write_text(json.dumps({"trials": 6, "hostEnd": host_snapshot(), "authority": False}))

if __name__ == "__main__":
    main()
