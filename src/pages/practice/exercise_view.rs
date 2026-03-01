use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;

use crate::components::editor::code_editor::CodeEditor;
use crate::components::editor::output_panel::OutputPanel;
use crate::components::editor::run_button::RunButton;
use crate::i18n::{use_i18n, Translations};
use crate::i18n::locale::Locale;
use crate::models::exercise::{Exercise, ExerciseType};
use crate::models::progress::{UserProgress, ProgressStatus};

/// Renders a single exercise with a split layout:
/// instructions on the left, interactive editor on the right.
/// Supports WriteCode, FixBug, and PredictOutput exercise types.
#[component]
pub fn ExerciseView() -> impl IntoView {
    let i18n = use_i18n();
    let params = use_params_map();

    let exercise_id = move || {
        params.read().get("exercise_id").unwrap_or_default()
    };

    // Async-loaded exercise
    let exercise_data: RwSignal<Option<Exercise>> = RwSignal::new(None);
    let load_error: RwSignal<Option<String>> = RwSignal::new(None);
    let is_loading_content: RwSignal<bool> = RwSignal::new(true);

    // Editor state
    let code = RwSignal::new(String::from("fn main() {\n    // Loading...\n}\n"));
    let (output, set_output) = signal(String::new());
    let (is_error, set_is_error) = signal(false);
    let (is_loading, set_is_loading) = signal(false);
    let (hints_shown, set_hints_shown) = signal(0_usize);
    let (show_solution, set_show_solution) = signal(false);

    // PredictOutput state
    let (selected_option, set_selected_option) = signal(Option::<usize>::None);
    let (prediction_checked, set_prediction_checked) = signal(false);
    let (prediction_correct, set_prediction_correct) = signal(false);

    // Progress tracking
    let (attempts, set_attempts) = signal(0_i32);
    let expected_output_signal: RwSignal<String> = RwSignal::new(String::new());
    let exercise_completed: RwSignal<bool> = RwSignal::new(false);

    // Load exercise data
    {
        let params = params.clone();
        leptos::task::spawn_local(async move {
            let eid = params.get_untracked().get("exercise_id").unwrap_or_default();
            if eid.is_empty() {
                load_error.set(Some("No exercise ID provided".to_string()));
                is_loading_content.set(false);
                return;
            }

            match crate::services::content_service::resolve_exercise_path(&eid).await {
                Ok((module_dir, exercise_file)) => {
                    match crate::services::content_service::load_exercise(&module_dir, &exercise_file).await {
                        Ok(exercise) => {
                            // For FixBug, start with broken code; for others, use starter_code
                            let initial_code = if exercise.meta.exercise_type == ExerciseType::FixBug {
                                exercise.broken_code.clone().unwrap_or_else(|| exercise.starter_code.clone())
                            } else {
                                exercise.starter_code.clone()
                            };
                            code.set(initial_code);
                            expected_output_signal.set(exercise.expected_output.clone());
                            exercise_data.set(Some(exercise));
                        }
                        Err(e) => load_error.set(Some(e)),
                    }
                }
                Err(e) => load_error.set(Some(e)),
            }
            is_loading_content.set(false);
        });
    }

    let on_run = move |_| {
        let current_code = code.get();
        set_is_loading.set(true);
        set_output.set(String::new());
        set_is_error.set(false);
        set_attempts.update(|a| *a += 1);

        let exercise = exercise_data.get();
        let expected = expected_output_signal.get();

        leptos::task::spawn_local(async move {
            match crate::services::compiler_service::compile_and_run(&current_code).await {
                Ok(result) => {
                    if result.success {
                        set_output.set(result.stdout.clone());
                        set_is_error.set(false);

                        // Validate output against expected
                        if let Some(ref ex) = exercise {
                            let stdout_trimmed = result.stdout.trim();
                            let expected_trimmed = expected.trim();
                            let matches = stdout_trimmed == expected_trimmed;
                            let exercise_id = ex.meta.id.clone();
                            let current_attempts = attempts.get_untracked();

                            if matches && !exercise_completed.get_untracked() {
                                exercise_completed.set(true);
                                let progress = UserProgress {
                                    id: exercise_id,
                                    category: "exercise".to_string(),
                                    status: ProgressStatus::Completed,
                                    score: 100,
                                    attempts: current_attempts,
                                    completed_at: None,
                                    updated_at: String::new(),
                                };
                                let _ = crate::services::progress_service::save_progress(&progress).await;
                            } else if !matches && !exercise_completed.get_untracked() {
                                // Partial credit: compiled but wrong output
                                let score = std::cmp::max(10, 50 - (current_attempts - 1) * 10);
                                let progress = UserProgress {
                                    id: exercise_id,
                                    category: "exercise".to_string(),
                                    status: ProgressStatus::InProgress,
                                    score,
                                    attempts: current_attempts,
                                    completed_at: None,
                                    updated_at: String::new(),
                                };
                                let _ = crate::services::progress_service::save_progress(&progress).await;
                            }
                        }
                    } else {
                        set_output.set(result.stderr);
                        set_is_error.set(true);
                    }
                }
                Err(e) => {
                    set_output.set(format!("Error: {e}"));
                    set_is_error.set(true);
                }
            }
            set_is_loading.set(false);
        });
    };

    view! {
        <div class="max-w-6xl mx-auto">
            // Breadcrumb
            <nav class="flex items-center text-sm text-gray-500 dark:text-gray-400 space-x-2 mb-6">
                <A
                    href="/practice"
                    attr:class="hover:text-orange-500 transition-colors"
                >
                    {move || {
                        let tr = Translations::get(i18n.locale.get());
                        tr.nav_practice.to_string()
                    }}
                </A>
                <span class="text-gray-400">"/"</span>
                <span class="font-medium text-gray-900 dark:text-white">{exercise_id}</span>
            </nav>

            {move || {
                if is_loading_content.get() {
                    return view! {
                        <div class="text-center py-12">
                            <div class="inline-block w-8 h-8 border-4 border-orange-500 border-t-transparent rounded-full animate-spin"></div>
                            <p class="mt-4 text-gray-500 dark:text-gray-400">
                                {move || match i18n.locale.get() {
                                    Locale::Es => "Cargando ejercicio...".to_string(),
                                    Locale::En => "Loading exercise...".to_string(),
                                }}
                            </p>
                        </div>
                    }.into_any();
                }

                if let Some(err) = load_error.get() {
                    return view! {
                        <div class="text-center py-12">
                            <p class="text-red-500 text-lg mb-4">{format!("Error: {}", err)}</p>
                            <A
                                href="/practice"
                                attr:class="inline-block px-4 py-2 bg-orange-600 text-white rounded-lg hover:bg-orange-700 transition-colors"
                            >
                                {move || match i18n.locale.get() {
                                    Locale::Es => "Volver a ejercicios".to_string(),
                                    Locale::En => "Back to exercises".to_string(),
                                }}
                            </A>
                        </div>
                    }.into_any();
                }

                let Some(exercise) = exercise_data.get() else {
                    return view! {
                        <div class="text-center py-12">
                            <p class="text-gray-500">"Exercise not found"</p>
                        </div>
                    }.into_any();
                };

                let exercise_type = exercise.meta.exercise_type.clone();

                match exercise_type {
                    ExerciseType::WriteCode => render_write_code(exercise, i18n, code, output, is_error, is_loading, hints_shown, set_hints_shown, show_solution, set_show_solution, on_run, expected_output_signal),
                    ExerciseType::FixBug => render_fix_bug(exercise, i18n, code, output, is_error, is_loading, hints_shown, set_hints_shown, show_solution, set_show_solution, on_run, expected_output_signal),
                    ExerciseType::PredictOutput => render_predict_output(exercise, i18n, code, output, is_error, is_loading, hints_shown, set_hints_shown, show_solution, set_show_solution, on_run, selected_option, set_selected_option, prediction_checked, set_prediction_checked, prediction_correct, set_prediction_correct),
                }
            }}
        </div>
    }
}

/// Shared hints section used by all exercise types.
fn render_hints_section(
    i18n: crate::i18n::I18nContext,
    hints_es: Vec<String>,
    hints_en: Vec<String>,
    total_hints: usize,
    hints_shown: ReadSignal<usize>,
    _set_hints_shown: WriteSignal<usize>,
) -> impl IntoView {
    view! {
        <div class="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-sm border border-gray-200 dark:border-gray-700">
            <div class="flex items-center justify-between mb-3">
                <h3 class="font-medium text-gray-800 dark:text-gray-200">
                    {move || {
                        let tr = Translations::get(i18n.locale.get());
                        format!("{} ({}/{})", tr.common_hint, hints_shown.get(), total_hints)
                    }}
                </h3>
                <button
                    class="px-3 py-1.5 text-sm font-medium rounded-lg bg-yellow-100 dark:bg-yellow-900/30 text-yellow-700 dark:text-yellow-400 hover:bg-yellow-200 dark:hover:bg-yellow-900/50 transition-colors disabled:opacity-50"
                    disabled=move || hints_shown.get() >= total_hints
                    on:click=move |_| set_hints_shown.update(|n| *n += 1)
                >
                    {move || {
                        let tr = Translations::get(i18n.locale.get());
                        tr.exercise_show_hint.to_string()
                    }}
                </button>
            </div>
            <div class="space-y-2">
                {move || {
                    let shown = hints_shown.get();
                    let locale = i18n.locale.get();
                    let hints = match locale {
                        Locale::Es => &hints_es,
                        Locale::En => &hints_en,
                    };
                    hints.iter().take(shown).enumerate().map(|(idx, hint)| {
                        let hint_text = hint.clone();
                        view! {
                            <div class="p-3 bg-yellow-50 dark:bg-yellow-900/20 border-l-4 border-yellow-400 rounded-r-lg">
                                <p class="text-sm text-yellow-800 dark:text-yellow-300">
                                    <span class="font-bold">{format!("#{}: ", idx + 1)}</span>
                                    {hint_text}
                                </p>
                            </div>
                        }
                    }).collect::<Vec<_>>()
                }}
            </div>
        </div>
    }
}

/// Shared solution toggle section used by all exercise types.
fn render_solution_section(
    i18n: crate::i18n::I18nContext,
    solution_code: String,
    show_solution: ReadSignal<bool>,
    set_show_solution: WriteSignal<bool>,
) -> impl IntoView {
    view! {
        <div class="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-sm border border-gray-200 dark:border-gray-700">
            <button
                class="flex items-center w-full text-left font-medium text-gray-800 dark:text-gray-200 hover:text-orange-500 transition-colors"
                on:click=move |_| set_show_solution.update(|v| *v = !*v)
            >
                <svg
                    class=move || format!("w-4 h-4 mr-2 transition-transform {}", if show_solution.get() { "rotate-90" } else { "" })
                    fill="none" stroke="currentColor" viewBox="0 0 24 24"
                >
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
                </svg>
                {move || {
                    let tr = Translations::get(i18n.locale.get());
                    tr.common_solution.to_string()
                }}
            </button>
            {move || if show_solution.get() {
                view! {
                    <div class="mt-3 p-4 bg-gray-100 dark:bg-gray-900 rounded-lg">
                        <pre class="text-sm font-mono text-green-600 dark:text-green-400 overflow-x-auto whitespace-pre-wrap">
                            {solution_code.clone()}
                        </pre>
                    </div>
                }.into_any()
            } else {
                view! { <span></span> }.into_any()
            }}
        </div>
    }
}

/// Render a WriteCode exercise (original behavior).
#[allow(clippy::too_many_arguments)]
fn render_write_code(
    exercise: Exercise,
    i18n: crate::i18n::I18nContext,
    code: RwSignal<String>,
    output: ReadSignal<String>,
    is_error: ReadSignal<bool>,
    is_loading: ReadSignal<bool>,
    hints_shown: ReadSignal<usize>,
    set_hints_shown: WriteSignal<usize>,
    show_solution: ReadSignal<bool>,
    set_show_solution: WriteSignal<bool>,
    on_run: impl Fn(leptos::ev::MouseEvent) + 'static,
    expected_output_signal: RwSignal<String>,
) -> AnyView {
    let title_es = exercise.title.es.clone();
    let title_en = exercise.title.en.clone();
    let desc_es = exercise.description.es.clone();
    let desc_en = exercise.description.en.clone();
    let expected_output = exercise.expected_output.clone();
    let hints_es = exercise.hints.es.clone();
    let hints_en = exercise.hints.en.clone();
    let total_hints = hints_es.len();
    let solution_code = exercise.solution.clone();
    let starter_code = exercise.starter_code.clone();

    view! {
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
            // Left side: Instructions
            <div class="space-y-4">
                <div class="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-sm border border-gray-200 dark:border-gray-700">
                    // Type badge
                    <div class="flex items-center gap-2 mb-3">
                        <span class="inline-flex items-center gap-1 px-2 py-0.5 rounded-full text-xs font-medium bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-300">
                            <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4" />
                            </svg>
                            {move || {
                                let tr = Translations::get(i18n.locale.get());
                                tr.exercise_type_write_code.to_string()
                            }}
                        </span>
                    </div>
                    <h1 class="text-xl font-bold text-gray-900 dark:text-white mb-4">
                        {move || match i18n.locale.get() {
                            Locale::Es => title_es.clone(),
                            Locale::En => title_en.clone(),
                        }}
                    </h1>
                    <div class="prose dark:prose-invert max-w-none text-gray-700 dark:text-gray-300">
                        <p class="whitespace-pre-line">
                            {move || match i18n.locale.get() {
                                Locale::Es => desc_es.clone(),
                                Locale::En => desc_en.clone(),
                            }}
                        </p>
                    </div>
                    <div class="mt-4 p-3 bg-blue-50 dark:bg-blue-900/20 rounded-lg border border-blue-200 dark:border-blue-800">
                        <p class="text-sm text-blue-700 dark:text-blue-300 font-medium">
                            {move || {
                                let tr = Translations::get(i18n.locale.get());
                                tr.exercise_expected_output.to_string()
                            }}
                        </p>
                        <pre class="mt-1 text-sm font-mono text-blue-800 dark:text-blue-200 whitespace-pre-wrap">
                            {expected_output.clone()}
                        </pre>
                    </div>
                </div>
                {render_hints_section(i18n, hints_es, hints_en, total_hints, hints_shown, set_hints_shown)}
                {render_solution_section(i18n, solution_code, show_solution, set_show_solution)}
            </div>
            // Right side: Editor + Output
            <div class="space-y-4">
                <CodeEditor code=code />
                <div class="flex items-center gap-3">
                    <RunButton on_click=on_run is_loading=is_loading />
                    <button
                        class="px-4 py-2 rounded-lg text-sm font-medium text-gray-700 dark:text-gray-300 bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors"
                        on:click={
                            let starter = starter_code.clone();
                            move |_| code.set(starter.clone())
                        }
                    >
                        {move || {
                            let tr = Translations::get(i18n.locale.get());
                            tr.common_reset.to_string()
                        }}
                    </button>
                </div>
                <OutputPanel output=output is_error=is_error is_loading=is_loading expected_output=expected_output_signal />
            </div>
        </div>
    }.into_any()
}

/// Render a FixBug exercise with compiler error display and broken code in editor.
#[allow(clippy::too_many_arguments)]
fn render_fix_bug(
    exercise: Exercise,
    i18n: crate::i18n::I18nContext,
    code: RwSignal<String>,
    output: ReadSignal<String>,
    is_error: ReadSignal<bool>,
    is_loading: ReadSignal<bool>,
    hints_shown: ReadSignal<usize>,
    set_hints_shown: WriteSignal<usize>,
    show_solution: ReadSignal<bool>,
    set_show_solution: WriteSignal<bool>,
    on_run: impl Fn(leptos::ev::MouseEvent) + 'static,
    expected_output_signal: RwSignal<String>,
) -> AnyView {
    let title_es = exercise.title.es.clone();
    let title_en = exercise.title.en.clone();
    let desc_es = exercise.description.es.clone();
    let desc_en = exercise.description.en.clone();
    let expected_output = exercise.expected_output.clone();
    let hints_es = exercise.hints.es.clone();
    let hints_en = exercise.hints.en.clone();
    let total_hints = hints_es.len();
    let solution_code = exercise.solution.clone();
    let broken_code = exercise.broken_code.clone().unwrap_or_else(|| exercise.starter_code.clone());

    let compiler_error_es = exercise.compiler_error.as_ref().map(|e| e.es.clone()).unwrap_or_default();
    let compiler_error_en = exercise.compiler_error.as_ref().map(|e| e.en.clone()).unwrap_or_default();

    view! {
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
            // Left side: Instructions with compiler error
            <div class="space-y-4">
                <div class="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-sm border border-gray-200 dark:border-gray-700">
                    // FixBug type badge
                    <div class="flex items-center gap-2 mb-3">
                        <span class="inline-flex items-center gap-1 px-2 py-0.5 rounded-full text-xs font-medium bg-red-100 dark:bg-red-900/30 text-red-700 dark:text-red-300">
                            <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.066 2.573c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.573 1.066c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.066-2.573c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                            </svg>
                            {move || {
                                let tr = Translations::get(i18n.locale.get());
                                tr.exercise_type_fix_bug.to_string()
                            }}
                        </span>
                    </div>

                    <h1 class="text-xl font-bold text-gray-900 dark:text-white mb-4">
                        {move || match i18n.locale.get() {
                            Locale::Es => title_es.clone(),
                            Locale::En => title_en.clone(),
                        }}
                    </h1>

                    <div class="prose dark:prose-invert max-w-none text-gray-700 dark:text-gray-300">
                        <p class="whitespace-pre-line">
                            {move || match i18n.locale.get() {
                                Locale::Es => desc_es.clone(),
                                Locale::En => desc_en.clone(),
                            }}
                        </p>
                    </div>
                </div>

                // Compiler error display (prominent)
                <div class="bg-red-50 dark:bg-red-900/20 rounded-xl p-6 shadow-sm border-2 border-red-300 dark:border-red-700">
                    <div class="flex items-center gap-2 mb-3">
                        <svg class="w-5 h-5 text-red-600 dark:text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L4.082 16.5c-.77.833.192 2.5 1.732 2.5z" />
                        </svg>
                        <h3 class="font-bold text-red-700 dark:text-red-300">
                            {move || {
                                let tr = Translations::get(i18n.locale.get());
                                tr.exercise_compiler_error.to_string()
                            }}
                        </h3>
                    </div>
                    <pre class="text-sm font-mono text-red-800 dark:text-red-200 whitespace-pre-wrap bg-red-100 dark:bg-red-900/30 p-3 rounded-lg">
                        {move || match i18n.locale.get() {
                            Locale::Es => compiler_error_es.clone(),
                            Locale::En => compiler_error_en.clone(),
                        }}
                    </pre>
                </div>

                // Instructions callout
                <div class="p-4 bg-orange-50 dark:bg-orange-900/20 rounded-lg border border-orange-200 dark:border-orange-800">
                    <p class="text-sm text-orange-800 dark:text-orange-300 font-medium">
                        {move || {
                            let tr = Translations::get(i18n.locale.get());
                            tr.exercise_fix_instructions.to_string()
                        }}
                    </p>
                </div>

                // Expected output
                <div class="p-3 bg-blue-50 dark:bg-blue-900/20 rounded-lg border border-blue-200 dark:border-blue-800">
                    <p class="text-sm text-blue-700 dark:text-blue-300 font-medium">
                        {move || {
                            let tr = Translations::get(i18n.locale.get());
                            tr.exercise_expected_output.to_string()
                        }}
                    </p>
                    <pre class="mt-1 text-sm font-mono text-blue-800 dark:text-blue-200 whitespace-pre-wrap">
                        {expected_output.clone()}
                    </pre>
                </div>

                {render_hints_section(i18n, hints_es, hints_en, total_hints, hints_shown, set_hints_shown)}
                {render_solution_section(i18n, solution_code, show_solution, set_show_solution)}
            </div>

            // Right side: Editor (pre-filled with broken code) + Output
            <div class="space-y-4">
                <CodeEditor code=code />
                <div class="flex items-center gap-3">
                    <RunButton on_click=on_run is_loading=is_loading />
                    <button
                        class="px-4 py-2 rounded-lg text-sm font-medium text-gray-700 dark:text-gray-300 bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors"
                        on:click={
                            let broken = broken_code.clone();
                            move |_| code.set(broken.clone())
                        }
                    >
                        {move || {
                            let tr = Translations::get(i18n.locale.get());
                            tr.common_reset.to_string()
                        }}
                    </button>
                </div>
                <OutputPanel output=output is_error=is_error is_loading=is_loading expected_output=expected_output_signal />
            </div>
        </div>
    }.into_any()
}

/// Render a PredictOutput exercise with read-only code and radio buttons.
#[allow(clippy::too_many_arguments)]
fn render_predict_output(
    exercise: Exercise,
    i18n: crate::i18n::I18nContext,
    _code: RwSignal<String>,
    output: ReadSignal<String>,
    is_error: ReadSignal<bool>,
    is_loading: ReadSignal<bool>,
    hints_shown: ReadSignal<usize>,
    set_hints_shown: WriteSignal<usize>,
    show_solution: ReadSignal<bool>,
    set_show_solution: WriteSignal<bool>,
    on_run: impl Fn(leptos::ev::MouseEvent) + 'static,
    selected_option: ReadSignal<Option<usize>>,
    set_selected_option: WriteSignal<Option<usize>>,
    prediction_checked: ReadSignal<bool>,
    set_prediction_checked: WriteSignal<bool>,
    prediction_correct: ReadSignal<bool>,
    set_prediction_correct: WriteSignal<bool>,
) -> AnyView {
    let title_es = exercise.title.es.clone();
    let title_en = exercise.title.en.clone();
    let desc_es = exercise.description.es.clone();
    let desc_en = exercise.description.en.clone();
    let hints_es = exercise.hints.es.clone();
    let hints_en = exercise.hints.en.clone();
    let total_hints = hints_es.len();
    let solution_code = exercise.solution.clone();
    let display_code = exercise.starter_code.clone();
    let options = exercise.options.clone().unwrap_or_default();
    let options_for_check = options.clone();

    view! {
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
            // Left side: Instructions + options
            <div class="space-y-4">
                <div class="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-sm border border-gray-200 dark:border-gray-700">
                    // PredictOutput type badge
                    <div class="flex items-center gap-2 mb-3">
                        <span class="inline-flex items-center gap-1 px-2 py-0.5 rounded-full text-xs font-medium bg-purple-100 dark:bg-purple-900/30 text-purple-700 dark:text-purple-300">
                            <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8.228 9c.549-1.165 2.03-2 3.772-2 2.21 0 4 1.343 4 3 0 1.4-1.278 2.575-3.006 2.907-.542.104-.994.54-.994 1.093m0 3h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                            </svg>
                            {move || {
                                let tr = Translations::get(i18n.locale.get());
                                tr.exercise_type_predict_output.to_string()
                            }}
                        </span>
                    </div>

                    <h1 class="text-xl font-bold text-gray-900 dark:text-white mb-4">
                        {move || match i18n.locale.get() {
                            Locale::Es => title_es.clone(),
                            Locale::En => title_en.clone(),
                        }}
                    </h1>

                    <div class="prose dark:prose-invert max-w-none text-gray-700 dark:text-gray-300">
                        <p class="whitespace-pre-line">
                            {move || match i18n.locale.get() {
                                Locale::Es => desc_es.clone(),
                                Locale::En => desc_en.clone(),
                            }}
                        </p>
                    </div>
                </div>

                // Instructions callout
                <div class="p-4 bg-purple-50 dark:bg-purple-900/20 rounded-lg border border-purple-200 dark:border-purple-800">
                    <p class="text-sm text-purple-800 dark:text-purple-300 font-medium">
                        {move || {
                            let tr = Translations::get(i18n.locale.get());
                            tr.exercise_predict_instructions.to_string()
                        }}
                    </p>
                </div>

                // Code display (read-only)
                <div class="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-sm border border-gray-200 dark:border-gray-700">
                    <h3 class="font-medium text-gray-800 dark:text-gray-200 mb-3">
                        {move || match i18n.locale.get() {
                            Locale::Es => "Codigo:".to_string(),
                            Locale::En => "Code:".to_string(),
                        }}
                    </h3>
                    <pre class="p-4 bg-gray-900 rounded-lg text-sm font-mono text-green-400 overflow-x-auto whitespace-pre-wrap">
                        {display_code.clone()}
                    </pre>
                </div>

                // Multiple choice options
                <div class="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-sm border border-gray-200 dark:border-gray-700">
                    <h3 class="font-medium text-gray-800 dark:text-gray-200 mb-4">
                        {move || {
                            let tr = Translations::get(i18n.locale.get());
                            tr.exercise_predict_select.to_string()
                        }}
                    </h3>
                    <div class="space-y-2">
                        {options.into_iter().enumerate().map(|(idx, opt)| {
                            let opt_es = opt.es.clone();
                            let opt_en = opt.en.clone();
                            let is_correct = opt.correct;
                            view! {
                                <label
                                    class=move || {
                                        let base = "flex items-center gap-3 p-3 rounded-lg border-2 cursor-pointer transition-all";
                                        let selected = selected_option.get() == Some(idx);
                                        let checked = prediction_checked.get();

                                        if checked && selected && is_correct {
                                            format!("{base} border-green-500 bg-green-50 dark:bg-green-900/20")
                                        } else if checked && selected && !is_correct {
                                            format!("{base} border-red-500 bg-red-50 dark:bg-red-900/20")
                                        } else if checked && is_correct {
                                            format!("{base} border-green-300 bg-green-50/50 dark:bg-green-900/10")
                                        } else if selected {
                                            format!("{base} border-purple-500 bg-purple-50 dark:bg-purple-900/20")
                                        } else {
                                            format!("{base} border-gray-200 dark:border-gray-600 hover:border-purple-300 dark:hover:border-purple-500")
                                        }
                                    }
                                >
                                    <input
                                        type="radio"
                                        name="predict-option"
                                        class="text-purple-600"
                                        checked=move || selected_option.get() == Some(idx)
                                        disabled=move || prediction_checked.get()
                                        on:change=move |_| set_selected_option.set(Some(idx))
                                    />
                                    <span class="text-sm text-gray-700 dark:text-gray-300 font-mono">
                                        {move || match i18n.locale.get() {
                                            Locale::Es => opt_es.clone(),
                                            Locale::En => opt_en.clone(),
                                        }}
                                    </span>
                                </label>
                            }
                        }).collect::<Vec<_>>()}
                    </div>

                    // Check button
                    <div class="mt-4">
                        <button
                            class="px-4 py-2 rounded-lg text-sm font-medium bg-purple-600 text-white hover:bg-purple-700 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
                            disabled=move || selected_option.get().is_none() || prediction_checked.get()
                            on:click={
                                let opts = options_for_check.clone();
                                let exercise_id = exercise.meta.id.clone();
                                move |_| {
                                    if let Some(idx) = selected_option.get() {
                                        let correct = opts.get(idx).map(|o| o.correct).unwrap_or(false);
                                        set_prediction_correct.set(correct);
                                        set_prediction_checked.set(true);

                                        // Save progress for PredictOutput exercises
                                        let eid = exercise_id.clone();
                                        leptos::task::spawn_local(async move {
                                            let progress = UserProgress {
                                                id: eid,
                                                category: "exercise".to_string(),
                                                status: if correct { ProgressStatus::Completed } else { ProgressStatus::InProgress },
                                                score: if correct { 100 } else { 0 },
                                                attempts: 1,
                                                completed_at: None,
                                                updated_at: String::new(),
                                            };
                                            let _ = crate::services::progress_service::save_progress(&progress).await;
                                        });
                                    }
                                }
                            }
                        >
                            {move || {
                                let tr = Translations::get(i18n.locale.get());
                                tr.exercise_predict_check.to_string()
                            }}
                        </button>
                    </div>

                    // Result feedback
                    {move || {
                        if !prediction_checked.get() {
                            return view! { <span></span> }.into_any();
                        }
                        if prediction_correct.get() {
                            view! {
                                <div class="mt-4 p-3 bg-green-50 dark:bg-green-900/20 rounded-lg border border-green-300 dark:border-green-700">
                                    <p class="text-sm text-green-700 dark:text-green-300 font-medium flex items-center gap-2">
                                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                                        </svg>
                                        {move || {
                                            let tr = Translations::get(i18n.locale.get());
                                            tr.exercise_predict_correct.to_string()
                                        }}
                                    </p>
                                </div>
                            }.into_any()
                        } else {
                            view! {
                                <div class="mt-4 p-3 bg-red-50 dark:bg-red-900/20 rounded-lg border border-red-300 dark:border-red-700">
                                    <p class="text-sm text-red-700 dark:text-red-300 font-medium flex items-center gap-2">
                                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                                        </svg>
                                        {move || {
                                            let tr = Translations::get(i18n.locale.get());
                                            tr.exercise_predict_incorrect.to_string()
                                        }}
                                    </p>
                                </div>
                            }.into_any()
                        }
                    }}
                </div>

                {render_hints_section(i18n, hints_es, hints_en, total_hints, hints_shown, set_hints_shown)}
                {render_solution_section(i18n, solution_code, show_solution, set_show_solution)}
            </div>

            // Right side: Run to verify + Output
            <div class="space-y-4">
                <div class="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-sm border border-gray-200 dark:border-gray-700">
                    <h3 class="font-medium text-gray-800 dark:text-gray-200 mb-3">
                        {move || match i18n.locale.get() {
                            Locale::Es => "Verifica ejecutando el codigo".to_string(),
                            Locale::En => "Verify by running the code".to_string(),
                        }}
                    </h3>
                    <p class="text-sm text-gray-500 dark:text-gray-400 mb-4">
                        {move || match i18n.locale.get() {
                            Locale::Es => "Despues de hacer tu prediccion, ejecuta el codigo para verificar.".to_string(),
                            Locale::En => "After making your prediction, run the code to verify.".to_string(),
                        }}
                    </p>
                    <RunButton on_click=on_run is_loading=is_loading />
                </div>
                <OutputPanel output=output is_error=is_error is_loading=is_loading />
            </div>
        </div>
    }.into_any()
}
