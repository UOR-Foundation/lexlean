//! Bounded source staging for one bootstrap root; no compiler acceptance.
use lexlean::calculus::rust::{Profile, lower};
use lexlean::production::certificate::{Certificate, certificate};
use lexlean::production::lower::{linked_modules, lower_root, roots};
use lexlean::production::preserve;
use lexlean::production::rust_cert::{certificate_b, module_for};
use repo_conformance::preservation::Certified;
use repo_conformance::support::{self, P};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Serialize, Deserialize)]
struct SavedCertificate {
    module: String,
    text: String,
    theorem: String,
    denote: String,
    imports: Vec<String>,
    data: Option<lexlean::ir::core::CoreModule>,
    source: Option<String>,
}
fn git(root: &Path, args: &[&str]) -> String {
    let o = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .unwrap();
    assert!(o.status.success());
    String::from_utf8(o.stdout).unwrap().trim().to_owned()
}
fn sha(path: &Path) -> String {
    use std::io::Read;
    let mut f = std::fs::File::open(path).unwrap();
    let mut h = Sha256::new();
    let mut b = [0u8; 65536];
    loop {
        let n = f.read(&mut b).unwrap();
        if n == 0 {
            break;
        }
        h.update(&b[..n]);
    }
    format!("{:x}", h.finalize())
}
fn certificates(project: &P, selected: usize) -> Vec<Certified> {
    let checked = support::checked_project(project);
    let modules = linked_modules(&checked);
    let hex32: String = checked.semantic_id.to_hex()[..32].to_owned();
    let limits = support::limits(project);
    roots(&checked)
        .expect("the eligibility reports")
        .iter()
        .enumerate()
        .filter(|(index, _)| *index == selected)
        .map(|(index, root)| {
            let lowered = lower_root(&modules, &root.module, &root.name, root.report, &limits)
                .unwrap_or_else(|diagnostic| {
                    panic!("{}: lowering failed: {diagnostic:?}", root.report.root)
                });
            let module = format!("LexLeanPreserve.C{hex32}.R{index}");
            let certificate = certificate(
                &modules,
                &root.module,
                &root.name,
                root.report,
                &lowered,
                &module,
                limits.max_file_bytes,
            )
            .unwrap_or_else(|diagnostic| {
                panic!(
                    "{}: certificate generation failed: {diagnostic:?}",
                    root.report.root
                )
            });
            let renderings = root
                .report
                .targets
                .iter()
                .map(|row| {
                    let profile = Profile::named(&row.target).expect("a Rust profile");
                    let krate = lower(&lowered.program, profile).unwrap_or_else(|reason| {
                        panic!(
                            "{} has no {} rendering: {reason}",
                            root.report.root, row.target
                        )
                    });
                    let module_b = module_for(&module, &row.target).expect("a Rust target");
                    let certificate_b =
                        certificate_b(&lowered.program, &krate, &module_b, limits.max_file_bytes)
                            .unwrap_or_else(|reason| {
                                panic!(
                                    "{} on {}: certificate B: {reason}",
                                    root.report.root, row.target
                                )
                            });
                    (row.target.clone(), certificate_b.into_certificate())
                })
                .collect();
            let fallible = lexlean::calculus::rust::fallible_functions(&lowered.program)
                .expect("a valid program")[lowered.entry() as usize];
            let composed = root
                .report
                .targets
                .iter()
                .map(|row| {
                    let module_b = module_for(&module, &row.target).expect("a Rust target");
                    let module_e =
                        module_for(&format!("{module}.Compose"), &row.target).expect("a target");
                    let certificate_e = lexlean::production::certificate::certificate_e(
                        &modules,
                        &root.module,
                        &root.name,
                        &lowered,
                        &module,
                        &module_b,
                        &module_e,
                        fallible,
                        limits.max_file_bytes,
                    )
                    .unwrap_or_else(|diagnostic| {
                        panic!(
                            "{} on {}: certificate E: {diagnostic:?}",
                            root.report.root, row.target
                        )
                    });
                    (row.target.clone(), certificate_e)
                })
                .collect();
            Certified {
                root: root.report.root.clone(),
                targets: root
                    .report
                    .targets
                    .iter()
                    .map(|row| row.target.clone())
                    .collect(),
                certificate,
                entry: lowered.entry(),
                program: lowered.program,
                renderings,
                composed,
            }
        })
        .collect()
}

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    assert!(args.len() == 2 || args.len() == 3);
    let output = PathBuf::from(&args[1]);
    if args[0] == "audit" {
        let manifest: serde_json::Value =
            serde_json::from_reader(std::fs::File::open(output.join("manifest.json")).unwrap())
                .unwrap();
        assert_eq!(
            manifest["generator_sha256"],
            sha(&std::env::current_exe().unwrap())
        );
        assert_eq!(
            manifest["source_checkpoint"],
            env!("LEXLEAN_DIAGNOSTIC_SOURCE")
        );
        assert_eq!(
            manifest["compiler_semantics"],
            lexlean::compiler_semantics_id_for("1.2").to_hex()
        );
        let targets = manifest["targets"].as_array().unwrap();
        assert!(!targets.is_empty());
        assert_eq!(
            manifest["saved_certificates"].as_array().unwrap().len(),
            1 + 2 * targets.len()
        );
        let base = format!(
            "LexLeanPreserve.C{}.R{}",
            &manifest["semantic_id"].as_str().unwrap()[..32],
            manifest["root_index"].as_u64().unwrap()
        );
        let expected: Vec<_> = std::iter::once(base.clone())
            .chain(
                targets
                    .iter()
                    .map(|t| module_for(&base, t.as_str().unwrap()).unwrap()),
            )
            .chain(
                targets
                    .iter()
                    .map(|t| module_for(&format!("{base}.Compose"), t.as_str().unwrap()).unwrap()),
            )
            .collect();
        let mut certs = Vec::new();
        for row in manifest["saved_certificates"].as_array().unwrap() {
            let path = output.join(row["path"].as_str().unwrap());
            assert_eq!(sha(&path), row["sha256"].as_str().unwrap());
            let s: SavedCertificate =
                serde_json::from_reader(std::fs::File::open(path).unwrap()).unwrap();
            assert_eq!(s.module, expected[certs.len()]);
            assert_eq!(row["module"], s.module);
            assert_eq!(row["theorem"], s.theorem);
            assert!(s.data.is_some() && s.source.is_some());
            certs.push(Certificate {
                module: s.module,
                text: s.text,
                theorem: s.theorem,
                denote: s.denote,
                imports: s.imports,
                data: s.data,
                source: s.source,
            });
        }
        preserve::audit(&std::fs::read_to_string(&args[2]).unwrap(), &certs)
            .expect("exact certificate and library policies");
        println!("Exact selected-root A/B/E axiom policies passed; compiler acceptance pending");
        return;
    }
    assert_eq!(args.len(), 2);
    let selected: usize = args[0].parse().unwrap();
    let repo = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    assert!(git(repo, &["status", "--porcelain", "--untracked-files=normal"]).is_empty());
    let head = git(repo, &["rev-parse", "HEAD"]);
    assert_eq!(head, env!("LEXLEAN_DIAGNOSTIC_SOURCE"));
    std::fs::create_dir(&output).expect("new output directory");
    let project = P::bootstrap();
    // Qualification uses the composed compiler identity; retain the newly computed lock.
    project.relock();
    std::fs::write(output.join("lexlean.lock"), project.read("lexlean.lock")).unwrap();
    let checked = support::checked_project(&project);
    let all_roots = roots(&checked).unwrap();
    assert!(selected < all_roots.len());
    let root_names: Vec<_> = all_roots
        .iter()
        .enumerate()
        .map(|(i, r)| json!({"index":i,"root":r.report.root}))
        .collect();
    let certified = certificates(&project, selected);
    assert_eq!(certified.len(), 1);
    let certs = certified[0].all();
    let rendered = support::rendered(&project);
    let user: Vec<_> = rendered
        .modules
        .iter()
        .map(|m| (m.lean_module.clone(), m.lean_text.clone()))
        .collect();
    let staged = preserve::workspace(&user, &certs).unwrap();
    let mut files = Vec::new();
    for file in staged.files.iter().chain([&staged.audit]) {
        let path = output.join("src").join(&file.path);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, &file.text).unwrap();
        files.push(json!({"module":file.module,"path":file.path,"sha256":sha(&path)}));
    }
    let mut saved = Vec::new();
    for (i, c) in certs.into_iter().enumerate() {
        let name = format!("certificate-{i}.json");
        let path = output.join(&name);
        let s = SavedCertificate {
            module: c.module,
            text: c.text,
            theorem: c.theorem,
            denote: c.denote,
            imports: c.imports,
            data: c.data,
            source: c.source,
        };
        serde_json::to_writer(std::fs::File::create(&path).unwrap(), &s).unwrap();
        saved.push(json!({"path":name,"sha256":sha(&path),"module":s.module,"theorem":s.theorem}));
    }
    let manifest = json!({"accepted":false,"source_only":true,"source_checkpoint":head,"source_tree":git(repo,&["rev-parse","HEAD^{tree}"]),"generator_sha256":sha(&std::env::current_exe().unwrap()),"compiler_semantics":lexlean::compiler_semantics_id_for("1.2").to_hex(),"semantic_id":checked.semantic_id.to_hex(),"root_index":selected,"roots":root_names,"root":certified[0].root,"targets":certified[0].targets,"lock_sha256":sha(&output.join("lexlean.lock")),"files":files,"certificates":staged.certificates,"audit":staged.audit.module,"saved_certificates":saved,"not_qualified":["aggregate production budget","full production verification and export","full compiler acceptance"]});
    std::fs::write(
        output.join("manifest.json"),
        serde_json::to_vec_pretty(&manifest).unwrap(),
    )
    .unwrap();
    println!("{}", manifest);
}
