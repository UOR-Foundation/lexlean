//! One construction path for Lake-hosted Lean consumers. Keep the existing
//! command and recorded target identity together: changing either affects
//! verification evidence, even when the target's arguments are unchanged.

use camino::Utf8Path;

use super::child::{ChildHome, ChildSpec};
use super::toolchain::Toolchain;

pub(super) fn lean<'a>(
    toolchain: &'a Toolchain,
    module: String,
    arguments: Vec<String>,
    workspace: &'a Utf8Path,
    bin_dir: &'a Utf8Path,
    lean_path: &str,
) -> ChildSpec<'a> {
    lake_spec(
        toolchain, false, module, arguments, workspace, bin_dir, lean_path,
    )
}

pub(super) fn leanchecker<'a>(
    toolchain: &'a Toolchain,
    module: &str,
    workspace: &'a Utf8Path,
    bin_dir: &'a Utf8Path,
    lean_path: &str,
) -> ChildSpec<'a> {
    lake_spec(
        toolchain,
        true,
        module.to_owned(),
        vec![module.to_owned()],
        workspace,
        bin_dir,
        lean_path,
    )
}

fn lake_spec<'a>(
    toolchain: &'a Toolchain,
    replay: bool,
    module: String,
    arguments: Vec<String>,
    workspace: &'a Utf8Path,
    bin_dir: &'a Utf8Path,
    lean_path: &str,
) -> ChildSpec<'a> {
    let (tool, executable_sha256, command) = if replay {
        (
            "leanchecker",
            toolchain.leanchecker.sha256,
            toolchain.leanchecker.path.to_string(),
        )
    } else {
        ("lean", toolchain.lean.sha256, "lean".to_owned())
    };
    let mut argv = vec!["env".to_owned(), command];
    argv.extend(arguments);
    ChildSpec {
        tool,
        module: Some(module),
        program: &toolchain.lake.path,
        executable_sha256,
        argv,
        cwd: workspace,
        extra_env: vec![("LEAN_PATH".to_owned(), lean_path.to_owned())],
        home: ChildHome::Toolchain {
            toolchain_bin: bin_dir,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifact::content_id::Sha256Digest;
    use crate::verify::toolchain::Tool;

    fn toolchain() -> Toolchain {
        let tool = |name: &str| Tool {
            path: Utf8Path::new("/tools/bin").join(name),
            sha256: Sha256Digest::of(name.as_bytes()),
            version_output: String::new(),
        };
        Toolchain {
            root: "/tools".into(),
            lean: tool("lean"),
            lake: tool("lake"),
            leanchecker: tool("leanchecker"),
        }
    }

    fn common(spec: &ChildSpec<'_>, path: &str) {
        assert_eq!(spec.program, Utf8Path::new("/tools/bin/lake"));
        assert_eq!(spec.cwd, Utf8Path::new("/workspace/lake"));
        assert_eq!(
            spec.extra_env,
            vec![("LEAN_PATH".to_owned(), path.to_owned())]
        );
        match &spec.home {
            ChildHome::Toolchain { toolchain_bin } => {
                assert_eq!(*toolchain_bin, Utf8Path::new("/tools/bin"));
            }
            _ => panic!("Lake consumers retain the toolchain environment"),
        }
    }

    #[test]
    fn compile_and_preservation_requests_keep_exact_arguments_and_paths() {
        let tools = toolchain();
        for path in ["/staging/oleans", "/staging/preserve:/staging/oleans"] {
            for output in [true, false] {
                let mut arguments = vec![
                    "-R",
                    "/source root",
                    "-o",
                    "/objects/A.olean",
                    "/source root/A.lean",
                ];
                if !output {
                    arguments.drain(2..4);
                }
                let spec = lean(
                    &tools,
                    "A".to_owned(),
                    arguments.iter().map(|s| (*s).to_owned()).collect(),
                    Utf8Path::new("/workspace/lake"),
                    Utf8Path::new("/tools/bin"),
                    path,
                );
                common(&spec, path);
                assert_eq!(spec.tool, "lean");
                assert_eq!(spec.module.as_deref(), Some("A"));
                assert_eq!(spec.executable_sha256, tools.lean.sha256);
                assert_eq!(spec.argv, [vec!["env", "lean"], arguments].concat());
            }
        }
    }

    #[test]
    fn probe_audit_and_extraction_keep_source_argument_without_output_flags() {
        let tools = toolchain();
        for module in ["Probe", "Audit", "Extract"] {
            let source = format!("/staging/{module}.lean");
            let spec = lean(
                &tools,
                module.to_owned(),
                vec![source.clone()],
                Utf8Path::new("/workspace/lake"),
                Utf8Path::new("/tools/bin"),
                "/staging/oleans",
            );
            common(&spec, "/staging/oleans");
            assert_eq!(spec.argv, vec!["env".to_owned(), "lean".to_owned(), source]);
            assert_eq!(spec.module.as_deref(), Some(module));
        }
    }

    #[test]
    fn replay_keeps_absolute_checked_target_and_identity() {
        let tools = toolchain();
        let spec = leanchecker(
            &tools,
            "Rule30UOR.Generated",
            Utf8Path::new("/workspace/lake"),
            Utf8Path::new("/tools/bin"),
            "/staging/oleans",
        );
        common(&spec, "/staging/oleans");
        assert_eq!(spec.tool, "leanchecker");
        assert_eq!(spec.module.as_deref(), Some("Rule30UOR.Generated"));
        assert_eq!(spec.executable_sha256, tools.leanchecker.sha256);
        assert_eq!(
            spec.argv,
            ["env", "/tools/bin/leanchecker", "Rule30UOR.Generated"]
        );
    }
}
