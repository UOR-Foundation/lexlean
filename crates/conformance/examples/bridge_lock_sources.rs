//! Offline public lock API with strict identity-only output acceptance.
use camino::Utf8PathBuf;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let root = Utf8PathBuf::from(args.next().ok_or("repository root required")?);
    let output = Utf8PathBuf::from(args.next().ok_or("new receipt path required")?);
    if args.next().is_some() {
        return Err("unexpected argument".into());
    }
    let mut receipt = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output)?;
    let old = "59f64efe727898bcf6a337630c38eb4ea7ffc08cd83a49a0ea669c61c3b23587";
    let new = "b6c666bbad446ffded2244b5f88430d601c5068cfd6915187e2cc49dbf8746f3";
    assert_eq!(lexlean::compiler_semantics_id_for("1.2").to_hex(), new);
    let inventory: serde_json::Value = serde_json::from_slice(&std::fs::read(
        root.join("diagnostics/rule30-inspection-b02/bridge-identity-inventory.json"),
    )?)?;
    let mut rows = Vec::new();
    for row in inventory["locks"].as_array().ok_or("lock rows required")? {
        let relative = row["path"].as_str().ok_or("lock path required")?;
        let path = root.join(relative);
        let before = std::fs::read_to_string(&path)?;
        if relative == "tests/negative/lock-schema-language-mismatch/project/lexlean.lock" {
            rows.push(serde_json::json!({"path":relative,"status":"preserved-deliberate-schema-mismatch"}));
            continue;
        }
        if !before.contains(old) {
            if !before.contains(new) {
                return Err(format!("unexpected identity: {relative}").into());
            }
            rows.push(serde_json::json!({"path":relative,"status":"already-updated-cli"}));
            continue;
        }
        let engine =
            lexlean::Engine::load(&path.parent().ok_or("parent required")?.join("lexlean.toml"))?;
        let result = engine.lock(lexlean::LockRequest {
            check_only: false,
            allow_network: false,
        });
        let after = std::fs::read_to_string(&path)?;
        if result.is_err() || after != before.replace(old, new) {
            std::fs::write(&path, before)?;
            rows.push(serde_json::json!({"path":relative,"status":"restored-needs-review","error":result.err().map(|e|e.to_string())}));
        } else {
            rows.push(serde_json::json!({"path":relative,"status":"updated-official-api"}));
        }
    }
    serde_json::to_writer_pretty(&mut receipt, &rows)?;
    if rows.iter().any(|r| r["status"] == "restored-needs-review") {
        return Err("unexpected lock result; originals restored".into());
    }
    Ok(())
}
