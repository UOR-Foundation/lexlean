//! Embeds the normative language data into the compiler binary (SPEC.md §12.3,
//! §21.2): every regular file under `language/` and `schemas/`, and the
//! committed axiom-parser and canonical-JSON golden fixtures. The runtime
//! computes the compiler-semantics ID from exactly this set, and repository
//! tests recompute it from disk and compare (RP-10).
//!
//! The data lives at the repository root (§7). The crate reaches it through
//! the in-crate links `language`, `schemas`, `tests/golden`, and
//! `model/errors.toml`, which `cargo package` dereferences into real files,
//! so the packaged crate embeds byte-identical data and reports the same
//! compiler-semantics ID as the in-repository build (RP-12). A checkout that
//! could not materialize the links (a Windows checkout without symlink
//! support stores the link target as text) is resolved through that text.
//! A missing normative directory is a build failure, never an empty
//! language set.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

/// The crate-relative directories whose regular files participate in the
/// compiler-semantics digest (SPEC.md §21.2), with their repository-relative
/// names.
const SEMANTIC_DIRS: [(&str, &str); 4] = [
    ("language", "language"),
    ("schemas", "schemas"),
    ("tests/golden/axiom-parser", "tests/golden/axiom-parser"),
    ("tests/golden/canonical-json", "tests/golden/canonical-json"),
];

/// The embedded directory the `production-data` feature carries.
const PRODUCTION_DATA_DIR: &str = "language/preservation-1.2/library";

fn main() {
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("cargo sets this"));

    let mut files: Vec<(String, PathBuf)> = Vec::new();
    for (crate_relative, repo_relative) in SEMANTIC_DIRS {
        let root = resolve(&manifest, crate_relative);
        println!("cargo:rerun-if-changed={}", root.display());
        let before = files.len();
        collect(&root, repo_relative, &mut files);
        assert!(
            files.len() > before,
            "{}: the normative directory `{repo_relative}` is empty; the language data must be present (SPEC.md §7, §21.2)",
            root.display()
        );
    }
    // Bytewise-sorted repository-relative path order, the §11.5 tree order.
    files.sort_by(|a, b| a.0.as_bytes().cmp(b.0.as_bytes()));

    // Without `production-data` the native preservation library is left out
    // (SPEC.md §8.4): only certificate generation reads it, and it is most of
    // the embedded bytes. Its files are absent, not empty, so a reader that
    // reached one would find it missing rather than read an empty library.
    let production_data = env::var_os("CARGO_FEATURE_PRODUCTION_DATA").is_some();
    let mut out = String::new();
    out.push_str(&format!(
        "/// Whether this build embeds the production data (`{PRODUCTION_DATA_DIR}/`).\n\
         pub const PRODUCTION_DATA: bool = {production_data};\n\n"
    ));
    out.push_str(
        "/// Every embedded normative file as `(repository-relative path, bytes)`,\n\
         /// in bytewise-sorted path order.\n\
         pub static FILES: &[(&str, &[u8])] = &[\n",
    );
    for (rel, abs) in &files {
        println!("cargo:rerun-if-changed={}", abs.display());
        if !production_data && rel.starts_with(&format!("{PRODUCTION_DATA_DIR}/")) {
            continue;
        }
        out.push_str(&format!(
            "    ({:?}, include_bytes!({:?})),\n",
            rel,
            abs.display().to_string()
        ));
    }
    out.push_str("];\n");

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("cargo sets this"));
    fs::write(out_dir.join("embedded.rs"), out).expect("write embedded.rs");

    // The standard library's SHA-256 package (SPEC.md §17.22, `SL-06`): the
    // committed production export, linked and not copied, wrapped as one
    // module. The file's bytes follow the opening brace unchanged, so its
    // inner `forbid(unsafe_code)` stays the module's first item.
    let package = resolve_file(&manifest, "sha256_package.rs");
    println!("cargo:rerun-if-changed={}", package.display());
    let body = fs::read_to_string(&package).unwrap_or_else(|error| {
        panic!(
            "{}: the SHA-256 package must be present (SPEC.md §17.22): {error}",
            package.display()
        )
    });
    assert!(
        body.starts_with("#![forbid(unsafe_code)]"),
        "{}: not the committed SHA-256 package",
        package.display()
    );
    fs::write(
        out_dir.join("sha256_package.rs"),
        format!(
            "#[allow(warnings, missing_docs, clippy::all, clippy::pedantic, clippy::nursery)]\npub(crate) mod sha256_package {{\n{body}\n}}\n"
        ),
    )
    .expect("write sha256_package.rs");

    // The closed error registry, for `lexlean explain` (§23.4). Resolved
    // through the same link mechanism and copied to OUT_DIR so the CLI
    // includes one path in every layout.
    let errors = resolve(&manifest, "model/errors.toml");
    println!("cargo:rerun-if-changed={}", errors.display());
    let bytes = fs::read(&errors).unwrap_or_else(|error| {
        panic!(
            "{}: the diagnostic registry must be present (SPEC.md §26.1): {error}",
            errors.display()
        )
    });
    assert!(
        !bytes.is_empty(),
        "{}: the diagnostic registry is empty",
        errors.display()
    );
    fs::write(out_dir.join("errors.toml"), bytes).expect("write errors.toml");

    rustc_identity(&out_dir, &body);
}

/// The address of the compiler that builds this crate (SPEC.md §22.11): the
/// SHA-256 of one line `<sha256 of the file>  <sysroot-relative path>` per
/// file of the `rustc` executable and its driver library, in path order, and
/// what `rustc -vV` reports. The `rustc` executable is a thin launcher; the
/// compiler is the driver library it loads, so both are addressed. A compiler
/// whose sysroot cannot be read is a build failure, never an empty address.
///
/// The digests are the standard library's certified `Sha256` package (§17.22,
/// `SL-05`), the crate's only SHA-256: a build script cannot link the crate it
/// builds, so the package (`package`, the text `main` read) is compiled by the
/// same `rustc` into a small hasher in `OUT_DIR` and run on the files.
fn rustc_identity(out_dir: &Path, package: &str) {
    use std::process::Command;

    println!("cargo:rerun-if-env-changed=RUSTC");
    let rustc = env::var("RUSTC").unwrap_or_else(|_| "rustc".to_owned());
    let hasher_source = out_dir.join("rustc_identity_hasher.rs");
    fs::write(
        &hasher_source,
        format!(
            "#[allow(warnings, missing_docs, clippy::all, clippy::pedantic, clippy::nursery)]\nmod sha256_package {{\n{package}\n}}\n{HASHER_MAIN}"
        ),
    )
    .expect("write the hasher source");
    let hasher = out_dir.join(format!("rustc_identity_hasher{}", env::consts::EXE_SUFFIX));
    let built = Command::new(&rustc)
        .args(["--edition", "2021", "-O", "--crate-type", "bin", "-o"])
        .arg(&hasher)
        .arg(&hasher_source)
        .output()
        .unwrap_or_else(|error| panic!("running `{rustc}` on the hasher: {error}"));
    assert!(
        built.status.success(),
        "`{rustc}` could not build the SHA-256 hasher: {}",
        String::from_utf8_lossy(&built.stderr)
    );
    let digests = |paths: &[PathBuf]| -> Vec<String> {
        let output = Command::new(&hasher)
            .args(paths)
            .output()
            .unwrap_or_else(|error| panic!("running {}: {error}", hasher.display()));
        assert!(
            output.status.success(),
            "the SHA-256 hasher failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let text = String::from_utf8(output.stdout).expect("the hasher prints hexadecimal");
        let lines: Vec<String> = text.lines().map(str::to_owned).collect();
        assert_eq!(
            lines.len(),
            paths.len(),
            "the hasher prints one digest a file"
        );
        lines
    };
    let run = |argument: &str| {
        let output = Command::new(&rustc)
            .arg(argument)
            .output()
            .unwrap_or_else(|error| panic!("running `{rustc} {argument}`: {error}"));
        assert!(
            output.status.success(),
            "`{rustc} {argument}` failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8(output.stdout).expect("rustc reports UTF-8")
    };
    let version = run("-vV").replace("\r\n", "\n");
    let sysroot = PathBuf::from(run("--print=sysroot").trim());
    let mut files: Vec<(String, PathBuf)> = Vec::new();
    for directory in ["bin", "lib"] {
        let Ok(entries) = fs::read_dir(sysroot.join(directory)) else {
            continue;
        };
        for entry in entries.filter_map(Result::ok) {
            let name = entry.file_name().to_string_lossy().into_owned();
            let compiler = name == "rustc"
                || name == "rustc.exe"
                || name.starts_with("rustc_driver-")
                || name.starts_with("librustc_driver-");
            if compiler && entry.path().is_file() {
                files.push((format!("{directory}/{name}"), entry.path()));
            }
        }
    }
    files.sort();
    assert!(
        files.iter().any(|(path, _)| path.starts_with("bin/rustc")),
        "{}: no `rustc` executable in the compiler's sysroot",
        sysroot.display()
    );
    let paths: Vec<PathBuf> = files.iter().map(|(_, path)| path.clone()).collect();
    for path in &paths {
        println!("cargo:rerun-if-changed={}", path.display());
    }
    let mut listing = String::new();
    for ((relative, _), digest) in files.iter().zip(digests(&paths)) {
        listing.push_str(&digest);
        listing.push_str("  ");
        listing.push_str(relative);
        listing.push('\n');
    }
    let listing_path = out_dir.join("rustc_identity_listing.txt");
    fs::write(&listing_path, &listing).expect("write the compiler listing");
    let address = digests(&[listing_path])
        .pop()
        .expect("one digest of the listing");
    fs::write(
        out_dir.join("rustc_identity.rs"),
        format!(
            "/// What `rustc -vV` reported for the compiler that built this crate.\n\
             pub const RUSTC_VERSION: &str = {version:?};\n\
             /// The address of that compiler's executable and driver library.\n\
             pub const RUSTC_SHA256: &str = {address:?};\n"
        ),
    )
    .expect("write rustc_identity.rs");
}

/// The hasher's entry point: the lowercase hexadecimal SHA-256 of each file
/// named on its command line, one a line.
const HASHER_MAIN: &str = r#"
fn main() {
    for path in std::env::args_os().skip(1) {
        let bytes = std::fs::read(&path).unwrap_or_else(|error| {
            eprintln!("{}: {error}", std::path::Path::new(&path).display());
            std::process::exit(1)
        });
        let Ok(sha256_package::Adt0::C0(a, b, c, d, e, f, g, h)) =
            sha256_package::run(sha256_package::Bytes::lit(&bytes))
        else {
            eprintln!("a file beyond the domain of FIPS 180-4");
            std::process::exit(1)
        };
        let words = [a, b, c, d, e, f, g, h];
        let hex: String = words.iter().map(|word| format!("{word:08x}")).collect();
        println!("{hex}");
    }
}
"#;

/// Resolve a crate-relative normative path: a directory or file as such; a
/// checked-out symlink stored as text (its relative target); otherwise a
/// build failure that names what is missing.
fn resolve(manifest: &Path, crate_relative: &str) -> PathBuf {
    let direct = manifest.join(crate_relative);
    let metadata = fs::metadata(&direct);
    match metadata {
        Ok(found) if found.is_dir() || found.is_file() => {
            // A directory (or a file for the registry). A symlink stored as
            // text is a small file whose content is a relative path; when
            // the target is a directory we expect a directory, so a file
            // there is the text form.
            let wants_file = crate_relative.ends_with(".toml");
            if found.is_file() && !wants_file {
                return resolve_text_link(&direct, crate_relative);
            }
            if found.is_file() && wants_file {
                let text = fs::read_to_string(&direct).unwrap_or_default();
                let trimmed = text.trim();
                if !trimmed.contains('\n') && trimmed.starts_with("../") {
                    let candidate = direct.parent().unwrap_or(manifest).join(trimmed);
                    if candidate.is_file() {
                        return candidate;
                    }
                }
            }
            direct
        }
        // The link may be an *interior* component: `tests/golden` is one
        // link and `tests/golden/axiom-parser` lies beneath it, so on a
        // checkout that stored the link as text the full path does not
        // exist while the prefix does. Resolve the longest existing prefix
        // through the same text form and re-join the remainder, or the
        // §8.3 Windows host could never build the crate.
        _ => resolve_through_prefix(manifest, crate_relative).unwrap_or_else(|| {
            panic!(
                "{}: the normative path `{crate_relative}` is missing; the lexlean crate embeds the repository's language data and refuses to build without it (SPEC.md §7, §21.2)",
                direct.display()
            )
        }),
    }
}

/// Resolve `crate_relative` when one of its leading components, rather than
/// the whole path, is the link: the longest prefix that exists as a text
/// link is followed and the remaining components are re-joined onto it.
fn resolve_through_prefix(manifest: &Path, crate_relative: &str) -> Option<PathBuf> {
    let components: Vec<&str> = crate_relative.split('/').collect();
    for split in (1..components.len()).rev() {
        let prefix = components[..split].join("/");
        let candidate = manifest.join(&prefix);
        if !candidate.is_file() {
            continue;
        }
        let target = resolve_text_link(&candidate, &prefix);
        let mut resolved = target;
        for component in &components[split..] {
            resolved = resolved.join(component);
        }
        if resolved.exists() {
            return Some(resolved);
        }
    }
    None
}

fn resolve_text_link(link: &Path, crate_relative: &str) -> PathBuf {
    let text = fs::read_to_string(link).unwrap_or_default();
    let target = text.trim();
    let candidate = link.parent().map(|parent| parent.join(target));
    match candidate {
        Some(path) if !target.is_empty() && path.is_dir() => path,
        _ => panic!(
            "{}: `{crate_relative}` is neither a directory nor a link to one; the normative data is missing (SPEC.md §7)",
            link.display()
        ),
    }
}

fn collect(dir: &Path, rel: &str, out: &mut Vec<(String, PathBuf)>) {
    let entries = fs::read_dir(dir).unwrap_or_else(|error| {
        panic!(
            "{}: the normative directory `{rel}` cannot be read: {error}",
            dir.display()
        )
    });
    let mut children: Vec<_> = entries.filter_map(Result::ok).collect();
    children.sort_by_key(std::fs::DirEntry::file_name);
    for child in children {
        let path = child.path();
        let name = child.file_name();
        let name = name.to_str().expect("normative paths are UTF-8");
        let child_rel = format!("{rel}/{name}");
        let file_type = fs::metadata(&path)
            .expect("file type is readable")
            .file_type();
        if file_type.is_dir() {
            collect(&path, &child_rel, out);
        } else if file_type.is_file() {
            out.push((child_rel, path));
        } else {
            panic!("{child_rel}: normative data must contain only regular files");
        }
    }
}

/// Resolve a crate-relative file link: the file itself, or the relative path
/// a checkout without symlink support stores in its place.
fn resolve_file(manifest: &Path, crate_relative: &str) -> PathBuf {
    let direct = manifest.join(crate_relative);
    let text = fs::read_to_string(&direct).unwrap_or_else(|error| {
        panic!(
            "{}: the link `{crate_relative}` is missing: {error}",
            direct.display()
        )
    });
    let target = text.trim();
    if !target.contains('\n') && target.starts_with("../") {
        let candidate = manifest.join(target);
        if candidate.is_file() {
            return candidate;
        }
    }
    direct
}
