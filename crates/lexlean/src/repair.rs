//! Machine-applicable repairs (SPEC.md §20.7).
//!
//! A repair is offered only where applying it removes the failure it is
//! attached to: a repair that would trade one refusal for another, or that
//! the verifier could not confirm, is withheld, because an agent applies
//! what it is given and a wrong edit costs more than no edit.
//!
//! The source-level repairs here read the normalized text of the file a
//! diagnostic points into. The semantic notation keeps no spans below the
//! module, so they locate the construct inside the diagnostic's span by its
//! canonical spelling; a construct that is not found exactly is not
//! repaired. The `unfold` repair needs Lean and is made by verification
//! (`verify::repair`), through the list helpers below.

use crate::diagnostic::{Diagnostic, Edit, Repair, RepairKind, Span};

/// The axioms `linear_arithmetic`, `simplify`, and `cases` introduce in the
/// ordinary course: a policy is widened to these and no further. Widening a
/// policy to `Classical.choice`, or to an axiom the document states, is a
/// decision about what the theorem claims, which is the author's.
const ROUTINE_AXIOMS: [&str; 2] = ["Quot.sound", "propext"];

/// Attach the source-level repairs of every diagnostic whose file `read`
/// returns the normalized text of.
pub(crate) fn attach_source_repairs(
    diagnostics: &mut [Diagnostic],
    read: &dyn Fn(&str) -> Option<String>,
) {
    for diagnostic in diagnostics.iter_mut() {
        let Some(primary) = diagnostic.primary.clone() else {
            continue;
        };
        let kinds: &[RepairKind] = match diagnostic.code.as_str() {
            "LLT4001" => &[RepairKind::Sort, RepairKind::Quote],
            "LLV7005" => &[RepairKind::Axioms],
            _ => continue,
        };
        let Some(text) = read(&primary.path) else {
            continue;
        };
        if primary.byte_end > text.len()
            || primary.byte_start > primary.byte_end
            || !text.is_char_boundary(primary.byte_start)
            || !text.is_char_boundary(primary.byte_end)
        {
            continue;
        }
        for kind in kinds {
            if diagnostic.repairs.iter().any(|repair| repair.kind == *kind) {
                continue;
            }
            let repair = match kind {
                RepairKind::Sort => sort_repair(&diagnostic.message, &primary, &text),
                RepairKind::Quote => quote_repair(&diagnostic.message, &primary, &text),
                RepairKind::Axioms => axioms_repair(&diagnostic.message, &primary, &text),
                RepairKind::Unfold => None,
            };
            if let Some(repair) = repair {
                diagnostic.repairs.push(repair);
            }
        }
    }
}

/// The byte index of the bracket that closes the one at `open`, skipping
/// quoted strings; `None` when it is unbalanced.
fn closing(text: &str, open: usize) -> Option<usize> {
    let bytes = text.as_bytes();
    let mut depth = 0usize;
    let mut quoted = false;
    for (index, byte) in bytes.iter().enumerate().skip(open) {
        match byte {
            b'"' => quoted = !quoted,
            b'[' | b'{' | b'(' if !quoted => depth += 1,
            b']' | b'}' | b')' if !quoted => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    return Some(index);
                }
            }
            _ => {}
        }
    }
    None
}

/// The member names of a canonical list body `a, b, c`; `None` when an
/// element is not a plain or module-qualified name.
pub(crate) fn list_names(body: &str) -> Option<Vec<String>> {
    if body.trim().is_empty() {
        return Some(Vec::new());
    }
    body.split(',')
        .map(|element| {
            let name = element.trim();
            let plain = |segment: &str| {
                segment
                    .chars()
                    .next()
                    .is_some_and(|first| first.is_ascii_alphabetic() || first == '_')
                    && segment
                        .chars()
                        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.')
            };
            let legal = match name.split_once("::") {
                Some((module, member)) => plain(module) && plain(member),
                None => plain(name),
            };
            legal.then(|| name.to_owned())
        })
        .collect()
}

/// The names of the top-level declarations of a semantic module that begin
/// before `before`: a line that opens `kind{` and names it first.
pub(crate) fn prior_definitions(text: &str, before: usize) -> Vec<String> {
    let mut names = Vec::new();
    let mut from = 0;
    while let Some(found) = text[from..before].find("{") {
        let open = from + found;
        let start = open + 1;
        from = start;
        let line = text[..open].rfind('\n').map_or(0, |index| index + 1);
        if !text[line..open]
            .chars()
            .all(|c| c.is_ascii_alphabetic() || c == '_')
            || line == open
        {
            continue;
        }
        let rest = text[start..].trim_start();
        if let Some(rest) = rest.strip_prefix("name: ") {
            let name: String = rest
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                .collect();
            if !name.is_empty() {
                names.push(name);
            }
        }
    }
    names
}

/// Every `definitions: [...]` list inside `start..end`: the byte range of
/// the bracketed list and its names.
fn definition_lists(text: &str, start: usize, end: usize) -> Vec<(usize, usize, Vec<String>)> {
    const KEY: &str = "definitions: [";
    let mut lists = Vec::new();
    let mut from = start;
    while let Some(found) = text[from..end].find(KEY) {
        let open = from + found + KEY.len() - 1;
        let Some(close) = closing(text, open) else {
            break;
        };
        if close >= end {
            break;
        }
        if let Some(names) = list_names(&text[open + 1..close]) {
            lists.push((open, close + 1, names));
        }
        from = close;
    }
    lists
}

/// Is the list strictly ascending, which is what "sorted and unique" means?
fn strictly_sorted(names: &[String]) -> bool {
    names.windows(2).all(|pair| pair[0] < pair[1])
}

/// The canonical spelling of a list of names.
pub(crate) fn render_list(names: &[String]) -> String {
    format!("[{}]", names.join(", "))
}

/// `sort`: every list of the span that is not sorted and unique, sorted and
/// deduplicated — offered only when each name it keeps is a prior document
/// definition or a module-qualified member, since otherwise the sorted list
/// is refused all the same.
fn sort_repair(message: &str, primary: &Span, text: &str) -> Option<Repair> {
    if !message.ends_with("definitions are not sorted and unique") {
        return None;
    }
    let mut edits = Vec::new();
    for (open, close, names) in definition_lists(text, primary.byte_start, primary.byte_end) {
        if strictly_sorted(&names) {
            continue;
        }
        let mut sorted = names.clone();
        sorted.sort();
        sorted.dedup();
        let prior = prior_definitions(text, open);
        if !sorted
            .iter()
            .all(|name| name.contains("::") || prior.contains(name))
        {
            return None;
        }
        edits.push(Edit {
            span: Span::of_text(&primary.path, text, open, close),
            replacement: render_list(&sorted),
        });
    }
    if edits.is_empty() {
        return None;
    }
    let count = edits.len();
    Some(Repair::new(
        RepairKind::Sort,
        format!(
            "sort {} definitions list{} into strictly ascending order without repeats",
            count,
            if count == 1 { "" } else { "s" }
        ),
        edits,
    ))
}

/// The `[start, end)` range of the innermost `keyword{...}` block that
/// contains `position`.
fn enclosing_block(text: &str, keyword: &str, position: usize) -> Option<(usize, usize)> {
    let opener = format!("{keyword}{{");
    let mut best = None;
    let mut from = 0;
    while let Some(found) = text[from..position].find(&opener) {
        let start = from + found;
        let open = start + keyword.len();
        if let Some(close) = closing(text, open) {
            if close > position {
                best = Some((start, close + 1));
            }
        }
        from = start + 1;
    }
    best
}

/// The parameters of a semantic declaration block, as `(name, type)`.
fn parameters(block: &str) -> Vec<(String, String)> {
    const KEY: &str = "parameters: [";
    let Some(found) = block.find(KEY) else {
        return Vec::new();
    };
    let open = found + KEY.len() - 1;
    let Some(close) = closing(block, open) else {
        return Vec::new();
    };
    let body = &block[open + 1..close];
    let mut out = Vec::new();
    let mut depth = 0usize;
    let mut piece = String::new();
    for c in body.chars().chain(std::iter::once(',')) {
        match c {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => depth = depth.saturating_sub(1),
            _ => {}
        }
        if c == ',' && depth == 0 {
            if let Some((name, ty)) = piece.split_once(" : ") {
                out.push((name.trim().to_owned(), ty.trim().to_owned()));
            }
            piece.clear();
        } else {
            piece.push(c);
        }
    }
    out
}

/// `quote`: a Boolean branch constructor written bare, `false`, is the
/// string `"false"` in the closed schema. Offered only for a branch of a
/// `cases` whose scrutinee is a `Bool` parameter of its theorem: anywhere
/// else, quoting the word leaves a branch that names no constructor.
fn quote_repair(message: &str, primary: &Span, text: &str) -> Option<Repair> {
    if !message.contains("invalid type: boolean") {
        return None;
    }
    let mut edits = Vec::new();
    for word in ["false", "true"] {
        let needle = format!("constructor: {word}");
        let mut from = primary.byte_start;
        while let Some(found) = text[from..primary.byte_end].find(&needle) {
            let at = from + found + "constructor: ".len();
            let after = at + word.len();
            from = after;
            if text[after..]
                .chars()
                .next()
                .is_some_and(|c| c.is_ascii_alphanumeric() || c == '_')
            {
                continue;
            }
            let (cases_start, cases_end) = enclosing_block(text, "cases", at)?;
            let block = &text[cases_start..cases_end];
            let scrutinee: String = block
                .rfind("scrutinee: ")
                .map(|index| {
                    block[index + "scrutinee: ".len()..]
                        .chars()
                        .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                        .collect()
                })
                .unwrap_or_default();
            let (theorem_start, theorem_end) = enclosing_block(text, "theorem", at)?;
            let boolean = parameters(&text[theorem_start..theorem_end])
                .iter()
                .any(|(name, ty)| *name == scrutinee && ty == "Bool");
            if !boolean {
                return None;
            }
            edits.push(Edit {
                span: Span::of_text(&primary.path, text, at, after),
                replacement: format!("\"{word}\""),
            });
        }
    }
    if edits.is_empty() {
        return None;
    }
    Some(Repair::new(
        RepairKind::Quote,
        "quote the Boolean branch constructors: a constructor is a string, `\"false\"` or `\"true\"`",
        edits,
    ))
}

/// The observed set a policy-violation message names, sorted and unique.
fn observed_axioms(message: &str) -> Option<Vec<String>> {
    let start = message.rfind("observed [")? + "observed [".len();
    let end = start + message[start..].find(']')?;
    let mut names: Vec<String> = message[start..end]
        .split(',')
        .map(|name| name.trim().to_owned())
        .filter(|name| !name.is_empty())
        .collect();
    names.sort();
    names.dedup();
    Some(names)
}

/// `axioms`: the semantic theorem's policy made equal to the observed set.
/// Offered only when every axiom it adds is one the proof forms introduce
/// routinely (`propext`, `Quot.sound`); narrowing a policy is always offered.
fn axioms_repair(message: &str, primary: &Span, text: &str) -> Option<Repair> {
    let observed = observed_axioms(message)?;
    let start = primary.byte_start;
    if !text[start..].starts_with("theorem{") {
        return None;
    }
    let close = closing(text, start + "theorem".len())?;
    if close + 1 > primary.byte_end {
        return None;
    }
    // The policy member at the top level of the theorem.
    let block = &text[start..close];
    let mut depth = 0usize;
    let mut quoted = false;
    let mut member = None;
    for (index, byte) in block.bytes().enumerate() {
        match byte {
            b'"' => quoted = !quoted,
            b'[' | b'{' | b'(' if !quoted => depth += 1,
            b']' | b'}' | b')' if !quoted => depth = depth.saturating_sub(1),
            _ => {}
        }
        if depth == 1 && !quoted && block[index..].starts_with("axioms: [") {
            member = Some(start + index);
            break;
        }
    }
    let current = match member {
        Some(at) => {
            let open = at + "axioms: ".len();
            let list_close = closing(text, open)?;
            list_names(&text[open + 1..list_close])?
        }
        None => Vec::new(),
    };
    let added: Vec<&String> = observed
        .iter()
        .filter(|axiom| !current.contains(axiom))
        .collect();
    if added
        .iter()
        .any(|axiom| !ROUTINE_AXIOMS.contains(&axiom.as_str()))
    {
        return None;
    }
    let edit = match (member, observed.is_empty()) {
        (Some(at), false) => {
            let open = at + "axioms: ".len();
            let list_close = closing(text, open)?;
            Edit {
                span: Span::of_text(&primary.path, text, open, list_close + 1),
                replacement: render_list(&observed),
            }
        }
        (Some(at), true) => {
            // No axiom observed: the member goes, with the separator before it.
            let list_close = closing(text, at + "axioms: ".len())?;
            let separator = text[..at].rfind(',')?;
            Edit {
                span: Span::of_text(&primary.path, text, separator, list_close + 1),
                replacement: String::new(),
            }
        }
        (None, false) => {
            let last = text[..close].trim_end().len();
            let multiline = text[last..close].contains('\n');
            Edit {
                span: Span::of_text(&primary.path, text, last, close),
                replacement: if multiline {
                    format!(",\n  axioms: {}\n", render_list(&observed))
                } else {
                    format!(", axioms: {}", render_list(&observed))
                },
            }
        }
        (None, true) => return None,
    };
    Some(Repair::new(
        RepairKind::Axioms,
        format!(
            "set the axiom policy to the observed set {}",
            render_list(&observed)
        ),
        vec![edit],
    ))
}

/// Apply the edits of `repairs` to the text of `path`, last offset first.
#[must_use]
pub fn apply(text: &str, path: &str, repairs: &[Repair]) -> String {
    let mut edits: Vec<&Edit> = repairs
        .iter()
        .flat_map(|repair| repair.edits.iter())
        .filter(|edit| edit.span.path == path)
        .collect();
    edits.sort_by_key(|edit| std::cmp::Reverse(edit.span.byte_start));
    let mut out = text.to_owned();
    for edit in edits {
        out.replace_range(edit.span.byte_start..edit.span.byte_end, &edit.replacement);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::code;

    fn whole(text: &str) -> Span {
        Span::of_text("src/Main.lex.tex", text, 0, text.len())
    }

    fn repaired(code: crate::diagnostic::DiagnosticCode, message: &str, text: &str) -> Diagnostic {
        let mut diagnostics = [Diagnostic::new(code, message).with_span(whole(text))];
        attach_source_repairs(&mut diagnostics, &|_| Some(text.to_owned()));
        let [diagnostic] = diagnostics;
        diagnostic
    }

    const MODULE: &str = "definition{name: a, parameters: [], result: Nat, body: 0}\n\ndefinition{name: b, parameters: [], result: Nat, body: 1}\n\ntheorem{\n  name: t,\n  parameters: [same : Bool, n : Nat],\n  statement: ((a + b) = 1),\n  proof: cases{\n    branches: [\n      {proof: linear_arithmetic{definitions: [b, a, b]}, binders: [], constructor: false},\n      {proof: linear_arithmetic{definitions: [a, b]}, binders: [], constructor: \"true\"}\n    ],\n    scrutinee: same\n  }\n}\n";

    #[test]
    fn sort_offers_the_sorted_unique_list() {
        let diagnostic = repaired(
            code!("LLT4001"),
            "phase link: linear_arithmetic definitions are not sorted and unique",
            MODULE,
        );
        assert_eq!(diagnostic.repairs.len(), 1);
        let after = apply(MODULE, "src/Main.lex.tex", &diagnostic.repairs);
        assert!(after.contains("definitions: [a, b]}, binders: [], constructor: false"));
        assert!(!after.contains("[b, a, b]"));
    }

    #[test]
    fn sort_is_withheld_for_a_name_that_is_no_prior_definition() {
        let text = MODULE.replace("[b, a, b]", "[b, a, c]");
        let diagnostic = repaired(
            code!("LLT4001"),
            "phase link: linear_arithmetic definitions are not sorted and unique",
            &text,
        );
        assert!(diagnostic.repairs.is_empty());
    }

    #[test]
    fn quote_offers_the_string_constructor_of_a_bool_scrutinee() {
        let diagnostic = repaired(
            code!("LLT4001"),
            "phase link: invalid semantic-module JSON: invalid type: boolean `false`, expected a string at line 1 column 10",
            MODULE,
        );
        let after = apply(MODULE, "src/Main.lex.tex", &diagnostic.repairs);
        assert!(after.contains("constructor: \"false\"}"));
    }

    #[test]
    fn quote_is_withheld_when_the_scrutinee_is_not_bool() {
        let text = MODULE.replace("scrutinee: same", "scrutinee: n");
        let diagnostic = repaired(
            code!("LLT4001"),
            "phase link: invalid semantic-module JSON: invalid type: boolean `false`, expected a string",
            &text,
        );
        assert!(diagnostic.repairs.is_empty());
    }

    #[test]
    fn axioms_inserts_replaces_and_withholds() {
        let theorem = "theorem{\n  name: t,\n  parameters: [],\n  statement: (1 = 1),\n  proof: reflexivity{}\n}\n";
        let message =
            |set: &str| format!("`M.t` violates its exact axiom policy: observed [{set}]");
        let inserted = repaired(code!("LLV7005"), &message("propext, Quot.sound"), theorem);
        assert_eq!(
            apply(theorem, "src/Main.lex.tex", &inserted.repairs),
            "theorem{\n  name: t,\n  parameters: [],\n  statement: (1 = 1),\n  proof: reflexivity{},\n  axioms: [Quot.sound, propext]\n}\n"
        );
        let stated = theorem.replace("reflexivity{}\n", "reflexivity{},\n  axioms: [propext]\n");
        let widened = repaired(code!("LLV7005"), &message("Quot.sound, propext"), &stated);
        assert!(apply(&stated, "src/Main.lex.tex", &widened.repairs)
            .contains("axioms: [Quot.sound, propext]\n"));
        let narrowed = repaired(code!("LLV7005"), &message(""), &stated);
        assert_eq!(
            apply(&stated, "src/Main.lex.tex", &narrowed.repairs),
            theorem
        );
        let classical = repaired(
            code!("LLV7005"),
            &message("Classical.choice, propext"),
            &stated,
        );
        assert!(classical.repairs.is_empty());
    }
}
