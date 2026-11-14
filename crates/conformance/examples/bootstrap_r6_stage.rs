//! Stage retained R6 source and its exact audit without generating certificates again.
use lexlean::production::{certificate::Certificate, preserve};
use repo_conformance::support::{self, P};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

fn retained(directory: &Path) -> Certificate {
    let receipt_digest = format!(
        "{:x}",
        Sha256::digest(std::fs::read(directory.join("source-receipt.json")).unwrap())
    );
    assert!(
        [
            "178fd32041599b1ed9f02ec19c52c99cd6b39b8036b1d3a93d9c5a60a29b4bc3",
            "37120628b07394d522658bf3ea10432eb65b81ea0d457c6f917d2d45291f6414",
        ]
        .contains(&receipt_digest.as_str()),
        "reviewed source-generation receipt"
    );
    let receipt: Value =
        serde_json::from_slice(&std::fs::read(directory.join("source-receipt.json")).unwrap())
            .unwrap();
    assert_eq!(
        receipt["compiler_semantics"],
        lexlean::compiler_semantics_id_for("1.2").to_hex()
    );
    for (name, row) in receipt["files"].as_object().unwrap() {
        let bytes = std::fs::read(directory.join(name)).unwrap();
        assert_eq!(row["sha256"], format!("{:x}", Sha256::digest(&bytes)));
    }
    let source = std::fs::read_to_string(directory.join("R6.lex.tex")).unwrap();
    let body = source
        .split_once("\\begin{coremodule}\n")
        .unwrap()
        .1
        .split_once("\\end{coremodule}")
        .unwrap()
        .0;
    let data = lexlean::surface::core::parse_module(body).expect("retained closed core notation");
    let module = receipt["module"].as_str().unwrap().to_owned();
    assert_eq!(
        module,
        "LexLeanPreserve.C6ec49c97f5334c1cfee5e969876350b6.R6"
    );
    Certificate {
        theorem: receipt["theorem"].as_str().unwrap().to_owned(),
        denote: format!("{module}.denote"),
        module,
        imports: serde_json::from_value(receipt["imports"].clone()).unwrap(),
        text: std::fs::read_to_string(directory.join("R6.lean")).unwrap(),
        data: Some(data),
        source: Some(source),
    }
}

fn main() {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    assert!(
        args.len() == 2 || args.len() == 3,
        "retained-source directory, workspace, optional audit-output file"
    );
    let cert = retained(Path::new(&args[0]));
    let output = PathBuf::from(&args[1]);
    if args.len() == 3 {
        let text = std::fs::read_to_string(&args[2]).unwrap();
        preserve::audit(&text, &[cert]).expect("exact library and all R6 declaration policies");
        println!("R6 certificate A exact axiom audit passed; not full compiler qualification");
        return;
    }
    std::fs::create_dir(&output).expect("new staging directory");
    let project = P::bootstrap();
    let rendered = support::rendered(&project);
    let user: Vec<_> = rendered
        .modules
        .iter()
        .map(|m| (m.lean_module.clone(), m.lean_text.clone()))
        .collect();
    let workspace = preserve::workspace(&user, &[cert]).expect("audited source workspace");
    let mut rows = Vec::new();
    for file in workspace.files.iter().chain([&workspace.audit]) {
        let path = output.join("src").join(&file.path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, &file.text).unwrap();
        rows.push(json!({"module":file.module,"path":file.path,"sha256":format!("{:x}", Sha256::digest(file.text.as_bytes()))}));
    }
    let manifest = json!({"accepted":false,"source_only":true,"files":rows,"certificates":workspace.certificates,"audit":workspace.audit.module});
    std::fs::write(
        output.join("manifest.json"),
        serde_json::to_vec_pretty(&manifest).unwrap(),
    )
    .unwrap();
}
