//! The `cli-api` suite, `check` on WASI: CL-25 (SPEC.md §8.3), and `declare`
//! natively and on WASI: CL-29 (§23.4).
//!
//! The executable is built for `wasm32-wasip1` from this tree and run by
//! Node's `node:wasi`, a WASI host over the real file system, so a project
//! is read through the same paths, confinement checks, and file identities
//! as on the native host. The native side is `lexlean::cli::run`, the
//! function `main` calls, as the fixture runner runs it (§28.2).

use std::process::Command;

use camino::{Utf8Path, Utf8PathBuf};

use crate::fixtures;
use crate::support::{self, P};

const TARGET: &str = "wasm32-wasip1";

/// The WASI host: the module, the directory it sees as `/`, then the
/// arguments after `lexlean`. The project is the root of the instance's file
/// system because a WASI program starts in `/` and LexLean looks for
/// `lexlean.toml` from where it starts (§23.2). The environment is empty, as
/// it is for a native run that sets nothing (§25.4).
const HOST: &str = r#"import { readFile } from "node:fs/promises";
import { WASI } from "node:wasi";
const [module, root, ...args] = process.argv.slice(2);
const wasi = new WASI({
  version: "preview1",
  args: ["lexlean", ...args],
  env: {},
  preopens: { "/": root },
  returnOnExit: true,
});
const instance = await WebAssembly.instantiate(
  await WebAssembly.compile(await readFile(module)),
  wasi.getImportObject(),
);
process.exitCode = wasi.start(instance);
"#;

/// What one run produced: the exit code, stdout, and stderr.
type Run = (i32, String, String);

/// Is there a WASI host and the pinned toolchain's standard library for the
/// target? On the normative host both must be present (`rust-toolchain.toml`
/// names the target, and CI installs Node's `node:wasi` with the runner); on
/// any other host the case says it did not run, never passes vacuously.
fn wasi_backed(id: &str) -> bool {
    let node = Command::new("node")
        .args(["--input-type=module", "-e", "await import('node:wasi')"])
        .output()
        .is_ok_and(|output| output.status.success());
    let target = Command::new("rustc")
        .args(["--print", "target-libdir", "--target", TARGET])
        .current_dir(support::repo_root().as_std_path())
        .output()
        .ok()
        .filter(|output| output.status.success())
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .is_some_and(|dir| Utf8Path::new(dir.trim()).is_dir());
    if node && target {
        return true;
    }
    assert!(
        !support::normative_host(),
        "{id}: the normative host (Linux x86-64) must have `node` with `node:wasi` (found: {node}) and the pinned toolchain's {TARGET} standard library (found: {target}); this case never passes vacuously here (§8.3)"
    );
    eprintln!(
        "{id}: no WASI host or no {TARGET} standard library on this host; nothing was run (§8.3)"
    );
    false
}

/// Build the executable for the target from this tree. It is the release
/// profile the executable ships with, in the workspace's target directory,
/// where nothing else builds for this target.
fn build() -> Utf8PathBuf {
    let target_dir = std::env::var("CARGO_TARGET_DIR")
        .map_or_else(|_| support::repo_root().join("target"), Utf8PathBuf::from);
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned());
    let output = Command::new(cargo)
        .args([
            "build",
            "--locked",
            "--release",
            "--target",
            TARGET,
            "-p",
            "lexlean",
            "--bin",
            "lexlean",
        ])
        .current_dir(support::repo_root().as_std_path())
        .output()
        .expect("cargo runs");
    assert!(
        output.status.success(),
        "CL-25: lexlean does not build for {TARGET}:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let module = target_dir.join(TARGET).join("release/lexlean.wasm");
    assert!(module.is_file(), "CL-25: {module} was built");
    module
}

/// Run the module on `root` under the WASI host.
fn wasi(module: &Utf8Path, host: &Utf8Path, root: &Utf8Path, arguments: &[&str]) -> Run {
    // Node 22 collects garbage inside its fast call of WASI's
    // `path_filestat_get` and dies with SIGSEGV on a language-1.2 project;
    // the ordinary call path does not.
    let output = Command::new("node")
        .args(["--no-warnings", "--no-turbo-fast-api-calls"])
        .arg(host.as_str())
        .arg(module.as_str())
        .arg(root.as_str())
        .args(arguments)
        .output()
        .expect("node runs");
    (
        output.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&output.stdout).into_owned(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
    )
}

/// Run the executable natively on `root`, holding the environment lock as
/// the fixture runner does.
fn native(root: &Utf8Path, arguments: &[&str]) -> Run {
    let _guard = support::env_shared();
    support::cli_in(root, arguments)
}

/// A run with the project's own location written as `$PROJECT`: the native
/// run sees its temporary directory, the instance sees `/`.
fn normalized((exit, stdout, stderr): Run, root: &str) -> Run {
    let root = root.trim_end_matches('/');
    let replace = |text: &str| {
        if root.is_empty() {
            text.replace("\"/", "\"$PROJECT/")
        } else {
            text.replace(root, "$PROJECT")
        }
    };
    (exit, replace(&stdout), replace(&stderr))
}

/// A copy of `source` for one side of a comparison, symlinks kept.
fn copy(source: &Utf8Path) -> P {
    let project = P::empty();
    fixtures::copy_tree_preserving_links(source.as_std_path(), project.root.as_std_path())
        .expect("the project copies");
    for generated in [".lexlean", "expected"] {
        let path = project.root.join(generated);
        if path.is_dir() {
            std::fs::remove_dir_all(path.as_std_path()).expect("remove a generated directory");
        }
    }
    project
}

/// One project to check on both hosts: its source and the `check` arguments.
struct Subject {
    name: String,
    source: Utf8PathBuf,
    args: Vec<String>,
}

/// Every single-step `check` fixture that needs no toolchain, overlay, or
/// environment of its own, and every example project.
fn subjects() -> Vec<Subject> {
    let root = support::repo_root();
    let mut subjects = Vec::new();
    for dir in fixtures::discover(&root) {
        let case = fixtures::load_case(&dir).expect("a fixture loads");
        let [invocation] = case.invocations.as_slice() else {
            continue;
        };
        if invocation.command != "check"
            || !case.environment.is_empty()
            || dir.join("toolchain").exists()
            || dir.join("overlay").exists()
        {
            continue;
        }
        subjects.push(Subject {
            name: dir.strip_prefix(&root).unwrap_or(&dir).to_string(),
            source: dir.join("project"),
            args: invocation.args.clone(),
        });
    }
    let fixtures = subjects.len();
    assert!(
        fixtures >= 100,
        "CL-25: {fixtures} check fixtures were found"
    );
    let mut examples: Vec<Utf8PathBuf> = std::fs::read_dir(root.join("examples").as_std_path())
        .expect("examples")
        .flatten()
        .filter(|entry| entry.path().join("lexlean.toml").is_file())
        .filter_map(|entry| Utf8PathBuf::from_path_buf(entry.path()).ok())
        .collect();
    examples.sort();
    for source in examples {
        subjects.push(Subject {
            name: source.strip_prefix(&root).unwrap_or(&source).to_string(),
            source,
            args: Vec::new(),
        });
    }
    assert!(subjects.len() > fixtures, "CL-25: the examples were found");
    subjects
}

/// Check `subject` on both hosts and report any difference.
fn compare(module: &Utf8Path, host: &Utf8Path, subject: &Subject) -> Option<String> {
    let mut arguments = vec!["--diagnostic-format", "json", "check"];
    arguments.extend(subject.args.iter().map(String::as_str));
    let left = copy(&subject.source);
    let right = copy(&subject.source);
    let on_native = normalized(native(&left.root, &arguments), left.root.as_str());
    let on_wasi = normalized(wasi(module, host, &right.root, &arguments), "/");
    (on_native != on_wasi).then(|| {
        format!(
            "{}: native exit {} stdout {} stderr {}\n  wasi exit {} stdout {} stderr {}",
            subject.name, on_native.0, on_native.1, on_native.2, on_wasi.0, on_wasi.1, on_wasi.2
        )
    })
}

/// CL-25.
pub(crate) fn cli() {
    if !wasi_backed("CL-25") {
        return;
    }
    let module = build();
    let scratch = tempfile::Builder::new()
        .prefix("lexlean-wasi-host-")
        .tempdir()
        .expect("tempdir");
    let host = Utf8PathBuf::from_path_buf(scratch.path().join("host.mjs")).expect("utf8 path");
    std::fs::write(host.as_std_path(), HOST).expect("write the host");

    // The same exit code, command result, and diagnostics on every project.
    let subjects = subjects();
    let next = std::sync::atomic::AtomicUsize::new(0);
    let differences = std::sync::Mutex::new(Vec::new());
    let workers = std::thread::available_parallelism()
        .map_or(1, usize::from)
        .min(8);
    std::thread::scope(|scope| {
        for _ in 0..workers {
            scope.spawn(|| loop {
                let at = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                let Some(subject) = subjects.get(at) else {
                    break;
                };
                if let Some(difference) = compare(&module, &host, subject) {
                    differences
                        .lock()
                        .unwrap_or_else(std::sync::PoisonError::into_inner)
                        .push(difference);
                }
            });
        }
    });
    let differences = differences
        .into_inner()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    assert!(
        differences.is_empty(),
        "CL-25: {} of {} projects differ between the native and the WASI executable:\n{}",
        differences.len(),
        subjects.len(),
        differences.join("\n")
    );
    eprintln!(
        "CL-25: {} projects agree between the native and the WASI executable",
        subjects.len()
    );

    // Two names of one file are refused on WASI too (§25.1): WASI reports the
    // device and inode that tell them apart.
    let example = support::repo_root().join("examples/nat-add-zero");
    let linked = |project: &P| {
        std::fs::hard_link(
            project.root.join("src/Main.lex.tex").as_std_path(),
            project.root.join("src/Twin.lex.tex").as_std_path(),
        )
        .expect("a hard link");
    };
    let left = copy(&example);
    let right = copy(&example);
    linked(&left);
    linked(&right);
    let arguments = ["--diagnostic-format", "json", "check"];
    let on_native = normalized(native(&left.root, &arguments), left.root.as_str());
    let on_wasi = normalized(wasi(&module, &host, &right.root, &arguments), "/");
    assert!(
        on_native.1.contains("LLC0104") && on_native.1.contains("are the same file"),
        "CL-25: the native executable refuses a hard-linked module: {on_native:?}"
    );
    assert_eq!(on_wasi, on_native, "CL-25: a hard-linked module on WASI");

    // A command that changes the project is refused before it touches it:
    // WASI has no file lock (§21.8).
    let project = copy(&example);
    let before = support::file_set(&project.root);
    let (exit, stdout, stderr) = wasi(
        &module,
        &host,
        &project.root,
        &["--diagnostic-format", "json", "fmt"],
    );
    assert_eq!(exit, 3, "CL-25: fmt on WASI exits 3: {stdout}{stderr}");
    assert!(
        stdout.contains("LLV7010") && stdout.contains("this host has no file lock"),
        "CL-25: fmt on WASI is refused with LLV7010: {stdout}{stderr}"
    );
    assert_eq!(
        support::file_set(&project.root),
        before,
        "CL-25: a refused fmt leaves the project as it was"
    );

    // The depth the linked stack hosts is enforced as on every host: input
    // within it checks as it does natively, and input beyond it is LLS8002,
    // never a trap, however deep the project allows (§8.3, §25.5).
    let hosted = lexlean::config::Limits {
        max_scope_depth: u64::MAX,
        ..support::limits_of(&P::example())
    };
    let hosted = usize::try_from(hosted.compile_stack_plan().effective_max_scope_depth)
        .expect("the native plan hosts a depth");
    assert!(
        hosted > 1024,
        "CL-25: the native host hosts more than WASI's 1,024"
    );
    for (nesting, refused) in [(1000, false), (1040, true)] {
        let project = copy(&example);
        let nested = format!("{}n{}", "(".repeat(nesting), ")".repeat(nesting));
        for (file, from, to) in [
            (
                "lexlean.toml",
                "max_scope_depth = 1024".to_owned(),
                "max_scope_depth = 100000000".to_owned(),
            ),
            (
                "src/Main.lex.tex",
                r"\(n + 0 = n\)".to_owned(),
                format!(r"\({nested} + 0 = n\)"),
            ),
        ] {
            let path = project.root.join(file);
            let text = std::fs::read_to_string(path.as_std_path()).expect("a project file");
            assert!(text.contains(&from), "CL-25: {file} holds {from}");
            std::fs::write(path.as_std_path(), text.replace(&from, &to)).expect("edit");
        }
        let locked = native(&project.root, &["lock"]);
        assert_eq!(
            locked.0, 0,
            "CL-25: the deep project locks natively: {locked:?}"
        );
        let arguments = ["--diagnostic-format", "json", "check"];
        let on_native = normalized(native(&project.root, &arguments), project.root.as_str());
        let on_wasi = normalized(wasi(&module, &host, &project.root, &arguments), "/");
        assert_eq!(
            on_native.0, 0,
            "CL-25: nesting {nesting} checks natively: {on_native:?}"
        );
        if refused {
            assert_eq!(
                on_wasi.0, 4,
                "CL-25: nesting {nesting} on WASI exits 4: {on_wasi:?}"
            );
            assert!(
                on_wasi.1.contains("LLS8002") && on_wasi.1.contains("max_scope_depth exceeded"),
                "CL-25: nesting {nesting} on WASI is LLS8002: {on_wasi:?}"
            );
        } else {
            assert_eq!(on_wasi, on_native, "CL-25: nesting {nesting} on WASI");
        }
    }
}

/// The example projects that check: every directory of `examples/` with a
/// `lexlean.toml` whose native `check --all` exits 0.
fn checking_examples() -> Vec<Utf8PathBuf> {
    let root = support::repo_root();
    let mut examples: Vec<Utf8PathBuf> = std::fs::read_dir(root.join("examples").as_std_path())
        .expect("examples")
        .flatten()
        .filter(|entry| entry.path().join("lexlean.toml").is_file())
        .filter_map(|entry| Utf8PathBuf::from_path_buf(entry.path()).ok())
        .collect();
    examples.sort();
    examples
        .into_iter()
        .filter(|source| native(&copy(source).root, &["check", "--all"]).0 == 0)
        .collect()
}

/// The document `lexlean declare` must print for `project`: the bytes of
/// `kernel::declarations_document` over its `--all` snapshot.
fn expected_document(project: &P, library: Option<&str>) -> Vec<u8> {
    let snapshot = project
        .engine()
        .snapshot(lexlean::api::CheckRequest {
            selection: lexlean::api::Selection::All,
        })
        .expect("the snapshot of a project that checks");
    let value = serde_json::to_value(&snapshot).expect("the snapshot serializes");
    let modules = lexlean::backend::kernel::modules_from_snapshot(&value)
        .expect("the snapshot's modules");
    lexlean::backend::kernel::declarations_document(&modules, library)
        .expect("the declarations of a project that checks")
        .to_file_bytes()
}

/// A field of a JSON command result or document.
fn result_field(stdout: &str, field: &str) -> serde_json::Value {
    let value: serde_json::Value = serde_json::from_str(stdout).expect("JSON");
    value[field].clone()
}

/// CL-29.
pub(crate) fn declare() {
    let examples = checking_examples();
    assert!(
        examples.len() >= 3,
        "CL-29: {} example projects check",
        examples.len()
    );
    // A lean4export 3.1.0 export of one name: the declarations continue its numbering.
    let library = P::empty();
    let export = library.root.join("library.ndjson");
    let export_text = "{\"in\":1,\"str\":{\"pre\":0,\"str\":\"Nat\"}}\n";
    std::fs::write(export.as_std_path(), export_text).expect("the export is written");
    let mut native_bytes = Vec::new();
    for source in &examples {
        let project = copy(source);
        let before = support::file_set(&project.root);
        // Human mode, self-contained: stdout is exactly the document's canonical bytes.
        let (exit, stdout, stderr) = native(&project.root, &["declare", "--all"]);
        assert_eq!(exit, 0, "CL-29: {source}: declare exits 0: {stderr}");
        assert!(stderr.is_empty(), "CL-29: {source}: nothing on stderr: {stderr}");
        assert_eq!(
            stdout.as_bytes(),
            expected_document(&project, None).as_slice(),
            "CL-29: {source}: stdout is the document's canonical bytes"
        );
        assert_eq!(
            result_field(&stdout, "spec"),
            "lexlean/kernel-declarations/2",
            "CL-29: {source}: the document's spec"
        );
        assert_eq!(
            result_field(&stdout, "library"),
            false,
            "CL-29: {source}: self-contained without a library"
        );
        // The suffix is the lines `kernel::declare` writes, read independently of the document builder.
        let snapshot = project
            .engine()
            .snapshot(lexlean::api::CheckRequest {
                selection: lexlean::api::Selection::All,
            })
            .expect("the snapshot of a project that checks");
        let modules = lexlean::backend::kernel::modules_from_snapshot(
            &serde_json::to_value(&snapshot).expect("the snapshot serializes"),
        )
        .expect("the snapshot's modules");
        let declared = lexlean::backend::kernel::declare(&modules, None).expect("the declarations");
        assert_eq!(
            result_field(&stdout, "suffix"),
            serde_json::Value::String(declared.production.suffix.clone()),
            "CL-29: {source}: the suffix is the lean4export lines kernel::declare writes"
        );
        assert_eq!(
            result_field(&stdout, "theorems").as_array().map_or(0, Vec::len),
            declared.production.rows.len(),
            "CL-29: {source}: one theorem row per theorem production saw"
        );
        // Over a library export: the same, continuing its numbering.
        let (exit, with_library, stderr) =
            native(&project.root, &["declare", "--all", "--library", export.as_str()]);
        assert_eq!(exit, 0, "CL-29: {source}: declare over a library exits 0: {stderr}");
        assert_eq!(
            with_library.as_bytes(),
            expected_document(&project, Some(export_text)).as_slice(),
            "CL-29: {source}: stdout over a library is the document's canonical bytes"
        );
        assert_eq!(
            result_field(&with_library, "library"),
            true,
            "CL-29: {source}: the document says it continues a library"
        );
        // JSON mode: one command result named declare, with check's identities.
        let (exit, json, _) =
            native(&project.root, &["--diagnostic-format", "json", "declare", "--all"]);
        let (_, checked, _) =
            native(&project.root, &["--diagnostic-format", "json", "check", "--all"]);
        assert_eq!(exit, 0, "CL-29: {source}: declare in json mode exits 0");
        assert_eq!(
            result_field(&json, "command"),
            "declare",
            "CL-29: {source}: the result names declare"
        );
        for id in ["source_id", "semantic_id"] {
            assert_eq!(
                result_field(&json, id),
                result_field(&checked, id),
                "CL-29: {source}: the {id} check reports"
            );
        }
        assert_eq!(
            support::file_set(&project.root),
            before,
            "CL-29: {source}: declare writes nothing"
        );
        native_bytes.push((source.clone(), stdout));
    }
    // A project check refuses: the same exit code and diagnostics, and no document.
    let refused = copy(&examples[0]);
    let module = walk_modules(&refused.root.join("src"))
        .into_iter()
        .next()
        .expect("a module");
    std::fs::write(module.as_std_path(), "not a LexLean module\n").expect("the module is broken");
    let (check_exit, check_json, _) =
        native(&refused.root, &["--diagnostic-format", "json", "check", "--all"]);
    let (declare_exit, declare_json, _) =
        native(&refused.root, &["--diagnostic-format", "json", "declare", "--all"]);
    assert_ne!(check_exit, 0, "CL-29: the broken project is refused by check");
    assert_eq!(declare_exit, check_exit, "CL-29: declare exits as check does");
    assert_eq!(
        result_field(&declare_json, "diagnostics"),
        result_field(&check_json, "diagnostics"),
        "CL-29: declare reports check's diagnostics"
    );
    let (_, human, _) = native(&refused.root, &["declare", "--all"]);
    assert!(human.is_empty(), "CL-29: no document for a refused project: {human}");
    // A file that is not lean4export 3.1.0 is LLV7103; one that cannot be read is CLI misuse.
    let project = copy(&examples[0]);
    let not_export = library.root.join("not-an-export.ndjson");
    std::fs::write(not_export.as_std_path(), "this is not an export\n").expect("written");
    let (exit, json, _) = native(
        &project.root,
        &["--diagnostic-format", "json", "declare", "--all", "--library", not_export.as_str()],
    );
    assert_ne!(exit, 0, "CL-29: a file that is not an export is refused");
    assert!(json.contains("LLV7103"), "CL-29: it is LLV7103: {json}");
    let absent = library.root.join("absent.ndjson");
    let (exit, _, _) = native(&project.root, &["declare", "--all", "--library", absent.as_str()]);
    assert_eq!(exit, 2, "CL-29: a library that cannot be read is CLI misuse");
    // The executable built for wasm32-wasip1 prints the same bytes on every example that checks.
    if !wasi_backed("CL-29") {
        return;
    }
    let module = build();
    let scratch = P::empty();
    let host = scratch.root.join("host.mjs");
    std::fs::write(host.as_std_path(), HOST).expect("the WASI host is written");
    let mut differ = Vec::new();
    for (source, native_stdout) in &native_bytes {
        let project = copy(source);
        let (exit, stdout, stderr) = wasi(&module, &host, &project.root, &["declare", "--all"]);
        if exit != 0 || &stdout != native_stdout {
            let stderr: String = stderr.chars().take(200).collect();
            differ.push(format!(
                "{source}: exit {exit}, {} bytes against {}: {stderr}",
                stdout.len(),
                native_stdout.len()
            ));
        }
    }
    assert!(
        differ.is_empty(),
        "CL-29: {} of {} examples differ on WASI:\n{}",
        differ.len(),
        native_bytes.len(),
        differ.join("\n")
    );
}

/// Every `.lex.tex` file under `dir`, sorted.
fn walk_modules(dir: &Utf8Path) -> Vec<Utf8PathBuf> {
    let mut found = Vec::new();
    let mut stack = vec![dir.to_owned()];
    while let Some(next) = stack.pop() {
        for entry in std::fs::read_dir(next.as_std_path()).into_iter().flatten().flatten() {
            let Ok(path) = Utf8PathBuf::from_path_buf(entry.path()) else {
                continue;
            };
            if path.is_dir() {
                stack.push(path);
            } else if path.as_str().ends_with(".lex.tex") {
                found.push(path);
            }
        }
    }
    found.sort();
    found
}
