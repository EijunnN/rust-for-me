use std::time::Instant;

use serde::{Deserialize, Serialize};
use tempfile::TempDir;
use tokio::process::Command;

/// The result returned to the frontend after compiling and running user code.
#[derive(Debug, Serialize)]
pub struct CompileResult {
    pub stdout: String,
    pub stderr: String,
    pub success: bool,
    pub execution_time_ms: u64,
    /// "remote" if compiled via Rust Playground API, "local" if via local rustc.
    #[serde(default)]
    pub mode: String,
}

/// Response from the Rust Playground /execute endpoint.
#[derive(Debug, Deserialize)]
struct PlaygroundExecuteResponse {
    success: bool,
    stdout: String,
    stderr: String,
}

/// Response from the Rust Playground /meta/clippy endpoint.
#[derive(Debug, Deserialize)]
struct PlaygroundClippyResponse {
    success: bool,
    stdout: String,
    stderr: String,
}

/// Result of a Clippy analysis.
#[derive(Debug, Serialize)]
pub struct ClippyResult {
    pub success: bool,
    pub output: String,
}

/// Result of checking whether rustc is available locally.
#[derive(Debug, Serialize)]
pub struct RustAvailability {
    pub available: bool,
    pub version: Option<String>,
}

/// Compile the provided Rust source code with `rustc` and, if compilation
/// succeeds, execute the resulting binary. An optional timeout (in seconds)
/// prevents runaway programs; the default is 10 seconds.
#[tauri::command]
pub async fn compile_and_run(
    code: String,
    timeout_secs: Option<u64>,
) -> Result<CompileResult, String> {
    let timeout = std::time::Duration::from_secs(timeout_secs.unwrap_or(10));

    // Create a temporary directory that will be cleaned up when dropped.
    let tmp_dir = TempDir::new().map_err(|e| format!("Failed to create temp dir: {e}"))?;
    let source_path = tmp_dir.path().join("main.rs");
    let binary_path = tmp_dir.path().join("main.exe");

    // Write the user code to a temporary file.
    tokio::fs::write(&source_path, &code)
        .await
        .map_err(|e| format!("Failed to write source file: {e}"))?;

    // ── Compilation ──────────────────────────────────────────────────────
    let compile_output = Command::new("rustc")
        .arg(&source_path)
        .arg("-o")
        .arg(&binary_path)
        .output()
        .await
        .map_err(|e| format!("Failed to start rustc: {e}"))?;

    if !compile_output.status.success() {
        return Ok(CompileResult {
            stdout: String::new(),
            stderr: String::from_utf8_lossy(&compile_output.stderr).to_string(),
            success: false,
            execution_time_ms: 0,
            mode: "local".into(),
        });
    }

    // ── Execution ────────────────────────────────────────────────────────
    let start = Instant::now();

    let run_result = tokio::time::timeout(timeout, Command::new(&binary_path).output()).await;

    let elapsed_ms = start.elapsed().as_millis() as u64;

    match run_result {
        Ok(Ok(output)) => Ok(CompileResult {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            success: output.status.success(),
            execution_time_ms: elapsed_ms,
            mode: "local".into(),
        }),
        Ok(Err(e)) => Err(format!("Failed to execute binary: {e}")),
        Err(_) => Ok(CompileResult {
            stdout: String::new(),
            stderr: format!("Execution timed out after {} seconds", timeout.as_secs()),
            success: false,
            execution_time_ms: elapsed_ms,
            mode: "local".into(),
        }),
    }
}

// ── Remote Playground API helpers ──────────────────────────────────────────

/// Try to compile and run code via the Rust Playground API.
async fn try_playground_execute(code: &str) -> Result<CompileResult, String> {
    let client = reqwest::Client::new();
    let start = Instant::now();

    let body = serde_json::json!({
        "channel": "stable",
        "mode": "debug",
        "edition": "2021",
        "crateType": "bin",
        "tests": false,
        "code": code,
        "backtrace": false
    });

    let resp = client
        .post("https://play.rust-lang.org/execute")
        .json(&body)
        .timeout(std::time::Duration::from_secs(30))
        .send()
        .await
        .map_err(|e| format!("Playground request failed: {e}"))?;

    let elapsed_ms = start.elapsed().as_millis() as u64;

    let result: PlaygroundExecuteResponse = resp
        .json()
        .await
        .map_err(|e| format!("Failed to parse playground response: {e}"))?;

    let (stdout, stderr, success) = if result.success {
        (result.stdout, result.stderr, true)
    } else {
        (result.stdout, result.stderr, false)
    };

    Ok(CompileResult {
        stdout,
        stderr,
        success,
        execution_time_ms: elapsed_ms,
        mode: "remote".into(),
    })
}

/// Try to compile and run code using the local rustc installation.
async fn try_local_compile(code: &str, timeout_secs: u64) -> Result<CompileResult, String> {
    let timeout = std::time::Duration::from_secs(timeout_secs);

    let tmp_dir = TempDir::new().map_err(|e| format!("Failed to create temp dir: {e}"))?;
    let source_path = tmp_dir.path().join("main.rs");
    let binary_path = tmp_dir.path().join("main.exe");

    tokio::fs::write(&source_path, code)
        .await
        .map_err(|e| format!("Failed to write source file: {e}"))?;

    let compile_output = Command::new("rustc")
        .arg(&source_path)
        .arg("-o")
        .arg(&binary_path)
        .output()
        .await
        .map_err(|e| format!("rustc not available: {e}"))?;

    if !compile_output.status.success() {
        return Ok(CompileResult {
            stdout: String::new(),
            stderr: String::from_utf8_lossy(&compile_output.stderr).to_string(),
            success: false,
            execution_time_ms: 0,
            mode: "local".into(),
        });
    }

    let start = Instant::now();
    let run_result = tokio::time::timeout(timeout, Command::new(&binary_path).output()).await;
    let elapsed_ms = start.elapsed().as_millis() as u64;

    match run_result {
        Ok(Ok(output)) => Ok(CompileResult {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            success: output.status.success(),
            execution_time_ms: elapsed_ms,
            mode: "local".into(),
        }),
        Ok(Err(e)) => Err(format!("Failed to execute binary: {e}")),
        Err(_) => Ok(CompileResult {
            stdout: String::new(),
            stderr: format!("Execution timed out after {} seconds", timeout_secs),
            success: false,
            execution_time_ms: elapsed_ms,
            mode: "local".into(),
        }),
    }
}

// ── New hybrid commands ────────────────────────────────────────────────────

/// Compile and run code using the Rust Playground API first, falling back to
/// local rustc if the remote call fails (e.g. no internet).
#[tauri::command]
pub async fn compile_and_run_hybrid(code: String) -> Result<CompileResult, String> {
    // Try remote first
    match try_playground_execute(&code).await {
        Ok(result) => return Ok(result),
        Err(_) => {} // Fall through to local
    }

    // Try local rustc
    match try_local_compile(&code, 10).await {
        Ok(result) => return Ok(result),
        Err(_) => {} // Both failed
    }

    // Neither worked — friendly error
    Err(
        "Could not compile your code. \
         The online Rust Playground is unreachable and rustc is not installed locally. \
         Please check your internet connection or install Rust from https://rustup.rs"
            .into(),
    )
}

/// Run Clippy analysis via the Rust Playground API.
#[tauri::command]
pub async fn clippy_check(code: String) -> Result<ClippyResult, String> {
    let client = reqwest::Client::new();

    let body = serde_json::json!({
        "channel": "stable",
        "edition": "2021",
        "crateType": "bin",
        "code": code
    });

    let resp = client
        .post("https://play.rust-lang.org/meta/clippy")
        .json(&body)
        .timeout(std::time::Duration::from_secs(30))
        .send()
        .await
        .map_err(|e| format!("Clippy request failed: {e}. Check your internet connection."))?;

    let result: PlaygroundClippyResponse = resp
        .json()
        .await
        .map_err(|e| format!("Failed to parse Clippy response: {e}"))?;

    let output = if !result.stderr.is_empty() {
        result.stderr
    } else {
        result.stdout
    };

    Ok(ClippyResult {
        success: result.success,
        output,
    })
}

/// Check whether rustc is available locally and return its version.
#[tauri::command]
pub async fn check_rust_available() -> RustAvailability {
    match Command::new("rustc").arg("--version").output().await {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
            RustAvailability {
                available: true,
                version: Some(version),
            }
        }
        _ => RustAvailability {
            available: false,
            version: None,
        },
    }
}
