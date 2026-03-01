use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;

use crate::components::lesson::theory_block::TheoryBlock;
use crate::components::lesson::code_example::CodeExample;
use crate::components::lesson::interactive_quiz::InteractiveQuiz;
use crate::components::lesson::interactive_quiz::QuizOptionData;
use crate::components::lesson::lesson_nav::LessonNav;
use crate::i18n::{use_i18n, Translations};
use crate::i18n::locale::Locale;
use crate::models::lesson::{ContentBlock, Lesson};
use crate::models::progress::{UserProgress, ProgressStatus};

/// Renders a single lesson identified by route params `module_id` and `lesson_id`.
/// Content blocks (text, code, callout, quiz) are loaded from TOML files.
#[component]
pub fn LessonView() -> impl IntoView {
    let i18n = use_i18n();
    let params = use_params_map();

    let module_id = move || {
        params.read().get("module_id").unwrap_or_default()
    };
    let lesson_id = move || {
        params.read().get("lesson_id").unwrap_or_default()
    };

    // Async-loaded lesson
    let lesson_data: RwSignal<Option<Lesson>> = RwSignal::new(None);
    let load_error: RwSignal<Option<String>> = RwSignal::new(None);
    let is_loading: RwSignal<bool> = RwSignal::new(true);

    // Progress tracking
    let lesson_completed: RwSignal<bool> = RwSignal::new(false);

    // Navigation: prev/next lesson URLs
    let prev_lesson_url: RwSignal<Option<String>> = RwSignal::new(None);
    let next_lesson_url: RwSignal<Option<String>> = RwSignal::new(None);

    // Reactive effect: re-loads whenever the URL params change
    Effect::new(move || {
        let lid = params.read().get("lesson_id").unwrap_or_default();
        let mid = params.read().get("module_id").unwrap_or_default();

        // Reset state for the new lesson
        lesson_data.set(None);
        load_error.set(None);
        is_loading.set(true);
        lesson_completed.set(false);
        prev_lesson_url.set(None);
        next_lesson_url.set(None);

        leptos::task::spawn_local(async move {
            if lid.is_empty() {
                load_error.set(Some("No lesson ID provided".to_string()));
                is_loading.set(false);
                return;
            }

            // Load the lesson content
            match crate::services::content_service::resolve_lesson_path(&lid).await {
                Ok((module_dir, lesson_file)) => {
                    match crate::services::content_service::load_lesson(&module_dir, &lesson_file).await {
                        Ok(lesson) => lesson_data.set(Some(lesson)),
                        Err(e) => load_error.set(Some(e)),
                    }
                }
                Err(e) => load_error.set(Some(e)),
            }

            // Load the module to figure out prev/next lessons
            if let Ok(modules) = crate::services::content_service::load_all_modules().await {
                if let Some(module) = modules.iter().find(|m| m.id == mid) {
                    let lessons = &module.lessons;
                    if let Some(current_idx) = lessons.iter().position(|l| l.id == lid) {
                        if current_idx > 0 {
                            let prev = &lessons[current_idx - 1];
                            prev_lesson_url.set(Some(format!("/theory/{}/{}", mid, prev.id)));
                        }
                        if current_idx + 1 < lessons.len() {
                            let next = &lessons[current_idx + 1];
                            next_lesson_url.set(Some(format!("/theory/{}/{}", mid, next.id)));
                        }
                    }
                }
            }

            // Check if lesson is already completed
            if let Ok(Some(progress)) = crate::services::progress_service::get_progress(&lid, "lesson").await {
                if progress.status == ProgressStatus::Completed {
                    lesson_completed.set(true);
                }
            }

            is_loading.set(false);
        });
    });

    view! {
        <div class="max-w-3xl mx-auto space-y-6">
            // Breadcrumb navigation
            <nav class="flex items-center text-sm text-gray-500 dark:text-gray-400 space-x-2">
                <A
                    href="/theory"
                    attr:class="hover:text-orange-500 transition-colors"
                >
                    {move || {
                        let tr = Translations::get(i18n.locale.get());
                        tr.nav_theory.to_string()
                    }}
                </A>
                <span class="text-gray-400">"/"</span>
                <A
                    href=move || format!("/theory/{}", module_id())
                    attr:class="hover:text-orange-500 transition-colors"
                >
                    {module_id}
                </A>
                <span class="text-gray-400">"/"</span>
                <span class="font-medium text-gray-900 dark:text-white">{lesson_id}</span>
            </nav>

            // Content area
            {move || {
                if is_loading.get() {
                    return view! {
                        <div class="text-center py-12">
                            <div class="inline-block w-8 h-8 border-4 border-orange-500 border-t-transparent rounded-full animate-spin"></div>
                            <p class="mt-4 text-gray-500 dark:text-gray-400">
                                {move || match i18n.locale.get() {
                                    Locale::Es => "Cargando leccion...".to_string(),
                                    Locale::En => "Loading lesson...".to_string(),
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
                                href=move || format!("/theory/{}", module_id())
                                attr:class="inline-block px-4 py-2 bg-orange-600 text-white rounded-lg hover:bg-orange-700 transition-colors"
                            >
                                {move || match i18n.locale.get() {
                                    Locale::Es => "Volver al modulo".to_string(),
                                    Locale::En => "Back to module".to_string(),
                                }}
                            </A>
                        </div>
                    }.into_any();
                }

                let Some(lesson) = lesson_data.get() else {
                    return view! {
                        <div class="text-center py-12">
                            <p class="text-gray-500 dark:text-gray-400">
                                {move || match i18n.locale.get() {
                                    Locale::Es => "Leccion no encontrada".to_string(),
                                    Locale::En => "Lesson not found".to_string(),
                                }}
                            </p>
                        </div>
                    }.into_any();
                };

                let title_es = lesson.title.es.clone();
                let title_en = lesson.title.en.clone();
                let blocks = lesson.blocks.clone();

                view! {
                    <div class="space-y-6">
                        // Lesson title
                        <div class="border-b border-gray-200 dark:border-gray-700 pb-4">
                            <h1 class="text-2xl font-bold text-gray-900 dark:text-white">
                                {move || match i18n.locale.get() {
                                    Locale::Es => title_es.clone(),
                                    Locale::En => title_en.clone(),
                                }}
                            </h1>
                            <p class="text-sm text-gray-500 dark:text-gray-400 mt-1">
                                {move || {
                                    let mid = module_id();
                                    match i18n.locale.get() {
                                        Locale::Es => format!("Modulo: {}", mid),
                                        Locale::En => format!("Module: {}", mid),
                                    }
                                }}
                            </p>
                        </div>

                        // Render content blocks from TOML
                        {blocks.into_iter().map(|block| {
                            render_content_block(block)
                        }).collect::<Vec<_>>()}

                        // Mark as completed button
                        <div class="mt-8 flex justify-center">
                            {move || {
                                let lid = lesson_id();
                                if lesson_completed.get() {
                                    view! {
                                        <div class="flex items-center gap-2 px-6 py-3 rounded-lg bg-green-50 dark:bg-green-900/20 border border-green-300 dark:border-green-700">
                                            <svg class="w-5 h-5 text-green-600 dark:text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                                            </svg>
                                            <span class="text-sm font-medium text-green-700 dark:text-green-300">
                                                {move || match i18n.locale.get() {
                                                    Locale::Es => "Leccion completada".to_string(),
                                                    Locale::En => "Lesson completed".to_string(),
                                                }}
                                            </span>
                                        </div>
                                    }.into_any()
                                } else {
                                    view! {
                                        <button
                                            class="px-6 py-3 rounded-lg text-sm font-medium text-white bg-orange-600 hover:bg-orange-700 transition-colors shadow-md"
                                            on:click=move |_| {
                                                let lesson_id_val = lid.clone();
                                                leptos::task::spawn_local(async move {
                                                    let progress = UserProgress {
                                                        id: lesson_id_val,
                                                        category: "lesson".to_string(),
                                                        status: ProgressStatus::Completed,
                                                        score: 100,
                                                        attempts: 1,
                                                        completed_at: None,
                                                        updated_at: String::new(),
                                                    };
                                                    if crate::services::progress_service::save_progress(&progress).await.is_ok() {
                                                        lesson_completed.set(true);
                                                    }
                                                });
                                            }
                                        >
                                            {move || match i18n.locale.get() {
                                                Locale::Es => "Marcar como completada".to_string(),
                                                Locale::En => "Mark as completed".to_string(),
                                            }}
                                        </button>
                                    }.into_any()
                                }
                            }}
                        </div>

                        // Lesson navigation
                        {move || {
                            let prev = prev_lesson_url.get()
                                .unwrap_or_else(|| format!("/theory/{}", module_id()));
                            let next = next_lesson_url.get();
                            match next {
                                Some(next_url) => view! {
                                    <LessonNav
                                        prev_url=prev
                                        next_url=next_url
                                    />
                                }.into_any(),
                                None => view! {
                                    <LessonNav
                                        prev_url=prev
                                    />
                                }.into_any(),
                            }
                        }}
                    </div>
                }.into_any()
            }}
        </div>
    }
}

/// Render a single ContentBlock as a Leptos view.
fn render_content_block(block: ContentBlock) -> impl IntoView {
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
            let i18n = crate::i18n::use_i18n();
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
