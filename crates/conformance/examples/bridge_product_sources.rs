//! Source-only official package/tooling generation and selected fixture checks.
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let root = PathBuf::from(args.next().ok_or("repository root required")?);
    let output = PathBuf::from(args.next().ok_or("new output directory required")?);
    if args.next().is_some() {
        return Err("unexpected argument".into());
    }
    std::fs::create_dir(&output)?;
    let mut generated = repo_conformance::rust_packages::files();
    for fixture in repo_conformance::tooling::fixtures(&root)? {
        let relative = fixture.strip_prefix(&root)?.to_string_lossy();
        let (_temp, files) = repo_conformance::tooling::files(&fixture)?;
        for (path, bytes) in files {
            generated.insert(format!("{relative}/expected/build/{path}"), bytes);
        }
    }
    let mut rows = Vec::new();
    for (relative, bytes) in generated {
        let target = output.join(&relative);
        std::fs::create_dir_all(target.parent().ok_or("parent required")?)?;
        std::fs::write(&target, &bytes)?;
        rows.push(serde_json::json!({"path": relative, "bytes": bytes.len(),
            "matches_committed": std::fs::read(root.join(&relative)).ok().as_ref() == Some(&bytes)}));
    }
    std::fs::write(
        output.join("receipt.json"),
        serde_json::to_vec_pretty(&rows)?,
    )?;
    println!(
        "{} official source products; no Lean, exports or verification",
        rows.len()
    );
    Ok(())
}
