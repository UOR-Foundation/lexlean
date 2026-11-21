//! The Lean side of a browser prover (SPEC.md §30.3): the runtime modules as
//! a Lake package, so a release builds their `.olean` files with the pinned
//! Lean instead of every host compiling them itself.

use std::path::Path;

use crate::Fail;

/// The package's Lake library: every runtime module sits under it.
const LIBRARY: &str = "LexLeanRuntime";

/// Write the runtime as a Lake package at `--output DIR`. The modules are the
/// Lean this compiler generates from the shipped native sources, the bytes a
/// verification compiles (§17.12); the toolchain file is the pinned one.
pub fn runtime_package(root: &Path, arguments: &[String]) -> Result<(), Fail> {
    let output = match arguments {
        [flag, dir] if flag == "--output" => Path::new(dir).to_path_buf(),
        _ => return Err("usage: cargo xtask runtime-package --output DIR".into()),
    };
    if output.exists() && std::fs::read_dir(&output)?.next().is_some() {
        return Err(format!("{}: the output directory is not empty", output.display()).into());
    }
    std::fs::create_dir_all(&output)?;
    let modules = lexlean::backend::runtime::modules()
        .map_err(|diagnostic| format!("the runtime: {}", diagnostic.message))?;
    for module in modules {
        let path = output.join(format!("{}.lean", module.name.replace('.', "/")));
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&path, module.text.as_bytes())?;
    }
    std::fs::copy(root.join("lean-toolchain"), output.join("lean-toolchain"))?;
    std::fs::write(
        output.join("lakefile.toml"),
        format!(
            "name = \"lexlean-runtime\"\ndefaultTargets = [\"{LIBRARY}\"]\n\n[[lean_lib]]\nname = \"{LIBRARY}\"\nglobs = [\"{LIBRARY}.+\"]\n"
        ),
    )?;
    let names: Vec<&str> = modules.iter().map(|module| module.name.as_str()).collect();
    std::fs::write(
        output.join("modules.txt"),
        format!("{}\n", names.join("\n")),
    )?;
    println!(
        "runtime-package: {} modules at {}",
        modules.len(),
        output.display()
    );
    Ok(())
}
