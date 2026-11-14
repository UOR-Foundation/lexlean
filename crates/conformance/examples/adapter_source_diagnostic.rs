//! Generate only the inspection adapter's registry and source in a new directory.
//! This elaborates the registry exporter and adapter listing; it does not run
//! production exports, verify a project, or rewrite the checkout's products.

use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args_os().skip(1);
    let output = PathBuf::from(args.next().ok_or("expected one new output directory")?);
    if args.next().is_some() {
        return Err("expected one new output directory".into());
    }
    std::fs::create_dir(&output)?;
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or("missing crates directory")?
        .parent()
        .ok_or("missing repository directory")?
        .to_owned();
    let (rows, plumbing) = repo_conformance::lcnf_registry::derived(&root)?;
    let mut files = repo_conformance::lcnf_registry::files(&rows, &plumbing);
    files.insert(
        repo_conformance::lcnf_registry::REGISTRY_PATH.to_owned(),
        repo_conformance::lcnf_registry::registry_bytes(&rows, &plumbing),
    );
    let mut inventory = Vec::new();
    for (name, bytes) in files {
        let path = output.join(&name);
        std::fs::create_dir_all(path.parent().ok_or("missing output parent")?)?;
        std::fs::write(&path, &bytes)?;
        inventory.push(serde_json::json!({
            "path": name,
            "bytes": bytes.len(),
            "sha256": lexlean::artifact::content_id::Sha256Digest::of(&bytes).to_hex(),
        }));
    }
    let receipt = serde_json::json!({
        "status": "adapter-source-generated",
        "acceptance": false,
        "authority": repo_conformance::lcnf_registry::AUTHORITY,
        "registry_rows": rows.len(),
        "plumbing_names": plumbing.len(),
        "files": inventory,
    });
    std::fs::write(
        output.join("receipt.json"),
        serde_json::to_vec_pretty(&receipt)?,
    )?;
    println!("{}", receipt);
    Ok(())
}
