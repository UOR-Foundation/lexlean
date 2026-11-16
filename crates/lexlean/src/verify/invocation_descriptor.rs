//! Strict, canonical data for a future Lake-hosted invocation chain.
//!
//! This codec does not authorize execution. The caller must establish tool,
//! source, producer and complete view custody, resolve typed roots, and admit
//! platform argv/environment limits. Hash-consistent data is not provenance.
//! No process record or descriptor digest is included in its own preimage.
//! Search-path entries are typed paths or the empty-string sentinel for an
//! actual empty entry; absence differs from a present empty list. Callers must
//! preserve the pinned platform's splitting/joining semantics, not drop entries.
//! Other inherited environment variables and the launcher's own import closure
//! are not established here. In particular Lake-shadowed Lean/Init imports must
//! not become trusted merely because the fixed launcher source hash matches.

use std::collections::{BTreeMap, BTreeSet};

use crate::artifact::canonical_json::Json;
use crate::artifact::content_id::{FramedHasher, Sha256Digest};

const SPEC: &str = "lexlean/invocation-chain/1";
const PAYLOAD_SPEC: &str = "lexlean/invocation-payload/1";
const MAX_DEPTH: usize = 16;
const INVOCATION_FIELDS: &[&str] = &[
    "spec",
    "context",
    "lake",
    "launcher",
    "target",
    "arguments",
    "cwd",
    "environment",
    "view",
];

/// A stable identity, never a host absolute path. Library roots are identified
/// by an independently checked inventory digest, not an arbitrary alias.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Root {
    Toolchain,
    Workspace,
    Project,
    Staging,
    Library(Sha256Digest),
}

impl Root {
    fn text(&self) -> String {
        match self {
            Self::Toolchain => "toolchain".into(),
            Self::Workspace => "workspace".into(),
            Self::Project => "project".into(),
            Self::Staging => "staging".into(),
            Self::Library(hash) => format!("library:{}", hash.to_hex()),
        }
    }
    fn parse(value: &str) -> Result<Self, String> {
        match value {
            "toolchain" => Ok(Self::Toolchain),
            "workspace" => Ok(Self::Workspace),
            "project" => Ok(Self::Project),
            "staging" => Ok(Self::Staging),
            _ => value
                .strip_prefix("library:")
                .ok_or_else(|| error("unknown root"))
                .and_then(Sha256Digest::from_hex)
                .map(Self::Library),
        }
    }
}

/// Paths are constructed from already separated components. Neither parsing
/// host paths nor substring replacement of arbitrary argument text is allowed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct LogicalPath {
    root: Root,
    components: Vec<String>,
}

impl LogicalPath {
    pub(crate) fn new(root: Root, components: Vec<String>) -> Result<Self, String> {
        for component in &components {
            component_ok(component)?;
        }
        Ok(Self { root, components })
    }
    pub(crate) fn to_json(&self) -> Json {
        Json::object(vec![
            ("root", Json::Str(self.root.text())),
            (
                "relative",
                Json::Arr(self.components.iter().cloned().map(Json::Str).collect()),
            ),
        ])
    }
    fn parse(value: &Json) -> Result<Self, String> {
        let fields = object(value, &["root", "relative"])?;
        let root = Root::parse(text(&fields["root"])?)?;
        let components = array(&fields["relative"])?
            .iter()
            .map(|v| text(v).map(str::to_owned))
            .collect::<Result<_, _>>()?;
        Self::new(root, components)
    }
}

/// Literal text is kept verbatim, including text resembling an absolute path.
/// Only an explicitly typed path receives root normalization.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Argument {
    Literal(String),
    Path(LogicalPath),
}

impl Argument {
    pub(crate) fn to_json(&self) -> Result<Json, String> {
        let (kind, value) = match self {
            Self::Literal(value) => {
                no_nul(value)?;
                ("literal", Json::Str(value.clone()))
            }
            Self::Path(value) => ("path", value.to_json()),
        };
        Ok(Json::object(vec![
            ("kind", Json::Str(kind.into())),
            ("value", value),
        ]))
    }
    fn parse(value: &Json) -> Result<Self, String> {
        let fields = object(value, &["kind", "value"])?;
        match text(&fields["kind"])? {
            "literal" => {
                let value = text(&fields["value"])?;
                no_nul(value)?;
                Ok(Self::Literal(value.into()))
            }
            "path" => Ok(Self::Path(LogicalPath::parse(&fields["value"])?)),
            _ => Err(error("unknown argument kind")),
        }
    }
}

/// A validated immutable descriptor. Construction and decoding share the same
/// closed schema. Consumers must still check every referenced byte stream.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Descriptor(Json);

impl Descriptor {
    /// The byte limit is caller supplied and checked before JSON parsing. It is
    /// a codec budget, not an assertion about OS per-argument or total limits.
    pub(crate) fn parse(bytes: &[u8], byte_limit: usize) -> Result<Self, String> {
        if bytes.is_empty() || bytes.len() > byte_limit {
            return Err(error("descriptor byte budget"));
        }
        depth_bound(bytes)?;
        let value = Json::parse(bytes)?;
        // Also rejects escaped duplicate aliases even if the underlying raw
        // duplicate-key scanner did not identify their decoded equivalence.
        if value.to_canonical_string().as_bytes() != bytes {
            return Err(error("descriptor is not exact canonical JSON"));
        }
        Self::new(value)
    }

    pub(crate) fn new(value: Json) -> Result<Self, String> {
        validate(&value)?;
        Ok(Self(value))
    }

    pub(crate) fn to_bytes(&self) -> Vec<u8> {
        self.0.to_canonical_string().into_bytes()
    }
    pub(crate) fn json(&self) -> &Json {
        &self.0
    }

    /// Project only after full descriptor validation. The launcher source pin
    /// belongs to the outer binding, never to its own generated preimage.
    pub(crate) fn payload(&self) -> Payload {
        let Json::Obj(mut invocation) = self.0.clone() else {
            unreachable!("validated descriptor object");
        };
        invocation.remove("source");
        Payload(Json::object(vec![
            ("spec", Json::Str(PAYLOAD_SPEC.into())),
            ("invocation", Json::Obj(invocation)),
        ]))
    }
}

/// Source-independent input to typed launcher generation. A payload authorizes
/// no execution: bind the rendered source and verify its custody separately.
/// Root bindings, predecessor view and environment must already be fixed;
/// none may depend on the resulting launcher or invocation receipt. The source
/// member is absent, consistent with the restricted canonical JSON language.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Payload(Json);

impl Payload {
    pub(crate) fn parse(bytes: &[u8], byte_limit: usize) -> Result<Self, String> {
        if bytes.is_empty() || bytes.len() > byte_limit {
            return Err(error("payload byte budget"));
        }
        // The versioned envelope adds one object around the descriptor shape.
        depth_bound_at(bytes, MAX_DEPTH + 1)?;
        let value = Json::parse(bytes)?;
        if value.to_canonical_string().as_bytes() != bytes {
            return Err(error("payload is not exact canonical JSON"));
        }
        Self::new(value)
    }

    pub(crate) fn new(value: Json) -> Result<Self, String> {
        let envelope = object(&value, &["spec", "invocation"])?;
        if text(&envelope["spec"])? != PAYLOAD_SPEC {
            return Err(error("unknown payload version"));
        }
        let invocation = &envelope["invocation"];
        let fields = object(invocation, INVOCATION_FIELDS)?;
        validate_common(fields)?;
        Ok(Self(value))
    }

    pub(crate) fn to_bytes(&self) -> Vec<u8> {
        self.0.to_canonical_string().into_bytes()
    }

    /// Attach an actual source pin only after rendering. Structural validity
    /// here is not proof that those bytes exist or that an executable used them.
    pub(crate) fn bind_source(&self, source: Json) -> Result<Descriptor, String> {
        let Json::Obj(envelope) = &self.0 else {
            unreachable!("validated payload object");
        };
        let Json::Obj(mut invocation) = envelope["invocation"].clone() else {
            unreachable!("validated invocation object");
        };
        invocation.insert("source".into(), source);
        Descriptor::new(Json::Obj(invocation))
    }
}

fn error(message: &str) -> String {
    format!("invocation descriptor: {message}")
}
fn text(value: &Json) -> Result<&str, String> {
    match value {
        Json::Str(value) => Ok(value),
        _ => Err(error("expected string")),
    }
}
fn array(value: &Json) -> Result<&[Json], String> {
    match value {
        Json::Arr(value) => Ok(value),
        _ => Err(error("expected array")),
    }
}
fn object<'a>(value: &'a Json, keys: &[&str]) -> Result<&'a BTreeMap<String, Json>, String> {
    let Json::Obj(fields) = value else {
        return Err(error("expected object"));
    };
    if fields.len() != keys.len() || keys.iter().any(|k| !fields.contains_key(*k)) {
        return Err(error("unknown or missing field"));
    }
    Ok(fields)
}
fn hash(value: &Json) -> Result<Sha256Digest, String> {
    Sha256Digest::from_hex(text(value)?)
}
fn size(value: &Json) -> Result<u64, String> {
    match value {
        Json::Int(n) if *n >= 0 => Ok(*n as u64),
        _ => Err(error("invalid byte length")),
    }
}
fn no_nul(value: &str) -> Result<(), String> {
    if value.contains('\0') {
        Err(error("NUL in argument"))
    } else {
        Ok(())
    }
}
fn component_ok(value: &str) -> Result<(), String> {
    if value.is_empty()
        || value == "."
        || value == ".."
        || value
            .chars()
            .any(|c| c.is_control() || matches!(c, '/' | '\\' | ':'))
    {
        return Err(error("unconfined path component"));
    }
    Ok(())
}
fn file_path(value: &Json) -> Result<LogicalPath, String> {
    let path = LogicalPath::parse(value)?;
    if path.components.is_empty() {
        return Err(error("file path names a root"));
    }
    Ok(path)
}
fn tool(value: &Json, allowed: &[&str]) -> Result<(), String> {
    let fields = object(value, &["kind", "path", "sha256"])?;
    if !allowed.contains(&text(&fields["kind"])?) {
        return Err(error("wrong tool role"));
    }
    let path = file_path(&fields["path"])?;
    if path.root != Root::Toolchain {
        return Err(error("tool is outside toolchain root"));
    }
    hash(&fields["sha256"])?;
    Ok(())
}
fn paths(value: &Json) -> Result<(), String> {
    let fields = object(value, &["present", "entries"])?;
    let Json::Bool(present) = fields["present"] else {
        return Err(error("path presence is not boolean"));
    };
    let entries = array(&fields["entries"])?;
    if !present && !entries.is_empty() {
        return Err(error("absent path has entries"));
    }
    for entry in entries {
        if entry != &Json::Str(String::new()) {
            LogicalPath::parse(entry)?;
        }
    }
    Ok(())
}

/// Hash only the root and complete ordered object inventory. No descriptor or
/// process containing this digest is part of the digest's own preimage.
pub(crate) fn view_digest(root: &Json, objects: &Json) -> Sha256Digest {
    let mut framed = FramedHasher::new("lexlean-invocation-view-v1");
    framed.frame("root", root.to_canonical_string().as_bytes());
    framed.frame("objects", objects.to_canonical_string().as_bytes());
    framed.finish()
}

fn validate(value: &Json) -> Result<(), String> {
    let mut keys = INVOCATION_FIELDS.to_vec();
    keys.push("source");
    let fields = object(value, &keys)?;
    validate_common(fields)?;
    let source = object(&fields["source"], &["path", "bytes", "sha256"])?;
    if file_path(&source["path"])?.root != Root::Staging || size(&source["bytes"])? == 0 {
        return Err(error("invalid launcher source"));
    }
    hash(&source["sha256"])?;
    Ok(())
}

fn validate_common(f: &BTreeMap<String, Json>) -> Result<(), String> {
    if text(&f["spec"])? != SPEC {
        return Err(error("unknown version"));
    }
    let context = object(&f["context"], &["source_id", "semantic_id", "build_id"])?;
    for value in context.values() {
        hash(value)?;
    }
    tool(&f["lake"], &["lake"])?;
    tool(&f["launcher"], &["lean"])?;
    tool(&f["target"], &["lean", "leanchecker"])?;
    for argument in array(&f["arguments"])? {
        Argument::parse(argument)?;
    }
    if LogicalPath::parse(&f["cwd"])?.root != Root::Workspace {
        return Err(error("cwd is outside workspace"));
    }
    let environment = object(&f["environment"], &["captured", "effective"])?;
    paths(&environment["captured"])?;
    paths(&environment["effective"])?;
    let view = object(&f["view"], &["root", "objects", "sha256"])?;
    let view_root = LogicalPath::parse(&view["root"])?;
    if view_root.root != Root::Staging || view_root.components.is_empty() {
        return Err(error("view is not a directory beneath staging"));
    }
    let effective = object(&environment["effective"], &["present", "entries"])?;
    let captured = object(&environment["captured"], &["present", "entries"])?;
    let entries = array(&effective["entries"])?;
    if effective["present"] != Json::Bool(true)
        || entries.first() != Some(&view["root"])
        || entries.get(1..) != Some(array(&captured["entries"])?)
    {
        return Err(error(
            "effective path is not view followed by captured path",
        ));
    }
    let objects = array(&view["objects"])?;
    let mut previous: Option<String> = None;
    let mut modules = BTreeMap::new();
    let mut primaries = BTreeSet::new();
    for item in objects {
        let o = object(item, &["module", "relative", "bytes", "sha256", "producer"])?;
        let module = text(&o["module"])?;
        if module.is_empty() || module.split('.').any(|s| component_ok(s).is_err()) {
            return Err(error("invalid module"));
        }
        let relative = text(&o["relative"])?;
        for component in relative.split('/') {
            component_ok(component)?;
        }
        if previous.as_deref().is_some_and(|old| old >= relative) {
            return Err(error("view paths duplicate or unordered"));
        }
        previous = Some(relative.into());
        let stem = module.replace('.', "/");
        if ![".olean", ".olean.private", ".olean.server", ".ir"]
            .iter()
            .any(|suffix| relative == format!("{stem}{suffix}"))
        {
            return Err(error("module and companion path disagree"));
        }
        if relative == format!("{stem}.olean") {
            primaries.insert(module);
        }
        size(&o["bytes"])?;
        hash(&o["sha256"])?;
        let producer = object(&o["producer"], &["kind", "sha256"])?;
        if !["imported", "generated"].contains(&text(&producer["kind"])?) {
            return Err(error("unknown producer kind"));
        }
        hash(&producer["sha256"])?;
        let identity = o["producer"].to_canonical_string();
        if let Some(old) = modules.insert(module, identity.clone()) {
            if old != identity {
                return Err(error("companions have different producers"));
            }
        }
    }
    if modules.keys().any(|m| !primaries.contains(m)) {
        return Err(error("companion has no primary object"));
    }
    if hash(&view["sha256"])? != view_digest(&view["root"], &view["objects"]) {
        return Err(error("view digest mismatch"));
    }
    Ok(())
}

fn depth_bound(bytes: &[u8]) -> Result<(), String> {
    depth_bound_at(bytes, MAX_DEPTH)
}

fn depth_bound_at(bytes: &[u8], maximum: usize) -> Result<(), String> {
    let (mut depth, mut quoted, mut escaped) = (0_usize, false, false);
    for &byte in bytes {
        if quoted {
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == b'"' {
                quoted = false;
            }
        } else {
            match byte {
                b'"' => quoted = true,
                b'{' | b'[' => {
                    depth += 1;
                    if depth > maximum {
                        return Err(error("JSON nesting budget"));
                    }
                }
                b'}' | b']' => {
                    depth = depth
                        .checked_sub(1)
                        .ok_or_else(|| error("unbalanced JSON"))?;
                }
                _ => {}
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn digest(text: &str) -> Json {
        Json::Str(Sha256Digest::of(text.as_bytes()).to_hex())
    }
    fn path(root: Root, relative: &[&str]) -> Json {
        LogicalPath::new(root, relative.iter().map(|s| (*s).into()).collect())
            .unwrap()
            .to_json()
    }
    fn tool_value(kind: &str) -> Json {
        Json::object(vec![
            ("kind", Json::Str(kind.into())),
            ("path", path(Root::Toolchain, &["bin", kind])),
            ("sha256", digest(kind)),
        ])
    }
    fn sample() -> Json {
        let root = path(Root::Staging, &["resolution"]);
        let producer = Json::object(vec![
            ("kind", Json::Str("imported".into())),
            ("sha256", digest("independent producer receipt")),
        ]);
        let objects = Json::Arr(vec![Json::object(vec![
            ("module", Json::Str("Rule30UOR.Base".into())),
            ("relative", Json::Str("Rule30UOR/Base.olean".into())),
            ("bytes", Json::Int(4)),
            ("sha256", digest("base")),
            ("producer", producer),
        ])]);
        let view_hash = Json::Str(view_digest(&root, &objects).to_hex());
        Json::object(vec![
            ("spec", Json::Str(SPEC.into())),
            (
                "context",
                Json::object(vec![
                    ("source_id", digest("source")),
                    ("semantic_id", digest("semantic")),
                    ("build_id", digest("build")),
                ]),
            ),
            ("lake", tool_value("lake")),
            ("launcher", tool_value("lean")),
            ("target", tool_value("leanchecker")),
            (
                "source",
                Json::object(vec![
                    ("path", path(Root::Staging, &["invoke.lean"])),
                    ("bytes", Json::Int(42)),
                    ("sha256", digest("fixed launcher source")),
                ]),
            ),
            (
                "arguments",
                Json::Arr(vec![Argument::Literal("Rule30UOR.Generated".into())
                    .to_json()
                    .unwrap()]),
            ),
            ("cwd", path(Root::Workspace, &[])),
            (
                "environment",
                Json::object(vec![
                    (
                        "captured",
                        Json::object(vec![
                            ("present", Json::Bool(false)),
                            ("entries", Json::Arr(vec![])),
                        ]),
                    ),
                    (
                        "effective",
                        Json::object(vec![
                            ("present", Json::Bool(true)),
                            ("entries", Json::Arr(vec![root.clone()])),
                        ]),
                    ),
                ]),
            ),
            (
                "view",
                Json::object(vec![
                    ("root", root),
                    ("objects", objects),
                    ("sha256", view_hash),
                ]),
            ),
        ])
    }
    fn fields(value: &mut Json) -> &mut BTreeMap<String, Json> {
        let Json::Obj(fields) = value else {
            panic!("test object");
        };
        fields
    }
    fn object_rows(value: &mut Json) -> &mut Vec<Json> {
        let view = fields(value).get_mut("view").unwrap();
        let Json::Arr(rows) = fields(view).get_mut("objects").unwrap() else {
            panic!("test array");
        };
        rows
    }
    fn reseal_view(value: &mut Json) {
        let view = fields(fields(value).get_mut("view").unwrap());
        let hash = view_digest(&view["root"], &view["objects"]);
        view.insert("sha256".into(), Json::Str(hash.to_hex()));
    }

    #[test]
    fn payload_domain_roundtrip_and_source_binding() {
        let descriptor = Descriptor::new(sample()).unwrap();
        let payload = descriptor.payload();
        let bytes = payload.to_bytes();
        assert_eq!(Payload::parse(&bytes, bytes.len()).unwrap(), payload);
        assert!(Payload::parse(&bytes, bytes.len() - 1).is_err());
        assert!(Descriptor::parse(&bytes, bytes.len()).is_err());
        let original = descriptor.to_bytes();
        assert!(Payload::parse(&original, original.len()).is_err());
        let source = fields(&mut sample())["source"].clone();
        assert_eq!(payload.bind_source(source).unwrap(), descriptor);
        let mut with_source = payload.0.clone();
        let invocation = fields(&mut with_source).get_mut("invocation").unwrap();
        fields(invocation).insert("source".into(), fields(&mut sample())["source"].clone());
        assert!(Payload::new(with_source).is_err());
        let Json::Obj(envelope) = &payload.0 else {
            unreachable!()
        };
        assert!(Descriptor::new(envelope["invocation"].clone()).is_err());
    }

    #[test]
    fn payload_preserves_every_non_source_binding() {
        let descriptor = Descriptor::new(sample()).unwrap();
        let payload = descriptor.payload();
        for key in [
            "context",
            "lake",
            "launcher",
            "target",
            "arguments",
            "cwd",
            "environment",
            "view",
        ] {
            let mut value = payload.0.clone();
            let invocation = fields(&mut value).get_mut("invocation").unwrap();
            fields(invocation).remove(key);
            assert!(
                Payload::new(value).is_err(),
                "missing {key} is not a payload"
            );
        }
        let mut value = payload.0.clone();
        fields(&mut value).insert("extra".into(), Json::Bool(true));
        assert!(Payload::new(value).is_err());
        let mut value = payload.0.clone();
        let invocation = fields(&mut value).get_mut("invocation").unwrap();
        fields(invocation).insert("source".into(), Json::Str("not a pin".into()));
        assert!(Payload::new(value).is_err());
        let mut changed = sample();
        fields(&mut changed).insert(
            "arguments".into(),
            Json::Arr(vec![Argument::Literal("Rule30UOR.Other".into())
                .to_json()
                .unwrap()]),
        );
        assert_ne!(Descriptor::new(changed).unwrap().payload(), payload);
        let mut changed = sample();
        let row = &mut object_rows(&mut changed)[0];
        fields(row).insert("sha256".into(), digest("different bytes"));
        reseal_view(&mut changed);
        assert_ne!(Descriptor::new(changed).unwrap().payload(), payload);
    }

    #[test]
    fn payload_canonical_bytes_depth_and_literal_preservation() {
        let literal = "quote\" slash\\ line\n tab\t backspace\u{8} formfeed\u{c} Ω /absolute/path";
        let mut value = sample();
        fields(&mut value).insert(
            "arguments".into(),
            Json::Arr(vec![Argument::Literal(literal.into()).to_json().unwrap()]),
        );
        let descriptor = Descriptor::new(value).unwrap();
        let payload = descriptor.payload();
        let bytes = payload.to_bytes();
        let decoded = Payload::parse(&bytes, bytes.len()).unwrap();
        let source = fields(&mut sample())["source"].clone();
        assert_eq!(decoded.bind_source(source).unwrap(), descriptor);
        let text = String::from_utf8(bytes).unwrap();
        let duplicate = format!("{{\"s\\u0070ec\":\"{PAYLOAD_SPEC}\",{}", &text[1..]);
        assert!(Payload::parse(duplicate.as_bytes(), duplicate.len()).is_err());
        let null_source = text.replacen("\"invocation\":{", "\"invocation\":{\"source\":null,", 1);
        assert!(Payload::parse(null_source.as_bytes(), null_source.len()).is_err());
        let noncanonical = format!(" {text}");
        assert!(Payload::parse(noncanonical.as_bytes(), noncanonical.len()).is_err());
        let deep = format!(
            "{}0{}",
            "[".repeat(MAX_DEPTH + 2),
            "]".repeat(MAX_DEPTH + 2)
        );
        assert!(Payload::parse(deep.as_bytes(), deep.len()).is_err());
    }

    #[test]
    fn payload_cannot_replace_outer_source_validation() {
        let payload = Descriptor::new(sample()).unwrap().payload();
        let source = fields(&mut sample())["source"].clone();
        for key in ["path", "bytes", "sha256"] {
            let mut invalid = source.clone();
            fields(&mut invalid).remove(key);
            assert!(payload.bind_source(invalid).is_err());
        }
        let mut invalid = source.clone();
        fields(&mut invalid).insert("path".into(), path(Root::Toolchain, &["bin", "lean"]));
        assert!(payload.bind_source(invalid).is_err());
        let mut invalid = source.clone();
        fields(&mut invalid).insert("bytes".into(), Json::Int(0));
        assert!(payload.bind_source(invalid).is_err());
        let first = payload.bind_source(source.clone()).unwrap();
        let mut other = source;
        fields(&mut other).insert("sha256".into(), digest("another source"));
        let second = payload.bind_source(other).unwrap();
        assert_ne!(first, second);
        assert_eq!(first.payload(), second.payload());
        // Both pins are structurally valid. Only the caller's actual-byte
        // custody check can admit one; projection equality cannot do that.
    }

    #[test]
    fn roundtrip_and_byte_and_depth_budgets() {
        let descriptor = Descriptor::new(sample()).unwrap();
        let bytes = descriptor.to_bytes();
        assert_eq!(Descriptor::parse(&bytes, bytes.len()).unwrap(), descriptor);
        assert_eq!(descriptor.json(), &sample());
        assert!(Descriptor::parse(&bytes, bytes.len() - 1).is_err());
        let nested = format!(
            "{}0{}",
            "[".repeat(MAX_DEPTH + 1),
            "]".repeat(MAX_DEPTH + 1)
        );
        assert!(Descriptor::parse(nested.as_bytes(), nested.len()).is_err());
    }

    #[test]
    fn version_unknown_missing_and_duplicate_fields_refuse() {
        let mut value = sample();
        fields(&mut value).insert("extra".into(), Json::Bool(true));
        assert!(Descriptor::new(value).is_err());
        let mut value = sample();
        fields(&mut value).remove("source");
        assert!(Descriptor::new(value).is_err());
        let mut value = sample();
        fields(&mut value).insert("spec".into(), Json::Str("unknown".into()));
        assert!(Descriptor::new(value).is_err());
        let text = sample().to_canonical_string();
        for key in ["spec", "s\\u0070ec"] {
            let duplicate = format!("{{\"{key}\":\"{SPEC}\",{}", &text[1..]);
            assert!(Descriptor::parse(duplicate.as_bytes(), duplicate.len()).is_err());
        }
        let mut value = sample();
        fields(fields(&mut value).get_mut("target").unwrap()).insert("extra".into(), Json::Int(0));
        assert!(Descriptor::new(value).is_err());
    }

    #[test]
    fn typed_paths_confine_without_changing_literal_arguments() {
        for bad in ["", ".", "..", "a/b", "a\\b", "C:", "/root", "a\0b"] {
            assert!(LogicalPath::new(Root::Staging, vec![bad.into()]).is_err());
        }
        let literal = "C:\\workspace\\literal /tmp/stage/thing : slash/colon";
        let args = vec![
            Argument::Literal(literal.into()).to_json().unwrap(),
            Argument::Path(LogicalPath::new(Root::Staging, vec!["thing".into()]).unwrap())
                .to_json()
                .unwrap(),
        ];
        let mut value = sample();
        fields(&mut value).insert("arguments".into(), Json::Arr(args.clone()));
        let decoded = Descriptor::parse(value.to_canonical_string().as_bytes(), 16_384).unwrap();
        let Json::Obj(fields) = decoded.json() else {
            panic!("descriptor");
        };
        assert_eq!(fields["arguments"], Json::Arr(args));
        assert_eq!(
            Argument::parse(&array(&fields["arguments"]).unwrap()[0]).unwrap(),
            Argument::Literal(literal.into())
        );
    }

    #[test]
    fn view_collisions_and_wrong_digests_refuse_even_when_resealed() {
        let mut value = sample();
        let row = object_rows(&mut value)[0].clone();
        object_rows(&mut value).push(row);
        reseal_view(&mut value);
        assert!(Descriptor::new(value).is_err());
        let mut value = sample();
        fields(fields(&mut value).get_mut("view").unwrap())
            .insert("sha256".into(), digest("wrong"));
        assert!(Descriptor::new(value).is_err());
        let mut value = sample();
        fields(&mut object_rows(&mut value)[0])
            .insert("sha256".into(), Json::Str("not a digest".into()));
        reseal_view(&mut value);
        assert!(Descriptor::new(value).is_err());
    }

    #[test]
    fn companion_producer_and_effective_path_mismatches_refuse() {
        let mut value = sample();
        let mut companion = object_rows(&mut value)[0].clone();
        fields(&mut companion).insert(
            "relative".into(),
            Json::Str("Rule30UOR/Base.olean.private".into()),
        );
        fields(fields(&mut companion).get_mut("producer").unwrap())
            .insert("sha256".into(), digest("other producer"));
        object_rows(&mut value).push(companion);
        reseal_view(&mut value);
        assert!(Descriptor::new(value).is_err());
        let mut value = sample();
        fields(&mut object_rows(&mut value)[0])
            .insert("relative".into(), Json::Str("Rule30UOR/Base.ir".into()));
        reseal_view(&mut value);
        assert!(Descriptor::new(value).is_err());
        let mut value = sample();
        let env = fields(fields(&mut value).get_mut("environment").unwrap());
        fields(env.get_mut("effective").unwrap()).insert("entries".into(), Json::Arr(vec![]));
        assert!(Descriptor::new(value).is_err());
    }
    #[test]
    fn merged_view_root_and_search_path_presence_are_explicit() {
        let mut value = sample();
        let staging_root = path(Root::Staging, &[]);
        fields(fields(&mut value).get_mut("view").unwrap())
            .insert("root".into(), staging_root.clone());
        let env = fields(fields(&mut value).get_mut("environment").unwrap());
        fields(env.get_mut("effective").unwrap())
            .insert("entries".into(), Json::Arr(vec![staging_root]));
        reseal_view(&mut value);
        assert!(Descriptor::new(value).is_err());

        let mut value = sample();
        let env = fields(fields(&mut value).get_mut("environment").unwrap());
        fields(env.get_mut("effective").unwrap()).insert("present".into(), Json::Bool(false));
        assert!(Descriptor::new(value).is_err());
        let mut value = sample();
        let env = fields(fields(&mut value).get_mut("environment").unwrap());
        fields(env.get_mut("effective").unwrap()).insert(
            "entries".into(),
            Json::Arr(vec![path(Root::Workspace, &[])]),
        );
        assert!(Descriptor::new(value).is_err());

        let mut absent = sample();
        let mut present_empty = absent.clone();
        let env = fields(fields(&mut present_empty).get_mut("environment").unwrap());
        fields(env.get_mut("captured").unwrap()).insert("present".into(), Json::Bool(true));
        assert_ne!(
            Descriptor::new(absent.clone()).unwrap(),
            Descriptor::new(present_empty).unwrap()
        );
        let env = fields(fields(&mut absent).get_mut("environment").unwrap());
        fields(env.get_mut("captured").unwrap()).insert("present".into(), Json::Bool(true));
        fields(env.get_mut("captured").unwrap())
            .insert("entries".into(), Json::Arr(vec![Json::Str(String::new())]));
        fields(env.get_mut("effective").unwrap()).insert(
            "entries".into(),
            Json::Arr(vec![
                path(Root::Staging, &["resolution"]),
                Json::Str(String::new()),
            ]),
        );
        let descriptor = Descriptor::new(absent).unwrap();
        assert_eq!(
            Descriptor::parse(&descriptor.to_bytes(), 16_384).unwrap(),
            descriptor
        );
    }
}
