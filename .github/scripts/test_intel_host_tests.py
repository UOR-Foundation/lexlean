"""Behavioral tests for one-run Intel construction, inventory and execution."""

import contextlib
import copy
import importlib.util
import io
import json
import os
from pathlib import Path
import shutil
import stat
import sys
import tarfile
import tempfile
import unittest
from unittest.mock import patch
import zipfile

SPEC = importlib.util.spec_from_file_location("intel_host_tests", Path(__file__).with_name("intel_host_tests.py"))
factory = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(factory)


def listing(names):
    return "".join(name + ": test\n" for name in sorted(names)) + f"\n{len(names)} tests, 0 benchmarks\n"


def execution(names, ignored=(), failed=()):
    lines = [f"test {name} ... " + ("FAILED" if name in failed else "ignored" if name in ignored else "ok") for name in names]
    lines.append(f"test result: {'FAILED' if failed else 'ok'}. {len(names)-len(ignored)-len(failed)} passed; {len(failed)} failed; {len(ignored)} ignored; 0 measured; 0 filtered out; finished in 0.01s")
    return "\n".join(lines) + "\n"


class Fixture:
    """Small, fully logged Cargo construction and execution, with no toolchain."""
    def __init__(self, directory):
        self.root = directory / "repo"
        self.root.mkdir()
        (self.root / ".github").mkdir()
        self.bdd = {"parts": {"a": ["BS-07"], "b": ["BS-06"], "c": ["BS-05"]}, "targets": {"honesty": ["bdd"]}, "remainder": "rest"}
        self.lanes = {"producer": ["honesty"], "lanes": {"lower": ["a"], "print": ["b"], "validate": ["c"], "moves": ["rest"]}}
        factory.write_json(self.root / factory.BDD, self.bdd)
        factory.write_json(self.root / factory.LANES, self.lanes)
        (self.root / "Cargo.lock").write_text('version = 4\n')
        (self.root / "rust-toolchain.toml").write_text('[toolchain]\nchannel = "1.97.1"\n')
        self.fetched = directory / "fetched"
        self.artifacts = self.fetched / "artifacts"
        self.artifacts.mkdir(parents=True)
        self.producer = self.artifacts / "intel-construction-a1"
        self.producer.mkdir()
        self.native = {"checkout": str(self.root), "host": factory.HOST, "pin": "1.97.1", "rustc": f"rustc 1.97.1\nhost: {factory.HOST}\nrelease: 1.97.1\n", "cargo": "cargo 1.97.1\n", "sysroot": "/toolchain", "profile": "test", "features": "--workspace --all-features", "locked": True}
        self.binding = {"run": {"GITHUB_REPOSITORY": "UOR-Foundation/LexLean", "GITHUB_RUN_ID": "100", "GITHUB_RUN_ATTEMPT": "1"}, "source": {"commit": "a"*40, "tree": "b"*40, "files": {}}, "producer_job": "intel-construction", "native": self.native, "loader": {"DYLD_FALLBACK_LIBRARY_PATH": "/toolchain/lib"}, "cargo_environment": {"CARGO_HOME": "/temp/producer-cargo", "CARGO_REGISTRIES_CRATES_IO_PROTOCOL": "sparse", "CARGO_TERM_COLOR": "never", "CARGO_INCREMENTAL": "0"}}
        self.metadata = {"workspace_members": ["repo-conformance", "ordinary"], "packages": []}
        self.compiler = []
        self.inventories = {}
        for package_name, target_data in (("repo-conformance", [("lib", "repo_conformance", ["library_test"]), ("test", "conformance", ["conformance_bs_05", "conformance_bs_06", "conformance_bs_07", "future_case", "ignored_future"]), ("test", "bdd", ["honesty_test"])]), ("ordinary", [("lib", "ordinary", ["ordinary_test"]), ("test", "future_integration", [])])):
            package_root = self.root / package_name
            package_root.mkdir()
            package = {"id": package_name, "name": package_name, "version": "0.4.0", "manifest_path": str(package_root / "Cargo.toml"), "targets": [], "authors": ["Someone"]}
            for kind, name, names in target_data:
                target = {"name": name, "kind": [kind], "test": True, "doctest": kind == "lib"}
                package["targets"].append(target)
                binary = self.root / "target/debug/deps" / (name + "-abc123")
                binary.parent.mkdir(parents=True, exist_ok=True)
                binary.write_text("binary " + name)
                binary.chmod(0o755)
                self.compiler.append({"reason": "compiler-artifact", "package_id": package_name, "profile": {"test": True}, "target": target, "executable": str(binary)})
                self.inventories[str(binary)] = names
            self.metadata["packages"].append(package)
        targets, conformance, docs = factory.built_targets(self.metadata, self.build_output(), self.root)
        self.target = next(t for t in targets if t["id"] == conformance)
        self.records = []
        self.identity_checks(self.producer, self.records)
        self.add(self.producer, self.records, ["cargo", "metadata", "--locked", "--all-features", "--format-version", "1"], json.dumps(self.metadata), env=self.binding["cargo_environment"])
        self.add(self.producer, self.records, factory.BUILD, self.build_output(), env=self.binding["cargo_environment"])
        for target in targets:
            binary = target["executable"]
            target["file"] = factory.file_info(binary, Path(binary).relative_to(self.root).as_posix())
            target["inventory"] = sorted(self.inventories[binary])
            target["ignored"] = ["ignored_future"] if target["name"] == "conformance" else []
            env = {**target["env"], **self.binding["loader"], **self.binding["cargo_environment"]}
            self.add(self.producer, self.records, [binary, "--list"], listing(target["inventory"]), env=env, cwd=target["cwd"])
            self.add(self.producer, self.records, [binary, "--list", "--ignored"], listing(target["ignored"]), env=env, cwd=target["cwd"])
        filters = factory.lane_filters(self.bdd, self.lanes)
        inventories = {"lower": ["conformance_bs_07"], "print": ["conformance_bs_06"], "validate": ["conformance_bs_05"], "moves": ["future_case", "ignored_future"]}
        for lane, names in inventories.items():
            self.add(self.producer, self.records, [self.target["executable"], "--list", *filters[lane]], listing(names), env={**self.target["env"], **self.binding["loader"], **self.binding["cargo_environment"]}, cwd=self.target["cwd"])
        for flag in ("-L", "-l"):
            self.add(self.producer, self.records, ["otool", flag, self.target["executable"]], "binary:\n\t/usr/lib/libSystem.B.dylib (compatibility version 1.0.0, current version 1.0.0)\n")
        archives = {}
        for name, files in (("conformance.tar.gz", {"harness/" + Path(self.target["executable"]).name: self.target["executable"]}), ("dependencies.tar.gz", {})):
            members = factory.archive(self.producer / name, files)
            archives[name] = {"file": factory.file_info(self.producer / name, name), "members": members}
        self.construction = {"schema": 1, "binding": self.binding, "targets": targets, "conformance": conformance, "doc_targets": docs, "filters": filters, "lanes": inventories, "archives": archives, "checks": copy.deepcopy(self.records)}
        factory.write_json(self.producer / "construction.json", self.construction)
        self.receipts = {}
        invocations = []
        for target in targets:
            if target["id"] != conformance:
                invocations.append(self.add(self.producer, self.records, [target["executable"], "--format=pretty"], execution(target["inventory"], target["ignored"]), kind="harness", env={**target["env"], **self.binding["loader"], **self.binding["cargo_environment"]}, cwd=target["cwd"], expected=target["inventory"], ignored=target["ignored"]))
        invocations.append(self.add(self.producer, self.records, factory.DOCS, execution([])*len(docs), stderr="".join(f"   Doc-tests {doc}\n" for doc in docs), kind="doctests", env=self.binding["cargo_environment"], expected=docs))
        self.receipts["producer"] = {"schema": 1, "part": "producer", "job": "intel-construction", "binding": self.binding, "construction_sha256": factory.digest(self.producer / "construction.json"), "execution": {"attempt": "1", "artifact": "intel-construction-a1"}, "invocations": invocations}
        factory.write_json(self.producer / "producer.json", self.receipts["producer"])
        for lane, names in inventories.items():
            directory = self.lane_dir(lane)
            directory.mkdir()
            records = []
            self.identity_checks(directory, records)
            binary = "/temp/consumer/runtime/harness/" + Path(self.target["executable"]).name
            env = {**self.target["env"], **self.binding["loader"], "CARGO_HOME": "/temp/consumer/dependencies/cargo", "CARGO_NET_OFFLINE": "true", "CARGO_REGISTRIES_CRATES_IO_PROTOCOL": "sparse", "CARGO_TERM_COLOR": "never", "CARGO_INCREMENTAL": "0", "DYLD_LIBRARY_PATH": "/temp/consumer/runtime/dylibs"}
            ignored = sorted(set(names) & set(self.target["ignored"]))
            self.add(directory, records, [binary, "--list", *filters[lane]], listing(names), cwd=self.target["cwd"], env=env)
            self.add(directory, records, [binary, "--list", "--ignored", *filters[lane]], listing(ignored), cwd=self.target["cwd"], env=env)
            run = self.add(directory, records, [binary, "--format=pretty", *filters[lane]], execution(names, ignored), kind="harness", cwd=self.target["cwd"], env=env, expected=names, ignored=ignored)
            self.receipts[lane] = {"schema": 1, "part": lane, "job": "intel-part", "binding": self.binding, "construction_sha256": factory.digest(self.producer / "construction.json"), "execution": {"attempt": "1", "artifact": "intel-" + lane + "-a1"}, "checks": records[:-1], "invocations": [run]}
            factory.write_json(directory / "receipt.json", self.receipts[lane])
        self.lineage = self.history([{role: "success" for role in ("construction", *inventories)}])

    def build_output(self):
        return "".join(json.dumps(record) + "\n" for record in self.compiler)

    def identity_checks(self, directory, records):
        for command, output in ((["rustc", "-vV"], self.native["rustc"]), (["cargo", "-V"], self.native["cargo"]), (["rustc", "--print", "sysroot"], self.native["sysroot"] + "\n")):
            self.add(directory, records, command, output)

    def add(self, directory, records, command, stdout, stderr="", kind="setup", env=None, cwd=None, expected=None, ignored=None, exit_code=0):
        record = {"command": command, "cwd": cwd or str(self.root), "env": env or {}, "kind": kind, "exit_code": exit_code}
        if expected is not None:
            record.update(expected=expected, ignored=ignored or [])
        for channel, data in (("stdout", stdout), ("stderr", stderr)):
            path = f"logs/{len(records):04d}.{channel}.log"
            (directory / path).parent.mkdir(exist_ok=True)
            (directory / path).write_text(data)
            record[channel] = factory.file_info(directory / path, path)
        records.append(record)
        return record

    def lane_dir(self, lane, attempt=1):
        return self.artifacts / f"intel-{lane}-a{attempt}"

    HEAD = "c" * 40

    def history(self, attempts, retained=()):
        """The provider's whole records of `attempts`, a list of {role:
        conclusion}, one per attempt. A role in `retained[k]` is carried into
        attempt k + 2 with a new job ID and creation, as GitHub does."""
        names = {"construction": factory.CONSTRUCTION_JOB, **{lane: f"Intel conformance ({lane})" for lane in self.lanes["lanes"]}}
        run = {"id": 100, "run_attempt": len(attempts), "head_sha": self.HEAD, "path": factory.WORKFLOW, "workflow_id": 55, "name": "ci",
               "repository": {"full_name": "UOR-Foundation/lexlean", "id": 7}}
        pages, previous, made, number = [], {}, [], 900
        for index, conclusions in enumerate(attempts, 1):
            day = f"2026-01-{index:02d}"
            jobs = [self.job(number, "plan", index, "success", f"{day}T00:01:00Z", f"{day}T00:02:00Z")]
            number += 1
            for role, conclusion in conclusions.items():
                if index > 1 and role in (retained[index - 2] if index - 2 < len(retained) else ()):
                    job = {**copy.deepcopy(previous[role]), "id": number, "run_attempt": index, "created_at": f"{day}T00:00:30Z", "runner_group_id": None}
                else:
                    start, end = ("01:00", "02:00") if role == "construction" else ("02:10", "03:00")
                    job = self.job(number, names[role], index, conclusion, f"{day}T{start}:00Z", f"{day}T{end}:00Z")
                    made.append({"id": 5000 + number, "name": f"intel-{role}-a{index}", "digest": "sha256:" + "a" * 64, "expired": False,
                                 "size_in_bytes": 10, "created_at": job["completed_at"],
                                 "workflow_run": {"id": 100, "head_sha": self.HEAD, "repository_id": 7}})
                number += 1
                previous[role] = job
                jobs.append(job)
            header = {"id": 100, "run_attempt": index, "head_sha": self.HEAD, "workflow_id": 55, "path": factory.WORKFLOW, "run_started_at": f"{day}T00:00:00Z",
                      "repository": {"full_name": "UOR-Foundation/lexlean", "id": 7}}
            pages.append({"attempt": header, "jobs": self.pages(jobs, "jobs")})
        made.append({"id": 4000, "name": "acceptance-gate-evidence-rest", "digest": "sha256:" + "b" * 64, "expired": False, "size_in_bytes": 5,
                     "created_at": "2026-01-01T00:30:00Z", "workflow_run": {"id": 100, "head_sha": self.HEAD, "repository_id": 7}})
        return {"run": run, "attempts": pages, "artifacts": self.pages(made, "artifacts")}

    @staticmethod
    def job(identifier, name, attempt, conclusion, started, completed):
        # The artifact is created as its upload step completes (run 45553276269).
        steps = [{"number": 1, "name": "Set up job", "status": "completed", "conclusion": "success", "started_at": started, "completed_at": started},
                 {"number": 2, "name": factory.UPLOAD_STEP, "status": "completed", "conclusion": "success", "started_at": started, "completed_at": completed}]
        return {"id": identifier, "run_id": 100, "run_attempt": attempt, "head_sha": Fixture.HEAD, "workflow_name": "ci", "name": name,
                "status": "completed", "conclusion": conclusion, "started_at": started, "completed_at": completed, "created_at": started,
                "runner_id": 7000 + identifier, "runner_name": f"GitHub Actions {7000 + identifier}", "runner_group_id": 0, "labels": ["macos-15-intel"],
                "steps": steps}

    @staticmethod
    def pages(items, key, size=4):
        return [{"total_count": len(items), key: items[i:i + size]} for i in range(0, len(items), size)] or [{"total_count": 0, key: []}]

    def trusted(self, attempt):
        return {**self.binding["run"], "GITHUB_REPOSITORY": "UOR-Foundation/lexlean", "GITHUB_RUN_ATTEMPT": str(attempt), "head_sha": self.HEAD, "repository_id": 7}

    def save(self, part):
        path = self.producer / "producer.json" if part == "producer" else self.lane_dir(part, int(self.receipts[part]["execution"]["attempt"])) / "receipt.json"
        factory.write_json(path, self.receipts[part])

    def join(self, attempt="1", run=None, source=None, fetched=None):
        """Join what fetch would have left: the lineage and the selected
        artifacts' identities beside the artifact directories."""
        trusted = self.trusted(attempt)
        factory.write_json(self.fetched / "lineage.json", self.lineage)
        if fetched is None:
            try:
                selected = factory.resolve_lineage(self.lineage, self.lanes["lanes"], trusted)
                fetched = {item["artifact"]: {"id": item["id"], "digest": item["digest"], "size": item["size"]} for item in selected.values()}
            except factory.Invalid:
                fetched = {}
        factory.write_json(self.fetched / "fetched.json", fetched)
        identity = run or {**self.binding["run"], "GITHUB_RUN_ATTEMPT": attempt}
        with patch.object(factory, "run_identity", return_value=identity), patch.object(factory, "source_identity", return_value=source or self.binding["source"]), contextlib.redirect_stdout(io.StringIO()) as printed:
            result = factory.join(self.root, self.fetched, trusted)
        self.printed = printed.getvalue()
        return result

    def rerun_lane(self, lane, attempt=2):
        """Execute `lane` again in `attempt`: its receipt and artifact move to
        that attempt. Only selected artifacts are fetched, so the earlier one
        is explained by the lineage and absent here."""
        shutil.move(str(self.lane_dir(lane)), str(self.lane_dir(lane, attempt)))
        self.receipts[lane] = copy.deepcopy(self.receipts[lane])
        self.receipts[lane]["execution"] = {"attempt": str(attempt), "artifact": f"intel-{lane}-a{attempt}"}
        self.save(lane)


class FactoryTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.directory = Path(self.temp.name)

    def test_committed_lane_plan_is_complete_and_preserves_existing_parts(self):
        root = Path(__file__).resolve().parents[2]
        bdd, lanes = factory.plan(root)
        self.assertGreater(len(lanes["lanes"]), 0)
        self.assertEqual(set(lanes["producer"]), set(bdd["targets"]))

    def test_run_identity_names_this_repository_in_any_case(self):
        run = {"GITHUB_RUN_ID": "100", "GITHUB_RUN_ATTEMPT": "1"}
        for name in ("UOR-Foundation/LexLean", "UOR-Foundation/lexlean"):
            with patch.dict(os.environ, dict(run, GITHUB_REPOSITORY=name)):
                self.assertEqual(factory.run_identity()["GITHUB_REPOSITORY"], name)
        for name in ("someone/lexlean", "UOR-Foundation/lexlean-fork"):
            with patch.dict(os.environ, dict(run, GITHUB_REPOSITORY=name)):
                with self.assertRaisesRegex(factory.Invalid, "foreign repository"):
                    factory.run_identity()

    def test_plan_missing_duplicate_foreign_and_added_part_are_refused(self):
        fixture = Fixture(self.directory)
        for change in (lambda p: p["lanes"]["lower"].clear(), lambda p: p["lanes"]["print"].append("a"), lambda p: p["lanes"]["moves"].append("unknown")):
            with self.subTest(change=change):
                value = copy.deepcopy(fixture.lanes)
                change(value)
                factory.write_json(fixture.root / factory.LANES, value)
                with self.assertRaises(factory.Invalid):
                    factory.plan(fixture.root)
        factory.write_json(fixture.root / factory.LANES, fixture.lanes)
        fixture.bdd["parts"]["new-part"] = ["ZZ-01"]
        factory.write_json(fixture.root / factory.BDD, fixture.bdd)
        with self.assertRaisesRegex(factory.Invalid, "plan drift"):
            factory.plan(fixture.root)

    def test_list_parser_empty_duplicate_and_benchmark_semantics(self):
        self.assertEqual(factory.parse_list(listing([])), [])
        self.assertEqual(factory.parse_list("one: test\n\n1 test, 0 benchmarks\n"), ["one"])
        for text in ("", "x: test\nx: test\n2 tests, 0 benchmarks\n", "x: benchmark\n0 tests, 1 benchmark\n", "x: test\n0 tests, 0 benchmarks\n"):
            with self.subTest(text=text), self.assertRaises(factory.Invalid):
                factory.parse_list(text)

    def test_partition_rejects_empty_missing_duplicate_foreign_and_named_drift(self):
        bdd = {"parts": {"p": ["AA-01"]}}
        whole = ["conformance_aa_01", "future"]
        factory.partition(whole, {"p": ["conformance_aa_01"], "rest": ["future"]}, bdd)
        for original, selected in (([], {}), (whole, {"p": whole, "rest": []}), (whole, {"p": [whole[0]]}), (whole, {"p": whole, "rest": ["future"]}), (whole, {"p": whole, "rest": ["foreign"]}), (["future"], {"p": ["future"]})):
            with self.subTest(selected=selected), self.assertRaises(factory.Invalid):
                factory.partition(original, selected, bdd)

    def test_listing_is_not_execution_even_for_empty_target(self):
        factory.check_execution(execution([]), [], [])
        for names in ([], ["a"]):
            with self.assertRaises(factory.Invalid):
                factory.check_execution(listing(names), names, [])

    def test_ignored_cases_preserve_default_cargo_semantics(self):
        factory.check_execution(execution(["a", "b"], ["b"]), ["a", "b"], ["b"])
        for output in (execution(["a", "b"]), execution(["a", "b"], ["a", "b"]), execution(["a", "b"], ["b"], ["a"]), execution(["a"], [])):
            with self.subTest(output=output), self.assertRaises(factory.Invalid):
                factory.check_execution(output, ["a", "b"], ["b"])

    def test_archive_roundtrip_preserves_executable_mode(self):
        binary = self.directory / "binary"
        binary.write_bytes(b"binary")
        binary.chmod(0o755)
        path = self.directory / "payload.tar.gz"
        entries = factory.archive(path, {"harness/binary": binary})
        factory.inspect_archive(path, entries, self.directory / "extracted")
        self.assertEqual((self.directory / "extracted/harness/binary").read_bytes(), b"binary")
        self.assertEqual(stat.S_IMODE((self.directory / "extracted/harness/binary").stat().st_mode), 0o755)
        with self.assertRaises(factory.Invalid):
            factory.inspect_archive(path, entries, self.directory / "extracted")

    def test_archive_rejects_paths_links_devices_duplicates_and_extra_members(self):
        for name, kind in (("../escape", tarfile.REGTYPE), ("/absolute", tarfile.REGTYPE), ("harness/../escape", tarfile.REGTYPE), ("harness\\escape", tarfile.REGTYPE), ("harness/link", tarfile.SYMTYPE), ("harness/link", tarfile.LNKTYPE), ("harness/device", tarfile.CHRTYPE)):
            with self.subTest(name=name, kind=kind):
                path = self.directory / "bad.tar.gz"
                with tarfile.open(path, "w:gz") as tar:
                    member = tarfile.TarInfo(name)
                    member.type = kind
                    member.linkname = "/outside" if kind in (tarfile.SYMTYPE, tarfile.LNKTYPE) else ""
                    tar.addfile(member, io.BytesIO())
                entry = {"path": name, "size": 0, "mode": 0o644, "sha256": factory.hashlib.sha256(b"").hexdigest()}
                with self.assertRaises(factory.Invalid):
                    factory.inspect_archive(path, [entry])
        path = self.directory / "duplicate.tar.gz"
        with tarfile.open(path, "w:gz") as tar:
            for _ in range(2):
                tar.addfile(tarfile.TarInfo("same"), io.BytesIO())
        with self.assertRaises(factory.Invalid):
            factory.inspect_archive(path, [{"path": "same", "size": 0, "mode": 0o644, "sha256": "x"}])

    def test_archive_rejects_symlink_destination_and_digest_or_mode_changes(self):
        source = self.directory / "source"
        source.write_bytes(b"x")
        path = self.directory / "good.tar.gz"
        entries = factory.archive(path, {"harness/x": source})
        for key, value in (("sha256", "0"*64), ("mode", 0o777), ("size", 2)):
            altered = copy.deepcopy(entries)
            altered[0][key] = value
            with self.subTest(key=key), self.assertRaises(factory.Invalid):
                factory.inspect_archive(path, altered)
        (self.directory / "link").symlink_to(self.directory, target_is_directory=True)
        with self.assertRaises(factory.Invalid):
            factory.inspect_archive(path, entries, self.directory / "link/extract")

    def test_dependency_archive_never_includes_home_credentials_config_or_git(self):
        home = self.directory / "cargo-home"
        (home / "registry/src/index.crates.io-1234/package").mkdir(parents=True)
        (home / "registry/src/index.crates.io-1234/package/Cargo.toml").write_text("public package")
        (home / "credentials.toml").write_text("private")
        (home / "config.toml").write_text("private")
        files = factory.dependency_files(home)
        self.assertEqual(list(files), ["cargo/registry/src/index.crates.io-1234/package/Cargo.toml"])
        (home / "git/db/private").mkdir(parents=True)
        with self.assertRaisesRegex(factory.Invalid, "Git"):
            factory.dependency_files(home)

    def test_resolved_metadata_rejects_external_path_and_non_crates_io_sources(self):
        fixture = Fixture(self.directory)
        factory.dependency_sources(fixture.metadata, fixture.root)
        for source, manifest in ((None, "/outside/Cargo.toml"), (None, str(fixture.root / "target/generated/Cargo.toml")),
                                 ("git+https://example.com/repo", str(fixture.root / "git/Cargo.toml")),
                                 ("registry+https://private.example", str(fixture.root / "registry/Cargo.toml"))):
            metadata = copy.deepcopy(fixture.metadata)
            metadata["packages"].append({"name": "unsupported", "source": source, "manifest_path": manifest})
            with self.subTest(source=source, manifest=manifest), self.assertRaises(factory.Invalid):
                factory.dependency_sources(metadata, fixture.root)
        metadata = copy.deepcopy(fixture.metadata)
        metadata["packages"].append({"name": "public", "source": "registry+https://github.com/rust-lang/crates.io-index", "manifest_path": "/cargo-home/registry/src/public/Cargo.toml"})
        factory.dependency_sources(metadata, fixture.root)

    def test_dependency_symlink_alternate_registry_and_index_config_refused(self):
        home = self.directory / "home"
        (home / "registry/index/index.crates.io-1234").mkdir(parents=True)
        evil = home / "registry/index/index.crates.io-1234/config"
        evil.write_text("extraheader = private")
        with self.assertRaises(factory.Invalid):
            factory.dependency_files(home)
        evil.unlink()
        evil.symlink_to(self.directory)
        with self.assertRaises(factory.Invalid):
            factory.dependency_files(home)
        evil.unlink()
        (home / "registry/index/private.example-1234").mkdir()
        with self.assertRaises(factory.Invalid):
            factory.dependency_files(home)

    def test_complete_join_accepts_future_targets_and_ignored_tests_on_non_intel(self):
        fixture = Fixture(self.directory)
        self.assertEqual(fixture.join(), 0)
        self.assertIn("ordinary/test/future_integration", [t["id"] for t in fixture.construction["targets"]])

    def test_join_current_source_run_and_attempt_must_match_construction(self):
        fixture = Fixture(self.directory)
        for field, reason in (("commit", "source commit"), ("tree", "source commit"), ("run", "foreign run"), ("attempt", "the provider's run is not this run")):
            source, run = copy.deepcopy(fixture.binding["source"]), copy.deepcopy(fixture.binding["run"])
            if field in ("commit", "tree"):
                source[field] = "f" * 40
            elif field == "run":
                run["GITHUB_RUN_ID"] = "999"
            with self.subTest(field=field), self.assertRaisesRegex(factory.Invalid, reason):
                fixture.join("999" if field == "attempt" else "1", run=run, source=source)

    def test_join_missing_duplicate_and_unexpected_artifact_refused(self):
        for mode in ("missing", "duplicate", "extra"):
            base = self.directory / mode
            base.mkdir()
            fixture = Fixture(base)
            if mode == "missing":
                shutil.rmtree(fixture.lane_dir("lower"))
            elif mode == "duplicate":
                shutil.copytree(fixture.lane_dir("lower"), fixture.artifacts / "intel-lower-copy")
            else:
                (fixture.artifacts / "unrelated").write_text("unexpected")
            with self.subTest(mode=mode), self.assertRaises(factory.Invalid):
                fixture.join()

    def test_join_foreign_failed_cancelled_and_mismatched_receipts_refused(self):
        for mode in ("foreign", "failed", "cancelled", "digest", "job", "part"):
            base = self.directory / mode
            base.mkdir()
            fixture = Fixture(base)
            receipt = fixture.receipts["lower"]
            if mode == "foreign":
                receipt["binding"] = copy.deepcopy(receipt["binding"])
                receipt["binding"]["run"]["GITHUB_RUN_ATTEMPT"] = "2"
            elif mode in ("failed", "cancelled"):
                receipt["invocations"][0]["exit_code"] = 101 if mode == "failed" else -15
            elif mode == "digest":
                receipt["construction_sha256"] = "0"*64
            elif mode == "job":
                receipt["job"] = "someone-else"
            else:
                receipt["part"] = "moves"
            fixture.save("lower")
            with self.subTest(mode=mode), self.assertRaises(factory.Invalid):
                fixture.join()

    def test_join_changed_log_extra_file_missing_and_duplicate_invocations_refused(self):
        for mode in ("log", "file", "missing", "duplicate"):
            base = self.directory / mode
            base.mkdir()
            fixture = Fixture(base)
            record = fixture.receipts["lower"]["invocations"][0]
            if mode == "log":
                (fixture.lane_dir("lower") / record["stdout"]["path"]).write_text("forged")
            elif mode == "file":
                (fixture.lane_dir("lower") / "other.txt").write_text("unexpected")
            elif mode == "missing":
                fixture.receipts["lower"]["invocations"].clear()
            else:
                fixture.receipts["lower"]["invocations"].append(record)
            fixture.save("lower")
            with self.subTest(mode=mode), self.assertRaises(factory.Invalid):
                fixture.join()

    def test_join_rejects_resealed_listing_as_execution_and_changed_environment(self):
        for mode in ("listing", "RUSTFLAGS", "RUSTC_WRAPPER", "sparse", "CARGO_INCREMENTAL"):
            base = self.directory / mode
            base.mkdir()
            fixture = Fixture(base)
            record = fixture.receipts["lower"]["invocations"][0]
            if mode == "listing":
                path = fixture.lane_dir("lower") / record["stdout"]["path"]
                path.write_text(listing(record["expected"]))
                record["stdout"] = factory.file_info(path, record["stdout"]["path"])
            elif mode == "sparse":
                record["env"]["CARGO_REGISTRIES_CRATES_IO_PROTOCOL"] = "git"
            else:
                record["env"][mode] = "other"
            fixture.save("lower")
            with self.subTest(mode=mode), self.assertRaises(factory.Invalid):
                fixture.join()

    def test_actual_cargo_metadata_discovers_new_target_and_missing_binary_fails(self):
        fixture = Fixture(self.directory)
        target = {"name": "another_future", "kind": ["test"], "test": True, "doctest": False}
        fixture.metadata["packages"][0]["targets"].append(target)
        with self.assertRaisesRegex(factory.Invalid, "not constructed"):
            factory.built_targets(fixture.metadata, fixture.build_output(), fixture.root)

    def test_ordinary_failure_does_not_hide_later_target_and_doctest_records(self):
        fixture = Fixture(self.directory)
        receipt = fixture.receipts["producer"]
        receipt["invocations"][0]["exit_code"] = 101
        fixture.save("producer")
        with self.assertRaisesRegex(factory.Invalid, "producer: failed invocation"):
            fixture.join()
        self.assertGreater(len(receipt["invocations"]), 2)
        self.assertEqual(receipt["invocations"][-1]["kind"], "doctests")

    def test_should_panic_annotation_is_execution_of_original_name(self):
        factory.check_execution("test expects_panic - should panic ... ok\n" +
                                "test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s\n",
                                ["expects_panic"], [])

    def test_tampered_construction_list_logs_and_missing_checks_are_refused(self):
        for mode in ("missing", "list"):
            base = self.directory / mode
            base.mkdir()
            fixture = Fixture(base)
            if mode == "missing":
                fixture.construction["checks"].pop()
            else:
                record = next(r for r in fixture.construction["checks"] if r["command"] == [fixture.target["executable"], "--list"])
                path = fixture.producer / record["stdout"]["path"]
                path.write_text(listing(["foreign_case"]))
                record["stdout"] = factory.file_info(path, record["stdout"]["path"])
            factory.write_json(fixture.producer / "construction.json", fixture.construction)
            for part, receipt in fixture.receipts.items():
                receipt["construction_sha256"] = factory.digest(fixture.producer / "construction.json")
                fixture.save(part)
            with self.subTest(mode=mode), self.assertRaises(factory.Invalid):
                fixture.join()

    def test_native_identity_rejects_target_specific_compiler_overrides(self):
        fixture = Fixture(self.directory)
        for name in ("RUSTFLAGS", "CARGO_TARGET_X86_64_APPLE_DARWIN_RUSTFLAGS", "CARGO_BUILD_RUSTFLAGS", "RUSTC_WORKSPACE_WRAPPER"):
            with patch.dict(os.environ, {name: "unexpected"}), patch.object(factory.platform, "system", return_value="Darwin"), patch.object(factory.platform, "machine", return_value="x86_64"):
                with self.subTest(name=name), self.assertRaisesRegex(factory.Invalid, "overrides"):
                    factory.native_identity(fixture.root, factory.Recorder(self.directory / name))

    def test_real_fake_executables_construct_execute_and_join_with_producer_failure(self):
        # Exercise the actual orchestration, subprocess capture, tar transport,
        # consumer extraction and join. Only Rust/macOS tools and git identity
        # are substituted; no manually authored successful execution receipts.
        for fail in (False, True):
            base = self.directory / ("failure" if fail else "success")
            base.mkdir()
            fixture = Fixture(base)
            shutil.rmtree(fixture.artifacts)
            fixture.artifacts.mkdir()
            tools = base / "tools"
            tools.mkdir()
            def program(name, code):
                path = tools / name
                path.write_text("#!" + sys.executable + "\n" + code)
                path.chmod(0o755)
                return path
            cargo = "import json,sys\nargs=sys.argv[1:]\n"
            cargo += "if args[0] == '-V': print('cargo 1.97.1')\n"
            cargo += "elif args[0] == 'metadata': print(" + repr(json.dumps(fixture.metadata)) + ")\n"
            cargo += "elif '--no-run' in args: print(" + repr(fixture.build_output()) + ", end='')\n"
            docs = ["ordinary", "repo_conformance"]
            cargo += "elif '--doc' in args:\n"
            cargo += " for target in " + repr(docs) + ":\n  print('   Doc-tests ' + target, file=sys.stderr)\n  print(" + repr(execution([])) + ", end='')\n"
            cargo += "else: sys.exit(77)\n"
            program("cargo", cargo)
            program("rustc", "import sys\nprint(" + repr(fixture.native["rustc"].strip()) + " if '-vV' in sys.argv else '/toolchain')\n")
            program("otool", "import sys\nprint(sys.argv[-1] + ':\\n\\t/usr/lib/libSystem.B.dylib (compatibility version 1.0.0, current version 1.0.0)')\n")
            for binary, names in fixture.inventories.items():
                ignored = ["ignored_future"] if "ignored_future" in names else []
                failed = ["ordinary_test"] if fail and "ordinary_test" in names else []
                code = "import sys\nnames=" + repr(names) + "\nignored=" + repr(ignored) + "\nfailed=" + repr(failed) + "\n"
                code += "args=iter(sys.argv[1:]); positive=[]; skips=[]; only_ignored=False; is_list=False\n"
                code += "for arg in args:\n if arg == '--skip': skips.append(next(args))\n elif arg == '--ignored': only_ignored=True\n elif arg == '--list': is_list=True\n elif not arg.startswith('--'): positive.append(arg)\n"
                code += "selected=sorted(n for n in names if (not positive or n in positive) and n not in skips and (not only_ignored or n in ignored))\n"
                code += "if is_list:\n for name in selected: print(name + ': test')\n print(str(len(selected)) + ' tests, 0 benchmarks')\n"
                code += "else:\n for name in selected: print('test ' + name + ' ... ' + ('FAILED' if name in failed else 'ignored' if name in ignored else 'ok'))\n nfailed=len(set(selected)&set(failed)); nignored=len(set(selected)&set(ignored))\n print('test result: ' + ('FAILED' if nfailed else 'ok') + '. ' + str(len(selected)-nfailed-nignored) + ' passed; ' + str(nfailed) + ' failed; ' + str(nignored) + ' ignored; 0 measured; 0 filtered out; finished in 0.01s')\n sys.exit(1 if nfailed else 0)\n"
                Path(binary).write_text("#!" + sys.executable + "\n" + code)
                Path(binary).chmod(0o755)
            output_file = base / "outputs"
            env = {**fixture.binding["run"], "GITHUB_JOB": "intel-construction", "GITHUB_OUTPUT": str(output_file), "PATH": str(tools) + os.pathsep + os.environ["PATH"]}
            with patch.dict(os.environ, env), patch.object(factory, "source_identity", return_value=fixture.binding["source"]), patch.object(factory.platform, "system", return_value="Darwin"), patch.object(factory.platform, "machine", return_value="x86_64"):
                self.assertEqual(factory.build(fixture.root, fixture.producer), 0)
                outputs = dict(line.split("=", 1) for line in output_file.read_text().splitlines())
                lanes = json.loads(outputs["lanes"])
                self.assertEqual(set(lanes), set(fixture.lanes["lanes"]))
                with patch.dict(os.environ, {"GITHUB_JOB": "intel-part"}):
                    for lane in lanes:
                        self.assertEqual(factory.execute(fixture.root, fixture.producer, lane, fixture.lane_dir(lane), "intel-construction-a1"), 0)
                with patch.dict(os.environ, {"GITHUB_JOB": "intel-join"}):
                    if fail:
                        with self.assertRaisesRegex(factory.Invalid, "producer: failed invocation"):
                            fixture.join()
                    else:
                        self.assertEqual(fixture.join(), 0)
                self.assertEqual(outputs["artifact"], "intel-construction-a1")

    LANES = ("construction", "lower", "print", "validate", "moves")

    def outcomes(self, **changed):
        return {role: changed.get(role, "success") for role in self.LANES}

    def rerun_lower(self, fixture, lower="success"):
        """Attempt 1 loses `lower`; attempt 2 reruns it alone."""
        fixture.rerun_lane("lower")
        return fixture.history([self.outcomes(lower="cancelled"), self.outcomes(lower=lower)],
                               retained=[[r for r in self.LANES if r != "lower"]])

    @staticmethod
    def items(lineage, attempt=None):
        if attempt is None:
            return [a for page in lineage["artifacts"] for a in page["artifacts"]]
        return [j for page in lineage["attempts"][attempt - 1]["jobs"] for j in page["jobs"]]

    def test_a_lane_rerun_alone_is_joined_with_the_executions_it_retained(self):
        fixture = Fixture(self.directory)
        fixture.lineage = self.rerun_lower(fixture)
        self.assertEqual(fixture.join("2"), 0)
        self.assertIn("lower a2 intel-lower-a2", fixture.printed)
        self.assertIn("retained from earlier attempts: construction, moves, print, validate", fixture.printed)

    def test_lineage_selects_each_role_s_latest_execution(self):
        fixture = Fixture(self.directory)
        # Attempt 2 reruns lower and print, and print fails again; attempt 3
        # reruns print alone, so lower is retained twice from attempt 2.
        lineage = fixture.history([self.outcomes(lower="cancelled"), self.outcomes(print="failure"), self.outcomes()],
                                  retained=[["construction", "validate", "moves"], [r for r in self.LANES if r != "print"]])
        selected = factory.resolve_lineage(lineage, fixture.lanes["lanes"], fixture.trusted(3))
        self.assertEqual({role: item["attempt"] for role, item in selected.items()},
                         {"construction": 1, "lower": 2, "print": 3, "validate": 1, "moves": 1})

    def test_a_superseded_expired_artifact_is_explained_not_required(self):
        fixture = Fixture(self.directory)
        lineage = self.rerun_lower(fixture)
        next(a for a in self.items(lineage) if a["name"] == "intel-lower-a1")["expired"] = True
        fixture.lineage = lineage
        self.assertEqual(fixture.join("2"), 0)

    def test_malformed_provider_histories_are_refused_by_their_rule(self):
        # The six histories the first implementation accepted (review of #628),
        # then pagination, identity, interval and origin defects. Each names
        # its rule, so a later check cannot make it pass.
        cases = {
            "duplicate-artifact-id": ("missing, null or duplicate artifact ID",
                                      lambda l: next(a for a in self.items(l) if a["name"] == "intel-lower-a2").update(
                                          id=next(a for a in self.items(l) if a["name"] == "intel-construction-a1")["id"])),
            "foreign-job-run": ("a job of another run", lambda l: next(j for j in self.items(l, 2) if j["name"] == "Intel conformance (lower)").update(run_id=101)),
            "foreign-job-head": ("a job of another run", lambda l: next(j for j in self.items(l, 2) if j["name"] == "Intel conformance (lower)").update(head_sha="d" * 40)),
            "carried-runner-changed": ("a retained execution whose record changed",
                                       lambda l: next(j for j in self.items(l, 2) if j["name"] == factory.CONSTRUCTION_JOB).update(runner_name="another")),
            "carried-step-changed": ("a retained execution whose record changed",
                                     lambda l: next(j for j in self.items(l, 2) if j["name"] == "Intel conformance (print)")["steps"][1].update(conclusion="failure")),
            "missing-job-row": ("truncated attempt 2 job listing", lambda l: l["attempts"][1]["jobs"][0]["jobs"].pop(0)),
            "null-job-id": ("missing, null or duplicate attempt 2 job ID", lambda l: self.items(l, 2)[0].update(id=None)),
            "null-artifact-id": ("missing, null or duplicate artifact ID",
                                 lambda l: next(a for a in self.items(l) if a["name"] == "intel-lower-a2").update(id=None)),
            "page-totals-differ": ("inconsistent artifact pagination", lambda l: l["artifacts"][0].update(total_count=99)),
            "extra-page-item": ("truncated artifact listing", lambda l: l["artifacts"][-1]["artifacts"].append({"id": 1, "name": "other"})),
            "run-head": ("the provider's run is not this run", lambda l: l["run"].update(head_sha="d" * 40)),
            "run-workflow": ("the provider's run is not this run", lambda l: l["run"].update(path=".github/workflows/other.yml")),
            "attempt-head": ("attempt 2 is of another run", lambda l: l["attempts"][1]["attempt"].update(head_sha="d" * 40)),
            "missing-attempt": ("missing or duplicate attempt", lambda l: l["attempts"].pop(0)),
            "zero-length": ("an execution of no duration",
                            lambda l: next(j for j in self.items(l, 2) if j["name"] == "Intel conformance (lower)").update(started_at="2026-01-02T03:00:00Z")),
            "unparsed-time": ("not a provider UTC time",
                              lambda l: next(j for j in self.items(l, 2) if j["name"] == "Intel conformance (lower)").update(started_at="2026-01-02 02:10:00")),
            "same-second-origin": ("neither retained nor new",
                                   lambda l: next(j for j in self.items(l, 2) if j["name"] == "Intel conformance (lower)").update(started_at="2026-01-01T03:00:00Z")),
            "before-attempt": ("neither retained nor new",
                               lambda l: next(j for j in self.items(l, 2) if j["name"] == "Intel conformance (lower)").update(started_at="2026-01-01T23:00:00Z")),
            "outside-upload": ("made outside its upload step",
                               lambda l: next(a for a in self.items(l) if a["name"] == "intel-lower-a2").update(created_at="2026-01-02T03:00:01Z")),
            "upload-failed": ("an unfinished, expired or undigested accepted artifact",
                              lambda l: next(j for j in self.items(l, 2) if j["name"] == "Intel conformance (lower)")["steps"][1].update(conclusion="failure")),
            "artifact-repository": ("of another run or repository",
                                    lambda l: next(a for a in self.items(l) if a["name"] == "intel-lower-a2")["workflow_run"].update(repository_id=8)),
            "unsuffixed": ("not named by its execution", lambda l: next(a for a in self.items(l) if a["name"] == "intel-lower-a1").update(name="intel-lower")),
            "unmade": ("no execution of this run made", lambda l: next(a for a in self.items(l) if a["name"] == "intel-lower-a1").update(name="intel-print-a2")),
            "duplicate-name": ("an artifact name listed twice", lambda l: next(a for a in self.items(l) if a["name"] == "intel-lower-a1").update(name="intel-lower-a2")),
            "expired-selection": ("an unfinished, expired or undigested accepted artifact",
                                  lambda l: next(a for a in self.items(l) if a["name"] == "intel-lower-a2").update(expired=True)),
            "undigested-selection": ("an unfinished, expired or undigested accepted artifact",
                                     lambda l: next(a for a in self.items(l) if a["name"] == "intel-lower-a2").update(digest=None)),
            # The second review of #628: four more accepted histories.
            "upload-after-job": ("a step outside its job", lambda l: self.upload_after_job(l)),
            "upload-in-progress": ("an unfinished or unnamed step", lambda l: self.lower2(l)["steps"][1].update(status="in_progress")),
            "anonymous-runner": ("did not run on an identified macos-15-intel runner", lambda l: self.lower2(l).update(runner_id=None, runner_name="", labels=[])),
            "other-label": ("did not run on an identified macos-15-intel runner", lambda l: self.lower2(l).update(labels=["macos-15"])),
            "attempt-repository": ("attempt 2 is of another run, head, workflow or repository",
                                   lambda l: l["attempts"][1]["attempt"]["repository"].update(full_name="someone/else", id=8)),
            "run-repository-id": ("the provider's run is not this run", lambda l: l["run"]["repository"].update(id=8)),
            "steps-renumbered": ("steps out of order or renumbered", lambda l: self.lower2(l)["steps"].reverse()),
            "step-outside-job": ("a step outside its job", lambda l: self.lower2(l)["steps"][0].update(started_at="2026-01-02T02:00:00Z")),
            "skipped-history": ("was skipped or never started; recovery from a nonexecution is not supported",
                                lambda l: next(j for j in self.items(l, 1) if j["name"] == "Intel conformance (lower)").update(conclusion="skipped", started_at=None, completed_at=None)),
            "never-started-history": ("was skipped or never started; recovery from a nonexecution is not supported",
                                      lambda l: next(j for j in self.items(l, 1) if j["name"] == "Intel conformance (lower)").update(started_at=None)),
        }
        for mode, (reason, damage) in cases.items():
            base = self.directory / mode
            base.mkdir()
            fixture = Fixture(base)
            lineage = self.rerun_lower(fixture)
            damage(lineage)
            with self.subTest(mode=mode), self.assertRaisesRegex(factory.Invalid, reason):
                factory.resolve_lineage(lineage, fixture.lanes["lanes"], fixture.trusted(2))

    def lower2(self, lineage):
        return next(j for j in self.items(lineage, 2) if j["name"] == "Intel conformance (lower)")

    def upload_after_job(self, lineage):
        """The upload and the artifact both after the job ended, consistent
        with each other and with nothing else."""
        job = self.lower2(lineage)
        job["steps"][1].update(started_at="2026-01-02T03:01:00Z", completed_at="2026-01-02T03:02:00Z")
        next(a for a in self.items(lineage) if a["name"] == "intel-lower-a2")["created_at"] = "2026-01-02T03:02:00Z"

    def test_the_provider_s_real_records_pass_every_execution_check(self):
        """Run 45553276269 attempt 1 as the provider served it (each record's
        fields narrowed, pages and totals whole). Its artifacts predate the
        attempt-suffixed names, so resolution must pass every run, attempt,
        job, step and runner check on real records and then stop at the
        first artifact name, and nowhere earlier."""
        recorded = json.loads(Path(__file__).with_name("intel_lineage_45553276269.json").read_text())
        with self.assertRaisesRegex(factory.Invalid, "an Intel artifact not named by its execution: intel-"):
            factory.resolve_lineage(recorded["lineage"], recorded["lanes"], recorded["trusted"])

    def test_a_replaced_construction_invalidates_every_earlier_lane(self):
        fixture = Fixture(self.directory)
        fixture.rerun_lane("lower")
        # The construction ran again in attempt 2; `print` kept its attempt-1 run.
        lineage = fixture.history([self.outcomes(lower="failure"), self.outcomes()], retained=[["print", "validate", "moves"]])
        with self.assertRaisesRegex(factory.Invalid, "lane print began before its construction finished"):
            factory.resolve_lineage(lineage, fixture.lanes["lanes"], fixture.trusted(2))

    def test_a_lane_that_overlaps_its_construction_is_refused_in_one_attempt(self):
        # The attempt numbers agree; only the times show the lane did not run
        # against the construction it is joined with.
        fixture = Fixture(self.directory)
        lineage = fixture.history([self.outcomes()])
        lane = next(j for j in self.items(lineage, 1) if j["name"] == "Intel conformance (lower)")
        lane["started_at"] = lane["steps"][0]["started_at"] = lane["steps"][0]["completed_at"] = lane["steps"][1]["started_at"] = "2026-01-01T01:30:00Z"
        with self.assertRaisesRegex(factory.Invalid, "lane lower began before its construction finished"):
            factory.resolve_lineage(lineage, fixture.lanes["lanes"], fixture.trusted(1))

    def test_a_retained_job_is_known_by_its_execution_not_its_id(self):
        fixture = Fixture(self.directory)
        lineage = self.rerun_lower(fixture)
        first = next(j for j in self.items(lineage, 1) if j["name"] == factory.CONSTRUCTION_JOB)
        next(j for j in self.items(lineage, 2) if j["name"] == factory.CONSTRUCTION_JOB)["id"] = first["id"]
        with self.assertRaisesRegex(factory.Invalid, "a job ID listed twice"):
            factory.resolve_lineage(lineage, fixture.lanes["lanes"], fixture.trusted(2))

    def test_join_refuses_artifacts_that_are_not_the_fetched_selection(self):
        for mode in ("substituted-id", "missing-entry", "extra-directory", "stale-receipt"):
            base = self.directory / mode
            base.mkdir()
            fixture = Fixture(base)
            fixture.lineage = self.rerun_lower(fixture)
            selected = factory.resolve_lineage(fixture.lineage, fixture.lanes["lanes"], fixture.trusted(2))
            fetched = {item["artifact"]: {"id": item["id"], "digest": item["digest"], "size": item["size"]} for item in selected.values()}
            reason = "the fetched artifacts are not the accepted executions"
            if mode == "substituted-id":
                fetched["intel-lower-a2"]["id"] += 1
            elif mode == "missing-entry":
                del fetched["intel-print-a1"]
            elif mode == "extra-directory":
                shutil.copytree(fixture.lane_dir("print"), fixture.artifacts / "intel-print-a9")
                reason = "missing, duplicate or unexpected Intel artifact directories"
            else:
                fixture.receipts["lower"]["execution"] = {"attempt": "1", "artifact": "intel-lower-a1"}
                factory.write_json(fixture.lane_dir("lower", 2) / "receipt.json", fixture.receipts["lower"])
                reason = "foreign or mismatched receipt: lower"
            with self.subTest(mode=mode), self.assertRaisesRegex(factory.Invalid, reason):
                fixture.join("2", fetched=fetched)

    def zipped(self, directory, extra=None):
        buffer = io.BytesIO()
        with zipfile.ZipFile(buffer, "w") as bundle:
            for path in sorted(p for p in directory.rglob("*") if p.is_file()):
                bundle.write(path, path.relative_to(directory).as_posix())
            for name, info in (extra or {}).items():
                bundle.writestr(info, name)
        return buffer.getvalue()

    def test_fetch_takes_exactly_the_selected_bytes_by_id(self):
        cases = {"accepted": None, "substituted-bytes": "the fetched bytes are not the accepted artifact",
                 "short-bytes": "the fetched bytes are not the accepted artifact", "link-member": "non-regular archive member",
                 "escape-member": "unsafe artifact path"}
        for mode, reason in cases.items():
            base = self.directory / mode
            base.mkdir()
            fixture = Fixture(base)
            lineage = self.rerun_lower(fixture)
            archives, requested = {}, []
            for item in self.items(lineage):
                directory = fixture.artifacts / item["name"]
                if directory.is_dir():
                    extra = None
                    if item["name"] == "intel-lower-a2" and mode == "link-member":
                        link = zipfile.ZipInfo("logs/link")
                        link.external_attr = (stat.S_IFLNK | 0o777) << 16
                        extra = {"target": link}
                    elif item["name"] == "intel-lower-a2" and mode == "escape-member":
                        extra = {"x": zipfile.ZipInfo("../escape")}
                    data = self.zipped(directory, extra)
                    archives[item["id"]] = data
                    item.update(size_in_bytes=len(data), digest="sha256:" + factory.hashlib.sha256(data).hexdigest())
            lower = next(a for a in self.items(lineage) if a["name"] == "intel-lower-a2")["id"]
            if mode == "substituted-bytes":
                archives[lower] = archives[next(a for a in self.items(lineage) if a["name"] == "intel-print-a1")["id"]]
            elif mode == "short-bytes":
                archives[lower] = archives[lower][:-1]

            def read(path, binary=False):
                requested.append(path)
                return archives[int(path.split("/")[-2])]
            output = base / "fetched-out"
            with self.subTest(mode=mode):
                if reason is None:
                    factory.fetch(fixture.root, output, fixture.trusted(2), collect=lambda trusted: lineage, read=read)
                    self.assertEqual(sorted(p.name for p in (output / "artifacts").iterdir()), sorted(f"intel-{r}-a{2 if r == 'lower' else 1}" for r in self.LANES))
                    self.assertNotIn(f"/artifacts/{next(a for a in self.items(lineage) if a['name'] == 'intel-lower-a1')['id']}/zip", requested,
                                     "a superseded artifact is never fetched")
                    self.assertEqual((output / "artifacts/intel-lower-a2/receipt.json").read_bytes(), (fixture.lane_dir("lower", 2) / "receipt.json").read_bytes())
                else:
                    with self.assertRaisesRegex(factory.Invalid, reason):
                        factory.fetch(fixture.root, output, fixture.trusted(2), collect=lambda trusted: lineage, read=read)

    def test_a_consumer_takes_only_the_construction_it_was_handed(self):
        fixture = Fixture(self.directory)
        later = {**fixture.binding["run"], "GITHUB_RUN_ATTEMPT": "2"}
        with patch.object(factory, "run_identity", return_value=later), patch.object(factory, "source_identity", return_value=fixture.binding["source"]):
            factory.load_construction(fixture.root, fixture.producer, "1")
            for attempt in ("2", "3"):
                with self.subTest(attempt=attempt), self.assertRaisesRegex(factory.Invalid, "foreign run, repository, attempt"):
                    factory.load_construction(fixture.root, fixture.producer, attempt)
        with patch.dict(os.environ, {"GITHUB_JOB": "intel-part"}):
            for name in ("intel-construction", "intel-lower-a1", "intel-construction-a0"):
                with self.subTest(name=name), self.assertRaisesRegex(factory.Invalid, "not a construction artifact"):
                    factory.execute(fixture.root, fixture.producer, "lower", self.directory / name, name)

    def test_recorder_captures_real_process_failures_and_later_success(self):
        recorder = factory.Recorder(self.directory / "recorded")
        failure = recorder.run([sys.executable, "-c", "import sys; print('failed stdout'); print('failed stderr', file=sys.stderr); sys.exit(3)"], self.directory)
        success = recorder.run([sys.executable, "-c", "print('later success')"], self.directory)
        self.assertEqual((failure["exit_code"], success["exit_code"]), (3, 0))
        self.assertIn("failed stderr", recorder.output_text(failure, "stderr"))
        factory.check_records(recorder.output, recorder.records)


if __name__ == "__main__":
    unittest.main()
