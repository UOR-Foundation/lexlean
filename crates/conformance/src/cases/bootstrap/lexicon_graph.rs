//! `BS-56` (SPEC.md §17.23, §13.6, §13.11): the package graph of a lexicon
//! closure as language-1.2 source, `LexLeanBoot.LexiconGraph`, held to the
//! byte to the compiler's first stage of `Closure::build`:
//! `packageGraph` to `lexlean::lexicon::resolve::package_graph` (the
//! topological order and every package's import closure, or the refusals of a
//! duplicate ID, an absent or mismatched import, an import cycle with its
//! importers, and an import chain at the depth limit, in the order the oracle
//! finds them), and `packageImporters` to
//! `lexlean::lexicon::import_reachability::importers` (the exact references
//! with an import path to a set of packages). Each planted defect is detected
//! by the one vector it names.

use std::collections::BTreeMap;

use lexlean::artifact::content_id::Kappa;
use lexlean::calculus::Value;
use lexlean::lexicon::import_reachability::importers;
use lexlean::lexicon::package::{LexiconPackage, PackageRef};
use lexlean::lexicon::resolve::package_graph;
use serde_json::Value as Json2;

use super::calculus_check::nat;
use super::rust_validate::{eligibility_of, rustc_differential};
use super::{list, option, pair, text, texts, Run, Seeded};
use crate::production_export as harness;
use crate::support::{self, repo_root};

const MODULE: &str = "LexLeanBoot.LexiconGraph";
const SOURCE: &str = "src/LexiconGraph.lex.tex";
/// In the module's order, which is the eligibility report's.
const ROOTS: [(&str, &[&str]); 2] = [
    ("packageImporters", &["allocation", "overflow"]),
    ("packageGraph", &["allocation", "overflow"]),
];
const IMPORTERS: usize = 0;
const GRAPH: usize = 1;

const CERTIFICATE_BUDGET: u64 = 3_145_728;
const EVALUATOR_SEEDED: usize = 300;
const RUSTC_SEEDED: usize = 2_000;

/// One package of a graph: its ID, version, and imports, in import order.
#[derive(Clone)]
struct Node {
    id: String,
    version: String,
    imports: Vec<(String, String)>,
}

fn node(id: &str, version: &str, imports: &[(&str, &str)]) -> Node {
    Node {
        id: id.to_owned(),
        version: version.to_owned(),
        imports: imports
            .iter()
            .map(|(package, version)| ((*package).to_owned(), (*version).to_owned()))
            .collect(),
    }
}

/// The oracle's package: only the fields the graph stage reads are set.
fn oracle_package(node: &Node) -> LexiconPackage {
    LexiconPackage {
        id: node.id.clone(),
        version: node.version.clone(),
        imports: node
            .imports
            .iter()
            .map(|(package, version)| PackageRef {
                package: package.clone(),
                version: version.clone(),
            })
            .collect(),
        entries: BTreeMap::new(),
        manifest_kappa: Kappa::of(b""),
        tree_kappa: Kappa::of(b""),
        total_bytes: 0,
    }
}

fn enc_node(node: &Node) -> Value {
    Value::Adt {
        constructor: 0,
        fields: vec![
            text(&node.id),
            text(&node.version),
            list(
                node.imports
                    .iter()
                    .map(|(package, version)| pair(text(package), text(version)))
                    .collect(),
            ),
        ],
    }
}

fn enc_nodes(nodes: &[Node]) -> Value {
    list(nodes.iter().map(enc_node).collect())
}

fn enc_refs(members: &[(String, String)]) -> Value {
    list(
        members
            .iter()
            .map(|(package, version)| pair(text(package), text(version)))
            .collect(),
    )
}

/// `packageGraph`'s run: the oracle's outcome as `GraphOutcome`.
fn graph_run(nodes: &[Node], max_import_depth: u64) -> Run {
    let packages: Vec<LexiconPackage> = nodes.iter().map(oracle_package).collect();
    let outcome = match package_graph(&packages, max_import_depth) {
        Ok(graph) => Value::Adt {
            constructor: 0,
            fields: vec![
                list(
                    graph
                        .order
                        .iter()
                        .map(|&index| nat(u64::try_from(index).expect("an index")))
                        .collect(),
                ),
                list(
                    graph
                        .order
                        .iter()
                        .map(|&index| {
                            let id = &packages[index].id;
                            let visible: Vec<String> =
                                graph.import_closure[id].iter().cloned().collect();
                            pair(text(id), texts(&visible))
                        })
                        .collect(),
                ),
            ],
        },
        Err(diagnostics) => Value::Adt {
            constructor: 1,
            fields: vec![list(
                diagnostics
                    .iter()
                    .map(|diagnostic| {
                        let cycle = match diagnostic.detail() {
                            Some(lexlean::diagnostic::DiagnosticDetail::PackageImportCycle {
                                packages,
                                importers,
                            }) => Some(pair(texts(packages), texts(importers))),
                            _ => None,
                        };
                        Value::Adt {
                            constructor: 0,
                            fields: vec![
                                text(diagnostic.code.as_str()),
                                text(&diagnostic.message),
                                option(cycle),
                            ],
                        }
                    })
                    .collect(),
            )],
        },
    };
    (
        vec![enc_nodes(nodes), nat(max_import_depth)],
        serde_json::to_value(outcome).expect("a value is JSON"),
    )
}

/// `packageImporters`'s run: the oracle's sorted references.
fn importers_run(nodes: &[Node], members: &[(String, String)]) -> Run {
    let packages: Vec<LexiconPackage> = nodes.iter().map(oracle_package).collect();
    let refs: Vec<PackageRef> = members
        .iter()
        .map(|(package, version)| PackageRef {
            package: package.clone(),
            version: version.clone(),
        })
        .collect();
    (
        vec![enc_nodes(nodes), enc_refs(members)],
        serde_json::to_value(texts(&importers(&packages, &refs))).expect("a value is JSON"),
    )
}

fn members(refs: &[(&str, &str)]) -> Vec<(String, String)> {
    refs.iter()
        .map(|(package, version)| ((*package).to_owned(), (*version).to_owned()))
        .collect()
}

// ------------------------------------------------------------------ vectors

struct Vector {
    label: &'static str,
    root: usize,
    run: Run,
    /// The codes the oracle refuses this vector with (`packageGraph` only).
    codes: Vec<&'static str>,
}

fn refusal_codes(nodes: &[Node], max_import_depth: u64) -> Vec<&'static str> {
    let packages: Vec<LexiconPackage> = nodes.iter().map(oracle_package).collect();
    package_graph(&packages, max_import_depth)
        .err()
        .unwrap_or_default()
        .iter()
        .map(|diagnostic| diagnostic.code.as_str())
        .collect()
}

const V: &str = "1.0.0";
const W: &str = "2.0.0";

#[allow(clippy::too_many_lines)]
fn named() -> Vec<Vector> {
    let graph = |label: &'static str, nodes: Vec<Node>, depth: u64| Vector {
        label,
        root: GRAPH,
        run: graph_run(&nodes, depth),
        codes: refusal_codes(&nodes, depth),
    };
    let reach = |label: &'static str, nodes: Vec<Node>, refs: &[(&str, &str)]| Vector {
        label,
        root: IMPORTERS,
        run: importers_run(&nodes, &members(refs)),
        codes: Vec::new(),
    };
    vec![
        graph("no packages", Vec::new(), 128),
        graph("one package", vec![node("lexlean.core", V, &[])], 128),
        graph(
            "a chain is ordered imports first",
            vec![
                node("a", V, &[("b", V)]),
                node("b", V, &[("c", V)]),
                node("c", V, &[]),
            ],
            128,
        ),
        graph(
            "a diamond closes over both paths",
            vec![
                node("top", V, &[("left", V), ("right", V)]),
                node("left", V, &[("base", V)]),
                node("right", V, &[("base", V)]),
                node("base", V, &[]),
            ],
            128,
        ),
        graph(
            "independent roots keep their given order",
            vec![
                node("z", V, &[]),
                node("a", V, &[]),
                node("m", V, &[("a", V)]),
            ],
            128,
        ),
        graph(
            "a duplicate package is refused",
            vec![node("a", V, &[]), node("a", W, &[])],
            128,
        ),
        graph(
            "every later duplicate is refused",
            vec![
                node("a", V, &[]),
                node("b", V, &[]),
                node("a", V, &[]),
                node("a", W, &[]),
            ],
            128,
        ),
        graph(
            "a duplicate is refused before an absent import",
            vec![node("a", V, &[("missing", V)]), node("a", V, &[])],
            128,
        ),
        graph(
            "an absent import is refused",
            vec![node("a", V, &[("b", V)])],
            128,
        ),
        graph(
            "an import at another version is refused",
            vec![node("a", V, &[("b", W)]), node("b", V, &[])],
            128,
        ),
        graph(
            "every import refusal is reported in order",
            vec![
                node("a", V, &[("x", V), ("b", W)]),
                node("b", V, &[("y", V)]),
            ],
            128,
        ),
        graph(
            "a package importing itself is a cycle",
            vec![node("a", V, &[("a", V)])],
            128,
        ),
        graph(
            "a two-package cycle",
            vec![node("a", V, &[("b", V)]), node("b", V, &[("a", V)])],
            128,
        ),
        graph(
            "a cycle names the packages importing it",
            vec![
                node("user", V, &[("a", V)]),
                node("a", V, &[("b", V)]),
                node("b", V, &[("c", V)]),
                node("c", V, &[("b", V)]),
                node("bystander", V, &[]),
            ],
            128,
        ),
        graph(
            "a chain at the depth limit is refused",
            vec![
                node("a", V, &[("b", V)]),
                node("b", V, &[("c", V)]),
                node("c", V, &[]),
            ],
            2,
        ),
        graph(
            "a chain below the depth limit is accepted",
            vec![
                node("a", V, &[("b", V)]),
                node("b", V, &[("c", V)]),
                node("c", V, &[]),
            ],
            3,
        ),
        graph(
            "a depth limit of zero refuses every package",
            vec![node("a", V, &[]), node("b", V, &[])],
            0,
        ),
        graph(
            "a depth refusal is kept when a cycle follows",
            vec![
                node("deep", V, &[("mid", V)]),
                node("mid", V, &[("leaf", V)]),
                node("leaf", V, &[]),
                node("p", V, &[("q", V)]),
                node("q", V, &[("p", V)]),
            ],
            2,
        ),
        graph(
            "the depth is the longest import chain",
            vec![
                node("a", V, &[("b", V), ("leaf", V)]),
                node("b", V, &[("c", V)]),
                node("c", V, &[("leaf", V)]),
                node("leaf", V, &[]),
            ],
            3,
        ),
        reach("no members reach nothing", vec![node("a", V, &[])], &[]),
        reach(
            "a member reaches itself",
            vec![node("a", V, &[])],
            &[("a", V)],
        ),
        reach(
            "importers are reached transitively",
            vec![
                node("top", V, &[("mid", V)]),
                node("mid", V, &[("a", V)]),
                node("a", V, &[]),
                node("other", V, &[]),
            ],
            &[("a", V)],
        ),
        reach(
            "an import at another version reaches no importer",
            vec![node("user", V, &[("a", W)]), node("a", V, &[])],
            &[("a", W)],
        ),
        reach(
            "a member that is not loaded names its version",
            vec![node("user", V, &[("ghost", W)])],
            &[("ghost", W)],
        ),
        reach(
            "a duplicated package contributes no edge",
            vec![
                node("user", V, &[("a", V)]),
                node("user", V, &[("a", V)]),
                node("a", V, &[]),
            ],
            &[("a", V)],
        ),
        reach(
            "the members' own references are sorted with their importers",
            vec![
                node("b", V, &[("z", V)]),
                node("z", V, &[]),
                node("a", V, &[("z", V)]),
            ],
            &[("z", V)],
        ),
    ]
}

// --------------------------------------------------------------- committed

/// Every committed lexicon manifest: its package, version, and imports.
fn manifests() -> Vec<Node> {
    let root = repo_root();
    let mut out = Vec::new();
    for entry in walkdir::WalkDir::new(root.as_std_path())
        .sort_by_file_name()
        .into_iter()
        .filter_entry(|entry| {
            let name = entry.file_name().to_string_lossy();
            !(entry.file_type().is_dir()
                && (matches!(name.as_ref(), "target" | ".git" | ".lexlean" | "tests")
                    || support::holds_cache_tag(entry)))
        })
        .flatten()
    {
        if entry.file_name() != "lexicon.toml" {
            continue;
        }
        let Ok(content) = std::fs::read_to_string(entry.path()) else {
            continue;
        };
        let Ok(manifest) = content.parse::<toml::Table>() else {
            continue;
        };
        let (Some(id), Some(version)) = (
            manifest.get("package").and_then(toml::Value::as_str),
            manifest.get("version").and_then(toml::Value::as_str),
        ) else {
            continue;
        };
        let imports = manifest
            .get("imports")
            .and_then(toml::Value::as_array)
            .map(|items| {
                items
                    .iter()
                    .filter_map(toml::Value::as_str)
                    .filter_map(|reference| reference.split_once('@'))
                    .map(|(package, version)| (package.to_owned(), version.to_owned()))
                    .collect()
            })
            .unwrap_or_default();
        out.push(Node {
            id: id.to_owned(),
            version: version.to_owned(),
            imports,
        });
    }
    out
}

/// Each committed package with the committed packages its imports reach at
/// their exact versions, in the order they are first reached; and the whole
/// committed set, in which `lexlean.core` occurs at several versions.
fn committed_graphs(all: &[Node]) -> Vec<Vec<Node>> {
    let mut graphs = Vec::new();
    for start in all {
        let mut chosen: Vec<Node> = vec![start.clone()];
        let mut next = 0;
        while next < chosen.len() {
            for (package, version) in chosen[next].imports.clone() {
                if chosen.iter().any(|node| node.id == package) {
                    continue;
                }
                if let Some(found) = all
                    .iter()
                    .find(|node| node.id == package && node.version == version)
                {
                    chosen.push(found.clone());
                }
            }
            next += 1;
        }
        graphs.push(chosen);
    }
    graphs.push(all.to_vec());
    graphs
}

// -------------------------------------------------------------------- seeds

const IDS: &[&str] = &["a", "b", "c", "d", "e", "lexlean.core", "x.y"];
const VERSIONS: &[&str] = &["1.0.0", "1.2.0", "2.0.0"];

/// A seeded package set: a few packages over a small ID pool (so duplicates
/// occur), each importing a few IDs at a version that usually matches.
fn seeded_nodes(seeded: &mut Seeded) -> Vec<Node> {
    let count = seeded.below(7);
    let mut nodes: Vec<Node> = (0..count)
        .map(|_| Node {
            id: (*seeded.pick(IDS)).to_owned(),
            version: (*seeded.pick(VERSIONS)).to_owned(),
            imports: Vec::new(),
        })
        .collect();
    let known: Vec<(String, String)> = nodes
        .iter()
        .map(|node| (node.id.clone(), node.version.clone()))
        .collect();
    for node in &mut nodes {
        for _ in 0..seeded.below(4) {
            let import = if !known.is_empty() && seeded.chance(85) {
                let (id, version) = seeded.pick(&known).clone();
                if seeded.chance(10) {
                    (id, (*seeded.pick(VERSIONS)).to_owned())
                } else {
                    (id, version)
                }
            } else {
                (
                    (*seeded.pick(IDS)).to_owned(),
                    (*seeded.pick(VERSIONS)).to_owned(),
                )
            };
            node.imports.push(import);
        }
    }
    nodes
}

fn seeded_runs(count: usize) -> Vec<(usize, Run)> {
    let mut seeded = Seeded(0x0B55_0056);
    (0..count)
        .map(|step| {
            let nodes = seeded_nodes(&mut seeded);
            if step % 3 == 2 {
                let refs: Vec<(String, String)> = (0..seeded.below(3))
                    .map(|_| {
                        (
                            (*seeded.pick(IDS)).to_owned(),
                            (*seeded.pick(VERSIONS)).to_owned(),
                        )
                    })
                    .collect();
                (IMPORTERS, importers_run(&nodes, &refs))
            } else {
                let depth = u64::try_from(seeded.below(6)).expect("a depth");
                (GRAPH, graph_run(&nodes, depth))
            }
        })
        .collect()
}

// ------------------------------------------------------------------- plants

struct Plant {
    label: &'static str,
    site: &'static str,
    replacement: &'static str,
    vector: &'static str,
}

const PLANTS: &[Plant] = &[
    Plant {
        label: "a duplicate package is accepted",
        site: "(if (LexLeanStd.Map::setContains (first state) (project package id)) then",
        replacement: "(if false then",
        vector: "a duplicate package is refused",
    },
    Plant {
        label: "an import at another version is accepted",
        site: "(if (prim equal (project target version) (second import) : Bool) then kept else",
        replacement: "(if true then kept else",
        vector: "an import at another version is refused",
    },
    Plant {
        label: "an absent import is accepted",
        site: "| Option.none => (cons (plainDiag (lit \"LLR3001\") (cat shown (lit \" is not available\"))) kept)",
        replacement: "| Option.none => kept",
        vector: "an absent import is refused",
    },
    Plant {
        label: "a cycle is walked as a new package",
        site: "(if (state =? 1) then",
        replacement: "(if false then",
        vector: "a two-package cycle",
    },
    Plant {
        label: "the depth limit is off by one",
        site: "(if (maxImportDepth <=? deepest) then",
        replacement: "(if (maxImportDepth <? deepest) then",
        vector: "a chain at the depth limit is refused",
    },
    Plant {
        label: "the observed depth is reported without the package itself",
        site: "(LexLeanStd.Numeral::decimal (deepest + 1))",
        replacement: "(LexLeanStd.Numeral::decimal deepest)",
        vector: "a chain at the depth limit is refused",
    },
    Plant {
        label: "the depth is the last import's, not the longest",
        site: "(if (deepest <? reached) then reached else deepest)",
        replacement: "reached",
        vector: "the depth is the longest import chain",
    },
    Plant {
        label: "the import closure is not transitive",
        site: "(cons ((project package id), (closureOf closures package)) closures)",
        replacement: "(cons ((project package id), (closureOf (nil (Bytes * (List Bytes))) package)) closures)",
        vector: "a chain is ordered imports first",
    },
    Plant {
        label: "importers are reached one step only",
        site: "((prim length edges : Nat) + 2)",
        replacement: "1",
        vector: "importers are reached transitively",
    },
    Plant {
        label: "an import at another version reaches its importer",
        site: "(if (prim equal version (second import) : Bool) then (cons",
        replacement: "(if true then (cons",
        vector: "an import at another version reaches no importer",
    },
    Plant {
        label: "a duplicated package contributes edges",
        site: "(if ((countIn packages (project package id)) =? 1) then (prim list_fold",
        replacement: "(if ((countIn packages (project package id)) <=? 2) then (prim list_fold",
        vector: "a duplicated package contributes no edge",
    },
    Plant {
        label: "a cycle does not name its importers",
        site: "(ids, (packageImporters packages members))",
        replacement: "(ids, (nil Bytes))",
        vector: "a cycle names the packages importing it",
    },
];

fn plants(vectors: &[Vector]) {
    let mut missed = Vec::new();
    for plant in PLANTS {
        let vector = vectors
            .iter()
            .find(|vector| vector.label == plant.vector)
            .unwrap_or_else(|| panic!("{}: no vector `{}`", plant.label, plant.vector));
        let planted = support::P::bootstrap();
        let source = planted.read_raw(SOURCE);
        assert_eq!(
            source.matches(plant.site).count(),
            1,
            "{}: the plant site occurs once",
            plant.label
        );
        planted.write(SOURCE, &source.replacen(plant.site, plant.replacement, 1));
        let checked = support::checked_project(&planted);
        let low = super::lower(&checked, &planted, MODULE, ROOTS[vector.root].0);
        match super::evaluate(&low, std::slice::from_ref(&vector.run), 1).first() {
            Some(first) => eprintln!(
                "BS-56: plant detected: {}: {}",
                plant.label,
                first.chars().take(160).collect::<String>()
            ),
            None => missed.push(plant.label),
        }
    }
    assert!(
        missed.is_empty(),
        "planted defects their own vector does not detect: {missed:?}"
    );
}

// --------------------------------------------------------------------- case

fn export(lowered: &BTreeMap<&str, super::Lowered>) {
    let exported = harness::exported("bootstrap");
    let bundle = exported.bundle();
    let mut realized = 0usize;
    for exported_root in &exported.output.result.roots {
        let Some((member, low)) = exported_root
            .root
            .strip_prefix(&format!("{MODULE}."))
            .and_then(|member| lowered.get(member).map(|low| (member, low)))
        else {
            continue;
        };
        let largest = exported_root
            .outputs
            .iter()
            .flat_map(|output| &output.certification.certificates)
            .map(|certificate| certificate.file.byte_length)
            .max()
            .expect("a certificate");
        eprintln!(
            "BS-56: {}: largest certificate part {largest} bytes",
            exported_root.root
        );
        assert!(
            largest <= CERTIFICATE_BUDGET,
            "{}: a certificate part of {largest} bytes is beyond {CERTIFICATE_BUDGET}",
            exported_root.root
        );
        for output in &exported_root.outputs {
            let package: Json2 = serde_json::from_slice(
                &std::fs::read(
                    bundle
                        .join(&output.package.directory)
                        .join("package.json")
                        .as_std_path(),
                )
                .expect("package.json"),
            )
            .expect("package JSON");
            assert_eq!(
                package["program"],
                serde_json::to_value(&low.program).expect("a program is JSON"),
                "{member}: the exported package realizes the program the differential ran"
            );
            realized += 1;
        }
    }
    assert_eq!(realized, ROOTS.len(), "one certified package per root");
    let packages = harness::tree(&bundle.join("packages"));
    let committed_packages = harness::tree(&harness::fixture_dir("bootstrap").join("packages"));
    assert_eq!(
        packages.keys().collect::<Vec<_>>(),
        committed_packages.keys().collect::<Vec<_>>(),
        "the exported package files are the committed ones"
    );
    for (relative, content) in &packages {
        assert!(
            committed_packages.get(relative) == Some(content),
            "packages/{relative} differs from the committed package"
        );
    }
}

#[allow(clippy::too_many_lines)]
pub(super) fn case() {
    let started = std::time::Instant::now();
    let project = support::P::bootstrap();
    let checked_units = project.check_ok();
    assert!(
        checked_units.units.contains_key("LexiconGraph"),
        "LexiconGraph is a linked module of the project"
    );
    eligibility_of(&project, "LexiconGraph", MODULE, &ROOTS);

    let vectors = named();
    let labels: std::collections::BTreeSet<&str> =
        vectors.iter().map(|vector| vector.label).collect();
    assert_eq!(labels.len(), vectors.len(), "each vector has its own label");
    let refused_codes: std::collections::BTreeSet<&str> = vectors
        .iter()
        .flat_map(|vector| vector.codes.iter().copied())
        .collect();
    for code in ["LLR3001", "LLR3002", "LLR3003", "LLS8002"] {
        assert!(
            refused_codes.contains(code),
            "a named vector reaches {code}: {refused_codes:?}"
        );
    }

    let all = manifests();
    assert!(
        all.len() >= 20,
        "the repository commits {} lexicon manifests",
        all.len()
    );
    let committed: Vec<(usize, Run)> = committed_graphs(&all)
        .iter()
        .map(|graph| (GRAPH, graph_run(graph, 128)))
        .collect();
    eprintln!(
        "BS-56: {} vectors, {} committed graphs of {} manifests ({:?})",
        vectors.len(),
        committed.len(),
        all.len(),
        started.elapsed()
    );

    let normative = support::normative_host();
    let evaluator_seeded = if normative {
        seeded_runs(EVALUATOR_SEEDED)
    } else {
        Vec::new()
    };
    let rustc_seeded = if normative {
        seeded_runs(RUSTC_SEEDED)
    } else {
        Vec::new()
    };

    let checked = support::checked_project(&project);
    let lowered: BTreeMap<&str, super::Lowered> = ROOTS
        .iter()
        .map(|(root, _)| (*root, super::lower(&checked, &project, MODULE, root)))
        .collect();

    let by_root = |root: usize, extra: &[(usize, Run)]| -> Vec<Run> {
        let mut runs: Vec<Run> = vectors
            .iter()
            .filter(|vector| vector.root == root)
            .map(|vector| vector.run.clone())
            .collect();
        runs.extend(
            committed
                .iter()
                .filter(|(at, _)| *at == root)
                .map(|(_, run)| run.clone()),
        );
        runs.extend(
            extra
                .iter()
                .filter(|(at, _)| *at == root)
                .map(|(_, run)| run.clone()),
        );
        runs
    };

    if normative {
        for (index, (root, _)) in ROOTS.iter().enumerate() {
            let runs = by_root(index, &evaluator_seeded);
            let disagreements = super::evaluate(&lowered[root], &runs, 5);
            assert!(
                disagreements.is_empty(),
                "{root} and the oracle disagree: {disagreements:?}"
            );
            eprintln!(
                "BS-56: {root}: the evaluator agrees on {} runs ({:?})",
                runs.len(),
                started.elapsed()
            );
        }
        plants(&vectors);
    } else {
        eprintln!("BS-56: platform-bound host: the evaluator, the seeded runs, and the planted defects run on the normative host");
    }

    let graph_runs = by_root(GRAPH, &rustc_seeded);
    let importer_runs = by_root(IMPORTERS, &rustc_seeded);
    let suites: Vec<(&str, &super::Lowered, &[Run])> = vec![
        (
            "packageGraph",
            &lowered["packageGraph"],
            graph_runs.as_slice(),
        ),
        (
            "packageImporters",
            &lowered["packageImporters"],
            importer_runs.as_slice(),
        ),
    ];
    let total = rustc_differential("BS-56", &suites, "resolve::package_graph");
    eprintln!("BS-56: {total} rustc runs agree ({:?})", started.elapsed());

    if !support::lean_backed("BS-56") {
        return;
    }
    export(&lowered);
    eprintln!("BS-56: passed in {:?}", started.elapsed());
}
