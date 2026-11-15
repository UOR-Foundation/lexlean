//! The gate behind SP-17 (SPEC.md §17.17, issue #89): no Lean source text
//! template remains in the production compiler.
//!
//! A template is a Rust string literal that is Lean syntax. The scan is a
//! lexer over the sources, not a search for keywords: it reads comments, string
//! literals (plain, raw, byte), character literals and lifetimes the way the Rust
//! lexer does, decodes every escape of a plain literal (`\n`, `\t`, `\\`, `\"`,
//! `\x41`, `\u{2192}`, and the line continuation), drops every item under
//! `#[cfg(test)]`, and tests what remains. Lean is written to a file by the
//! backend from typed data (`backend::lean::AuxiliaryModule` and the core
//! backend), so a literal that is Lean syntax anywhere else is a template that
//! bypasses it.
//!
//! A text can be cut anywhere, so literals written one after another in one
//! expression (`concat!`, `+`, an array of lines, the arguments of `format!`)
//! are tested as RT-04 tests them: each alone, joined, and joined with a line
//! break between them. Two literals are one expression when no `;`, `{` or `}`
//! and at most 64 other characters of code lie between them.
//!
//! What counts as Lean syntax, by [`is_lean_syntax`], for the decoded text of a
//! literal:
//!
//! * a Lean symbol: `:=`, `=>`, `↦`, `→`, `∀`, `∃`, `⟨`, `⟩`, `(.`, a name
//!   quote `«`, an attribute `@[`, a `fun` binder (`fun` and a space, then a
//!   letter, `_` or `(`), a tactic block (`by` and a space, then a tactic of
//!   Lean's: `decide`, `simp`, `rfl`, `omega`, `exact`, ...), or a
//!   parenthesized application of a capitalized name (`(Option {})`: a
//!   parenthesis, a name that begins with a capital, a space, and more);
//! * a command line: a line that begins with `import`, `namespace`,
//!   `set_option`, `universe`, `open`, `#print`, `#check`, `#eval` or `#reduce`
//!   and a space and more, or `end` and one name;
//! * a declaration: a line that begins, after any of `private`, `protected`,
//!   `public`, `noncomputable`, `partial` and `unsafe`, with `theorem`, `def`,
//!   `example`, `instance`, `structure`, `inductive`, `abbrev`, or `class` and
//!   a space and more, in a literal that holds a `:` or a line that ends in
//!   ` where`.
//!
//! A bare word (`"inductive"`, `"theorem "`) is a name or a token a scan
//! compares against, not a template, and English diagnostics that happen to
//! begin with one of the words carry no `:`. The allow-list ([`ALLOWED`]) is
//! short, matches a whole text and never a prefix, and each entry says why: the
//! printouts of kernel terms that developers read, the quote of a name, the
//! tokens the term renderer writes from typed data, and the closed commands the
//! auxiliary module renderer writes.
//!
//! What the scan does not read, and VERIFICATION.md (SP-17) names: a file that
//! is not a `.rs` source of `production/` or `backend/lean.rs`; text assembled
//! from pieces that are not literals (a `char`, `stringify!`, a `format!`
//! argument that is itself a name of Lean's, a `const` joined to a literal in
//! another statement); and a literal split across two statements.

use std::path::Path;

/// One literal the scan reports.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    /// The file, relative to the repository.
    pub file: String,
    /// The one-based line of the literal.
    pub line: usize,
    /// The literal, as written between its quotes.
    pub literal: String,
}

/// The texts that are Lean syntax and are not templates: the file they are in
/// (a suffix of its path), the whole text, and why. A text matches only when it
/// is equal to the entry: an entry is never a prefix.
pub const ALLOWED: [(&str, &str, &str); 17] = [
    (
        "production/certificate.rs",
        "«{segment}»",
        "the quote of one segment of a Lean name, which is name syntax; the runtime type of a document names its constructors by it",
    ),
    (
        "production/kernel/ext.rs",
        "∀",
        "the printout of a kernel binder, read by developers in diagnostics",
    ),
    (
        "production/kernel/ext.rs",
        "(let {n} : {ty} := {value}; {rest})",
        "the printout of a kernel let, read by developers in diagnostics",
    ),
    (
        "backend/lean.rs",
        "set_option autoImplicit false",
        "the renderer of a closed auxiliary command",
    ),
    (
        "backend/lean.rs",
        "example : ",
        "the renderer of a closed auxiliary command",
    ),
    (
        "backend/lean.rs",
        " := ",
        "the renderer of a closed auxiliary command",
    ),
    (
        "backend/lean.rs",
        "#print axioms ",
        "the renderer of a closed auxiliary command",
    ),
    (
        "backend/lean.rs",
        "#check @",
        "the renderer of a closed auxiliary command",
    ),
    (
        "backend/lean.rs",
        " → ",
        "a token of Lean the term renderer writes from the typed lexicon term it renders",
    ),
    (
        "backend/lean.rs",
        " => ",
        "a token of Lean the term renderer writes from the typed lexicon term it renders",
    ),
    (
        "backend/lean.rs",
        "=>",
        "a token of Lean the term renderer writes from the typed lexicon term it renders",
    ),
    (
        "backend/lean.rs",
        "→",
        "a token of Lean the term renderer writes from the typed lexicon term it renders",
    ),
    (
        "backend/lean.rs",
        ":=",
        "a token of Lean the term renderer writes from the typed lexicon term it renders",
    ),
    (
        "backend/lean.rs",
        "⟨",
        "a token of Lean the term renderer writes from the typed lexicon term it renders",
    ),
    (
        "backend/lean.rs",
        "⟩",
        "a token of Lean the term renderer writes from the typed lexicon term it renders",
    ),
    (
        "backend/lean.rs",
        "@[",
        "a token of Lean the term renderer writes from the typed lexicon term it renders",
    ),
    (
        "backend/lean.rs",
        "(let {fresh} : {ty_text} := {value_text}; {body_text})",
        "the printout of a let in the text of a term the renderer prints, read by developers",
    ),
];

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Literal(usize, String),
    Symbol(char),
    Word(String),
}

fn tokens(source: &str) -> Vec<Token> {
    let chars: Vec<char> = source.chars().collect();
    let mut out = Vec::new();
    let mut line = 1;
    let mut at = 0;
    while at < chars.len() {
        let c = chars[at];
        if c == '\n' {
            line += 1;
            at += 1;
        } else if c.is_whitespace() {
            at += 1;
        } else if c == '/' && chars.get(at + 1) == Some(&'/') {
            while at < chars.len() && chars[at] != '\n' {
                at += 1;
            }
        } else if c == '/' && chars.get(at + 1) == Some(&'*') {
            let mut depth = 0;
            while at < chars.len() {
                if chars[at] == '/' && chars.get(at + 1) == Some(&'*') {
                    depth += 1;
                    at += 2;
                } else if chars[at] == '*' && chars.get(at + 1) == Some(&'/') {
                    depth -= 1;
                    at += 2;
                    if depth == 0 {
                        break;
                    }
                } else {
                    if chars[at] == '\n' {
                        line += 1;
                    }
                    at += 1;
                }
            }
        } else if let Some((hashes, start)) = raw_string_start(&chars, at) {
            let begin = line;
            let mut text = String::new();
            let mut end = start;
            while end < chars.len() {
                if chars[end] == '"' && (0..hashes).all(|k| chars.get(end + 1 + k) == Some(&'#')) {
                    break;
                }
                if chars[end] == '\n' {
                    line += 1;
                }
                text.push(chars[end]);
                end += 1;
            }
            out.push(Token::Literal(begin, text));
            at = end + 1 + hashes;
        } else if c == '"' || (c == 'b' && chars.get(at + 1) == Some(&'"')) {
            if c == 'b' {
                at += 1;
            }
            let begin = line;
            let mut text = String::new();
            at += 1;
            while at < chars.len() && chars[at] != '"' {
                if chars[at] == '\\' {
                    at += 1;
                    let (decoded, used, breaks) = decode_escape(&chars[at..]);
                    text.push_str(&decoded);
                    at += used;
                    line += breaks;
                } else {
                    if chars[at] == '\n' {
                        line += 1;
                    }
                    text.push(chars[at]);
                    at += 1;
                }
            }
            at += 1;
            out.push(Token::Literal(begin, text));
        } else if c == '\'' {
            // A character literal is `'x'` or `'\…'`; anything else is a
            // lifetime or a label.
            if chars.get(at + 1) == Some(&'\\') {
                at += 2;
                while at < chars.len() && chars[at] != '\'' {
                    at += 1;
                }
                at += 1;
            } else if chars.get(at + 2) == Some(&'\'') {
                at += 3;
            } else {
                at += 1;
            }
        } else if c.is_alphanumeric() || c == '_' {
            let mut word = String::new();
            while at < chars.len() && (chars[at].is_alphanumeric() || chars[at] == '_') {
                word.push(chars[at]);
                at += 1;
            }
            out.push(Token::Word(word));
        } else {
            out.push(Token::Symbol(c));
            at += 1;
        }
    }
    out
}

/// The escape that follows a backslash, read the way Rust reads it: the text it
/// stands for, the characters it takes, and the line breaks it spans. A line
/// continuation (a backslash at the end of a line) drops the break and the
/// white space that opens the next line. An escape that is none of Rust's
/// stays as written, so nothing a scan compares is lost.
fn decode_escape(rest: &[char]) -> (String, usize, usize) {
    match rest.first() {
        Some('n') => ("\n".to_owned(), 1, 0),
        Some('t') => ("\t".to_owned(), 1, 0),
        Some('r') => ("\r".to_owned(), 1, 0),
        Some('0') => ("\0".to_owned(), 1, 0),
        Some('\\') => ("\\".to_owned(), 1, 0),
        Some('"') => ("\"".to_owned(), 1, 0),
        Some('\'') => ("'".to_owned(), 1, 0),
        Some('x') => {
            let digits: String = rest.iter().skip(1).take(2).collect();
            u8::from_str_radix(&digits, 16).map_or_else(
                |_| ("\\x".to_owned(), 1, 0),
                |byte| ((byte as char).to_string(), 3, 0),
            )
        }
        Some('u') if rest.get(1) == Some(&'{') => {
            let close = rest.iter().position(|c| *c == '}');
            close
                .and_then(|close| {
                    let digits: String = rest[2..close].iter().filter(|c| **c != '_').collect();
                    u32::from_str_radix(&digits, 16)
                        .ok()
                        .and_then(char::from_u32)
                        .map(|decoded| (decoded.to_string(), close + 1, 0))
                })
                .unwrap_or_else(|| ("\\u".to_owned(), 1, 0))
        }
        Some('\n') | Some('\r') => {
            let mut used = 0;
            let mut breaks = 0;
            while let Some(next) = rest.get(used) {
                if !next.is_whitespace() {
                    break;
                }
                if *next == '\n' {
                    breaks += 1;
                }
                used += 1;
            }
            (String::new(), used, breaks)
        }
        Some(other) => (format!("\\{other}"), 1, 0),
        None => ("\\".to_owned(), 0, 0),
    }
}

/// A raw string begins at `at`: the number of `#` and the index after the quote.
fn raw_string_start(chars: &[char], at: usize) -> Option<(usize, usize)> {
    if at > 0 && (chars[at - 1].is_alphanumeric() || chars[at - 1] == '_') {
        return None;
    }
    let mut cursor = at;
    if chars.get(cursor) == Some(&'b') {
        cursor += 1;
    }
    if chars.get(cursor) != Some(&'r') {
        return None;
    }
    cursor += 1;
    let mut hashes = 0;
    while chars.get(cursor) == Some(&'#') {
        hashes += 1;
        cursor += 1;
    }
    (chars.get(cursor) == Some(&'"')).then_some((hashes, cursor + 1))
}

/// The tokens with every item under `#[cfg(test)]` removed.
fn outside_tests(tokens: Vec<Token>) -> Vec<Token> {
    let attribute = [
        Token::Symbol('#'),
        Token::Symbol('['),
        Token::Word("cfg".to_owned()),
        Token::Symbol('('),
        Token::Word("test".to_owned()),
        Token::Symbol(')'),
        Token::Symbol(']'),
    ];
    let mut out = Vec::new();
    let mut at = 0;
    while at < tokens.len() {
        if tokens[at..].starts_with(&attribute) {
            at += attribute.len();
            while at < tokens.len()
                && tokens[at] != Token::Symbol('{')
                && tokens[at] != Token::Symbol(';')
            {
                at += 1;
            }
            if tokens.get(at) == Some(&Token::Symbol('{')) {
                let mut depth = 0;
                while at < tokens.len() {
                    match tokens[at] {
                        Token::Symbol('{') => depth += 1,
                        Token::Symbol('}') => depth -= 1,
                        _ => {}
                    }
                    at += 1;
                    if depth == 0 {
                        break;
                    }
                }
            } else {
                at += 1;
            }
            // The removed item ends a statement, so literals on either side
            // of it are never one expression.
            out.push(Token::Symbol(';'));
        } else {
            out.push(tokens[at].clone());
            at += 1;
        }
    }
    out
}

/// Whether the text of a literal is Lean syntax (see the module's definition).
/// The text is the literal as the Rust lexer decodes it, line breaks included.
#[must_use]
pub fn is_lean_syntax(literal: &str) -> bool {
    const SYMBOLS: [&str; 10] = [":=", "=>", "↦", "→", "∀", "∃", "⟨", "⟩", "(.", "«"];
    const TACTICS: [&str; 17] = [
        "decide",
        "native_decide",
        "simp",
        "simp_all",
        "rfl",
        "omega",
        "exact",
        "intro",
        "intros",
        "apply",
        "cases",
        "induction",
        "rw",
        "trivial",
        "constructor",
        "assumption",
        "aesop",
    ];
    const MODIFIERS: [&str; 6] = [
        "private",
        "protected",
        "public",
        "noncomputable",
        "partial",
        "unsafe",
    ];
    if SYMBOLS.iter().any(|symbol| literal.contains(symbol)) || literal.contains("@[") {
        return true;
    }
    let text = literal;
    let characters: Vec<char> = text.chars().collect();
    for (at, character) in characters.iter().enumerate() {
        if *character == '(' && characters.get(at + 1).is_some_and(char::is_ascii_uppercase) {
            let name = characters[at + 1..]
                .iter()
                .take_while(|c| c.is_alphanumeric() || **c == '.' || **c == '_')
                .count();
            if characters.get(at + 1 + name) == Some(&' ')
                && characters
                    .get(at + 2 + name)
                    .is_some_and(|next| !next.is_whitespace())
            {
                return true;
            }
        }
    }
    if text.contains("fun ")
        && text
            .split("fun ")
            .skip(1)
            .any(|rest| rest.starts_with(|c: char| c.is_alphabetic() || c == '_' || c == '('))
    {
        return true;
    }
    // A tactic block: `by` and a space, then a tactic of Lean's.
    if text.split("by ").skip(1).enumerate().any(|(index, rest)| {
        let before = text.split("by ").take(index + 1).collect::<String>();
        let word_start =
            before.is_empty() || before.ends_with(|c: char| c.is_whitespace() || c == '(');
        let word: String = rest
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect();
        word_start && TACTICS.contains(&word.as_str())
    }) {
        return true;
    }
    let rest_after = |line: &str, word: &str| -> Option<String> {
        line.trim_start()
            .strip_prefix(word)
            .and_then(|rest| rest.strip_prefix(' '))
            .map(|rest| rest.trim().to_owned())
            .filter(|rest| !rest.is_empty())
    };
    let begins =
        |line: &str, words: &[&str]| words.iter().any(|word| rest_after(line, word).is_some());
    text.lines().any(|line| {
        // The closing of a namespace or section is `end` and one name.
        if rest_after(line, "end").is_some_and(|name| !name.contains(' ')) {
            return true;
        }
        // A declaration may be preceded by modifiers.
        let mut declaration = line.trim_start();
        while let Some(rest) = MODIFIERS.iter().find_map(|modifier| {
            declaration
                .strip_prefix(modifier)
                .and_then(|rest| rest.strip_prefix(' '))
        }) {
            declaration = rest;
        }
        begins(
            line,
            &[
                "import",
                "namespace",
                "set_option",
                "universe",
                "open",
                "#print",
                "#check",
                "#eval",
                "#reduce",
            ],
        ) || ((text.contains(':') || declaration.trim_end().ends_with(" where"))
            && begins(
                declaration,
                &[
                    "theorem",
                    "def",
                    "example",
                    "instance",
                    "structure",
                    "inductive",
                    "abbrev",
                    "class",
                ],
            ))
    })
}

/// The expressions of a source that hold string literals: the literals of one
/// expression, written one after another (`concat!`, `+`, an array of lines,
/// the arguments of `format!`), with the line of the first. A text can be cut
/// anywhere, so an expression is tested as its literals alone, joined, and
/// joined with a line break between, as RT-04 reads them.
fn expressions(tokens: &[Token]) -> Vec<(usize, Vec<String>)> {
    let mut found = Vec::new();
    let mut chain: Vec<(usize, String)> = Vec::new();
    let mut gap = 0;
    let flush = |chain: &mut Vec<(usize, String)>, found: &mut Vec<(usize, Vec<String>)>| {
        if let Some((line, _)) = chain.first() {
            found.push((*line, chain.iter().map(|(_, text)| text.clone()).collect()));
        }
        chain.clear();
    };
    for token in tokens {
        match token {
            Token::Literal(line, text) => {
                // The same expression: little code between the two, and no
                // statement or block boundary.
                if gap > 64 {
                    flush(&mut chain, &mut found);
                }
                gap = 0;
                chain.push((*line, text.clone()));
            }
            Token::Symbol(c) => {
                if matches!(c, ';' | '{' | '}') {
                    flush(&mut chain, &mut found);
                }
                gap += 1;
            }
            Token::Word(word) => gap += word.chars().count(),
        }
    }
    flush(&mut chain, &mut found);
    found
}

/// The readings of one expression's literals that are tested: each literal
/// alone, the literals joined, and the literals joined with a line break.
fn readings(parts: &[String]) -> Vec<String> {
    let mut all: Vec<String> = parts.to_vec();
    if parts.len() > 1 {
        all.push(parts.concat());
        all.push(parts.join("\n"));
    }
    all
}

/// Whether `literal`, whole, is the allow-list entry for `file`.
fn allowed(file: &str, literal: &str) -> bool {
    ALLOWED
        .iter()
        .any(|(suffix, whole, _)| file.ends_with(suffix) && literal == *whole)
}

/// The readings of every expression of `source` from line `from` on, outside
/// test items, with the line of the expression: what the scan tests and the
/// allow-list is matched against.
#[must_use]
pub fn texts(source: &str, from: usize) -> Vec<(usize, Vec<String>)> {
    expressions(&outside_tests(tokens(source)))
        .into_iter()
        .filter(|(line, _)| *line >= from)
        .map(|(line, parts)| (line, readings(&parts)))
        .collect()
}

/// The literals of `source` that are Lean syntax and are not allowed, in the
/// file named `file`, from line `from` on: one finding for an expression, its
/// first reading that is. A reading that is allowed whole excuses itself only:
/// an allowed literal joined to more text is a new text, which is tested as
/// one.
#[must_use]
pub fn scan_source(file: &str, source: &str, from: usize) -> Vec<Finding> {
    texts(source, from)
        .into_iter()
        .filter_map(|(line, readings)| {
            readings
                .into_iter()
                .find(|reading| is_lean_syntax(reading) && !allowed(file, reading))
                .map(|literal| Finding {
                    file: file.to_owned(),
                    line,
                    literal,
                })
        })
        .collect()
}

/// The findings of the repository at `root`: every source under
/// `crates/lexlean/src/production/`, and the whole of `backend/lean.rs`.
///
/// # Panics
///
/// Panics when a source cannot be read.
#[must_use]
pub fn scan_repository(root: &Path) -> Vec<Finding> {
    let base = root.join("crates/lexlean/src");
    let mut findings = Vec::new();
    for entry in walkdir::WalkDir::new(base.join("production"))
        .sort_by_file_name()
        .into_iter()
        .flatten()
        .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "rs"))
    {
        let source = std::fs::read_to_string(entry.path()).expect("a source reads");
        let file = entry
            .path()
            .strip_prefix(&base)
            .expect("under the sources")
            .to_string_lossy()
            .replace('\\', "/");
        findings.extend(scan_source(&file, &source, 1));
    }
    let backend = std::fs::read_to_string(base.join("backend/lean.rs")).expect("the backend reads");
    findings.extend(scan_source("backend/lean.rs", &backend, 1));
    findings
}
