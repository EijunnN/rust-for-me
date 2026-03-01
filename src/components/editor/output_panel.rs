use leptos::prelude::*;
use crate::i18n::{use_i18n, Translations};
use super::error_explainer::ErrorExplainer;
use super::success_panel::SuccessPanel;

/// Extract Rust compiler error codes (e.g., "E0382") from compiler output.
fn extract_error_codes(text: &str) -> Vec<String> {
    let mut codes = Vec::new();
    let mut i = 0;
    // Look for pattern: error[E####]
    while i < text.len() {
        if i + 7 < text.len() && &text[i..i + 6] == "error[" {
            // Find the closing bracket
            if let Some(end) = text[i + 6..].find(']') {
                let code = &text[i + 6..i + 6 + end];
                // Validate it looks like an error code (E followed by digits)
                if code.starts_with('E') && code.len() >= 4 && code[1..].chars().all(|c| c.is_ascii_digit()) {
                    let code_str = code.to_string();
                    if !codes.contains(&code_str) {
                        codes.push(code_str);
                    }
                }
                i = i + 6 + end + 1;
            } else {
                i += 1;
            }
        } else {
            i += 1;
        }
    }
    codes
}

/// Represents a parsed segment of compiler output with semantic coloring.
#[derive(Clone)]
enum OutputSegment {
    /// An error header line like "error[E0382]: borrow of moved value"
    ErrorHeader(String),
    /// A location line like " --> src/main.rs:3:5"
    Location(String),
    /// A suggestion/help line from the compiler
    Help(String),
    /// A warning line
    Warning(String),
    /// A note line
    Note(String),
    /// Regular output text
    Normal(String),
}

/// Parse compiler output into color-coded segments.
fn parse_output_segments(text: &str) -> Vec<OutputSegment> {
    let mut segments = Vec::new();
    for line in text.lines() {
        let trimmed = line.trim_start();
        let segment = if trimmed.starts_with("error[") || trimmed.starts_with("error:") {
            OutputSegment::ErrorHeader(line.to_string())
        } else if trimmed.starts_with("-->") || trimmed.starts_with(":::") {
            OutputSegment::Location(line.to_string())
        } else if trimmed.starts_with("help:") || trimmed.starts_with("= help:") {
            OutputSegment::Help(line.to_string())
        } else if trimmed.starts_with("warning[") || trimmed.starts_with("warning:") {
            OutputSegment::Warning(line.to_string())
        } else if trimmed.starts_with("note:") || trimmed.starts_with("= note:") {
            OutputSegment::Note(line.to_string())
        } else {
            OutputSegment::Normal(line.to_string())
        };
        segments.push(segment);
    }
    segments
}

/// Panel that displays compilation/execution output with educational annotations.
/// Parses compiler errors, color-codes them, and provides beginner-friendly explanations.
#[component]
pub fn OutputPanel(
    #[prop(into)] output: Signal<String>,
    #[prop(into)] is_error: Signal<bool>,
    #[prop(into)] is_loading: Signal<bool>,
    /// Optional expected output for exercise mode validation
    #[prop(optional, into)]
    expected_output: Option<RwSignal<String>>,
) -> impl IntoView {
    let i18n = use_i18n();

    view! {
        <div class="rounded-lg border border-gray-700 bg-gray-950 overflow-hidden">
            // Header bar
            <div class="flex items-center justify-between px-4 py-2 bg-gray-800 border-b border-gray-700">
                <span class="text-xs font-medium text-gray-400 uppercase tracking-wide">
                    {move || {
                        let locale = i18n.locale.get();
                        let tr = Translations::get(locale);
                        tr.output_title.to_string()
                    }}
                </span>
                {move || if is_loading.get() {
                    let locale = i18n.locale.get();
                    let tr = Translations::get(locale);
                    view! {
                        <span class="flex items-center text-xs text-yellow-400">
                            <span class="animate-spin inline-block w-3 h-3 border-2 border-yellow-400 border-t-transparent rounded-full mr-2"></span>
                            {tr.output_compiling.to_string()}
                        </span>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}
            </div>

            // Content area
            <div class="p-4 max-h-64 overflow-y-auto">
                {move || if is_loading.get() {
                    let locale = i18n.locale.get();
                    let tr = Translations::get(locale);
                    view! {
                        <div class="flex items-center justify-center py-4">
                            <div class="animate-spin rounded-full h-6 w-6 border-b-2 border-orange-500"></div>
                            <span class="ml-3 text-gray-400 text-sm">{tr.output_compiling_running.to_string()}</span>
                        </div>
                    }.into_any()
                } else {
                    let text = output.get();
                    let error = is_error.get();

                    if text.is_empty() {
                        let locale = i18n.locale.get();
                        let tr = Translations::get(locale);
                        view! {
                            <p class="text-gray-500 text-sm italic">{tr.output_click_run.to_string()}</p>
                        }.into_any()
                    } else if error {
                        // Parse and render error output with color coding
                        let error_codes = extract_error_codes(&text);
                        let segments = parse_output_segments(&text);

                        let segment_views: Vec<_> = segments.into_iter().map(|seg| {
                            match seg {
                                OutputSegment::ErrorHeader(line) => {
                                    view! {
                                        <div class="font-mono text-sm text-red-400 font-bold">{line}</div>
                                    }.into_any()
                                }
                                OutputSegment::Location(line) => {
                                    view! {
                                        <div class="font-mono text-sm text-blue-400">{line}</div>
                                    }.into_any()
                                }
                                OutputSegment::Help(line) => {
                                    view! {
                                        <div class="font-mono text-sm text-green-400">{line}</div>
                                    }.into_any()
                                }
                                OutputSegment::Warning(line) => {
                                    view! {
                                        <div class="font-mono text-sm text-yellow-400">{line}</div>
                                    }.into_any()
                                }
                                OutputSegment::Note(line) => {
                                    view! {
                                        <div class="font-mono text-sm text-cyan-400">{line}</div>
                                    }.into_any()
                                }
                                OutputSegment::Normal(line) => {
                                    view! {
                                        <div class="font-mono text-sm text-gray-400">{line}</div>
                                    }.into_any()
                                }
                            }
                        }).collect();

                        let explainer_views: Vec<_> = error_codes.into_iter().map(|code| {
                            view! {
                                <ErrorExplainer error_code=code />
                            }.into_any()
                        }).collect();

                        view! {
                            <div>
                                <div class="space-y-0">
                                    {segment_views}
                                </div>
                                <div class="mt-3">
                                    {explainer_views}
                                </div>
                            </div>
                        }.into_any()
                    } else {
                        // Success - show the success panel
                        let expected = expected_output.map(|s| s.get()).filter(|s| !s.is_empty());
                        match expected {
                            Some(exp) => view! {
                                <SuccessPanel output=text.clone() expected_output=exp />
                            }.into_any(),
                            None => view! {
                                <SuccessPanel output=text.clone() />
                            }.into_any(),
                        }
                    }
                }}
            </div>
        </div>
    }
}
