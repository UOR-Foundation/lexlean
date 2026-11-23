//! `BS-55` (SPEC.md §17.23, §13.1 to §13.6): a lexicon entry as language-1.2
//! source, `LexLeanBoot.LexiconEntry`, held to the byte to
//! `lexicon::entry::parse_entry`: the full `Result<Entry, Vec<Diagnostic>>`,
//! not collapsed to `Option` — past its four hard gates the root keeps
//! validating every remaining section and returns every diagnostic the
//! oracle raises, in the oracle's own order, never only the first. One
//! diagnostic (the TOML-parse failure's own message) is approximated: its
//! code and its refusal are exact, its message exact only up to the fixed
//! prefix the host's own TOML reader carries. Every other diagnostic's code,
//! refusal, and message are exact. Each planted defect is detected by the
//! one vector it names, including a planted reordering and a planted
//! dropped second diagnostic.

use std::collections::BTreeMap;

use lexlean::calculus::Value;
use lexlean::lexicon::entry::{
    parse_entry, Associativity, Category, Channel, Denotation, Entry, EntryContext, Frame,
};
use lexlean::lexicon::lre::Render;
use lexlean::lexicon::lse::QualifiedId;
use lexlean::source::atom::{Atom, AtomClass};
use serde_json::Value as Json2;

use super::lexicon_lse::enc_lse;
use super::rust_validate::{eligibility_of, rustc_differential};
use super::{list, option, pair, text, Run, Seeded};
use crate::production_export as harness;
use crate::support::{self, repo_root};

const MODULE: &str = "LexLeanBoot.LexiconEntry";
const ROOTS: [(&str, &[&str]); 1] = [("parseEntry", &["allocation", "overflow", "recursion"])];

const CATEGORIES: [&str; 17] = [
    "structural",
    "grammar",
    "label-word",
    "type-noun",
    "term-constant",
    "function",
    "prefix-function",
    "postfix-function",
    "infix-function",
    "noun-function",
    "binary-noun-function",
    "predicate-constant",
    "adjective-predicate",
    "intransitive-predicate",
    "transitive-predicate",
    "infix-predicate",
    "proof-constant",
];
const FRAMES: [&str; 10] = [
    "atom",
    "call",
    "prefix",
    "postfix",
    "infix",
    "noun-of",
    "binary-noun-of",
    "adjective",
    "intransitive",
    "transitive",
];
const CHANNELS: [&str; 3] = ["text", "math", "both"];
const ASSOCIATIVITIES: [&str; 3] = ["left", "right", "none"];
const ATOM_CLASSES: [AtomClass; 7] = [
    AtomClass::Control,
    AtomClass::Word,
    AtomClass::Numeral,
    AtomClass::Delimiter,
    AtomClass::AsciiSymbol,
    AtomClass::UnicodeSymbol,
    AtomClass::Whitespace,
];

/// The default `max_scope_depth` committed entries and most vectors run
/// under: the project's own configured limit.
const DEPTH: u64 = 1024;
const EVALUATOR_SEEDED: usize = 150;
const RUSTC_SEEDED: usize = 1_200;

// --------------------------------------------------------------- encoders

fn nat(value: u64) -> Value {
    Value::Nat {
        value: value.to_string(),
    }
}

fn category_index(category: Category) -> u64 {
    CATEGORIES
        .iter()
        .position(|candidate| *candidate == category.as_str())
        .expect("a category index") as u64
}

fn frame_index(frame: Frame) -> u64 {
    FRAMES
        .iter()
        .position(|candidate| *candidate == frame.as_str())
        .expect("a frame index") as u64
}

fn channel_index(channel: Channel) -> u64 {
    let name = match channel {
        Channel::Text => "text",
        Channel::Math => "math",
        Channel::Both => "both",
    };
    CHANNELS
        .iter()
        .position(|candidate| *candidate == name)
        .expect("a channel index") as u64
}

fn associativity_index(associativity: Associativity) -> u64 {
    let name = match associativity {
        Associativity::Left => "left",
        Associativity::Right => "right",
        Associativity::None => "none",
    };
    ASSOCIATIVITIES
        .iter()
        .position(|candidate| *candidate == name)
        .expect("an associativity index") as u64
}

fn atom_class_index(class: AtomClass) -> u64 {
    ATOM_CLASSES
        .iter()
        .position(|candidate| *candidate == class)
        .expect("an atom class index") as u64
}

fn enc_atom(atom: &Atom) -> Value {
    Value::Adt {
        constructor: 0,
        fields: vec![
            Value::Adt {
                constructor: atom_class_index(atom.class),
                fields: vec![],
            },
            nat(atom.byte_start as u64),
            nat(atom.byte_end as u64),
            nat(atom.line_start as u64),
            nat(atom.column_start as u64),
            nat(atom.line_end as u64),
            nat(atom.column_end as u64),
            text(&atom.text),
        ],
    }
}

/// `Render::Seq`'s list, as the `RNil`/`RItem` cons chain the Lean side
/// builds its own lists of `Render` from (constructors 13 and 14 of
/// `Render`, after the 13 the Rust enum states).
fn enc_render_list(items: &[Render]) -> Value {
    let mut out = Value::Adt {
        constructor: 13,
        fields: vec![],
    };
    for item in items.iter().rev() {
        out = Value::Adt {
            constructor: 14,
            fields: vec![enc_render(item), out],
        };
    }
    out
}

fn enc_render(render: &Render) -> Value {
    match render {
        Render::Form { entry, form } => Value::Adt {
            constructor: 0,
            fields: vec![text(&entry.package), text(&entry.entry), text(form)],
        },
        Render::SelfForm(id) => Value::Adt {
            constructor: 1,
            fields: vec![text(id)],
        },
        Render::Slot(n) => Value::Adt {
            constructor: 2,
            fields: vec![nat(u64::from(*n))],
        },
        Render::Seq(items) => Value::Adt {
            constructor: 3,
            fields: vec![enc_render_list(items)],
        },
        Render::Space => Value::Adt {
            constructor: 4,
            fields: vec![],
        },
        Render::Token(id) => Value::Adt {
            constructor: 5,
            fields: vec![text(id)],
        },
        Render::Group(inner) => Value::Adt {
            constructor: 6,
            fields: vec![enc_render(inner)],
        },
        Render::Paren(inner) => Value::Adt {
            constructor: 7,
            fields: vec![enc_render(inner)],
        },
        Render::Bracket(inner) => Value::Adt {
            constructor: 8,
            fields: vec![enc_render(inner)],
        },
        Render::Sub(base, script) => Value::Adt {
            constructor: 9,
            fields: vec![enc_render(base), enc_render(script)],
        },
        Render::Sup(base, script) => Value::Adt {
            constructor: 10,
            fields: vec![enc_render(base), enc_render(script)],
        },
        Render::Frac(numerator, denominator) => Value::Adt {
            constructor: 11,
            fields: vec![enc_render(numerator), enc_render(denominator)],
        },
        Render::OperatorName(name) => Value::Adt {
            constructor: 12,
            fields: vec![text(name)],
        },
    }
}

fn enc_form(form: &lexlean::lexicon::entry::Form) -> Value {
    Value::Adt {
        constructor: 0,
        fields: vec![
            text(&form.id),
            nat(channel_index(form.channel)),
            text(&form.surface),
            list(form.atoms.iter().map(enc_atom).collect()),
            Value::Bool {
                value: form.canonical_source,
            },
            list(form.features.iter().map(|f| text(f)).collect()),
        ],
    }
}

fn enc_denotation(denotation: &Denotation) -> Value {
    match denotation {
        Denotation::Core { constructor } => Value::Adt {
            constructor: 0,
            fields: vec![text(constructor)],
        },
        Denotation::Lean { module, name } => Value::Adt {
            constructor: 1,
            fields: vec![text(module), text(name)],
        },
        Denotation::Document { module, component } => Value::Adt {
            constructor: 2,
            fields: vec![text(module), text(component)],
        },
        Denotation::Defined { value, text: txt } => Value::Adt {
            constructor: 3,
            fields: vec![enc_lse(value), text(txt)],
        },
    }
}

fn enc_qualified(id: &QualifiedId) -> (Value, Value) {
    (text(&id.package), text(&id.entry))
}

fn enc_eliminator_constructor(
    constructor: &lexlean::lexicon::entry::EliminatorConstructor,
) -> Value {
    let (package, entry) = enc_qualified(&constructor.entry);
    Value::Adt {
        constructor: 0,
        fields: vec![
            package,
            entry,
            text(&constructor.lean_name),
            list(constructor.fields.iter().map(|f| text(f)).collect()),
            list(
                constructor
                    .induction_hypotheses
                    .iter()
                    .map(|f| text(f))
                    .collect(),
            ),
        ],
    }
}

fn enc_eliminator(eliminator: &lexlean::lexicon::entry::Eliminator) -> Value {
    Value::Adt {
        constructor: 0,
        fields: vec![
            text(&eliminator.cases_lean_name),
            text(&eliminator.induction_lean_name),
            list(
                eliminator
                    .constructors
                    .iter()
                    .map(enc_eliminator_constructor)
                    .collect(),
            ),
        ],
    }
}

fn enc_entry(entry: &Entry) -> Value {
    Value::Adt {
        constructor: 0,
        fields: vec![
            text(&entry.id),
            nat(category_index(entry.category)),
            nat(frame_index(entry.frame)),
            nat(u64::from(entry.surface_arity)),
            option(entry.signature.as_ref().map(enc_lse)),
            option(entry.signature_canonical.as_deref().map(text)),
            option(entry.signature_hash.as_ref().map(|h| text(&h.label()))),
            list(entry.universes.iter().map(|u| text(u)).collect()),
            option(entry.precedence.map(|p| nat(u64::from(p)))),
            option(entry.associativity.map(|a| nat(associativity_index(a)))),
            enc_denotation(&entry.denotation),
            list(entry.forms.iter().map(enc_form).collect()),
            option(entry.render_math.as_ref().map(enc_render)),
            option(entry.render_text.as_ref().map(enc_render)),
            Value::Bool {
                value: entry.calculation,
            },
            option(entry.eliminator.as_ref().map(enc_eliminator)),
        ],
    }
}

fn entry_value(result: &Result<Entry, Vec<lexlean::diagnostic::Diagnostic>>) -> Value {
    match result {
        Ok(entry) => Value::Ok {
            value: Box::new(enc_entry(entry)),
        },
        Err(diagnostics) => Value::Error {
            value: Box::new(list(
                diagnostics
                    .iter()
                    .map(|d| pair(text(d.code.as_str()), text(&d.message)))
                    .collect(),
            )),
        },
    }
}

fn ctx(is_core: bool, forbidden: &[String], max_scope_depth: u64) -> EntryContext<'_> {
    EntryContext {
        is_core,
        forbidden_controls: forbidden,
        max_scope_depth,
    }
}

fn entry_run(text_input: &str, is_core: bool, forbidden: &[String], max_scope_depth: u64) -> Run {
    let result = parse_entry(
        "<entry>",
        text_input,
        &ctx(is_core, forbidden, max_scope_depth),
    );
    (
        vec![
            text(text_input),
            Value::Bool { value: is_core },
            super::texts(forbidden),
            nat(max_scope_depth),
        ],
        serde_json::to_value(entry_value(&result)).expect("a value is JSON"),
    )
}

// ------------------------------------------------------------------ vectors

/// The labels of the vectors whose message is approximated: the two
/// causes `toml::from_str::<RawEntry>` collapses to its one message, the
/// text is not TOML at all, or the TOML does not have the entry's shape (a
/// required key missing, an unknown key, or the wrong type). Checked
/// structurally in `case()`, so the approximation cannot quietly spread to
/// a message either side could reproduce exactly.
const APPROXIMATED_NOT_TOML: [&str; 1] =
    ["a malformed TOML document is refused at the first hard gate"];
const APPROXIMATED_WRONG_SHAPE: [&str; 5] = [
    "a missing required scalar is refused as a shape failure",
    "an entry omitting form entirely (no sections, no flat array) is refused as a shape failure",
    "an eliminator omitting constructor entirely is refused as a shape failure",
    "an unknown key at the root is refused as a shape failure",
    "a wrong-typed scalar is refused as a shape failure",
];

/// `run`'s expected message, trimmed to [`TOML_FAILURE_PREFIX`]: the real
/// oracle's message carries parse-error or serde detail that neither
/// `LexiconEntry.lex.tex`'s evaluator nor its rendered `rust-std` package
/// can reproduce (both emit the bare literal, with no detail to add), so
/// the vector's own expected value is the bare literal too. This is not a
/// looser comparison grafted on afterward: every comparison in `case()`,
/// including the generic rendered-package harness, stays exact, because
/// this is what both sides actually, and correctly, produce.
fn approximate(run: Run) -> Run {
    let (arguments, mut expected) = run;
    assert_eq!(
        expected["kind"], "error",
        "an approximated vector is refused"
    );
    let items = expected["value"]["items"]
        .as_array_mut()
        .expect("a diagnostic list");
    assert_eq!(
        items.len(),
        1,
        "an approximated vector carries exactly one diagnostic"
    );
    let code = items[0]["left"]["hex"].as_str().expect("a code").to_owned();
    assert_eq!(
        hex_to_text(&code),
        "LLR3004",
        "an approximated vector's one diagnostic is LLR3004"
    );
    let message_hex = items[0]["right"]["hex"]
        .as_str()
        .expect("a message")
        .to_owned();
    let message = hex_to_text(&message_hex);
    assert!(
        message.starts_with(TOML_FAILURE_PREFIX),
        "the oracle's real message `{message}` does not start with the fixed prefix `{TOML_FAILURE_PREFIX}` -- \
         the relation this approximation depends on no longer holds, so the case must not \
         silently compare against a stale prefix"
    );
    items[0]["right"] = serde_json::to_value(text(TOML_FAILURE_PREFIX)).expect("a value is JSON");
    (arguments, expected)
}

/// A `Value::Bytes` JSON node's hex, decoded back to text.
fn hex_to_text(hex: &str) -> String {
    let raw: Vec<u8> = (0..hex.len())
        .step_by(2)
        .map(|at| u8::from_str_radix(&hex[at..at + 2], 16).expect("hex digit"))
        .collect();
    String::from_utf8_lossy(&raw).into_owned()
}

struct Vector {
    label: &'static str,
    run: Run,
}

const FORBIDDEN: [&str; 2] = ["\\def", "\\input"];

fn forbidden() -> Vec<String> {
    FORBIDDEN.iter().map(|s| (*s).to_owned()).collect()
}

const PROBE: &str = r#"spec = "lexlean/entry/1"
id = "probe"
category = "term-constant"
signature = "(const lexlean.std.nat::nat)"
surface_arity = 0
frame = "atom"

[denotation]
kind = "lean"
module = "Init"
name = "Nat.zero"

[[form]]
id = "probe"
channel = "both"
surface = "probe"
canonical_source = true
features = []

[render]
math = "(operator-name probe)"
"#;

#[allow(clippy::too_many_lines)]
fn named() -> Vec<Vector> {
    let forbidden = forbidden();
    let mut out = Vec::new();
    let mut add = |label: &'static str, text_input: String| {
        out.push(Vector {
            label,
            run: entry_run(&text_input, false, &forbidden, DEPTH),
        });
    };

    add("the minimal entry parses", PROBE.to_owned());
    add(
        "a malformed TOML document is refused at the first hard gate",
        "not valid toml =".to_owned(),
    );
    add(
        "an unsupported entry schema is refused",
        PROBE.replace("\"lexlean/entry/1\"", "\"lexlean/entry/2\""),
    );
    add(
        "an unrecognized category is refused",
        PROBE.replace(
            "category = \"term-constant\"",
            "category = \"not-a-category\"",
        ),
    );
    add(
        "an unrecognized frame is refused",
        PROBE.replace("frame = \"atom\"", "frame = \"not-a-frame\""),
    );
    add(
        "an invalid entry ID accumulates and the rest keeps validating",
        {
            let bad_id = PROBE.replace("id = \"probe\"", "id = \"Not Valid\"");
            // Also break the signature's surface arity, so two unrelated
            // diagnostics accumulate from one run.
            bad_id.replace(
                "signature = \"(const lexlean.std.nat::nat)\"",
                "signature = \"(pi ((explicit n (const lexlean.std.nat::nat))) (const lexlean.std.nat::nat))\"",
            )
        },
    );
    add(
        "a non-core entry cannot use a core-only category",
        PROBE.replace("category = \"term-constant\"", "category = \"structural\""),
    );
    add(
        "a category does not permit the declared frame",
        PROBE
            .replace("category = \"term-constant\"", "category = \"function\"")
            .replace("surface_arity = 0", "surface_arity = 1"),
    );
    add(
        "surface_arity does not match the signature's explicit binders",
        PROBE.replace(
            "signature = \"(const lexlean.std.nat::nat)\"",
            "signature = \"(pi ((explicit n (const lexlean.std.nat::nat))) (const lexlean.std.nat::nat))\"",
        ),
    );
    add(
        "every semantic entry needs a signature",
        PROBE.replace("signature = \"(const lexlean.std.nat::nat)\"\n", ""),
    );
    add(
        "a structural entry carries no signature",
        PROBE
            .replace("category = \"term-constant\"", "category = \"structural\"")
            .replace("surface_arity = 0\n", "surface_arity = 0\nuniverses = []\n"),
    );
    add(
        "a core denotation is refused outside lexlean.core",
        PROBE.replace(
            "[denotation]\nkind = \"lean\"\nmodule = \"Init\"\nname = \"Nat.zero\"",
            "[denotation]\nkind = \"core\"\nconstructor = \"logic.eq\"",
        ),
    );
    add(
        "a lean denotation needs module and name in the Lean-name grammar",
        PROBE.replace("name = \"Nat.zero\"", "name = \"1Not\""),
    );
    add(
        "a document denotation has exactly module and component",
        PROBE.replace(
            "[denotation]\nkind = \"lean\"\nmodule = \"Init\"\nname = \"Nat.zero\"",
            "[denotation]\nkind = \"document\"\nmodule = \"Surface\"\ncomponent = \"c\"",
        ),
    );
    add(
        "an unknown denotation kind is refused",
        PROBE.replace("kind = \"lean\"", "kind = \"not-a-kind\""),
    );
    add(
        "a duplicate form ID accumulates with an invalid feature",
        PROBE.replace(
            "[render]",
            "[[form]]\nid = \"probe\"\nchannel = \"text\"\nsurface = \"again\"\ncanonical_source = false\nfeatures = [\"not-a-feature\"]\n\n[render]",
        ),
    );
    add(
        "leading, trailing, or repeated whitespace in a surface is forbidden",
        PROBE.replace("surface = \"probe\"", "surface = \" probe\""),
    );
    add(
        "the always-forbidden controls come from the context",
        PROBE.replace(
            "[render]",
            "[[form]]\nid = \"alias\"\nchannel = \"math\"\nsurface = \"\\\\def\"\ncanonical_source = false\nfeatures = []\n\n[render]",
        ),
    );
    add(
        "an unsafe canonical surface is a renderer-safety refusal",
        PROBE.replace("surface = \"probe\"", "surface = \"pr{obe}\""),
    );
    add(
        "two unrelated sections both fail and both diagnostics accumulate",
        PROBE
            .replace("id = \"probe\"", "id = \"Not Valid\"")
            .replace("surface = \"probe\"", "surface = \"pr{obe}\""),
    );
    add(
        "a math frame with surface arguments requires a math render",
        PROBE
            .replace("category = \"term-constant\"", "category = \"function\"")
            .replace("frame = \"atom\"", "frame = \"call\"")
            .replace("surface_arity = 0", "surface_arity = 1")
            .replace(
                "signature = \"(const lexlean.std.nat::nat)\"",
                "signature = \"(pi ((explicit n (const lexlean.std.nat::nat))) (const lexlean.std.nat::nat))\"",
            )
            .replace("[render]\nmath = \"(operator-name probe)\"\n", ""),
    );
    add(
        "a render slot set must be exactly 0..surface_arity",
        PROBE
            .replace("category = \"term-constant\"", "category = \"function\"")
            .replace("frame = \"atom\"", "frame = \"call\"")
            .replace("surface_arity = 0", "surface_arity = 1")
            .replace(
                "signature = \"(const lexlean.std.nat::nat)\"",
                "signature = \"(pi ((explicit n (const lexlean.std.nat::nat))) (const lexlean.std.nat::nat))\"",
            )
            .replace(
                "math = \"(operator-name probe)\"",
                "math = \"(operator-name probe)\"\ntext = \"(slot 0)\"",
            ),
    );
    add("a text render needs a text-canonical category", {
        let math_only = PROBE.replace(
            "category = \"term-constant\"",
            "category = \"predicate-constant\"",
        );
        math_only.replace(
            "math = \"(operator-name probe)\"",
            "math = \"(operator-name probe)\"\ntext = \"(self-form probe)\"",
        )
    });
    add(
        "a calculation descriptor belongs to an infix relation",
        format!("{PROBE}\n[calculation]\nkind = \"equality\"\n"),
    );
    add(
        "an eliminator needs a sort-valued signature",
        format!(
            "{PROBE}\n[eliminator]\ncases_lean_name = \"Nat.casesOn\"\ninduction_lean_name = \"Nat.rec\"\n\n[[eliminator.constructor]]\nentry = \"lexlean.std.nat::zero\"\nlean_name = \"Nat.zero\"\nfields = []\ninduction_hypotheses = []\n"
        ),
    );
    // `constructor = []`: in-shape (reaches extractConstructors with zero
    // rows, same as `form = []` at the root), so this reaches the ordinary
    // "an eliminator needs at least one constructor" diagnostic -- the one
    // vector that can actually catch the empty-eliminator-constructor-list
    // plant, since the vector above always has one constructor.
    add(
        "an eliminator with constructor = [] needs at least one constructor",
        format!("{PROBE}\n[eliminator]\ncases_lean_name = \"Nat.casesOn\"\ninduction_lean_name = \"Nat.rec\"\nconstructor = []\n"),
    );
    add(
        "a sort-valued entry accepts an eliminator with one constructor",
        {
            let typed = PROBE
                .replace("category = \"term-constant\"", "category = \"type-noun\"")
                .replace(
                    "signature = \"(const lexlean.std.nat::nat)\"",
                    "signature = \"(sort (type 0))\"",
                )
                .replace("channel = \"both\"", "channel = \"text\"")
                .replace("[render]\nmath = \"(operator-name probe)\"\n", "");
            format!(
                "{typed}\n[eliminator]\ncases_lean_name = \"Nat.casesOn\"\ninduction_lean_name = \"Nat.rec\"\n\n[[eliminator.constructor]]\nentry = \"lexlean.std.nat::zero\"\nlean_name = \"Nat.zero\"\nfields = []\ninduction_hypotheses = []\n"
            )
        },
    );
    add(
        "an entry-ID failure and a core-only-category failure fire together, in header order",
        "spec = \"lexlean/entry/1\"\nid = \"Not Valid\"\ncategory = \"structural\"\nsurface_arity = 0\nframe = \"atom\"\n\n[denotation]\nkind = \"document\"\nmodule = \"Surface\"\ncomponent = \"c\"\n\n[[form]]\nid = \"probe\"\nchannel = \"both\"\nsurface = \"probe\"\ncanonical_source = true\nfeatures = []\n"
            .to_owned(),
    );
    add(
        "a duplicate eliminator constructor and an invalid binder both accumulate",
        {
            let typed = PROBE
                .replace("category = \"term-constant\"", "category = \"type-noun\"")
                .replace(
                    "signature = \"(const lexlean.std.nat::nat)\"",
                    "signature = \"(sort (type 0))\"",
                )
                .replace("channel = \"both\"", "channel = \"text\"")
                .replace("[render]\nmath = \"(operator-name probe)\"\n", "");
            format!(
                "{typed}\n[eliminator]\ncases_lean_name = \"Nat.casesOn\"\ninduction_lean_name = \"Nat.rec\"\n\n[[eliminator.constructor]]\nentry = \"lexlean.std.nat::zero\"\nlean_name = \"Nat.zero\"\nfields = [\"n\", \"n\"]\ninduction_hypotheses = []\n\n[[eliminator.constructor]]\nentry = \"lexlean.std.nat::zero\"\nlean_name = \"Nat.zero\"\nfields = []\ninduction_hypotheses = []\n"
            )
        },
    );

    // `form = []`: the cheapest way to write "zero forms" explicitly, as
    // opposed to omitting `form` entirely (which is a shape failure, below).
    // It is in-shape (`form: Vec<RawForm>` accepts an empty array), so it
    // reaches the ordinary "an entry declares at least one form" diagnostic,
    // not the approximated boundary. `form = []` must land before
    // `[denotation]`: TOML has no way to close a table and return to the
    // root other than starting the next header, so a flat key placed where
    // `[[form]]` used to be (after `[denotation]`) is read as an unknown
    // field of `[denotation]` instead -- a bug in the vector, not in the
    // source, caught by the oracle's own refusal on the first run.
    add(
        "form = [] reaches the declares-at-least-one-form diagnostic, not a shape failure",
        PROBE
            .replace(
                "\n[[form]]\nid = \"probe\"\nchannel = \"both\"\nsurface = \"probe\"\ncanonical_source = true\nfeatures = []\n",
                "",
            )
            .replace("frame = \"atom\"\n", "frame = \"atom\"\nform = []\n"),
    );

    // The shape boundary (§13.1–§13.6, VERIFICATION.md "a lexicon entry's
    // shape boundary" table): every row below is a case where
    // `toml::from_str::<RawEntry>` fails in Rust (a required key missing,
    // an unknown key, or the wrong type) and so must fail at the same hard
    // gate in Lean, with the one approximated message, not fall through to
    // the accumulating pass.
    add(
        "a missing required scalar is refused as a shape failure",
        PROBE.replace("spec = \"lexlean/entry/1\"\n", ""),
    );
    add(
        "an entry omitting form entirely (no sections, no flat array) is refused as a shape failure",
        PROBE.replace(
            "\n[[form]]\nid = \"probe\"\nchannel = \"both\"\nsurface = \"probe\"\ncanonical_source = true\nfeatures = []\n",
            "",
        ),
    );
    add(
        "an eliminator omitting constructor entirely is refused as a shape failure",
        format!("{PROBE}\n[eliminator]\ncases_lean_name = \"Nat.casesOn\"\ninduction_lean_name = \"Nat.rec\"\n"),
    );
    add(
        "an unknown key at the root is refused as a shape failure",
        PROBE.replace(
            "frame = \"atom\"\n",
            "frame = \"atom\"\nunknown_field = \"x\"\n",
        ),
    );
    add(
        "a wrong-typed scalar is refused as a shape failure",
        PROBE.replace("surface_arity = 0", "surface_arity = \"zero\""),
    );

    // Trim the approximated vectors' own expected message to the fixed
    // prefix: not a looser comparison applied later, but the vector
    // stating the truth of what both the evaluator and the rendered
    // package actually produce, so every comparison in `case()` -- this
    // vector's own, and the generic rendered-package harness, which knows
    // nothing of approximation -- stays exact.
    for vector in &mut out {
        if APPROXIMATED_NOT_TOML.contains(&vector.label)
            || APPROXIMATED_WRONG_SHAPE.contains(&vector.label)
        {
            vector.run = approximate(std::mem::replace(
                &mut vector.run,
                (Vec::new(), serde_json::Value::Null),
            ));
        }
    }
    out
}

/// The fixed prefix of `toml::from_str::<RawEntry>`'s one message, shared
/// by both approximated causes (not TOML, or not the entry's shape).
/// `LexiconEntry.lex.tex`'s `parseEntry` has no detail to append for
/// either cause (no parse-error text, no serde detail): both of its hard
/// gates emit the bare literal `"invalid entry file"`, with no trailing
/// colon or space, so that is the whole shared prefix — not
/// `"invalid entry file: "`, which the Lean side's bare message does not
/// start with at all.
const TOML_FAILURE_PREFIX: &str = "invalid entry file";

// --------------------------------------------------------------- committed

/// Every real lexicon entry file the compiler's own project ships, with the
/// `is_core` of its owning package (the nearest ancestor `lexicon.toml`'s
/// `package` field, or `lexlean.core` for the builtin core package whose
/// entries live under `language/`).
fn committed() -> Vec<(String, bool)> {
    let root = repo_root();
    let walker = walkdir::WalkDir::new(root.as_std_path())
        .sort_by_file_name()
        .into_iter()
        .filter_entry(|entry| {
            let name = entry.file_name().to_string_lossy();
            !(entry.file_type().is_dir()
                && (matches!(name.as_ref(), "target" | ".git" | ".lexlean")
                    || support::holds_cache_tag(entry)))
        });
    let mut out = Vec::new();
    for entry in walker.flatten() {
        let path = entry.path();
        let shown = path.to_string_lossy().replace('\\', "/");
        if !entry.file_type().is_file() || !shown.ends_with(".toml") || !shown.contains("/entries/")
        {
            continue;
        }
        let Ok(content) = std::fs::read_to_string(path) else {
            continue;
        };
        let mut is_core = shown.contains("/language/");
        let mut dir = path.parent();
        while let Some(candidate) = dir {
            let manifest = candidate.join("lexicon.toml");
            if manifest.is_file() {
                if let Ok(manifest_text) = std::fs::read_to_string(&manifest) {
                    if let Ok(table) = manifest_text.parse::<toml::Table>() {
                        if table.get("package").and_then(toml::Value::as_str)
                            == Some("lexlean.core")
                        {
                            is_core = true;
                        }
                    }
                }
                break;
            }
            dir = candidate.parent();
        }
        out.push((content, is_core));
    }
    out
}

// -------------------------------------------------------------------- seeds

const CATEGORY_WORDS: &[&str] = &[
    "term-constant",
    "function",
    "type-noun",
    "predicate-constant",
];

fn seeded_entry(seeded: &mut Seeded, forbidden: &[String]) -> String {
    let category = *seeded.pick(CATEGORY_WORDS);
    let (frame, arity) = match category {
        "function" => ("call", 1),
        _ => ("atom", 0),
    };
    let signature = if arity == 0 {
        "(const lexlean.std.nat::nat)".to_owned()
    } else {
        "(pi ((explicit n (const lexlean.std.nat::nat))) (const lexlean.std.nat::nat))".to_owned()
    };
    let id = if seeded.chance(85) {
        "probe".to_owned()
    } else {
        "Not Valid".to_owned()
    };
    let surface: String = if seeded.chance(85) {
        "probe".to_owned()
    } else {
        (*seeded.pick(&["pr{obe}", " probe", "probe "])).to_owned()
    };
    let mut text_out = format!(
        "spec = \"lexlean/entry/1\"\nid = \"{id}\"\ncategory = \"{category}\"\nsignature = \"{signature}\"\nsurface_arity = {arity}\nframe = \"{frame}\"\n\n[denotation]\nkind = \"lean\"\nmodule = \"Init\"\nname = \"Nat.zero\"\n\n[[form]]\nid = \"probe\"\nchannel = \"both\"\nsurface = \"{surface}\"\ncanonical_source = true\nfeatures = []\n"
    );
    if arity == 1 {
        text_out.push_str("\n[render]\nmath = \"(slot 0)\"\n");
    } else if seeded.chance(60) {
        text_out.push_str("\n[render]\nmath = \"(operator-name probe)\"\n");
    }
    if seeded.chance(15) {
        // A TOML basic string escapes its own backslash, so the control
        // sequence's `\` must be doubled to survive as itself.
        let control = seeded.pick(forbidden).replace('\\', "\\\\");
        text_out = text_out.replace(
            "[[form]]\nid = \"probe\"",
            &format!(
                "[[form]]\nid = \"alias\"\nchannel = \"math\"\nsurface = \"{control}\"\ncanonical_source = false\nfeatures = []\n\n[[form]]\nid = \"probe\""
            ),
        );
    }
    text_out
}

fn seeded_runs(count: usize) -> Vec<Run> {
    let mut seeded = Seeded(0xE7B5_5E07);
    let forbidden = forbidden();
    (0..count)
        .map(|_| {
            entry_run(
                &seeded_entry(&mut seeded, &forbidden),
                false,
                &forbidden,
                DEPTH,
            )
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

/// Dropped-second-diagnostic plants: a check in the accumulating pass is
/// disabled outright, so a vector that should carry two diagnostics (an
/// unrelated earlier one, and the one this check would have raised) carries
/// only the first — proving the pass does not stop at it.
const PLANTS: &[Plant] = &[
    Plant {
        label: "the entry-ID check is dropped, so a second, unrelated diagnostic is lost",
        site: "(if (LexiconLse::isEntryId entryId) then (ctor Option.none [(Bytes * Bytes)]) else (ctor Option.some [(Bytes * Bytes)] (errD (cat (quoted entryId) (lit \" is not a valid entry ID\")))))",
        replacement: "(ctor Option.none [(Bytes * Bytes)])",
        vector: "an invalid entry ID accumulates and the rest keeps validating",
    },
    Plant {
        label: "the non-core-category check is dropped",
        site: "((project shape coreOnly) && (if isCore then false else true))",
        replacement: "false",
        vector: "a non-core entry cannot use a core-only category",
    },
    Plant {
        label: "an empty eliminator-constructor list is accepted",
        site: "(if ((prim length constructors : Nat) =? 0) then (ctor Option.some [(Bytes * Bytes)] (errD (lit \"an eliminator needs at least one constructor\"))) else (ctor Option.none [(Bytes * Bytes)]))",
        replacement: "(ctor Option.none [(Bytes * Bytes)])",
        vector: "an eliminator with constructor = [] needs at least one constructor",
    },
    Plant {
        label: "an entry needs no form",
        site: "(match raws\n    | List.nil => (ctor Option.some [(Bytes * Bytes)] (errD (lit \"an entry declares at least one form\")))\n    | List.cons h t => (ctor Option.none [(Bytes * Bytes)]))",
        replacement: "(ctor Option.none [(Bytes * Bytes)])",
        vector: "form = [] reaches the declares-at-least-one-form diagnostic, not a shape failure",
    },
];

/// The one ordering plant: within the header's chain, the entry-ID check
/// (`d1`) and the core-only-category check (`d2`) are swapped, so a run
/// whose entry fails both carries them in the opposite order. Applied as a
/// single atomic edit (via a placeholder, since one text is not a substring
/// of the other and both must move at once).
const ORDER_SITE_A: &str = "(if (LexiconLse::isEntryId entryId) then (ctor Option.none [(Bytes * Bytes)]) else (ctor Option.some [(Bytes * Bytes)] (errD (cat (quoted entryId) (lit \" is not a valid entry ID\")))))";
const ORDER_SITE_B: &str = "(if ((project shape coreOnly) && (if isCore then false else true)) then (ctor Option.some [(Bytes * Bytes)] (errD (cat3 (lit \"a non-core entry cannot use category `\") categoryText (lit \"`\")))) else (ctor Option.none [(Bytes * Bytes)]))";
const ORDER_VECTOR: &str =
    "an entry-ID failure and a core-only-category failure fire together, in header order";
const ORDER_PLACEHOLDER: &str = "@@BS55_ORDER_SWAP_PLACEHOLDER@@";

fn apply_order_swap(source: &str) -> Option<String> {
    if source.matches(ORDER_SITE_A).count() != 1 || source.matches(ORDER_SITE_B).count() != 1 {
        return None;
    }
    let step1 = source.replacen(ORDER_SITE_A, ORDER_PLACEHOLDER, 1);
    let step2 = step1.replacen(ORDER_SITE_B, ORDER_SITE_A, 1);
    Some(step2.replacen(ORDER_PLACEHOLDER, ORDER_SITE_B, 1))
}

fn plants(vectors: &[Vector]) {
    let mut missed = Vec::new();
    for plant in PLANTS {
        let vector = vectors
            .iter()
            .find(|vector| vector.label == plant.vector)
            .unwrap_or_else(|| panic!("{}: no vector `{}`", plant.label, plant.vector));
        let planted = support::P::bootstrap();
        let source = planted.read_raw("src/LexiconEntry.lex.tex");
        let occurrences = source.matches(plant.site).count();
        if occurrences != 1 {
            missed.push(format!(
                "{}: the plant site occurs {occurrences} times, not 1",
                plant.label
            ));
            continue;
        }
        planted.write(
            "src/LexiconEntry.lex.tex",
            &source.replacen(plant.site, plant.replacement, 1),
        );
        let checked = support::checked_project(&planted);
        let low = super::lower(&checked, &planted, MODULE, ROOTS[0].0);
        match super::evaluate(&low, std::slice::from_ref(&vector.run), 1).first() {
            Some(first) => eprintln!(
                "BS-55: plant detected: {}: {}",
                plant.label,
                first.chars().take(160).collect::<String>()
            ),
            None => missed.push(plant.label.to_owned()),
        }
    }

    // The ordering plant.
    let order_vector = vectors
        .iter()
        .find(|vector| vector.label == ORDER_VECTOR)
        .unwrap_or_else(|| panic!("ordering plant: no vector `{ORDER_VECTOR}`"));
    let planted = support::P::bootstrap();
    let source = planted.read_raw("src/LexiconEntry.lex.tex");
    match apply_order_swap(&source) {
        None => missed.push("ordering plant: one of the two sites is not unique".to_owned()),
        Some(swapped) => {
            planted.write("src/LexiconEntry.lex.tex", &swapped);
            let checked = support::checked_project(&planted);
            let low = super::lower(&checked, &planted, MODULE, ROOTS[0].0);
            match super::evaluate(&low, std::slice::from_ref(&order_vector.run), 1).first() {
                Some(first) => eprintln!(
                    "BS-55: plant detected: diagnostics reordered: {}",
                    first.chars().take(160).collect::<String>()
                ),
                None => missed.push("ordering plant".to_owned()),
            }
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
        checked_units.units.contains_key("LexiconEntry"),
        "LexiconEntry is a linked module of the project"
    );
    eligibility_of(&project, "LexiconEntry", MODULE, &ROOTS);

    let vectors = named();
    let labels: std::collections::BTreeSet<&str> =
        vectors.iter().map(|vector| vector.label).collect();
    assert_eq!(labels.len(), vectors.len(), "each vector has its own label");
    let refused = vectors
        .iter()
        .filter(|vector| vector.run.1["kind"] == "error")
        .count();
    assert!(
        refused >= 15,
        "at least fifteen vectors are refused: {refused}"
    );
    // Structural check that the approximation stays confined to the two
    // causes named above and cannot quietly spread to a message either
    // side could reproduce exactly: every labeled vector exists exactly
    // once, and no other vector's label collides with one of them.
    for label in APPROXIMATED_NOT_TOML
        .iter()
        .chain(&APPROXIMATED_WRONG_SHAPE)
    {
        assert_eq!(
            vectors
                .iter()
                .filter(|vector| vector.label == *label)
                .count(),
            1,
            "exactly one vector named `{label}`"
        );
    }

    let forbidden = forbidden();
    let corpus = committed();
    assert!(
        corpus.len() >= 20,
        "the committed lexicon packages hold {} entries",
        corpus.len()
    );
    let committed_runs: Vec<Run> = corpus
        .iter()
        .map(|(content, is_core)| entry_run(content, *is_core, &forbidden, DEPTH))
        .collect();
    eprintln!(
        "BS-55: {} vectors, {} committed entries ({:?})",
        vectors.len(),
        corpus.len(),
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
    let low = &lowered["parseEntry"];

    if normative {
        let exact: Vec<Run> = vectors
            .iter()
            .map(|vector| vector.run.clone())
            .chain(committed_runs.iter().cloned())
            .chain(evaluator_seeded.iter().cloned())
            .collect();
        let disagreements = super::evaluate(low, &exact, 5);
        assert!(
            disagreements.is_empty(),
            "parseEntry and the oracle disagree: {disagreements:?}"
        );
        eprintln!(
            "BS-55: parseEntry: the evaluator agrees on {} runs ({:?})",
            exact.len(),
            started.elapsed()
        );
        plants(&vectors);
    } else {
        eprintln!("BS-55: platform-bound host: the evaluator, the seeded runs, and the planted defects run on the normative host");
    }

    let mut compiled: Vec<Run> = vectors.into_iter().map(|vector| vector.run).collect();
    compiled.extend(committed_runs);
    compiled.extend(rustc_seeded);
    let suites: Vec<(&str, &super::Lowered, &[Run])> =
        vec![("parseEntry", low, compiled.as_slice())];
    let total = rustc_differential("BS-55", &suites, "entry::parse_entry");
    eprintln!("BS-55: {total} rustc runs agree ({:?})", started.elapsed());

    if !support::lean_backed("BS-55") {
        return;
    }
    export(&lowered);
    eprintln!("BS-55: passed in {:?}", started.elapsed());
}
