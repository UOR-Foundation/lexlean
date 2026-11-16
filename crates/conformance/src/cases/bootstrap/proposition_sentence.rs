//! `BS-44` (SPEC.md §17.23, §14.1, §15.3): `phraseCovers`, the depth-first
//! lattice walk over a phrase's complete covers, held to the byte to
//! `lexlean::grammar::proposition::phrase_covers`.
//!
//! The root reads the tokens of `BS-35` (`GrammarProof`) and the atoms of
//! `BS-03`, and shares `PropositionTerms`'s budget/edge memo (`BS-43`). The
//! glossary closure is the lexicon's, so the host gives it its projection:
//! the text-channel edges at each atom position, each with its form's
//! category (the extended set §13.3 adds past `PropositionTerms`'s: label
//! words and term constants in addition to the type/function nouns) and
//! whether the form is inflected.
//!
//! Each planted defect names the one vector that catches it.

use std::collections::BTreeSet;
use std::sync::OnceLock;

use lexlean::calculus::Value;
use lexlean::grammar::chart::{text_tokens, Budget, TextToken};
use lexlean::grammar::proposition::{phrase_covers, CoverFlow, PhraseItemAst, PropAst};
use lexlean::lexicon::entry::{Category, Channel};
use lexlean::lexicon::package::LoadContext;
use lexlean::lexicon::resolve::Closure;
use lexlean::source::atom::Atom;
use serde_json::Value as Json;

use super::proposition_terms::{binder_value, keywords_value, term_value, Forms};
use super::rust_validate::{eligibility_of, rustc_differential};
use super::scan::{atom_value, rejection_value};
use super::{bool_val, list, nat_val, text, Run};
use crate::production_export as harness;
use crate::support::{self, repo_root};

const MODULE: &str = "LexLeanBoot.PropositionSentence";
const SOURCE: &str = "src/PropositionSentence.lex.tex";
const ROOTS: [(&str, &[&str]); 2] = [
    ("phraseCovers", &["allocation", "overflow"]),
    ("propositionSentenceAt", &["allocation", "overflow"]),
];
/// The same committed example glossary `PropositionTerms` (`BS-43`) reads,
/// reused here so the case shares one closure shape with it rather than
/// inventing a second.
const GLOSSARY: &str = "examples/peano-arithmetic/glossary/peano.arith";
/// A second, real committed package, reused only for the "a cover limit of
/// one" vector: `nna`'s surface "Natural number addition" is a genuine
/// prefix-ambiguous overlap with `lexlean.std.nat`'s own "Natural number"
/// (`nat`) and "addition" (`addition`) forms, already proven to produce two
/// distinct covers by the `ambiguous-lexical-segmentation` negative test
/// (`LLP2002`: "2 distinct linked interpretations ... lexlean.std.nat::addition,
/// lexlean.std.nat::nat, test.seg::nna"). No committed glossary this case
/// already loads has that property on its own (checked: every multi-word
/// surface in `peano.arith`/`lexlean.core`/`lexlean.std.{nat,bool,int}` is
/// either one word or, for `nat`/`nat-list`'s "natural number"/"natural
/// number list" prefix, has no standalone continuation word to complete a
/// second cover), so this reuses the one that does instead of adding to the
/// Peano glossary.
const SEG_GLOSSARY: &str = "tests/negative/ambiguous-lexical-segmentation/project/lexicons/test-seg";
const VISIBLE: [&str; 4] = ["lexlean.core", "lexlean.std.nat", "peano.arith", "test.seg"];

const CERTIFICATE_BUDGET: u64 = 3_145_728;
const PATH: &str = "src/Phrase.lex.tex";
const MAX_EDGES: u64 = 4_000_000;
const MAX_STATES: u64 = 4_000_000;
const MAX_DEPTH: u64 = 1_024;

// ------------------------------------------------------------ the closure

fn closure() -> &'static Closure {
    static CLOSURE: OnceLock<Closure> = OnceLock::new();
    CLOSURE.get_or_init(|| {
        let bootstrap = lexlean::lexicon::load_bootstrap().expect("bootstrap loads");
        let ctx = LoadContext {
            language: lexlean::LANGUAGE_VERSION,
            forbidden_controls: &bootstrap.structural.forbidden_controls,
            max_scope_depth: 1024,
        };
        let mut packages: Vec<_> = bootstrap
            .builtin_packages
            .iter()
            .map(|row| lexlean::lexicon::load_builtin_package(row, &ctx).expect("a package"))
            .collect();
        packages.push(
            load_glossary_files(GLOSSARY, &ctx)
                .unwrap_or_else(|diagnostics| panic!("the Peano glossary: {diagnostics:?}")),
        );
        packages.push(
            load_glossary_files(SEG_GLOSSARY, &ctx)
                .unwrap_or_else(|diagnostics| panic!("the ambiguous-segmentation glossary: {diagnostics:?}")),
        );
        let registry = lexlean::lexicon::load_token_registry().expect("the registry");
        Closure::build(packages, registry, bootstrap, 128)
            .unwrap_or_else(|diagnostics| panic!("the case's closure: {diagnostics:?}"))
    })
}

fn visible() -> BTreeSet<String> {
    VISIBLE
        .iter()
        .map(|package| (*package).to_owned())
        .collect()
}

/// Reads one glossary package's `lexicon.toml` and `entries/*.toml` files
/// off disk, the same way for every package this case loads.
fn load_glossary_files(
    display_root: &str,
    ctx: &LoadContext<'_>,
) -> Result<lexlean::lexicon::package::LexiconPackage, Vec<lexlean::diagnostic::Diagnostic>> {
    let root = repo_root().join(display_root);
    let mut files: Vec<(String, Vec<u8>)> = walkdir::WalkDir::new(root.as_std_path())
        .into_iter()
        .flatten()
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| {
            let relative = entry
                .path()
                .strip_prefix(root.as_std_path())
                .expect("under the package")
                .to_string_lossy()
                .replace('\\', "/");
            (
                relative,
                std::fs::read(entry.path()).expect("a glossary file"),
            )
        })
        .filter(|(relative, _)| relative == "lexicon.toml" || relative.starts_with("entries/"))
        .collect();
    files.sort_by(|a, b| a.0.as_bytes().cmp(b.0.as_bytes()));
    lexlean::lexicon::package::load_package(display_root, &files, None, ctx)
}

// ------------------------------------------------------------- the encoders

/// The extended category set (SPEC.md §13.3) `phraseCovers` and
/// `propositionSentence` admit as a phrase concept or a predicate frame, past
/// the type/function nouns `PropositionTerms` already uses: label words (6)
/// and term constants (7) are committed and exercised below (`sum`'s a
/// binary-noun-function 2, `successor`'s a noun-function 1, `zero`'s a
/// term-constant 7, and the Peano package's many `label-word` entries are 6).
/// Adjective (3), intransitive (4), and transitive (5) predicates are
/// `propositionSentence`'s own range, chosen fresh in this module and kept to
/// one encoding across every root it holds, since `predicate_frame` and
/// `formMatches` both read the same projected edges `phraseCovers` built;
/// `even`/`positive` (adjective) and `divides` (transitive) are committed and
/// exercised below, but the glossary has no intransitive-predicate entry, so
/// 4 is never populated by this case. `Function`/`PrefixFunction`/
/// `PostfixFunction`/`InfixFunction` (10-13) are math-channel notation only
/// everywhere in this repository today, so no committed text-channel form
/// reaches them either; the range check at 10..13 still runs on every match
/// (it costs one more `formMatches` call), it is simply never populated by
/// this case's glossary. 8 is the unresolved default (matches
/// `PropositionTerms`'s own).
fn category_code(category: Category) -> usize {
    match category {
        Category::TypeNoun => 0,
        Category::NounFunction => 1,
        Category::BinaryNounFunction => 2,
        Category::AdjectivePredicate => 3,
        Category::IntransitivePredicate => 4,
        Category::TransitivePredicate => 5,
        Category::LabelWord => 6,
        Category::TermConstant => 7,
        Category::Function => 10,
        Category::PrefixFunction => 11,
        Category::PostfixFunction => 12,
        Category::InfixFunction => 13,
        _ => 8,
    }
}

fn edges_value(atoms: &[Atom], forms: &mut Forms) -> Value {
    let closure = closure();
    let visible = visible();
    list(
        (0..atoms.len())
            .map(|position| {
                list(
                    closure
                        .matches_at(atoms, position, Channel::Text, &visible)
                        .iter()
                        .map(|(form, end)| {
                            let (category, inflected) =
                                closure.form(form).map_or((9, false), |(entry, shape)| {
                                    (
                                        category_code(entry.category),
                                        shape.features.iter().any(|feature| {
                                            feature == "plural" || feature == "sentence-case"
                                        }),
                                    )
                                });
                            Value::Adt {
                                constructor: 0,
                                fields: vec![
                                    nat_val(forms.index(form)),
                                    nat_val(*end),
                                    nat_val(category),
                                    bool_val(inflected),
                                ],
                            }
                        })
                        .collect(),
                )
            })
            .collect(),
    )
}

fn adt(constructor: u64, fields: Vec<Value>) -> Value {
    Value::Adt {
        constructor,
        fields,
    }
}

fn token_value(token: &TextToken) -> Value {
    match token {
        TextToken::Atom(index) => adt(0, vec![nat_val(*index)]),
        TextToken::Island {
            open,
            inner_start,
            inner_end,
            close,
            display,
        } => adt(
            1,
            vec![
                nat_val(*open),
                nat_val(*inner_start),
                nat_val(*inner_end),
                nat_val(*close),
                bool_val(*display),
            ],
        ),
    }
}

/// A `PropAst`, matching `PropAst`'s constructor order in
/// `PropositionSentence.lex.tex`: `PACond`, `PAIff`, `PAImplies`, `PAOr`,
/// `PAAnd`, `PANot`, `PAForAll`, `PAExists`, `PAMath`, `PAPred`.
fn prop_value(prop: &PropAst, forms: &Forms) -> Value {
    match prop {
        PropAst::Conditional {
            antecedent,
            consequent,
            keywords,
        } => adt(
            0,
            vec![
                prop_value(antecedent, forms),
                prop_value(consequent, forms),
                keywords_value(keywords),
            ],
        ),
        PropAst::Iff {
            left,
            right,
            keywords,
        } => adt(
            1,
            vec![
                prop_value(left, forms),
                prop_value(right, forms),
                keywords_value(keywords),
            ],
        ),
        PropAst::Implies {
            left,
            right,
            keywords,
        } => adt(
            2,
            vec![
                prop_value(left, forms),
                prop_value(right, forms),
                keywords_value(keywords),
            ],
        ),
        PropAst::Or { items, keywords } => adt(
            3,
            vec![
                list(items.iter().map(|item| prop_value(item, forms)).collect()),
                keywords_value(keywords),
            ],
        ),
        PropAst::And { items, keywords } => adt(
            4,
            vec![
                list(items.iter().map(|item| prop_value(item, forms)).collect()),
                keywords_value(keywords),
            ],
        ),
        PropAst::Not { inner, keywords } => adt(
            5,
            vec![prop_value(inner, forms), keywords_value(keywords)],
        ),
        PropAst::ForAll {
            binders,
            body,
            keywords,
        } => adt(
            6,
            vec![
                list(
                    binders
                        .iter()
                        .map(|binder| binder_value(binder, forms))
                        .collect(),
                ),
                prop_value(body, forms),
                keywords_value(keywords),
            ],
        ),
        PropAst::Exists {
            binder,
            unique,
            body,
            keywords,
        } => adt(
            7,
            vec![
                binder_value(binder, forms),
                bool_val(*unique),
                prop_value(body, forms),
                keywords_value(keywords),
            ],
        ),
        PropAst::Math(token) => adt(8, vec![token_value(token)]),
        PropAst::Predicate {
            candidates,
            surface_atoms,
            args,
            keywords,
        } => adt(
            9,
            vec![
                list(
                    candidates
                        .iter()
                        .map(|form| nat_val(forms.known(form)))
                        .collect(),
                ),
                nat_val(surface_atoms.0),
                nat_val(surface_atoms.1),
                list(
                    args.iter()
                        .map(|arg| term_value(arg, forms))
                        .collect(),
                ),
                keywords_value(keywords),
            ],
        ),
    }
}

/// A `PhraseItemAst`, matching `PhraseItem`'s constructor order in
/// `PropositionSentence.lex.tex`: `PWord`, `PMath`, `PTermPhrase`, `PPunct`.
fn phrase_item_value(item: &PhraseItemAst, forms: &Forms) -> Value {
    match item {
        PhraseItemAst::Word { candidates, atoms } => adt(
            0,
            vec![
                list(
                    candidates
                        .iter()
                        .map(|form| nat_val(forms.known(form)))
                        .collect(),
                ),
                nat_val(atoms.0),
                nat_val(atoms.1),
            ],
        ),
        PhraseItemAst::Math(token) => adt(1, vec![token_value(token)]),
        PhraseItemAst::TermPhrase(term) => adt(2, vec![term_value(term, forms)]),
        PhraseItemAst::Punctuation { atom, entry } => {
            let code: usize = match *entry {
                "colon" => 0,
                "hyphen" => 1,
                "paren-open" => 2,
                "paren-close" => 3,
                other => panic!("phrase punctuation entry `{other}` has no case code"),
            };
            adt(3, vec![nat_val(*atom), nat_val(code)])
        }
    }
}

fn sentence(source: &str) -> (Vec<Atom>, Vec<TextToken>) {
    let atoms = lexlean::source::scan::scan(PATH, source, u64::MAX)
        .unwrap_or_else(|failure| panic!("{source:?}: {failure:?}"));
    let tokens = text_tokens(PATH, &atoms, 0, atoms.len())
        .unwrap_or_else(|failure| panic!("{source:?}: {failure:?}"));
    (atoms, tokens)
}

// ------------------------------------------------------------------ vectors

#[derive(Clone, Copy)]
struct Limits {
    edges: u64,
    states: u64,
    depth: u64,
    states_before: u64,
    edges_before: u64,
}

const DEFAULT_LIMITS: Limits = Limits {
    edges: MAX_EDGES,
    states: MAX_STATES,
    depth: MAX_DEPTH,
    states_before: 0,
    edges_before: 0,
};

/// The run of `phraseCovers` on a phrase's atoms and tokens, and the
/// oracle's refusal message when it refuses. `phrase_covers` is callback
/// shaped in Rust (it stops as soon as the consumer has enough covers); the
/// language-1.2 root instead returns up to `limit` covers at once (0 = all),
/// so the oracle side collects at most `limit` covers the same way before
/// comparing.
fn phrase_run_on(
    path: &str,
    atoms: &[Atom],
    tokens: &[TextToken],
    limit: u64,
    limits: Limits,
) -> (Run, Option<String>) {
    let mut forms = Forms::default();
    let edges = edges_value(atoms, &mut forms);
    let mut budget = Budget::new(limits.edges, limits.states, limits.depth, u64::MAX);
    for _ in 0..limits.states_before {
        budget
            .state()
            .expect("the states before are within the limit");
    }
    let parser = lexlean::grammar::proposition::TextParser {
        path,
        atoms,
        tokens,
        closure: closure(),
        visible: &visible(),
        sentence_initial: true,
    };
    let mut covers: Vec<Vec<PhraseItemAst>> = Vec::new();
    let result = phrase_covers(&parser, &mut budget, |items, _budget| {
        covers.push(items);
        if limit != 0 && covers.len() as u64 >= limit {
            Ok(CoverFlow::Stop)
        } else {
            Ok(CoverFlow::Continue)
        }
    });
    let (value, refusal) = match result {
        Ok(()) => (
            Value::Ok {
                value: Box::new(list(
                    covers
                        .iter()
                        .map(|cover| {
                            list(
                                cover
                                    .iter()
                                    .map(|item| phrase_item_value(item, &forms))
                                    .collect(),
                            )
                        })
                        .collect(),
                )),
            },
            None,
        ),
        Err(diagnostic) => (
            Value::Error {
                value: Box::new(rejection_value(&diagnostic)),
            },
            Some(format!(
                "{} {}",
                diagnostic.code.as_str(),
                diagnostic.message
            )),
        ),
    };
    (
        (
            vec![
                text(path),
                list(atoms.iter().map(atom_value).collect()),
                list(tokens.iter().map(token_value).collect()),
                edges,
                nat_val(limits.edges as usize),
                nat_val(limits.states as usize),
                nat_val(limits.depth as usize),
                nat_val(limits.states_before as usize),
                nat_val(limits.edges_before as usize),
                nat_val(limit as usize),
            ],
            serde_json::to_value(value).expect("a value is JSON"),
        ),
        refusal,
    )
}

/// The run of `propositionSentenceAt` on a phrase's atoms and tokens, and
/// the oracle's refusal message when it refuses. Unlike `phraseCovers`,
/// `proposition_sentence` returns every full-consumption parse at once
/// (there is no cover-limit concept here), so the run just calls it.
fn prop_run_on(path: &str, atoms: &[Atom], tokens: &[TextToken], limits: Limits) -> (Run, Option<String>) {
    let mut forms = Forms::default();
    let edges = edges_value(atoms, &mut forms);
    let mut budget = Budget::new(limits.edges, limits.states, limits.depth, u64::MAX);
    for _ in 0..limits.states_before {
        budget
            .state()
            .expect("the states before are within the limit");
    }
    let visible = visible();
    let parser = lexlean::grammar::proposition::TextParser {
        path,
        atoms,
        tokens,
        closure: closure(),
        visible: &visible,
        sentence_initial: true,
    };
    let result = parser.proposition_sentence(&mut budget);
    let (value, refusal) = match result {
        Ok(props) => (
            Value::Ok {
                value: Box::new(list(props.iter().map(|prop| prop_value(prop, &forms)).collect())),
            },
            None,
        ),
        Err(diagnostic) => (
            Value::Error {
                value: Box::new(rejection_value(&diagnostic)),
            },
            Some(format!(
                "{} {}",
                diagnostic.code.as_str(),
                diagnostic.message
            )),
        ),
    };
    (
        (
            vec![
                text(path),
                list(atoms.iter().map(atom_value).collect()),
                list(tokens.iter().map(token_value).collect()),
                edges,
                nat_val(limits.edges as usize),
                nat_val(limits.states as usize),
                nat_val(limits.depth as usize),
                nat_val(limits.states_before as usize),
                nat_val(limits.edges_before as usize),
            ],
            serde_json::to_value(value).expect("a value is JSON"),
        ),
        refusal,
    )
}

struct Vector {
    label: &'static str,
    source: &'static str,
    limit: u64,
}

impl Vector {
    fn run(&self) -> (Run, Option<String>) {
        let (atoms, tokens) = sentence(self.source);
        phrase_run_on(PATH, &atoms, &tokens, self.limit, DEFAULT_LIMITS)
    }
}

/// Every named vector: one phrase concept kind at a time, then the lattice
/// behaviors (a punctuation run, a `the ... of ...` noun-of phrase, a cover
/// limit that truncates, and `limit = 0` reading every cover) a single
/// concept kind cannot reach alone.
fn named() -> Vec<Vector> {
    vec![
        Vector {
            label: "an empty phrase",
            source: "",
            limit: 0,
        },
        Vector {
            label: "a single label word",
            source: "doubling",
            limit: 0,
        },
        Vector {
            label: "a single type noun",
            source: "count",
            limit: 0,
        },
        Vector {
            label: "a noun-function word",
            source: "successor",
            limit: 0,
        },
        Vector {
            label: "a binary-noun-function word",
            source: "sum",
            limit: 0,
        },
        Vector {
            label: "a term-constant word",
            source: "zero",
            limit: 0,
        },
        Vector {
            label: "a math island alone",
            source: "\\(zero\\)",
            limit: 0,
        },
        Vector {
            label: "two label words in sequence",
            source: "doubling parity",
            limit: 0,
        },
        Vector {
            label: "a colon and a hyphen",
            source: "doubling : parity - sums",
            limit: 0,
        },
        Vector {
            label: "a parenthesized label word",
            source: "( doubling )",
            limit: 0,
        },
        Vector {
            label: "the successor of a math island",
            source: "the successor of \\(zero\\)",
            limit: 0,
        },
        Vector {
            label: "the sum of two math islands",
            source: "the sum of \\(zero\\) and \\(zero\\)",
            limit: 0,
        },
        Vector {
            label: "a cover limit of one",
            // Genuinely ambiguous (not just a guess): "Natural number
            // addition" has two covers under `VISIBLE` -- `test.seg::nna`'s
            // own 3-word form, and `lexlean.std.nat`'s "Natural number"
            // (`nat`) followed by "addition" (`addition`) -- so `limit: 1`
            // actually exercises the early stop, unlike "doubling parity"
            // (confirmed via `explore_bs44_limit_budget` to have only one
            // cover at all, so the planted defect was never observable on
            // it).
            source: "Natural number addition",
            limit: 1,
        },
        Vector {
            label: "an unknown atom",
            source: "doubling glorp",
            limit: 0,
        },
    ]
}

struct PropVector {
    label: &'static str,
    source: &'static str,
    limits: Limits,
}

impl PropVector {
    fn run(&self) -> (Run, Option<String>) {
        let (atoms, tokens) = sentence(self.source);
        prop_run_on(PATH, &atoms, &tokens, self.limits)
    }
}

/// Every named vector for `propositionSentenceAt`: one connective, one
/// quantifier, and one predicate kind at a time against the committed
/// Peano/Order/Divisibility vocabulary (`even`, `positive`: adjective;
/// `divides`: transitive; no committed intransitive-predicate entry exists,
/// so code 4 is never populated here, matching `category_code`'s own note),
/// then the two budget boundaries: `max_parse_states` and
/// `max_token_lattice_edges` exceeded through cumulative consumption across
/// repeated calls into the opaque `termPhraseAt`/`binderListAt` boundary
/// (`divides`'s two term-phrase calls per conjunct, and the second `and`-joined
/// binder of a `for every` binder list), each asserted against the oracle's
/// own pre-charged budget so both sides refuse by the same margin.
fn prop_named() -> Vec<PropVector> {
    vec![
        PropVector {
            label: "a bare predicate",
            source: "\\(zero\\) divides \\(zero\\)",
            limits: DEFAULT_LIMITS,
        },
        PropVector {
            label: "an adjective predicate",
            source: "\\(zero\\) is even",
            limits: DEFAULT_LIMITS,
        },
        PropVector {
            label: "a conjunction of two predicates",
            source: "\\(zero\\) is even and \\(zero\\) is positive",
            limits: DEFAULT_LIMITS,
        },
        PropVector {
            label: "a disjunction of two predicates",
            source: "\\(zero\\) is even or \\(zero\\) is positive",
            limits: DEFAULT_LIMITS,
        },
        PropVector {
            label: "a negation",
            source: "not \\(zero\\) is even",
            limits: DEFAULT_LIMITS,
        },
        PropVector {
            label: "an implication",
            source: "\\(zero\\) is even implies \\(zero\\) is positive",
            limits: DEFAULT_LIMITS,
        },
        PropVector {
            label: "an if and only if",
            source: "\\(zero\\) is even if and only if \\(zero\\) is positive",
            limits: DEFAULT_LIMITS,
        },
        PropVector {
            label: "a conditional",
            source: "if \\(zero\\) is even, then \\(zero\\) is positive",
            limits: DEFAULT_LIMITS,
        },
        PropVector {
            label: "a universal quantifier over one binder",
            source: "for every natural number \\(n\\), \\(n\\) is even",
            limits: DEFAULT_LIMITS,
        },
        PropVector {
            label: "a universal quantifier over two and-joined binders",
            source: "for every natural number \\(n\\) and natural number \\(m\\), \\(n\\) divides \\(m\\)",
            limits: DEFAULT_LIMITS,
        },
        PropVector {
            label: "an existential quantifier",
            source: "there exists a natural number \\(n\\) such that \\(n\\) is even",
            limits: DEFAULT_LIMITS,
        },
        PropVector {
            label: "a unique existential quantifier",
            source: "there exists exactly one natural number \\(n\\) such that \\(n\\) is even",
            limits: DEFAULT_LIMITS,
        },
        PropVector {
            label: "a math-island proposition",
            source: "\\(zero = zero\\)",
            limits: DEFAULT_LIMITS,
        },
        // Derived empirically (explore_bs44), not hand-counted: this
        // sentence needs 11 states and 2 edges to complete (10/1 refuse,
        // 11/2 succeed), so `states: 10` and `edges: 1` refuse by the
        // smallest margin that still comes from cumulative consumption
        // across the sentence's two `divides` conjuncts, not from any one
        // call alone.
        PropVector {
            label: "max_parse_states exceeded through cumulative consumption across repeated calls",
            source: "\\(zero\\) divides \\(zero\\) and \\(zero\\) divides \\(zero\\)",
            limits: Limits {
                states: 10,
                ..DEFAULT_LIMITS
            },
        },
        PropVector {
            label: "max_token_lattice_edges exceeded through cumulative consumption across repeated calls",
            source: "\\(zero\\) divides \\(zero\\) and \\(zero\\) divides \\(zero\\)",
            limits: Limits {
                edges: 1,
                ..DEFAULT_LIMITS
            },
        },
        PropVector {
            label: "an unknown atom",
            source: "\\(zero\\) frobnicates \\(zero\\)",
            limits: DEFAULT_LIMITS,
        },
    ]
}

// --------------------------------------------------------------- the case

pub(crate) fn case() {
    let started = std::time::Instant::now();
    let project = support::P::bootstrap();
    let checked_units = project.check_ok();
    assert!(
        checked_units.units.contains_key("PropositionSentence"),
        "PropositionSentence is a linked module of the project"
    );
    eligibility_of(&project, "PropositionSentence", MODULE, &ROOTS);

    let vectors = named();
    let labels: BTreeSet<&str> = vectors.iter().map(|vector| vector.label).collect();
    assert_eq!(labels.len(), vectors.len(), "each vector has its own label");

    let mut runs: Vec<Run> = Vec::new();
    let mut messages: Vec<String> = Vec::new();
    for vector in &vectors {
        let (run, refusal) = vector.run();
        messages.extend(refusal);
        runs.push(run);
    }
    assert!(
        messages.iter().any(|message| message.starts_with("LLL1004")),
        "no vector reaches the unknown-atom refusal"
    );

    let prop_vectors = prop_named();
    let prop_labels: BTreeSet<&str> = prop_vectors.iter().map(|vector| vector.label).collect();
    assert_eq!(
        prop_labels.len(),
        prop_vectors.len(),
        "each propositionSentenceAt vector has its own label"
    );

    let mut prop_runs: Vec<Run> = Vec::new();
    let mut prop_messages: Vec<String> = Vec::new();
    for vector in &prop_vectors {
        let (run, refusal) = vector.run();
        prop_messages.extend(refusal);
        prop_runs.push(run);
    }
    assert!(
        prop_messages.iter().any(|message| message.starts_with("LLL1004")),
        "no propositionSentenceAt vector reaches the unknown-atom refusal"
    );
    assert!(
        prop_messages
            .iter()
            .any(|message| message.starts_with("LLS8002 max_parse_states")),
        "no propositionSentenceAt vector reaches max_parse_states"
    );
    assert!(
        prop_messages
            .iter()
            .any(|message| message.starts_with("LLS8002 max_token_lattice_edges")),
        "no propositionSentenceAt vector reaches max_token_lattice_edges"
    );

    let checked = support::checked_project(&project);
    let lowered = super::lower(&checked, &project, MODULE, ROOTS[0].0);
    let prop_lowered = super::lower(&checked, &project, MODULE, ROOTS[1].0);
    let normative = support::normative_host();
    if normative {
        let disagreements = super::evaluate(&lowered, &runs, 5);
        assert!(
            disagreements.is_empty(),
            "phraseCovers and the Rust oracle disagree: {disagreements:?}"
        );
        eprintln!(
            "BS-44: phraseCovers: the evaluator agrees on {} runs ({:?})",
            runs.len(),
            started.elapsed()
        );
        plants(&vectors);

        let prop_disagreements = super::evaluate(&prop_lowered, &prop_runs, 5);
        assert!(
            prop_disagreements.is_empty(),
            "propositionSentenceAt and the Rust oracle disagree: {prop_disagreements:?}"
        );
        eprintln!(
            "BS-44: propositionSentenceAt: the evaluator agrees on {} runs ({:?})",
            prop_runs.len(),
            started.elapsed()
        );
    } else {
        eprintln!("BS-44: platform-bound host: the evaluator and the planted defects run on the normative host");
    }

    let total = rustc_differential(
        "BS-44",
        &[
            (ROOTS[0].0, &lowered, runs.as_slice()),
            (ROOTS[1].0, &prop_lowered, prop_runs.as_slice()),
        ],
        "oracle",
    );
    eprintln!("BS-44: {total} rustc runs agree ({:?})", started.elapsed());

    if !support::lean_backed("BS-44") {
        return;
    }
    export(&lowered, ROOTS[0].0);
    export(&prop_lowered, ROOTS[1].0);
    eprintln!("BS-44: passed in {:?}", started.elapsed());
}

fn export(lowered: &super::Lowered, root_name: &str) {
    let exported = harness::exported("bootstrap");
    let bundle = exported.bundle();
    for exported_root in &exported.output.result.roots {
        let Some(member) = exported_root.root.strip_prefix(&format!("{MODULE}.")) else {
            continue;
        };
        if member != root_name {
            continue;
        }
        let largest = exported_root
            .outputs
            .iter()
            .flat_map(|output| &output.certification.certificates)
            .map(|certificate| certificate.file.byte_length)
            .max()
            .expect("a certificate");
        assert!(
            largest <= CERTIFICATE_BUDGET,
            "{}: a certificate part of {largest} bytes is beyond {CERTIFICATE_BUDGET}",
            exported_root.root
        );
        for output in &exported_root.outputs {
            let _package: Json = serde_json::from_slice(
                &std::fs::read(
                    bundle
                        .join(&output.package.directory)
                        .join("package.json")
                        .as_std_path(),
                )
                .expect("package.json"),
            )
            .expect("a JSON package");
        }
    }
    let _ = lowered;
}

// ------------------------------------------------------------------ plants

struct Plant {
    label: &'static str,
    site: &'static str,
    replacement: &'static str,
    vector: &'static str,
}

const PLANTS: &[Plant] = &[
    Plant {
        label: "the moves group never stops at the limit",
        site: "(PropositionTerms::both (0 <? limit) (limit <=? (prim length (project search covers) : Nat))) then (ctor Option.none",
        replacement: "false then (ctor Option.none",
        vector: "a cover limit of one",
    },
    Plant {
        label: "punctuation colon is dropped from the lattice",
        site: "(if (PropositionTerms::same text (PropositionTerms::lit \":\")) then (ctor Option.some [Nat] 0)",
        replacement: "(if false then (ctor Option.some [Nat] 0)",
        vector: "a colon and a hyphen",
    },
    Plant {
        label: "the noun-of branch is never tried",
        site: "(if (PropositionTerms::same text (PropositionTerms::lit \"the\")) then (match (PropositionTerms::termPhrase",
        replacement: "(if false then (match (PropositionTerms::termPhrase",
        vector: "the successor of a math island",
    },
    Plant {
        label: "an empty phrase is accepted",
        site: "(ctor Result.error [(List (List PhraseItem)), PrimitiveScan::Rejection] (PropositionTerms::placed (PropositionTerms::lit \"LLP2001\") (PropositionTerms::lit \"a phrase is a nonempty sequence of concepts\") (ctor Option.some [PrimitiveScan::Place] (wholeFilePlace path))))",
        replacement: "(let ctx : PropositionTerms::Ctx := (PropositionTerms::context path atoms tokens edges maxEdges maxStates maxDepth); (let final : CoverSearch := (coverSearchLoop ctx limit maxStates statesBefore edgesBefore); (match (project final error)\n      | Option.some problem => (ctor Result.error [(List (List PhraseItem)), PrimitiveScan::Rejection] problem)\n      | Option.none => (if (project final foundAny) then (ctor Result.ok [(List (List PhraseItem)), PrimitiveScan::Rejection] (project final covers)) else (ctor Result.error [(List (List PhraseItem)), PrimitiveScan::Rejection] (match (project final failure)\n          | Option.some dead => dead\n          | Option.none => (PropositionTerms::placed (PropositionTerms::lit \"LLP2001\") (PropositionTerms::lit \"a phrase is a nonempty sequence of concepts\") (ctor Option.some [PrimitiveScan::Place] (wholeFilePlace path)))))))))",
        vector: "an empty phrase",
    },
];

fn plants(vectors: &[Vector]) {
    let mut missed = Vec::new();
    for plant in PLANTS {
        let vector = vectors
            .iter()
            .find(|vector| vector.label == plant.vector)
            .unwrap_or_else(|| panic!("{}: no vector `{}`", plant.label, plant.vector));
        let run = vector.run().0;
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
        let low = super::lower(&checked, &planted, MODULE, ROOTS[0].0);
        match super::evaluate(&low, &[run], 1).first() {
            Some(first) => eprintln!(
                "BS-44: plant detected: {}: {}",
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

