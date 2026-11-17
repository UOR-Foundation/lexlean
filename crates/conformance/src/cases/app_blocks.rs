//! The application building blocks (SPEC.md §17.22): `SL-15`, the `std-apps`
//! source package of proved blocks; `SL-16`, the pinned identity of each of
//! its declarations; and `EX-10`, the `cart` example built on it.
//!
//! The package exists so that an application cites a proved lemma by name
//! instead of proving list and state facts again. Its value therefore rests
//! on two things this case holds it to: every lemma verifies in the shape an
//! application's own one-step proof can use, and a lemma an application cites
//! never changes under the same identity without the change being announced.

use std::collections::BTreeMap;

use lexlean::config::LexiconSource;
use lexlean::{CheckRequest, Selection, Sha256Digest};
use serde_json::{json, Value};

use crate::support::{self, P};

/// The package directory, which is also its workspace member.
const PACKAGE: &str = "std-apps";
/// Its Lean module prefix.
const PREFIX: &str = "LexLeanStdApps";
/// Its modules, one per block.
const MODULES: &[&str] = &[
    "Access", "Counter", "Ledger", "Lines", "Lists", "Machine", "Tables",
];
/// The committed record of every declaration's identity.
const PINS: &str = "std-apps/pins.json";
/// The example built on the package.
const CART: &str = "examples/cart";
/// The record of what the cart no longer writes.
const SAVINGS: &str = "docs/app-blocks/cart.json";
/// The axioms a block's proof may observe: what `linear_arithmetic`, `simp`,
/// and `omega` draw on under the pinned Lean, and nothing an application
/// would have to accept beyond its own proofs.
const ADMITTED_AXIOMS: &[&str] = &["Classical.choice", "Quot.sound", "propext"];

/// Run the case for `SL-15`, `SL-16`, or `EX-10`.
///
/// # Panics
///
/// Panics when the case's assertion fails.
pub(crate) fn run(id: &str) {
    match id {
        "SL-15" => package(),
        "SL-16" => pins(),
        "EX-10" => cart(),
        other => panic!("no application-block case is wired for {other}"),
    }
}

fn read(path: &str) -> String {
    std::fs::read_to_string(support::repo_root().join(path).as_std_path())
        .unwrap_or_else(|error| panic!("{path}: {error}"))
}

fn package_project() -> P {
    P::workspace_project(PACKAGE, &[PACKAGE])
}

fn cart_project() -> P {
    P::workspace_project(CART, &[CART, PACKAGE])
}

/// `SL-15`: the package is a source package of the workspace whose seven
/// modules verify alone, and a weakened lemma breaks the application that
/// cites it.
fn package() {
    let config = lexlean::config::parse_project(
        "lexlean.toml",
        read(&format!("{PACKAGE}/lexlean.toml")).as_bytes(),
    )
    .expect("the manifest parses");
    assert_eq!(config.language, "1.2", "SL-15");
    assert_eq!(config.module_prefix, PREFIX, "SL-15");
    assert_eq!(
        config.entrypoints,
        MODULES
            .iter()
            .map(|module| format!("src/{module}.lex.tex"))
            .collect::<Vec<_>>(),
        "SL-15: every block module is an entrypoint"
    );
    assert!(
        config.source_packages.is_empty() && config.artifact_sources.is_empty(),
        "SL-15: the package declares no source package and no artifact"
    );
    assert!(
        config
            .lexicon_sources
            .iter()
            .all(|source| matches!(source, LexiconSource::Builtin { .. })),
        "SL-15: the package declares builtin lexicons only"
    );
    assert!(config.pdf.is_none(), "SL-15: no [pdf]");

    let root = support::repo_root();
    let workspace = lexlean::source_package::Workspace::read(&root)
        .expect("the workspace file reads")
        .expect("the repository has a workspace file");
    for member in [PACKAGE, CART] {
        assert!(
            workspace.members.iter().any(|found| found == member),
            "SL-15: `{member}` is a member of the workspace"
        );
    }

    // No handwritten Lean: the blocks are source and nothing else.
    for entry in walkdir::WalkDir::new(root.join(PACKAGE).as_std_path())
        .into_iter()
        .flatten()
        .filter(|entry| entry.file_type().is_file())
    {
        assert!(
            entry
                .path()
                .extension()
                .is_none_or(|extension| extension != "lean"),
            "SL-15: {} is Lean source",
            entry.path().display()
        );
    }

    // The blocks a survey of applications chose, each stated where the
    // survey records it.
    let survey: Value =
        serde_json::from_str(&read("docs/app-blocks/survey.json")).expect("the survey is JSON");
    let blocks: Vec<&str> = survey["shapes"]
        .as_array()
        .expect("shapes")
        .iter()
        .flat_map(|shape| shape["blocks"].as_array().expect("blocks").iter())
        .filter_map(Value::as_str)
        .collect();
    for module in MODULES {
        assert!(
            blocks.contains(module),
            "SL-15: the survey records the shape `{module}` serves"
        );
    }

    let project = package_project();
    project.fmt_check_ok();
    let (exit, _, stderr) = project.cli(&["lock", "--check"]);
    assert_eq!(exit, 0, "SL-15: the committed lock is current: {stderr}");
    project.check_ok();

    if !support::lean_backed("SL-15") {
        return;
    }
    let verified = support::verify_ok(&project);
    let attestation: Value = serde_json::from_slice(
        &std::fs::read(
            project
                .root
                .join(".lexlean/verified")
                .join(verified.attestation_id.to_hex())
                .join("attestation.json")
                .as_std_path(),
        )
        .expect("attestation"),
    )
    .expect("the attestation is JSON");
    let rows = attestation["declarations"]
        .as_array()
        .expect("declarations");
    for module in MODULES {
        let prefix = format!("{PREFIX}.{module}.");
        assert!(
            rows.iter().any(|row| row["name"]
                .as_str()
                .is_some_and(|name| name.starts_with(&prefix))),
            "SL-15: `{module}` is audited"
        );
    }
    for row in rows.iter().filter(|row| {
        row["name"]
            .as_str()
            .is_some_and(|name| name.starts_with(&format!("{PREFIX}.")))
    }) {
        assert!(
            row["observed"]
                .as_array()
                .is_some_and(|observed| observed.iter().all(|axiom| axiom
                    .as_str()
                    .is_some_and(|axiom| ADMITTED_AXIOMS.contains(&axiom)))),
            "SL-15: a block observes only {ADMITTED_AXIOMS:?}: {row}"
        );
    }

    // A weakened lemma: the package still verifies, but the application that
    // cites the lemma by name is refused by the kernel, because its one-step
    // proof states what the lemma used to.
    let weakened = cart_project();
    let lines = "../../std-apps/src/Lines.lex.tex";
    let text = weakened.read_raw(lines);
    let site = "statement: (((total (removeIf allowed item lines)) + (removedAmount allowed item lines)) = (total lines)),";
    assert!(text.contains(site), "SL-15: the plant site exists");
    weakened.write_raw(
        lines,
        &text.replacen(
            site,
            "statement: ((total (removeIf allowed item lines)) <= (total lines)),",
            1,
        ),
    );
    let (exit, _, stderr) = weakened.cli(&["lock"]);
    assert_eq!(exit, 0, "SL-15: the weakened package relocks: {stderr}");
    let _environment = support::env_shared();
    weakened.verify_fails_with("LLV7002");
}

/// One declaration's pinned identity: the alpha identity of a definition
/// (SPEC.md §17.12 rule 5), and for every other declaration the digest of
/// its canonical linked form without its proof, so that a statement, a
/// parameter, or an axiom policy that changes moves it and a proof that
/// changes does not.
fn identities(project: &P) -> BTreeMap<(String, String), Value> {
    let snapshot = project
        .engine()
        .snapshot(CheckRequest {
            selection: Selection::Entrypoints,
        })
        .expect("snapshot");
    let alpha: BTreeMap<(String, String), Sha256Digest> = snapshot
        .alpha_ids()
        .into_iter()
        .map(|(module, name, id)| ((module, name), id))
        .collect();
    let mut rows = BTreeMap::new();
    for module in snapshot.modules() {
        for declaration in module.declarations() {
            let key = (
                module.name().to_owned(),
                declaration.logical_id().to_owned(),
            );
            let row = if let Some(id) = alpha.get(&key) {
                json!({"kind": declaration.kind(), "alpha_id": id.to_hex()})
            } else {
                let mut linked = declaration.linked_ir().clone();
                if let Some(object) = linked.as_object_mut() {
                    object.remove("proof");
                }
                let bytes = serde_json::to_vec(&linked).expect("linked IR serializes");
                json!({"kind": declaration.kind(), "statement_id": Sha256Digest::of(&bytes).to_hex()})
            };
            rows.insert(key, row);
        }
    }
    rows
}

/// The pins file the identities of a project would be.
fn pins_text(version: &str, rows: &BTreeMap<(String, String), Value>) -> String {
    let declarations: Vec<Value> = rows
        .iter()
        .map(|((module, name), row)| {
            let mut entry = json!({"module": module, "name": name});
            for (field, value) in row.as_object().expect("row") {
                entry[field] = value.clone();
            }
            entry
        })
        .collect();
    let mut text = serde_json::to_string_pretty(&json!({
        "spec": "lexlean/app-block-pins/1",
        "package": PACKAGE,
        "version": version,
        "declarations": declarations,
    }))
    .expect("pins serialize");
    text.push('\n');
    text
}

/// Every difference between the pinned and the found identities, naming the
/// declaration; empty when they agree.
fn unannounced(pinned: &Value, found: &BTreeMap<(String, String), Value>) -> Vec<String> {
    let mut pinned_rows = BTreeMap::new();
    for entry in pinned["declarations"].as_array().expect("declarations") {
        let module = entry["module"].as_str().expect("module").to_owned();
        let name = entry["name"].as_str().expect("name").to_owned();
        let mut row = entry.clone();
        let object = row.as_object_mut().expect("entry");
        object.remove("module");
        object.remove("name");
        pinned_rows.insert((module, name), row);
    }
    let mut changes = Vec::new();
    for (key, row) in found {
        match pinned_rows.get(key) {
            None => changes.push(format!("{}.{} is not pinned", key.0, key.1)),
            Some(pinned) if pinned != row => changes.push(format!(
                "{}.{} changed under its pin: pinned {pinned}, found {row}",
                key.0, key.1
            )),
            Some(_) => {}
        }
    }
    for key in pinned_rows.keys() {
        if !found.contains_key(key) {
            changes.push(format!("{}.{} is pinned and gone", key.0, key.1));
        }
    }
    changes
}

/// `SL-16`: the committed pins are the package's identities, every importer
/// declares the pinned version, and a changed definition or a weakened lemma
/// under a pinned identity is refused, naming it.
fn pins() {
    let pinned: Value = serde_json::from_str(&read(PINS)).expect("the pins file is JSON");
    assert_eq!(pinned["spec"], "lexlean/app-block-pins/1", "SL-16");
    assert_eq!(pinned["package"], PACKAGE, "SL-16");
    let version = pinned["version"].as_str().expect("version").to_owned();

    let project = package_project();
    let found = identities(&project);
    assert!(
        !found.is_empty(),
        "SL-16: the package declares something to pin"
    );
    let changes = unannounced(&pinned, &found);
    assert!(
        changes.is_empty(),
        "SL-16: the package changed under its pins without an announcement; if the change is intended, raise `version`, record it in CHANGELOG.md, and commit {PINS} as\n{}\nChanges:\n{}",
        pins_text(&version, &found),
        changes.join("\n")
    );
    assert_eq!(
        read(PINS),
        pins_text(&version, &found),
        "SL-16: {PINS} is canonical"
    );
    // Every definition is pinned by its alpha identity and every theorem by
    // its statement: none slips through as neither.
    for ((module, name), row) in &found {
        let field = if row["kind"] == "definition" {
            "alpha_id"
        } else {
            "statement_id"
        };
        assert!(
            row[field].as_str().is_some_and(|id| id.len() == 64),
            "SL-16: {module}.{name} is pinned by its {field}"
        );
    }

    // Every importer in the workspace declares the version the pins announce.
    let workspace = lexlean::source_package::Workspace::read(&support::repo_root())
        .expect("the workspace file reads")
        .expect("the repository has a workspace file");
    let mut importers = 0usize;
    for member in &workspace.members {
        let config = lexlean::config::parse_project(
            "lexlean.toml",
            read(&format!("{member}/lexlean.toml")).as_bytes(),
        )
        .expect("a member manifest parses");
        for package in config
            .source_packages
            .iter()
            .filter(|package| package.package == PACKAGE)
        {
            importers += 1;
            assert_eq!(
                package.version, version,
                "SL-16: `{member}` declares the pinned version of `{PACKAGE}`"
            );
        }
    }
    assert!(importers > 0, "SL-16: an importer of `{PACKAGE}` exists");

    // A changed definition under a pinned identity: the alpha identity moves
    // and the comparison names it.
    let changed = package_project();
    let lines = "src/Lines.lex.tex";
    let text = changed.read_raw(lines);
    let site = "| List.cons head rest => ((second head) + (total rest))),";
    assert!(
        text.contains(site),
        "SL-16: the definition plant site exists"
    );
    changed.write_raw(
        lines,
        &text.replacen(site, "| List.cons head rest => (total rest)),", 1),
    );
    let refused = unannounced(&pinned, &identities(&changed));
    assert!(
        refused
            .iter()
            .any(|change| change.starts_with("Lines.total changed under its pin")),
        "SL-16: a changed definition is refused by name: {refused:?}"
    );

    // A weakened lemma under a pinned identity: its statement identity moves.
    let weakened = package_project();
    let text = weakened.read_raw(lines);
    let site = "statement: ((amountOf item lines) <= (total lines)),";
    assert!(text.contains(site), "SL-16: the lemma plant site exists");
    weakened.write_raw(
        lines,
        &text.replacen(
            site,
            "statement: ((amountOf item lines) <= ((total lines) + 1)),",
            1,
        ),
    );
    let refused = unannounced(&pinned, &identities(&weakened));
    assert_eq!(
        refused.len(),
        1,
        "SL-16: only the weakened lemma moves: {refused:?}"
    );
    assert!(
        refused[0].starts_with("Lines.amountOf_le_total changed under its pin"),
        "SL-16: a weakened lemma is refused by name: {refused:?}"
    );

    // A proof that changes is not an identity change: the statement is what
    // an application cites.
    let reproved = package_project();
    let text = reproved.read_raw(lines);
    let site = "proof: linear_arithmetic{definitions: [addLine, total]},";
    assert!(text.contains(site), "SL-16: the proof plant site exists");
    reproved.write_raw(
        lines,
        &text.replacen(
            site,
            "proof: linear_arithmetic{definitions: [addLine, amountOf, total]},",
            1,
        ),
    );
    assert!(
        unannounced(&pinned, &identities(&reproved)).is_empty(),
        "SL-16: a changed proof keeps every pinned identity"
    );
}

/// `EX-10`: the cart rewritten on the package; every theorem is one step
/// that cites a block or unfolds the cart's own definitions, and the record of
/// what it no longer writes is the cart as committed.
fn cart() {
    let source = read(&format!("{CART}/src/Main.lex.tex"));
    let theorems: Vec<&str> = source.split("theorem{").skip(1).collect();
    assert!(!theorems.is_empty(), "EX-10: the cart states theorems");
    let mut cited = Vec::new();
    for theorem in &theorems {
        let proof = theorem
            .split("proof: ")
            .nth(1)
            .expect("a theorem has a proof");
        assert!(
            ["apply{", "linear_arithmetic{", "reflexivity{"]
                .iter()
                .any(|step| proof.starts_with(step)),
            "EX-10: every cart proof is one step: {theorem}"
        );
        if let Some(rest) = proof.strip_prefix("apply{") {
            let name = rest
                .split("theorem: ")
                .nth(1)
                .and_then(|tail| tail.split(['}', ',']).next())
                .expect("an apply names a theorem");
            assert!(
                name.starts_with(&format!("{PREFIX}.")),
                "EX-10: an applied theorem is a block lemma: {name}"
            );
            cited.push(name.trim().to_owned());
        }
    }
    assert!(
        !source.contains("recursive_argument"),
        "EX-10: the cart defines no recursion of its own"
    );

    // The savings record is the committed cart.
    let savings: Value = serde_json::from_str(&read(SAVINGS)).expect("the record is JSON");
    assert_eq!(
        savings["rewrite"]["lines"].as_u64(),
        u64::try_from(source.lines().count()).ok(),
        "EX-10: the record counts the cart's lines"
    );
    assert_eq!(
        savings["rewrite"]["theorems"].as_u64(),
        u64::try_from(theorems.len()).ok(),
        "EX-10: the record counts the cart's theorems"
    );
    cited.sort();
    cited.dedup();
    let recorded: Vec<String> = savings["rewrite"]["cited"]
        .as_array()
        .expect("cited")
        .iter()
        .map(|name| name.as_str().expect("a name").to_owned())
        .collect();
    assert_eq!(
        recorded, cited,
        "EX-10: the record lists the lemmas the cart cites"
    );

    let project = cart_project();
    project.fmt_check_ok();
    let (exit, _, stderr) = project.cli(&["lock", "--check"]);
    assert_eq!(exit, 0, "EX-10: the committed lock is current: {stderr}");
    project.check_ok();
    support::verify_ok_backed("EX-10", &project);
}
