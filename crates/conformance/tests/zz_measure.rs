//! Temporary measurement of certificate A sizes.
use repo_conformance::support::{self, P};

#[test]
fn measure_certificate_a() {
    let project = P::bootstrap();
    let checked = support::checked_project(&project);
    let modules = lexlean::production::lower::linked_modules(&checked);
    let limits = support::limits(&project);
    for root in lexlean::production::lower::roots(&checked).expect("roots") {
        let wanted = std::env::var("ROOTS").unwrap_or_else(|_| "siteTrees".into());
        if !wanted.split(',').any(|w| root.report.root.ends_with(&format!(".{w}"))) {
            continue;
        }
        let lowered = lexlean::production::lower::lower_root(&modules, &root.module, &root.name, root.report, &limits).expect("lowers");
        let cert = lexlean::production::certificate::certificate(&modules, &root.module, &root.name, root.report, &lowered, "LexLeanPreserve.Cc8ed8262bcbb0e43096883c266b224d2.R4", u64::MAX).expect("certificate");
        let src = cert.source.expect("source");
        println!("MEASURE {} {}", root.report.root, src.len());
        if let Ok(dir) = std::env::var("DUMP") {
            std::fs::write(format!("{dir}/{}.tex", root.name), &src).expect("dump");
            let names: Vec<String> = lowered.layout.functions.iter().map(|o| format!("{o:?}")).collect();
            std::fs::write(format!("{dir}/{}.fns", root.name), names.join("\n")).expect("dump");
        }
    }
}
