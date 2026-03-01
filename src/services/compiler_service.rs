use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompileResult {
    pub stdout: String,
    pub stderr: String,
    pub success: bool,
    pub execution_time_ms: u64,
    #[serde(default)]
    pub mode: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClippyResult {
    pub success: bool,
    pub output: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RustAvailability {
    pub available: bool,
    pub version: Option<String>,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI_INTERNALS__"])]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

/// Helper to parse a JsValue into a deserialized type.
fn parse_result<T: for<'de> Deserialize<'de>>(val: JsValue) -> Result<T, String> {
    let json_str = js_sys::JSON::stringify(&val).map_err(|e| format!("{:?}", e))?;
    let json_string: String = json_str.into();
    serde_json::from_str(&json_string).map_err(|e| e.to_string())
}

/// Compile and run a Rust code snippet via the Tauri backend (local rustc only).
pub async fn compile_and_run(code: &str) -> Result<CompileResult, String> {
    let args = js_sys::Object::new();
    js_sys::Reflect::set(&args, &"code".into(), &code.into())
        .map_err(|e| format!("{:?}", e))?;
    js_sys::Reflect::set(&args, &"timeoutSecs".into(), &10u32.into())
        .map_err(|e| format!("{:?}", e))?;

    let result = invoke("compile_and_run", args.into())
        .await
        .map_err(|e| format!("{:?}", e))?;

    parse_result(result)
}

/// Compile and run using hybrid strategy: Playground API first, local rustc fallback.
pub async fn compile_and_run_hybrid(code: &str) -> Result<CompileResult, String> {
    let args = js_sys::Object::new();
    js_sys::Reflect::set(&args, &"code".into(), &code.into())
        .map_err(|e| format!("{:?}", e))?;

    let result = invoke("compile_and_run_hybrid", args.into())
        .await
        .map_err(|e| format!("{:?}", e))?;

    parse_result(result)
}

/// Run Clippy analysis via the Playground API.
pub async fn clippy_check(code: &str) -> Result<ClippyResult, String> {
    let args = js_sys::Object::new();
    js_sys::Reflect::set(&args, &"code".into(), &code.into())
        .map_err(|e| format!("{:?}", e))?;

    let result = invoke("clippy_check", args.into())
        .await
        .map_err(|e| format!("{:?}", e))?;

    parse_result(result)
}

/// Check if rustc is available locally.
pub async fn check_rust_available() -> Result<RustAvailability, String> {
    let args = js_sys::Object::new();

    let result = invoke("check_rust_available", args.into())
        .await
        .map_err(|e| format!("{:?}", e))?;

    parse_result(result)
}
