//! Typed process fragment for a future merged-view launcher (§17.20).
//!
//! This is not an executable launcher or an execution authorization. The caller
//! must first enforce full Descriptor::parse, exact argv/source/root-binding
//! custody, support-import protection, tool identity, and supervision. It must
//! embed this fragment only after the fixed payload, cwd and captured LEAN_PATH
//! checks. No process is started by this module. The raw launcher draft is not
//! used. Native Lean 4.32.1 signature/constructor/closure probes remain required.
//!
//! Declarations below are transcribed from the exact pinned IO source, not from
//! successful elaboration. In particular SpawnArgs' inherited constructor field
//! and the dependent signatures must pass the existing authority probes before
//! activation. No instance-plumbing allowlist is invented here.

use std::path::{Component, Path};

use crate::ir::tooling::{
    BinderInfo, ParameterKind, Sig, SourcePin, ToolConstructor, ToolDeclaration, ToolField,
    ToolParameter, ToolTerm, ToolType, TypeParameter,
};

const IO_SOURCE: &str = "src/lean/Init/System/IO.lean";
const IO_SHA256: &str = "4612b91e65806364c814fc445b35598d8e26ee0f2a811ffde6c330d792dcddab";
const STDIO: &str = "IO.Process.Stdio";
const CONFIG: &str = "IO.Process.StdioConfig";
const ARGS: &str = "IO.Process.SpawnArgs";
const CHILD: &str = "IO.Process.Child";

/// Only the existing compiler child argument shapes. All paths are resolved
/// host paths; their correspondence with descriptor logical paths is a separate
/// mandatory custody check, not established by syntactic confinement here.
#[derive(Clone, Debug)]
pub(super) enum Request {
    Lean {
        source: String,
        root: Option<String>,
        output: Option<String>,
    },
    Kernel {
        module: String,
    },
}

/// Immutable constants for the process fragment, with no arbitrary argv or
/// environment entry injection. Construction validates syntax, not byte custody.
#[derive(Clone, Debug)]
pub(super) struct Process {
    executable: String,
    arguments: Vec<String>,
    effective_lean_path: String,
}

impl Process {
    pub(super) fn new(
        toolchain_root: &str,
        staging_root: &str,
        executable: String,
        request: Request,
        effective_lean_path: String,
    ) -> Result<Self, String> {
        absolute(toolchain_root)?;
        absolute(staging_root)?;
        beneath(&executable, toolchain_root)?;
        if effective_lean_path.is_empty() || effective_lean_path.contains('\0') {
            return Err("invalid effective Lean path".into());
        }
        let (tool, arguments) = match request {
            Request::Lean {
                source,
                root,
                output,
            } => {
                beneath(&source, staging_root)?;
                let mut arguments = Vec::new();
                if let Some(root) = root {
                    // The root itself may be the checked staging directory.
                    absolute(&root)?;
                    if !Path::new(&root).starts_with(staging_root) {
                        return Err("Lean root outside staging".into());
                    }
                    arguments.extend(["-R".into(), root]);
                } else if output.is_some() {
                    return Err("Lean output requires its compilation root".into());
                }
                if let Some(output) = output {
                    beneath(&output, staging_root)?;
                    arguments.extend(["-o".into(), output]);
                }
                arguments.push(source);
                ("lean", arguments)
            }
            Request::Kernel { module } => {
                if module.is_empty()
                    || module.split('.').any(|part| {
                        part.is_empty()
                            || !part.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
                            || part.as_bytes()[0].is_ascii_digit()
                    })
                {
                    return Err("unsupported kernel module name".into());
                }
                ("leanchecker", vec![module])
            }
        };
        let expected = if cfg!(windows) {
            format!("{tool}.exe")
        } else {
            tool.into()
        };
        if Path::new(&executable).file_name().and_then(|s| s.to_str()) != Some(expected.as_str()) {
            return Err("target tool does not match request".into());
        }
        Ok(Self {
            executable,
            arguments,
            effective_lean_path,
        })
    }

    /// `let args := <closed constructor>; bind (spawn args) (fun child => wait child)`.
    /// The let is essential: the checker resolves only this immutable value's
    /// toStdioConfig field, never an arbitrary argument or forged Child index.
    pub(super) fn body(&self) -> ToolTerm {
        let config = construct(
            "IO.Process.StdioConfig.mk",
            vec![],
            vec![
                construct("IO.Process.Stdio.inherit", vec![], vec![]),
                construct("IO.Process.Stdio.inherit", vec![], vec![]),
                construct("IO.Process.Stdio.inherit", vec![], vec![]),
            ],
        );
        let pair = ty(
            "Prod",
            vec![
                ty("String", vec![]),
                ty("Option", vec![ty("String", vec![])]),
            ],
        );
        let environment = construct(
            "Prod.mk",
            vec![
                ty("String", vec![]),
                ty("Option", vec![ty("String", vec![])]),
            ],
            vec![
                string("LEAN_PATH"),
                construct(
                    "Option.some",
                    vec![ty("String", vec![])],
                    vec![string(&self.effective_lean_path)],
                ),
            ],
        );
        let value = construct(
            "IO.Process.SpawnArgs.mk",
            vec![],
            vec![
                config,
                string(&self.executable),
                ToolTerm::Array {
                    element: ty("String", vec![]),
                    elements: self.arguments.iter().map(|s| string(s)).collect(),
                },
                construct("Option.none", vec![ty("System.FilePath", vec![])], vec![]),
                ToolTerm::Array {
                    element: pair,
                    elements: vec![environment],
                },
                construct("Bool.true", vec![], vec![]),
                construct("Bool.false", vec![], vec![]),
            ],
        );
        let child = ty(CHILD, vec![inherited_config()]);
        ToolTerm::Let {
            binder: "launchArgs".into(),
            ty: ty(ARGS, vec![]),
            value: Box::new(value),
            body: Box::new(call(
                "Bind.bind",
                vec![ty("IO", vec![]), child.clone(), ty("UInt32", vec![])],
                vec![
                    call("IO.Process.spawn", vec![], vec![variable("launchArgs")]),
                    ToolTerm::Lambda {
                        parameters: vec![ToolParameter {
                            name: "launchChild".into(),
                            ty: child,
                        }],
                        body: Box::new(call(
                            "IO.Process.Child.wait",
                            vec![],
                            vec![variable("launchChild")],
                        )),
                    },
                ],
            )),
        }
    }
}

/// Actual IO.getEnv is BaseIO, not IO. Preserve absent-versus-empty values for
/// the caller's subsequent comparison with the captured Lake overlay.
pub(super) fn environment_probe() -> ToolTerm {
    call(
        "BaseIO.toIO",
        vec![ty("Option", vec![ty("String", vec![])])],
        vec![call("IO.getEnv", vec![], vec![string("LEAN_PATH")])],
    )
}

/// Required baseline registry: Array, Bool, Option, Prod, String, UInt32, IO,
/// System.FilePath, Bind and the exact Bind.bind signature. They must come from
/// the independently pinned authority registry; this function does not erase or
/// synthesize those declarations. Returned rows are subject to native probes.
pub(super) fn process_declarations() -> Vec<ToolDeclaration> {
    let option_string = ty("Option", vec![ty("String", vec![])]);
    let stdio = declaration(
        STDIO,
        vec![],
        vec![
            constructor("piped", vec![]),
            constructor("inherit", vec![]),
            constructor("null", vec![]),
        ],
        false,
    );
    let config = declaration(
        CONFIG,
        vec![],
        vec![constructor(
            "mk",
            vec![
                field("stdin", ty(STDIO, vec![])),
                field("stdout", ty(STDIO, vec![])),
                field("stderr", ty(STDIO, vec![])),
            ],
        )],
        false,
    );
    let args = declaration(
        ARGS,
        vec![],
        vec![constructor(
            "mk",
            vec![
                field("toStdioConfig", ty(CONFIG, vec![])),
                field("cmd", ty("String", vec![])),
                field("args", ty("Array", vec![ty("String", vec![])])),
                field("cwd", ty("Option", vec![ty("System.FilePath", vec![])])),
                field(
                    "env",
                    ty(
                        "Array",
                        vec![ty(
                            "Prod",
                            vec![ty("String", vec![]), option_string.clone()],
                        )],
                    ),
                ),
                field("inheritEnv", ty("Bool", vec![])),
                field("setsid", ty("Bool", vec![])),
            ],
        )],
        false,
    );
    let child = declaration(
        CHILD,
        vec![TypeParameter {
            name: "cfg".into(),
            kind: ParameterKind::RecordIndex { of: CONFIG.into() },
        }],
        vec![],
        true,
    );
    vec![
        stdio,
        config,
        args,
        child,
        declaration(
            "BaseIO",
            vec![TypeParameter {
                name: "α".into(),
                kind: ParameterKind::Type,
            }],
            vec![],
            true,
        ),
        authority_call(
            "IO.Process.SpawnArgs.toStdioConfig",
            arrow(constant(ARGS), constant(CONFIG)),
        ),
        authority_call(
            "IO.Process.spawn",
            pi(
                "args",
                BinderInfo::Explicit,
                constant(ARGS),
                app(
                    "IO",
                    vec![app(
                        CHILD,
                        vec![app(
                            "IO.Process.SpawnArgs.toStdioConfig",
                            vec![sig_var("args")],
                        )],
                    )],
                ),
            ),
        ),
        authority_call(
            "IO.Process.Child.wait",
            pi(
                "cfg",
                BinderInfo::Implicit,
                constant(CONFIG),
                arrow(
                    app(CHILD, vec![sig_var("cfg")]),
                    app("IO", vec![constant("UInt32")]),
                ),
            ),
        ),
        authority_call(
            "IO.getEnv",
            arrow(
                constant("String"),
                app("BaseIO", vec![app("Option", vec![constant("String")])]),
            ),
        ),
        authority_call(
            "BaseIO.toIO",
            pi(
                "α",
                BinderInfo::Implicit,
                Sig::Type { level: None },
                arrow(
                    app("BaseIO", vec![sig_var("α")]),
                    app("IO", vec![sig_var("α")]),
                ),
            ),
        ),
    ]
}

fn absolute(value: &str) -> Result<(), String> {
    if value.contains('\0')
        || value
            .split(['/', '\\'])
            .any(|part| matches!(part, "." | ".."))
        || !Path::new(value).is_absolute()
        || Path::new(value)
            .components()
            .any(|c| matches!(c, Component::CurDir | Component::ParentDir))
    {
        Err("invalid resolved absolute path".into())
    } else {
        Ok(())
    }
}
fn beneath(value: &str, root: &str) -> Result<(), String> {
    absolute(value)?;
    let relative = Path::new(value)
        .strip_prefix(root)
        .map_err(|_| "path outside required root")?;
    if relative.as_os_str().is_empty() {
        Err("expected file beneath root".into())
    } else {
        Ok(())
    }
}
fn ty(name: &str, arguments: Vec<ToolType>) -> ToolType {
    ToolType::Const {
        name: name.into(),
        arguments,
    }
}
fn string(value: &str) -> ToolTerm {
    ToolTerm::String {
        value: value.into(),
    }
}
fn variable(name: &str) -> ToolTerm {
    ToolTerm::Var { name: name.into() }
}
fn call(name: &str, type_arguments: Vec<ToolType>, arguments: Vec<ToolTerm>) -> ToolTerm {
    ToolTerm::Call {
        function: name.into(),
        type_arguments,
        arguments,
    }
}
fn construct(name: &str, type_arguments: Vec<ToolType>, arguments: Vec<ToolTerm>) -> ToolTerm {
    ToolTerm::Construct {
        constructor: name.into(),
        type_arguments,
        arguments,
    }
}
fn inherited_config() -> ToolType {
    ToolType::RecordIndex {
        constructor: "IO.Process.StdioConfig.mk".into(),
        arguments: (0..3)
            .map(|_| ToolType::Index {
                constructor: "IO.Process.Stdio.inherit".into(),
            })
            .collect(),
    }
}
fn pin() -> SourcePin {
    SourcePin {
        file: IO_SOURCE.into(),
        sha256: IO_SHA256.into(),
    }
}
fn field(name: &str, ty: ToolType) -> ToolField {
    ToolField {
        name: name.into(),
        ty,
    }
}
fn constructor(name: &str, fields: Vec<ToolField>) -> ToolConstructor {
    ToolConstructor {
        name: name.into(),
        fields,
        requires: None,
    }
}
fn declaration(
    name: &str,
    parameters: Vec<TypeParameter>,
    constructors: Vec<ToolConstructor>,
    opaque: bool,
) -> ToolDeclaration {
    ToolDeclaration::Type {
        name: name.into(),
        parameters,
        constructors,
        opaque,
        alias: None,
        source: pin(),
    }
}
fn constant(name: &str) -> Sig {
    Sig::Const { name: name.into() }
}
fn sig_var(name: &str) -> Sig {
    Sig::Var { name: name.into() }
}
fn app(name: &str, arguments: Vec<Sig>) -> Sig {
    Sig::App {
        function: Box::new(constant(name)),
        arguments,
    }
}
fn arrow(domain: Sig, codomain: Sig) -> Sig {
    Sig::Arrow {
        domain: Box::new(domain),
        codomain: Box::new(codomain),
    }
}
fn pi(name: &str, info: BinderInfo, domain: Sig, codomain: Sig) -> Sig {
    Sig::Pi {
        name: Some(name.into()),
        info,
        domain: Box::new(domain),
        codomain: Box::new(codomain),
    }
}
fn authority_call(name: &str, signature: Sig) -> ToolDeclaration {
    ToolDeclaration::Call {
        name: name.into(),
        signature,
        role: "typed launcher process fragment; native authority probe required before execution"
            .into(),
        source: pin(),
    }
}
