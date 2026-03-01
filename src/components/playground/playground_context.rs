use leptos::prelude::*;
use crate::services::compiler_service;

/// Global state for the Playground drawer.
#[derive(Clone, Copy)]
pub struct PlaygroundContext {
    pub is_open: RwSignal<bool>,
    pub code: RwSignal<String>,
    pub output: RwSignal<String>,
    pub is_error: RwSignal<bool>,
    pub is_loading: RwSignal<bool>,
    pub execution_time_ms: RwSignal<u64>,
    /// "remote" or "local"
    pub compiler_mode: RwSignal<String>,
}

const DEFAULT_CODE: &str = r#"fn main() {
    println!("Hello, Rust!");
}
"#;

/// Provide the PlaygroundContext at the top of the component tree.
pub fn provide_playground() {
    let ctx = PlaygroundContext {
        is_open: RwSignal::new(false),
        code: RwSignal::new(DEFAULT_CODE.to_string()),
        output: RwSignal::new(String::new()),
        is_error: RwSignal::new(false),
        is_loading: RwSignal::new(false),
        execution_time_ms: RwSignal::new(0),
        compiler_mode: RwSignal::new("remote".to_string()),
    };
    provide_context(ctx);
}

/// Retrieve the PlaygroundContext from any descendant component.
pub fn use_playground() -> PlaygroundContext {
    expect_context::<PlaygroundContext>()
}

/// Compile and run the current playground code using the hybrid strategy.
pub async fn run_playground_code(ctx: PlaygroundContext) {
    let code = ctx.code.get_untracked();
    ctx.is_loading.set(true);
    ctx.output.set(String::new());
    ctx.is_error.set(false);

    match compiler_service::compile_and_run_hybrid(&code).await {
        Ok(result) => {
            ctx.compiler_mode.set(result.mode);
            ctx.execution_time_ms.set(result.execution_time_ms);

            if result.success {
                ctx.output.set(result.stdout);
                ctx.is_error.set(false);
            } else {
                let text = if result.stderr.is_empty() {
                    result.stdout
                } else {
                    result.stderr
                };
                ctx.output.set(text);
                ctx.is_error.set(true);
            }
        }
        Err(e) => {
            ctx.output.set(e);
            ctx.is_error.set(true);
        }
    }

    ctx.is_loading.set(false);
}

/// Run Clippy analysis on the current playground code.
pub async fn run_clippy(ctx: PlaygroundContext) {
    let code = ctx.code.get_untracked();
    ctx.is_loading.set(true);
    ctx.output.set(String::new());
    ctx.is_error.set(false);

    match compiler_service::clippy_check(&code).await {
        Ok(result) => {
            ctx.output.set(result.output);
            ctx.is_error.set(!result.success);
        }
        Err(e) => {
            ctx.output.set(e);
            ctx.is_error.set(true);
        }
    }

    ctx.is_loading.set(false);
}
