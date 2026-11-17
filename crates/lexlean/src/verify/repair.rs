//! The `unfold` repair (SPEC.md §20.7): confirmed by Lean before it is
//! offered.
//!
//! `omega` reports a goal it cannot prove with a counterexample over the
//! terms it treats as atoms, and an atom headed by a document definition the
//! proof does not unfold is the commonest reason. Which definitions to add is
//! a guess, and a wrong guess is worse than none, so the verifier tests its
//! guesses: it appends to the rejected module one copy of each failing
//! theorem per candidate list, elaborates that probe once, and offers the
//! first candidate whose copy Lean accepts without a message. A theorem no
//! candidate repairs is offered nothing; its failure is not a missing unfold.
//!
//! The probe is a separate file under the verification's staging directory.
//! It is never published, recorded, or hashed, and the verification fails
//! whatever it finds.

use std::collections::BTreeSet;

use camino::{Utf8Path, Utf8PathBuf};

use crate::api::RenderedModule;
use crate::diagnostic::{Diagnostic, Edit, Repair, RepairKind, Span};
use crate::link::CheckedProject;

/// The marker `linear_arithmetic` puts first after its definitions in its
/// simplifier list (§17.12): what precedes it is the definitions.
const BRIDGE: &str = "← Bool.not_eq_true";

/// The suffix of the probe's theorem copies; no source name contains it,
/// because a semantic name has no `_` followed by an upper-case letter in
/// the generated spelling.
const COPY: &str = "_LexLeanRepair";

/// The most theorem copies one probe elaborates.
const MAX_CANDIDATES: usize = 256;

/// Is `c` a character of a generated Lean identifier?
fn ident_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '\''
}

/// The identifiers of a text, in order.
fn identifiers(text: &str) -> BTreeSet<String> {
    text.split(|c: char| !ident_char(c))
        .filter(|word| !word.is_empty())
        .map(str::to_owned)
        .collect()
}

/// One document definition of the generated module.
struct Definition {
    offset: usize,
    name: String,
    body: String,
    recursive: bool,
}

/// The `def` blocks of a generated module, each up to the next blank line.
fn definitions(lean: &str) -> Vec<Definition> {
    let mut out = Vec::new();
    let mut offset = 0;
    for block in lean.split("\n\n") {
        let at = offset;
        offset += block.len() + 2;
        let Some(rest) = block.strip_prefix("@[expose] public def ") else {
            continue;
        };
        let name: String = rest.chars().take_while(|c| ident_char(*c)).collect();
        if name.is_empty() {
            continue;
        }
        let body = rest[name.len()..].to_owned();
        let recursive = identifiers(&body).contains(&name);
        out.push(Definition {
            offset: at,
            name,
            body,
            recursive,
        });
    }
    out
}

/// One `linear_arithmetic` script inside a theorem block: its byte range in
/// the module text, its indentation, and its definitions.
struct Script {
    start: usize,
    end: usize,
    pad: String,
    definitions: Vec<String>,
}

/// The `linear_arithmetic` scripts of `lean[start..end]`, each confirmed to
/// be exactly the fixed script for its definitions.
fn scripts(lean: &str, start: usize, end: usize) -> Option<Vec<Script>> {
    let mut out = Vec::new();
    let mut from = start;
    while let Some(found) = lean.get(from..end)?.find("simp only [") {
        let list = from + found + "simp only [".len();
        from = list;
        let Some(bridge) = lean.get(list..end)?.find(BRIDGE) else {
            continue;
        };
        let line_end = list + lean[list..].find('\n')?;
        if list + bridge > line_end {
            continue;
        }
        let names: Vec<String> = lean[list..list + bridge]
            .split(',')
            .map(str::trim)
            .filter(|name| !name.is_empty())
            .map(str::to_owned)
            .collect();
        // Walk back to the script's `intros` line.
        let mut line_start = lean[..list].rfind('\n').map_or(0, |index| index + 1);
        let pad: String = lean[line_start..]
            .chars()
            .take_while(|c| *c == ' ')
            .collect();
        loop {
            if lean[line_start..].starts_with(&format!("{pad}intros\n")) {
                break;
            }
            if line_start == 0 {
                return None;
            }
            line_start = lean[..line_start - 1]
                .rfind('\n')
                .map_or(0, |index| index + 1);
            if line_start < start {
                return None;
            }
        }
        let rendered = crate::backend::semantic::linear_arithmetic(&pad, &names);
        if !lean[line_start..].starts_with(&rendered) || line_start + rendered.len() > end {
            return None;
        }
        out.push(Script {
            start: line_start,
            end: line_start + rendered.len(),
            pad,
            definitions: names,
        });
        from = line_start + rendered.len();
    }
    Some(out)
}

/// The `[start, end)` range and name of the theorem block of `lean`
/// containing `offset`.
fn theorem_at(lean: &str, offset: usize) -> Option<(usize, usize, String)> {
    let offset = offset.min(lean.len());
    let start = lean[..offset].rfind("\n\n").map_or(0, |index| index + 2);
    // The block keeps its final line feed, which ends its last script.
    let end = start
        + lean[start..]
            .find("\n\n")
            .map_or(lean.len() - start, |index| index + 1);
    let header = &lean[start..end];
    let rest = header
        .strip_prefix("public theorem ")
        .or_else(|| header.strip_prefix("theorem "))?;
    let name: String = rest.chars().take_while(|c| ident_char(*c)).collect();
    (!name.is_empty()).then_some((start, end, name))
}

/// The `linear_arithmetic{...}` proofs inside `source[start..end]`, as byte
/// ranges, in order.
fn source_proofs(source: &str, start: usize, end: usize) -> Vec<(usize, usize)> {
    const OPEN: &str = "linear_arithmetic{";
    let mut out = Vec::new();
    let mut from = start;
    while let Some(found) = source[from..end].find(OPEN) {
        let at = from + found;
        let Some(close) = source[at..end].find('}') else {
            break;
        };
        out.push((at, at + close + 1));
        from = at + close + 1;
    }
    out
}

/// The definitions the candidate lists draw on: every document definition
/// reachable from `seed` through the bodies of definitions, recursive ones
/// only when `recursive`.
fn closure(seed: &BTreeSet<String>, all: &[&Definition], recursive: bool) -> BTreeSet<String> {
    let mut reached: BTreeSet<String> = BTreeSet::new();
    let mut frontier: Vec<String> = seed.iter().cloned().collect();
    while let Some(word) = frontier.pop() {
        let Some(definition) = all.iter().find(|definition| definition.name == word) else {
            continue;
        };
        if definition.recursive && !recursive {
            continue;
        }
        if reached.insert(definition.name.clone()) {
            frontier.extend(identifiers(&definition.body));
        }
    }
    reached
}

/// Everything the probe needs of one failing theorem.
struct Failing {
    name: String,
    start: usize,
    end: usize,
    scripts: Vec<Script>,
    failed: BTreeSet<usize>,
    atoms: BTreeSet<String>,
    first_diagnostic: usize,
}

/// Attach confirmed `unfold` repairs to the diagnostics of a module Lean
/// rejected. `output` is Lean's output, `diagnostics` its remapping (one per
/// message that is not `info`, in order), and `run` elaborates a probe file
/// under `root` and returns Lean's output.
pub(super) fn unfold(
    checked: &CheckedProject,
    module: &RenderedModule,
    output: &str,
    diagnostics: &mut [Diagnostic],
    root: &Utf8Path,
    run: &dyn Fn(&Utf8Path) -> Option<String>,
) {
    let Some(checked_module) = checked.modules.get(&module.module) else {
        return;
    };
    let source = &checked_module.normalized;
    let lean = &module.lean_text;
    let all = definitions(lean);
    let messages: Vec<super::LeanMessage> = super::parse_lean_messages(output)
        .into_iter()
        .filter(|message| message.severity != "info")
        .collect();
    if messages.len() != diagnostics.len() {
        return;
    }
    let mut failing: Vec<Failing> = Vec::new();
    for (index, message) in messages.iter().enumerate() {
        if message.severity != "error" {
            continue;
        }
        let Some(offset) = super::lean_position_to_byte(lean, message.line, message.column) else {
            continue;
        };
        let Some((start, end, name)) = theorem_at(lean, offset) else {
            continue;
        };
        let position = match failing.iter().position(|theorem| theorem.start == start) {
            Some(position) => position,
            None => {
                let Some(found) = scripts(lean, start, end) else {
                    continue;
                };
                failing.push(Failing {
                    name,
                    start,
                    end,
                    scripts: found,
                    failed: BTreeSet::new(),
                    atoms: BTreeSet::new(),
                    first_diagnostic: index,
                });
                failing.len() - 1
            }
        };
        let theorem = &mut failing[position];
        if let Some(script) = theorem
            .scripts
            .iter()
            .position(|script| script.start <= offset && offset < script.end)
        {
            theorem.failed.insert(script);
            theorem.atoms.extend(identifiers(&message.message));
        }
    }
    failing.retain(|theorem| !theorem.failed.is_empty());
    if failing.is_empty() {
        return;
    }
    // A candidate is a document definition the source spells as Lean does
    // and that precedes the theorem: the probe's copies come after every
    // declaration, so Lean alone would not refuse a later one.
    let spelled: BTreeSet<String> = crate::repair::prior_definitions(source, source.len())
        .into_iter()
        .collect();

    // Each candidate: the failing theorem, and one definitions list per
    // script of it.
    let mut candidates: Vec<(usize, Vec<Vec<String>>)> = Vec::new();
    for (theorem_index, theorem) in failing.iter().enumerate() {
        let header_end = lean[theorem.start..theorem.end]
            .find(":= by")
            .map_or(theorem.end, |index| theorem.start + index);
        let statement = identifiers(&lean[theorem.start..header_end]);
        let prior: Vec<&Definition> = all
            .iter()
            .filter(|definition| {
                definition.offset < theorem.start && spelled.contains(&definition.name)
            })
            .collect();
        let known: BTreeSet<String> = prior
            .iter()
            .map(|definition| definition.name.clone())
            .collect();
        // The additions to try, the same for every failing script of the
        // theorem: each reachable definition alone, the atoms `omega` named,
        // everything reachable without and with recursion, and each of those
        // two sets less one definition. Unfolding too much is a failure of
        // its own (a definition unfolded in a hypothesis leaves an `if` that
        // `split` does not reach, and one that both sides apply alike is best
        // left an atom), so the smallest addition Lean accepts is offered.
        let reach = |recursive: bool| {
            let mut seed = statement.clone();
            for (index, script) in theorem.scripts.iter().enumerate() {
                if theorem.failed.contains(&index) {
                    seed.extend(script.definitions.iter().cloned());
                }
            }
            closure(&seed, &prior, recursive)
        };
        let mut additions: Vec<BTreeSet<String>> =
            reach(true).into_iter().map(|name| [name].into()).collect();
        additions.push(theorem.atoms.intersection(&known).cloned().collect());
        for whole in [reach(false), reach(true)] {
            for name in &whole {
                let mut less = whole.clone();
                less.remove(name);
                additions.push(less);
            }
            additions.push(whole);
        }
        let mut seen: BTreeSet<Vec<Vec<String>>> = BTreeSet::new();
        for addition in additions {
            let lists: Vec<Vec<String>> = theorem
                .scripts
                .iter()
                .enumerate()
                .map(|(index, script)| {
                    let mut names: BTreeSet<String> = script.definitions.iter().cloned().collect();
                    if theorem.failed.contains(&index) {
                        names.extend(addition.iter().cloned());
                    }
                    names.into_iter().collect()
                })
                .collect();
            let unchanged = lists
                .iter()
                .zip(&theorem.scripts)
                .all(|(list, script)| *list == script.definitions);
            if !unchanged && seen.insert(lists.clone()) {
                candidates.push((theorem_index, lists));
            }
        }
    }
    // The probe holds one copy per candidate; a module with more is not
    // probed, which only withholds repairs.
    if candidates.len() > MAX_CANDIDATES {
        return;
    }
    if candidates.is_empty() {
        return;
    }

    // The probe: the module, then one renamed copy of a failing theorem per
    // candidate, before the namespace closes.
    let Some(closing) = lean.trim_end().rfind("\nend ") else {
        return;
    };
    let mut probe = lean[..closing + 1].to_owned();
    let mut ranges: Vec<(usize, usize)> = Vec::new();
    for (index, (theorem_index, lists)) in candidates.iter().enumerate() {
        let theorem = &failing[*theorem_index];
        let mut copy = String::new();
        let mut at = theorem.start;
        for (script, list) in theorem.scripts.iter().zip(lists) {
            copy.push_str(&lean[at..script.start]);
            copy.push_str(&crate::backend::semantic::linear_arithmetic(
                &script.pad,
                list,
            ));
            at = script.end;
        }
        copy.push_str(&lean[at..theorem.end]);
        let copy = copy.replacen(
            &format!("theorem {} ", theorem.name),
            &format!("theorem {}{COPY}{index} ", theorem.name),
            1,
        );
        probe.push('\n');
        let start = probe.len();
        probe.push_str(&copy);
        ranges.push((start, probe.len()));
        // Silence is not acceptance: a probe Lean stopped early (a limit, a
        // timeout) reports nothing about the copies after the stop. Each
        // copy must be seen in the kernel, free of `sorryAx`.
        probe.push_str(&format!("\n#print axioms {}{COPY}{index}\n", theorem.name));
    }
    probe.push_str(&lean[closing + 1..]);
    let file: Utf8PathBuf = root.join(format!("{}.lean", module.lean_module.replace('.', "/")));
    if let Some(parent) = file.parent() {
        if std::fs::create_dir_all(parent.as_std_path()).is_err() {
            return;
        }
    }
    if std::fs::write(file.as_std_path(), probe.as_bytes()).is_err() {
        return;
    }
    let Some(probe_output) = run(&file) else {
        return;
    };
    let located: Vec<usize> = super::parse_lean_messages(&probe_output)
        .iter()
        .filter_map(|message| super::lean_position_to_byte(&probe, message.line, message.column))
        .collect();
    let accepted = |index: usize| {
        let (start, end) = ranges[index];
        let quiet = !located
            .iter()
            .any(|offset| *offset >= start && *offset < end);
        // `#print axioms` answers on a line of its own, with or without an
        // `info:` opener; the copy's qualified name ends `.<copy>'`.
        let copy = format!(".{}{COPY}{index}'", failing[candidates[index].0].name);
        let sound = probe_output.lines().any(|line| {
            line.contains(&copy)
                && (line.contains("' depends on axioms: [")
                    || line.contains("' does not depend on any axioms"))
                && !line.contains("sorryAx")
        });
        quiet && sound
    };

    let path = module.map.sources.first().and_then(|source| match source {
        crate::artifact::source_map::MapSource::File { path, .. } => Some(path.clone()),
        _ => None,
    });
    let Some(path) = path else {
        return;
    };
    for (theorem_index, theorem) in failing.iter().enumerate() {
        let size = |lists: &[Vec<String>]| lists.iter().map(Vec::len).sum::<usize>();
        let Some(lists) = candidates
            .iter()
            .enumerate()
            .filter(|(index, (owner, _))| *owner == theorem_index && accepted(*index))
            .map(|(_, (_, lists))| lists)
            .min_by_key(|lists| size(lists))
        else {
            continue;
        };
        // The theorem's source: the declaration mapping of its block.
        let Some(range) = super::enclosing_declaration(module, theorem.end - 1)
            .and_then(|mapping| mapping.src_range)
        else {
            continue;
        };
        let proofs = source_proofs(source, range.0, range.1);
        if proofs.len() != theorem.scripts.len() {
            continue;
        }
        let mut edits = Vec::new();
        let mut added: BTreeSet<String> = BTreeSet::new();
        for ((script, list), (start, end)) in theorem.scripts.iter().zip(lists).zip(&proofs) {
            if *list == script.definitions {
                continue;
            }
            added.extend(
                list.iter()
                    .filter(|name| !script.definitions.contains(name))
                    .cloned(),
            );
            edits.push(Edit {
                span: Span::of_text(&path, source, *start, *end),
                replacement: format!(
                    "linear_arithmetic{{definitions: {}}}",
                    crate::repair::render_list(list)
                ),
            });
        }
        if edits.is_empty() {
            continue;
        }
        let names: Vec<String> = added.into_iter().map(|name| format!("`{name}`")).collect();
        diagnostics[theorem.first_diagnostic]
            .repairs
            .push(Repair::new(
                RepairKind::Unfold,
                format!(
                    "unfold {} in `{}`: Lean accepts the theorem with them",
                    names.join(", "),
                    theorem.name
                ),
                edits,
            ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scripts_are_read_back_exactly() {
        let pad = "  ";
        let script = crate::backend::semantic::linear_arithmetic(pad, &["f".to_owned()]);
        let lean = format!(
            "@[expose] public def f (n : Nat) : Nat := n\n\npublic theorem t (n : Nat) : (f n = n) := by\n{script}\nend X\n"
        );
        let start = lean.find("public theorem").expect("theorem");
        let (block_start, block_end, name) = theorem_at(&lean, start + 5).expect("block");
        assert_eq!(name, "t");
        let found = scripts(&lean, block_start, block_end).expect("scripts");
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].definitions, ["f"]);
        assert_eq!(&lean[found[0].start..found[0].end], script);
        let all = definitions(&lean);
        assert_eq!(all.len(), 1);
        assert!(!all[0].recursive);
    }

    #[test]
    fn the_closure_skips_recursive_definitions_unless_asked() {
        let lean = "@[expose] public def g : (xs : List Nat) -> Nat\n  | List.nil => 0\n  | List.cons h r => h + g r\n\n@[expose] public def f (xs : List Nat) : Nat := g xs\n";
        let owned = definitions(lean);
        let all: Vec<&Definition> = owned.iter().collect();
        let seed: BTreeSet<String> = ["f".to_owned()].into();
        assert_eq!(closure(&seed, &all, false), ["f".to_owned()].into());
        assert_eq!(
            closure(&seed, &all, true),
            ["f".to_owned(), "g".to_owned()].into()
        );
    }
}
