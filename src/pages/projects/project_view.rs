use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;

use crate::components::editor::code_editor::CodeEditor;
use crate::components::editor::output_panel::OutputPanel;
use crate::components::editor::run_button::RunButton;
use crate::components::common::completion_check::CompletionCheck;
use crate::components::lesson::theory_block::TheoryBlock;
use crate::components::lesson::code_example::CodeExample;
use crate::components::lesson::interactive_quiz::InteractiveQuiz;
use crate::components::lesson::interactive_quiz::QuizOptionData;
use crate::i18n::{use_i18n, Translations};
use crate::i18n::locale::Locale;
use crate::models::lesson::ContentBlock;
use crate::models::project::{Project, ProjectStep};
use crate::models::progress::{UserProgress, ProgressStatus};

/// Renders a single guided project with step-by-step instructions,
/// a stepper sidebar, and an interactive code editor.
/// Project data is loaded from TOML files.
#[component]
pub fn ProjectView() -> impl IntoView {
    let i18n = use_i18n();
    let params = use_params_map();

    let project_id = move || {
        params.read().get("project_id").unwrap_or_default()
    };

    // Async-loaded project metadata and steps
    let project_data: RwSignal<Option<Project>> = RwSignal::new(None);
    let loaded_steps: RwSignal<Vec<ProjectStep>> = RwSignal::new(Vec::new());
    let load_error: RwSignal<Option<String>> = RwSignal::new(None);
    let is_loading_content: RwSignal<bool> = RwSignal::new(true);

    // Step tracking
    let (current_step, set_current_step) = signal(0_usize);

    // Editor state
    let code = RwSignal::new(String::new());
    let (output, set_output) = signal(String::new());
    let (is_error, set_is_error) = signal(false);
    let (is_loading, set_is_loading) = signal(false);

    // Step completion tracking
    let completed_steps: RwSignal<Vec<bool>> = RwSignal::new(Vec::new());
    // Expected outputs per step for validation
    let step_expected_outputs: RwSignal<Vec<Option<String>>> = RwSignal::new(Vec::new());
    // Current step's expected output for OutputPanel
    let current_expected: RwSignal<String> = RwSignal::new(String::new());

    // Load project data
    {
        let params = params.clone();
        leptos::task::spawn_local(async move {
            let pid = params.get_untracked().get("project_id").unwrap_or_default();
            if pid.is_empty() {
                load_error.set(Some("No project ID provided".to_string()));
                is_loading_content.set(false);
                return;
            }

            // First load all projects to find the one matching the route
            match crate::services::content_service::load_all_projects().await {
                Ok(projects) => {
                    if let Some(project) = projects.into_iter().find(|p| p.id == pid) {
                        let step_count = project.steps.len();
                        project_data.set(Some(project));

                        // Discover the project directory name by scanning once
                        let dirs = crate::services::content_service::list_content_dir_raw("projects").await.unwrap_or_default();

                        // Find the matching project dir by trying to load step01 from each
                        let mut project_dir = None;
                        for dir_name in &dirs {
                            if crate::services::content_service::load_project_step(dir_name, "step01").await.is_ok() {
                                project_dir = Some(dir_name.clone());
                                break;
                            }
                        }

                        // Load all step content files
                        let mut full_steps = Vec::new();
                        if let Some(dir_name) = project_dir {
                            for step_idx in 0..step_count {
                                let step_file = format!("step{:02}", step_idx + 1);
                                if let Ok(step) = crate::services::content_service::load_project_step(&dir_name, &step_file).await {
                                    full_steps.push(step);
                                }
                            }
                        }

                        // Set initial code from first step and collect expected outputs
                        if let Some(first) = full_steps.first() {
                            if let Some(sc) = &first.starter_code {
                                code.set(sc.clone());
                            }
                            if let Some(exp) = &first.expected_output {
                                current_expected.set(exp.clone());
                            }
                        }

                        let expected_per_step: Vec<Option<String>> = full_steps.iter()
                            .map(|s| s.expected_output.clone())
                            .collect();
                        step_expected_outputs.set(expected_per_step);
                        completed_steps.set(vec![false; full_steps.len()]);
                        loaded_steps.set(full_steps);
                    } else {
                        load_error.set(Some(format!("Project not found: {pid}")));
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

        let step_idx = current_step.get();
        let expected = step_expected_outputs.get().get(step_idx).cloned().flatten();
        let pid = project_id();

        leptos::task::spawn_local(async move {
            match crate::services::compiler_service::compile_and_run(&current_code).await {
                Ok(result) => {
                    if result.success {
                        set_output.set(result.stdout.clone());
                        set_is_error.set(false);

                        // Validate output against expected for step completion
                        if let Some(ref exp) = expected {
                            let stdout_trimmed = result.stdout.trim();
                            let expected_trimmed = exp.trim();
                            if stdout_trimmed == expected_trimmed {
                                // Mark step as completed
                                completed_steps.update(|steps| {
                                    if let Some(s) = steps.get_mut(step_idx) {
                                        *s = true;
                                    }
                                });

                                // Save step progress
                                let step_id = format!("{}_step{:02}", pid, step_idx + 1);
                                let progress = UserProgress {
                                    id: step_id,
                                    category: "project_step".to_string(),
                                    status: ProgressStatus::Completed,
                                    score: 100,
                                    attempts: 1,
                                    completed_at: None,
                                    updated_at: String::new(),
                                };
                                let _ = crate::services::progress_service::save_progress(&progress).await;

                                // Check if all steps are completed
                                let all_done = completed_steps.get_untracked().iter().all(|&c| c);
                                if all_done {
                                    let project_progress = UserProgress {
                                        id: pid.clone(),
                                        category: "project".to_string(),
                                        status: ProgressStatus::Completed,
                                        score: 100,
                                        attempts: 1,
                                        completed_at: None,
                                        updated_at: String::new(),
                                    };
                                    let _ = crate::services::progress_service::save_progress(&project_progress).await;
                                }
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
                    href="/projects"
                    attr:class="hover:text-orange-500 transition-colors"
                >
                    {move || {
                        let tr = Translations::get(i18n.locale.get());
                        tr.nav_projects.to_string()
                    }}
                </A>
                <span class="text-gray-400">"/"</span>
                <span class="font-medium text-gray-900 dark:text-white">{project_id}</span>
            </nav>

            {move || {
                if is_loading_content.get() {
                    return view! {
                        <div class="text-center py-12">
                            <div class="inline-block w-8 h-8 border-4 border-orange-500 border-t-transparent rounded-full animate-spin"></div>
                            <p class="mt-4 text-gray-500 dark:text-gray-400">
                                {move || match i18n.locale.get() {
                                    Locale::Es => "Cargando proyecto...".to_string(),
                                    Locale::En => "Loading project...".to_string(),
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
                                href="/projects"
                                attr:class="inline-block px-4 py-2 bg-orange-600 text-white rounded-lg hover:bg-orange-700 transition-colors"
                            >
                                {move || match i18n.locale.get() {
                                    Locale::Es => "Volver a proyectos".to_string(),
                                    Locale::En => "Back to projects".to_string(),
                                }}
                            </A>
                        </div>
                    }.into_any();
                }

                let Some(project) = project_data.get() else {
                    return view! {
                        <div class="text-center py-12">
                            <p class="text-gray-500">"Project not found"</p>
                        </div>
                    }.into_any();
                };

                let project_title_es = project.title.es.clone();
                let project_title_en = project.title.en.clone();
                let steps = loaded_steps.get();
                let total_steps = steps.len();

                // Collect data for display
                let step_titles_es: Vec<String> = steps.iter().map(|s| s.title.es.clone()).collect();
                let step_titles_en: Vec<String> = steps.iter().map(|s| s.title.en.clone()).collect();
                let step_starter_codes: Vec<String> = steps.iter().map(|s| s.starter_code.clone().unwrap_or_default()).collect();

                // Content blocks per step for rendering
                let step_blocks: Vec<Vec<ContentBlock>> = steps.iter().map(|s| s.content.clone()).collect();

                let codes_for_sidebar = step_starter_codes.clone();
                let codes_for_prev = step_starter_codes.clone();
                let codes_for_next = step_starter_codes.clone();

                view! {
                    // Project title
                    <h1 class="text-2xl font-bold text-gray-900 dark:text-white mb-6">
                        {move || match i18n.locale.get() {
                            Locale::Es => project_title_es.clone(),
                            Locale::En => project_title_en.clone(),
                        }}
                    </h1>

                    <div class="grid grid-cols-1 lg:grid-cols-4 gap-6">
                        // Left sidebar: Step stepper
                        <div class="lg:col-span-1">
                            <div class="bg-white dark:bg-gray-800 rounded-xl p-4 shadow-sm border border-gray-200 dark:border-gray-700 sticky top-6">
                                <h3 class="font-semibold text-gray-800 dark:text-gray-200 mb-4 text-sm uppercase tracking-wide">
                                    {move || match i18n.locale.get() {
                                        Locale::Es => "Pasos".to_string(),
                                        Locale::En => "Steps".to_string(),
                                    }}
                                </h3>
                                <div class="space-y-1">
                                    {(0..total_steps).map(|step_idx| {
                                        let title_es = step_titles_es.get(step_idx).cloned().unwrap_or_default();
                                        let title_en = step_titles_en.get(step_idx).cloned().unwrap_or_default();
                                        let codes = codes_for_sidebar.clone();

                                        view! {
                                            <button
                                                class=move || {
                                                    let base = "flex items-center w-full text-left px-3 py-2.5 rounded-lg text-sm transition-all";
                                                    if current_step.get() == step_idx {
                                                        format!("{base} bg-orange-100 dark:bg-orange-900/30 text-orange-700 dark:text-orange-400 font-medium")
                                                    } else {
                                                        format!("{base} text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700")
                                                    }
                                                }
                                                on:click={
                                                    let codes = codes.clone();
                                                    move |_| {
                                                        set_current_step.set(step_idx);
                                                        if let Some(c) = codes.get(step_idx) {
                                                            if !c.is_empty() {
                                                                code.set(c.clone());
                                                            }
                                                        }
                                                        let exp = step_expected_outputs.get().get(step_idx).cloned().flatten().unwrap_or_default();
                                                        current_expected.set(exp);
                                                        set_output.set(String::new());
                                                    }
                                                }
                                            >
                                                <div class="mr-3 flex-shrink-0">
                                                    {move || {
                                                        let done = completed_steps.get().get(step_idx).copied().unwrap_or(false);
                                                        view! { <CompletionCheck completed=done /> }
                                                    }}
                                                </div>
                                                <div>
                                                    <span class="text-xs text-gray-400 block">
                                                        {format!("{}/{}", step_idx + 1, total_steps)}
                                                    </span>
                                                    <span>
                                                        {move || match i18n.locale.get() {
                                                            Locale::Es => title_es.clone(),
                                                            Locale::En => title_en.clone(),
                                                        }}
                                                    </span>
                                                </div>
                                            </button>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                            </div>
                        </div>

                        // Main content area
                        <div class="lg:col-span-3 space-y-4">
                            // Step instructions with content blocks
                            <div class="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-sm border border-gray-200 dark:border-gray-700">
                                <h2 class="text-lg font-semibold text-gray-800 dark:text-white mb-3">
                                    {move || {
                                        let idx = current_step.get();
                                        let locale = i18n.locale.get();
                                        let title = match locale {
                                            Locale::Es => step_titles_es.get(idx).cloned().unwrap_or_default(),
                                            Locale::En => step_titles_en.get(idx).cloned().unwrap_or_default(),
                                        };
                                        format!("{}/{}: {}", idx + 1, total_steps, title)
                                    }}
                                </h2>

                                // Render content blocks for current step
                                {move || {
                                    let idx = current_step.get();
                                    let blocks = step_blocks.get(idx).cloned().unwrap_or_default();
                                    blocks.into_iter().map(|block| {
                                        render_project_block(block)
                                    }).collect::<Vec<_>>()
                                }}
                            </div>

                            // Code editor
                            <CodeEditor code=code />

                            // Action buttons
                            {
                                let on_prev = move |_| {
                                    set_current_step.update(|s| {
                                        if *s > 0 {
                                            *s -= 1;
                                        }
                                    });
                                    let idx = current_step.get();
                                    if let Some(c) = codes_for_prev.get(idx) {
                                        if !c.is_empty() {
                                            code.set(c.clone());
                                        }
                                    }
                                    let exp = step_expected_outputs.get().get(idx).cloned().flatten().unwrap_or_default();
                                    current_expected.set(exp);
                                    set_output.set(String::new());
                                };
                                #[allow(unused_variables)]
                                let on_next = move |_: leptos::ev::MouseEvent| {
                                    set_current_step.update(|s| {
                                        if *s < total_steps - 1 {
                                            *s += 1;
                                        }
                                    });
                                    let idx = current_step.get();
                                    if let Some(c) = codes_for_next.get(idx) {
                                        if !c.is_empty() {
                                            code.set(c.clone());
                                        }
                                    }
                                    let exp = step_expected_outputs.get().get(idx).cloned().flatten().unwrap_or_default();
                                    current_expected.set(exp);
                                    set_output.set(String::new());
                                };
                                view! {
                                    <div class="flex items-center gap-3">
                                        <RunButton on_click=on_run is_loading=is_loading />
                                        <div class="flex-1"></div>
                                        <button
                                            class="px-4 py-2 rounded-lg text-sm font-medium text-gray-700 dark:text-gray-300 bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors disabled:opacity-50"
                                            disabled=move || current_step.get() == 0
                                            on:click=on_prev
                                        >
                                            {move || {
                                                let tr = Translations::get(i18n.locale.get());
                                                tr.common_previous.to_string()
                                            }}
                                        </button>
                                        <button
                                            class="px-4 py-2 rounded-lg text-sm font-medium text-white bg-orange-500 hover:bg-orange-600 transition-colors disabled:opacity-50"
                                            disabled=move || current_step.get() >= total_steps - 1
                                            on:click=on_next
                                        >
                                            {move || {
                                                let tr = Translations::get(i18n.locale.get());
                                                tr.common_next.to_string()
                                            }}
                                        </button>
                                    </div>
                                }
                            }

                            // Output panel
                            <OutputPanel output=output is_error=is_error is_loading=is_loading expected_output=current_expected />
                        </div>
                    </div>
                }.into_any()
            }}
        </div>
    }
}

/// Render a content block within a project step.
fn render_project_block(block: ContentBlock) -> impl IntoView {
    let i18n = crate::i18n::use_i18n();
    match block {
        ContentBlock::Text { es, en } => {
            view! {
                <TheoryBlock es=es en=en />
            }.into_any()
        }
        ContentBlock::Code { language: _, runnable, code } => {
            view! {
                <CodeExample code=code runnable=runnable />
            }.into_any()
        }
        ContentBlock::Callout { variant, es, en } => {
            let (bg, border, text) = match variant.as_str() {
                "warning" => (
                    "bg-yellow-50 dark:bg-yellow-900/20",
                    "border-yellow-400",
                    "text-yellow-800 dark:text-yellow-300",
                ),
                "tip" => (
                    "bg-green-50 dark:bg-green-900/20",
                    "border-green-400",
                    "text-green-800 dark:text-green-300",
                ),
                _ => (
                    "bg-blue-50 dark:bg-blue-900/20",
                    "border-blue-400",
                    "text-blue-800 dark:text-blue-300",
                ),
            };
            view! {
                <div class=format!("my-4 p-4 {bg} border-l-4 {border} rounded-r-lg")>
                    <p class=format!("text-sm {text}")>
                        {move || match i18n.locale.get() {
                            Locale::Es => es.clone(),
                            Locale::En => en.clone(),
                        }}
                    </p>
                </div>
            }.into_any()
        }
        ContentBlock::Quiz { es, en, options } => {
            let quiz_options: Vec<QuizOptionData> = options.into_iter().map(|o| {
                QuizOptionData {
                    es: o.es,
                    en: o.en,
                    correct: o.correct,
                }
            }).collect();
            view! {
                <InteractiveQuiz
                    question_es=es
                    question_en=en
                    options=quiz_options
                />
            }.into_any()
        }
    }
}
