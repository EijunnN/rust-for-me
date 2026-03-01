use leptos::prelude::*;
use crate::i18n::{use_i18n, Translations};
use crate::components::editor::output_panel::OutputPanel;
use super::playground_context::{use_playground, run_playground_code, run_clippy};
use super::codemirror_editor::CodeMirrorEditor;

/// Slide-in drawer containing the global Rust Playground.
#[component]
pub fn PlaygroundDrawer() -> impl IntoView {
    let pg = use_playground();
    let i18n = use_i18n();

    // ── Actions ────────────────────────────────────────────────────────────
    let on_run = move |_| {
        let ctx = pg.clone();
        leptos::task::spawn_local(async move {
            run_playground_code(ctx).await;
        });
    };

    let on_clippy = move |_| {
        let ctx = pg.clone();
        leptos::task::spawn_local(async move {
            run_clippy(ctx).await;
        });
    };

    let on_clear = move |_| {
        pg.output.set(String::new());
        pg.is_error.set(false);
        pg.execution_time_ms.set(0);
    };

    let on_close = move |_| {
        pg.is_open.set(false);
    };

    // Close when clicking the backdrop
    let on_backdrop = move |_| {
        pg.is_open.set(false);
    };

    view! {
        // Backdrop overlay
        <div
            class=move || {
                if pg.is_open.get() {
                    "fixed inset-0 bg-black/30 z-40 transition-opacity duration-300 opacity-100"
                } else {
                    "fixed inset-0 bg-black/30 z-40 transition-opacity duration-300 opacity-0 pointer-events-none"
                }
            }
            on:click=on_backdrop
        />

        // Drawer panel
        <div
            class=move || {
                let base = "fixed top-0 right-0 h-full w-[600px] max-w-[90vw] z-50 flex flex-col bg-gray-900 shadow-2xl transition-transform duration-300 ease-in-out";
                if pg.is_open.get() {
                    format!("{base} translate-x-0")
                } else {
                    format!("{base} translate-x-full")
                }
            }
        >
            // ── Header ─────────────────────────────────────────────────────
            <div class="flex items-center justify-between px-4 py-3 border-b border-gray-700 bg-gray-800">
                <div class="flex items-center space-x-3">
                    <span class="text-lg font-semibold text-white">
                        {move || {
                            let locale = i18n.locale.get();
                            let tr = Translations::get(locale);
                            tr.playground_title.to_string()
                        }}
                    </span>
                    // Compiler mode badge
                    <span class=move || {
                        let mode = pg.compiler_mode.get();
                        if mode == "local" {
                            "text-xs px-2 py-0.5 rounded-full bg-green-900/50 text-green-400 font-medium"
                        } else {
                            "text-xs px-2 py-0.5 rounded-full bg-blue-900/50 text-blue-400 font-medium"
                        }
                    }>
                        {move || {
                            let locale = i18n.locale.get();
                            let tr = Translations::get(locale);
                            let mode = pg.compiler_mode.get();
                            if mode == "local" {
                                tr.playground_mode_local.to_string()
                            } else {
                                tr.playground_mode_remote.to_string()
                            }
                        }}
                    </span>
                </div>
                <button
                    class="p-1.5 rounded-lg text-gray-400 hover:text-white hover:bg-gray-700 transition-colors"
                    on:click=on_close
                >
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                    </svg>
                </button>
            </div>

            // ── Editor area ────────────────────────────────────────────────
            <div class="flex-1 min-h-0 relative">
                <CodeMirrorEditor code=pg.code is_visible=pg.is_open />
            </div>

            // ── Action bar ─────────────────────────────────────────────────
            <div class="flex items-center justify-between px-4 py-2 border-t border-b border-gray-700 bg-gray-800">
                <div class="flex items-center space-x-2">
                    // Run button
                    <button
                        class="flex items-center px-3 py-1.5 rounded-lg font-medium text-white bg-orange-600 hover:bg-orange-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors text-sm"
                        on:click=on_run
                        disabled=move || pg.is_loading.get()
                    >
                        {move || if pg.is_loading.get() {
                            view! {
                                <span class="flex items-center">
                                    <span class="animate-spin inline-block w-3.5 h-3.5 border-2 border-white border-t-transparent rounded-full mr-1.5"></span>
                                    "..."
                                </span>
                            }.into_any()
                        } else {
                            view! {
                                <span class="flex items-center">
                                    <span class="mr-1.5">{"\u{25B6}"}</span>
                                    "Run"
                                </span>
                            }.into_any()
                        }}
                    </button>

                    // Clippy button
                    <button
                        class="flex items-center px-3 py-1.5 rounded-lg font-medium text-yellow-300 bg-yellow-900/30 hover:bg-yellow-900/50 disabled:opacity-50 disabled:cursor-not-allowed transition-colors text-sm"
                        on:click=on_clippy
                        disabled=move || pg.is_loading.get()
                    >
                        {move || {
                            let locale = i18n.locale.get();
                            let tr = Translations::get(locale);
                            tr.playground_clippy.to_string()
                        }}
                    </button>

                    // Clear button
                    <button
                        class="flex items-center px-3 py-1.5 rounded-lg font-medium text-gray-300 bg-gray-700 hover:bg-gray-600 transition-colors text-sm"
                        on:click=on_clear
                    >
                        {move || {
                            let locale = i18n.locale.get();
                            let tr = Translations::get(locale);
                            tr.playground_clear.to_string()
                        }}
                    </button>
                </div>

                // Execution time
                <span class="text-xs text-gray-500">
                    {move || {
                        let ms = pg.execution_time_ms.get();
                        if ms > 0 {
                            format!("{}ms", ms)
                        } else {
                            String::new()
                        }
                    }}
                </span>
            </div>

            // ── Output panel ───────────────────────────────────────────────
            <div class="h-48 overflow-y-auto">
                <OutputPanel
                    output=pg.output
                    is_error=pg.is_error
                    is_loading=pg.is_loading
                />
            </div>
        </div>
    }
}
