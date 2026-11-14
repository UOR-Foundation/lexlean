//! Read-only, offline checks of lock-reconciled single-step check fixtures.
use camino::Utf8PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let root = Utf8PathBuf::from(args.next().ok_or("repository root required")?);
    let receipt = Utf8PathBuf::from(args.next().ok_or("new receipt path required")?);
    if args.next().is_some() {
        return Err("unexpected argument".into());
    }
    let mut output = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&receipt)?;
    let inventory: serde_json::Value = serde_json::from_slice(&std::fs::read(
        root.join("diagnostics/rule30-inspection-b02/bridge-identity-inventory.json"),
    )?)?;
    let mut rows = Vec::new();
    for row in inventory["locks"].as_array().ok_or("lock rows required")? {
        let path = row["path"].as_str().ok_or("lock path required")?;
        let config_dir = root
            .join(path)
            .parent()
            .ok_or("lock parent required")?
            .to_owned();
        let Some(dir) = config_dir.parent() else {
            continue;
        };
        if !dir.join("case.toml").is_file() {
            continue;
        }
        let case = repo_conformance::fixtures::load_case(dir)?;
        if case.invocations.len() != 1 || case.invocations[0].command != "check" {
            rows.push(serde_json::json!({"path":path,"status":"outside-single-check-scope"}));
            continue;
        }
        let result = repo_conformance::fixtures::check(dir);
        let result = match result {
            Ok(observed) => {
                serde_json::json!({"path":path,"status":"passed","exit":observed.exit,"codes":observed.codes})
            }
            Err(error) => serde_json::json!({"path":path,"status":"failed","error":error}),
        };
        println!("{path}: {}", result["status"]);
        rows.push(result);
    }
    serde_json::to_writer_pretty(&mut output, &rows)?;
    if rows.iter().any(|r| r["status"] == "failed") {
        return Err("fixture checks failed; see receipt".into());
    }
    Ok(())
}
