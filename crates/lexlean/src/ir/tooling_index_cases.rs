// Shared executable TL-12 checker assertions; included only by test owners.
use serde_json::{json, Value};

fn ty(name: &str) -> Value {
    json!({"kind":"const","name":name,"arguments":[]})
}
fn sig(name: &str) -> Value {
    json!({"kind":"const","name":name})
}
fn idx(name: &str) -> Value {
    json!({"kind":"index","constructor":name})
}
fn cfg(mode: &str) -> Value {
    json!({"kind":"record_index","constructor":"Config.mk","arguments":[idx(mode),idx("Mode.inherit"),idx("Mode.inherit")]})
}
fn handle(index: Value) -> Value {
    json!({"kind":"const","name":"Handle","arguments":[index]})
}
fn var(name: &str) -> Value {
    json!({"kind":"var","name":name})
}
fn ctor(name: &str, arguments: Vec<Value>) -> Value {
    json!({"kind":"construct","constructor":name,"type_arguments":[],"arguments":arguments})
}
fn call(name: &str, arguments: Vec<Value>) -> Value {
    json!({"kind":"call","function":name,"type_arguments":[],"arguments":arguments})
}
fn fixture() -> Value {
    let pin = json!({"file":"src/lean/Init/System/IO.lean","sha256":"0".repeat(64)});
    let field = |name: &str, t: Value| json!({"name":name,"type":t});
    let type_row = |name: &str, parameters: Vec<Value>, constructors: Vec<Value>| {
        let mut row = json!({"kind":"type","name":name,"parameters":parameters,"constructors":constructors,"source":pin});
        if name == "String" {
            row["opaque"] = json!(true);
        }
        row
    };
    let call_row = |name: &str, signature: Value| json!({"kind":"call","name":name,"signature":signature,"role":"dependent-index checker fixture; no external authority execution","source":pin});
    let config = ctor(
        "Config.mk",
        vec![
            ctor("Mode.inherit", vec![]),
            ctor("Mode.inherit", vec![]),
            ctor("Mode.inherit", vec![]),
        ],
    );
    let request = ctor(
        "Request.mk",
        vec![config, json!({"kind":"string","value":"literal payload"})],
    );
    let body = json!({"kind":"let","binder":"request","type":ty("Request"),"value":request,"body":{
        "kind":"let","binder":"alias","type":ty("Request"),"value":var("request"),"body":{
            "kind":"let","binder":"child","type":handle(cfg("Mode.inherit")),"value":call("spawn",vec![var("alias")]),"body":call("wait",vec![var("child")])}}});
    json!({"spec":"lexlean/tooling-module/1","authority":{"id":"LEAN-LCNF-4-32-1","lean_version":"4.32.1","lean_githash":"f054605aea4b840552cca2e725580bffd1e1b704"},"imports":[],"declarations":[
        type_row("String",vec![],vec![]),
        type_row("Mode",vec![],vec![json!({"name":"inherit","fields":[]}),json!({"name":"piped","fields":[]})]),
        type_row("Config",vec![],vec![json!({"name":"mk","fields":[field("stdin",ty("Mode")),field("stdout",ty("Mode")),field("stderr",ty("Mode"))]})]),
        type_row("Request",vec![],vec![json!({"name":"mk","fields":[field("io",ty("Config")),field("payload",ty("String"))]})]),
        {"kind":"type","name":"Handle","parameters":[{"name":"cfg","kind":{"kind":"record_index","of":"Config"}}],"constructors":[],"opaque":true,"source":pin},
        call_row("Request.io",json!({"kind":"arrow","domain":sig("Request"),"codomain":sig("Config")})),
        call_row("Request.payload",json!({"kind":"arrow","domain":sig("Request"),"codomain":sig("String")})),
        call_row("spawn",json!({"kind":"pi","name":"args","info":"explicit","domain":sig("Request"),"codomain":{"kind":"app","function":sig("Handle"),"arguments":[{"kind":"app","function":sig("Request.io"),"arguments":[var("args")]}]}})),
        call_row("wait",json!({"kind":"pi","name":"cfg","info":"implicit","domain":sig("Config"),"codomain":{"kind":"arrow","domain":{"kind":"app","function":sig("Handle"),"arguments":[var("cfg")]},"codomain":sig("String")}})),
        {"kind":"function","name":"roundtrip","public":true,"partial":false,"type_parameters":[],"parameters":[],"result":ty("String"),"body":body}
    ]})
}
fn check(value: &Value) -> Result<ToolingModule, ToolingFailure> {
    let bytes = serde_json::to_vec(value).expect("fixture serializes");
    let canonical = CanonicalJson::parse(&bytes)
        .expect("fixture JSON")
        .to_canonical_string();
    ToolingModule::parse(
        &canonical,
        LANGUAGE_1_2,
        AuthorityPin {
            lean_version: "4.32.1",
            lean_githash: "f054605aea4b840552cca2e725580bffd1e1b704",
        },
    )
}
fn refuses(value: &Value) -> bool {
    let error = check(value).expect_err("planted semantic defect must refuse");
    assert!(
        matches!(error.code.as_str(), "LLT4014" | "LLT4015"),
        "{error:?}"
    );
    true
}
pub(super) fn run() {
    let good = fixture();
    check(&good).expect("closed record index survives spawn, immutable alias, and wait");
    let mut payload = good.clone();
    payload["declarations"][9]["parameters"] = json!([{"name":"payload","type":ty("String")}]);
    payload["declarations"][9]["body"]["value"]["arguments"][1] = var("payload");
    check(&payload)
        .expect("projection checks only the closed configuration, not unrelated payload");
    let mut different = good.clone();
    different["declarations"][9]["body"]["body"]["body"]["type"] = handle(cfg("Mode.piped"));
    assert!(
        refuses(&different),
        "a different stdio configuration is not the same handle"
    );
    let mut unrelated = good.clone();
    unrelated["declarations"][7]["signature"]["codomain"]["arguments"][0]["function"] =
        sig("Request.payload");
    assert!(
        refuses(&unrelated),
        "an unrelated projection cannot supply a Config index"
    );
    let mut unbound = good.clone();
    unbound["declarations"][8]["signature"]["codomain"]["domain"]["arguments"][0] = var("missing");
    assert!(refuses(&unbound), "an unbound index cannot be inferred");
    let mut missing = good.clone();
    missing["declarations"][4]["parameters"] = json!([]);
    assert!(
        refuses(&missing),
        "a registry cannot erase the handle index"
    );
    let mut erased = missing.clone();
    erased["declarations"][9]["body"]["body"]["body"]["type"] = ty("Handle");
    assert!(
        refuses(&erased),
        "coherent annotation erasure still refuses"
    );
    erased["declarations"].as_array_mut().unwrap().pop();
    assert!(
        refuses(&erased),
        "registry-only projection cannot be erased"
    );
    // Isolate implicit-index erasure too: no spawn projection can mask it.
    erased["declarations"].as_array_mut().unwrap().remove(7);
    assert!(
        refuses(&erased),
        "registry-only implicit index cannot be erased"
    );
    let mut shadow = good.clone();
    shadow["declarations"][8]["signature"] = json!({
        "kind":"pi","name":"cfg","info":"implicit","domain":sig("Config"),
        "codomain":{"kind":"pi","name":"h","info":"explicit",
            "domain":{"kind":"app","function":sig("Handle"),"arguments":[var("cfg")]},
            "codomain":{"kind":"pi","name":"cfg","info":"explicit","domain":sig("String"),
                "codomain":{"kind":"app","function":sig("Handle"),"arguments":[var("cfg")]}}}});
    shadow["declarations"].as_array_mut().unwrap().pop();
    assert!(
        refuses(&shadow),
        "explicit binder cannot shadow an implicit index"
    );
    shadow["declarations"][8]["signature"] = json!({
        "kind":"pi","name":"cfg","info":"explicit","domain":sig("String"),
        "codomain":{"kind":"pi","name":"cfg","info":"implicit","domain":sig("Config"),
            "codomain":{"kind":"arrow",
                "domain":{"kind":"app","function":sig("Handle"),"arguments":[var("cfg")]},
                "codomain":sig("String")}}});
    assert!(
        refuses(&shadow),
        "implicit index cannot shadow an explicit binder"
    );
    check(&nested_fixture()).expect("nested IO-like handle survives generic substitution");
    let mut unresolved = good.clone();
    unresolved["declarations"][9]["parameters"] = json!([{"name":"external","type":ty("Request")}]);
    unresolved["declarations"][9]["body"]["value"] = var("external");
    assert!(
        refuses(&unresolved),
        "a parameter is not a proven closed constructor value"
    );
    let mut cyclic = good.clone();
    cyclic["declarations"][2]["constructors"][0]["fields"][0]["type"] = ty("Config");
    assert!(
        refuses(&cyclic),
        "recursive record indices cannot overflow the checker"
    );
    let mut arity = good.clone();
    arity["declarations"][9]["body"]["body"]["body"]["type"]["arguments"][0]["arguments"] =
        json!([]);
    assert!(refuses(&arity), "every closed index field is required");
    let mut wrong_field = good.clone();
    wrong_field["declarations"][9]["body"]["body"]["body"]["type"]["arguments"][0]["arguments"]
        [0] = cfg("Mode.inherit");
    assert!(
        refuses(&wrong_field),
        "record fields have the exact declared index family"
    );
}

// A generic wrapper models IO's type-constructor layer without pretending to
// execute IO. The registered unwrap retains its exact polymorphic signature.
fn nested_fixture() -> Value {
    let mut value = fixture();
    let pin = value["declarations"][0]["source"].clone();
    let signature_app =
        |name: &str, arg: Value| json!({"kind":"app","function":sig(name),"arguments":[arg]});
    let handle_type = handle(cfg("Mode.inherit"));
    let wrapped_type = json!({"kind":"const","name":"Wrap","arguments":[handle_type.clone()]});
    let original_result = value["declarations"][7]["signature"]["codomain"].clone();
    value["declarations"][7]["signature"]["codomain"] = signature_app("Wrap", original_result);
    let old_body = value["declarations"][9]["body"]["body"]["body"].clone();
    value["declarations"][9]["body"]["body"]["body"] = json!({
        "kind":"let","binder":"wrapped","type":wrapped_type,
        "value":old_body["value"],"body":{
            "kind":"let","binder":"child","type":handle_type.clone(),
            "value":{"kind":"call","function":"unwrap","type_arguments":[handle_type],"arguments":[var("wrapped")]},
            "body":old_body["body"]}});
    let declarations = value["declarations"].as_array_mut().unwrap();
    declarations.insert(5,json!({"kind":"type","name":"Wrap","parameters":[{"name":"a","kind":{"kind":"type"}}],"constructors":[],"opaque":true,"source":pin}));
    declarations.insert(
        10,
        json!({"kind":"call","name":"unwrap","signature":{
        "kind":"pi","name":"a","info":"implicit","domain":{"kind":"type"},
        "codomain":{"kind":"arrow","domain":signature_app("Wrap",var("a")),"codomain":var("a")}},
        "role":"generic wrapper checker fixture; no external execution","source":pin}),
    );
    value
}
