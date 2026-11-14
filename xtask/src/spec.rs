//! `cargo xtask spec`: edit the chapter modules of `spec/` by item (`NS-07`).
//!
//! A thin command over `repo_model::spec_edit`, which does the editing through
//! the authoring SDK and refuses unless the regenerated SPEC.md is the intended
//! text. This writes the changed modules and then the projections, as
//! `model-write` does, so a person has one command and the tree is consistent
//! when it ends.

use std::path::Path;

use repo_model::spec_edit::{self, InsertKind, Outcome};

use crate::Fail;

const USAGE: &str = "cargo xtask spec <operation> [--dry-run]\n\
    \n\
    replace-block --chapter ChapterNN --first-line <text> --from <file>\n\
        replace the paragraph, table or fence that begins with <text> by the lines of <file>\n\
    insert-after --chapter ChapterNN --first-line <text> --kind para|table --from <file>\n\
        insert the lines of <file> as a block after the block that begins with <text>\n\
    port --merged <SPEC.md>\n\
        apply each hunk of a merged SPEC.md that is outside the derived regions\n\
    \n\
    --dry-run  verify the edit on a scratch copy and write nothing";

/// A refusal that names its rule, as the library's do (R5): an argument or a
/// file a person got wrong is a closed failure, never a panic.
fn refusal(message: &str, usage: bool) -> Fail {
    let line = format!("R1: {message} (NS-07)");
    if usage {
        format!("{line}\n\n{USAGE}").into()
    } else {
        line.into()
    }
}

fn option<'a>(arguments: &'a [String], name: &str) -> Result<&'a str, Fail> {
    arguments
        .iter()
        .position(|argument| argument == name)
        .and_then(|at| arguments.get(at + 1))
        .map(String::as_str)
        .ok_or_else(|| refusal(&format!("{name} is missing"), true))
}

fn lines_of_file(path: &str) -> Result<Vec<String>, Fail> {
    let text = std::fs::read_to_string(path)
        .map_err(|error| refusal(&format!("{path}: {error}"), false))?;
    if text.is_empty() {
        return Err(refusal(
            &format!("{path} holds no lines: a block is at least one line"),
            false,
        ));
    }
    Ok(text
        .strip_suffix('\n')
        .unwrap_or(&text)
        .split('\n')
        .map(str::to_owned)
        .collect())
}

/// Run the command on the arguments after `spec`.
pub fn run(root: &Path, arguments: &[String]) -> Result<(), Fail> {
    let dry_run = arguments.iter().any(|argument| argument == "--dry-run");
    let Some(operation) = arguments.first() else {
        return Err(refusal("an operation is missing", true));
    };
    let outcome: Outcome = match operation.as_str() {
        "replace-block" => spec_edit::replace_block(
            root,
            option(arguments, "--chapter")?,
            option(arguments, "--first-line")?,
            &lines_of_file(option(arguments, "--from")?)?,
        )?,
        "insert-after" => {
            let kind = match option(arguments, "--kind")? {
                "para" => InsertKind::Paragraph,
                "table" => InsertKind::Table,
                other => {
                    return Err(refusal(
                        &format!("--kind is para or table, not `{other}`"),
                        false,
                    ))
                }
            };
            spec_edit::insert_after(
                root,
                option(arguments, "--chapter")?,
                option(arguments, "--first-line")?,
                kind,
                &lines_of_file(option(arguments, "--from")?)?,
            )?
        }
        "port" => {
            let path = option(arguments, "--merged")?;
            let merged = std::fs::read_to_string(path)
                .map_err(|error| refusal(&format!("{path}: {error}"), false))?;
            // `--register`: a merged `model/ids.toml`, whose changed statements
            // are carried into their rows first (a forward merge from a line
            // without spec/, §9.4). A scratch run leaves the modules alone.
            if let Some(register) = arguments
                .iter()
                .position(|argument| argument == "--register")
                .and_then(|at| arguments.get(at + 1))
            {
                let ids = std::fs::read_to_string(register)
                    .map_err(|error| refusal(&format!("{register}: {error}"), false))?;
                if dry_run {
                    return Err(refusal(
                        "--register writes the register modules; it has no --dry-run",
                        false,
                    ));
                }
                let moved = crate::counts::carry_statements(root, &ids, true)?;
                println!("spec: carried {moved} register statement(s) from {register}");
            }
            spec_edit::port(root, &merged)?
        }
        other => {
            return Err(refusal(&format!("`{other}` is not an operation"), true));
        }
    };
    for line in &outcome.report {
        println!("spec: {line}");
    }
    if dry_run {
        println!(
            "spec: verified on a scratch copy ({} module(s) would change); nothing written",
            outcome.changes.len()
        );
        return Ok(());
    }
    spec_edit::apply(root, &outcome)?;
    crate::codegen::check_model(root, true)?;
    println!(
        "spec: wrote {} module(s) and regenerated SPEC.md and the model files (NS-07)",
        outcome.changes.len()
    );
    Ok(())
}
