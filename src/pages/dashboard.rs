use leptos::prelude::*;
use leptos_router::components::A;
use crate::components::common::progress_ring::ProgressRing;
use crate::i18n::{use_i18n, Translations};
use crate::i18n::locale::Locale;
use crate::models::module::Module;
use crate::models::progress::ProgressStatus;
use crate::services::learning_path::RecommendedNext;

/// The main dashboard / home page.
/// Displays a welcome message, learning progress summary, recommended next item,
/// and module grid. Module data is loaded from TOML files. Stats come from progress database.
#[component]
pub fn Dashboard() -> impl IntoView {
    let i18n = use_i18n();

    // Async-loaded modules
    let modules_data: RwSignal<Option<Vec<Module>>> = RwSignal::new(None);

    // Real progress stats
    let total_progress: RwSignal<f64> = RwSignal::new(0.0);
    let lessons_completed: RwSignal<u32> = RwSignal::new(0);
    let exercises_done: RwSignal<u32> = RwSignal::new(0);
    let projects_started: RwSignal<u32> = RwSignal::new(0);
    // Per-lesson completion data for module progress calculation
    let completed_lesson_ids: RwSignal<Vec<String>> = RwSignal::new(Vec::new());
    // Recommended next item from learning path
    let recommended: RwSignal<Option<RecommendedNext>> = RwSignal::new(None);
    // Track whether async loading has finished
    let data_loaded: RwSignal<bool> = RwSignal::new(false);

    leptos::task::spawn_local(async move {
        let modules = crate::services::content_service::load_all_modules().await.ok();
        if let Some(ref m) = modules {
            modules_data.set(Some(m.clone()));
        }

        // Load all progress data
        let all_progress = crate::services::progress_service::get_all_progress(None)
            .await
            .unwrap_or_default();

        let mut lesson_count = 0u32;
        let mut exercise_count = 0u32;
        let mut project_count = 0u32;
        let mut lesson_ids = Vec::new();

        for p in &all_progress {
            if p.status != ProgressStatus::Completed {
                continue;
            }
            match p.category.as_str() {
                "lesson" => {
                    lesson_count += 1;
                    lesson_ids.push(p.id.clone());
                }
                "exercise" => exercise_count += 1,
                "project" | "project_step" => {
                    if p.category == "project" {
                        project_count += 1;
                    }
                }
                _ => {}
            }
        }

        lessons_completed.set(lesson_count);
        exercises_done.set(exercise_count);
        projects_started.set(project_count);
        completed_lesson_ids.set(lesson_ids);

        // Load exercises and projects for learning path calculation
        let exercises = crate::services::content_service::load_all_exercises()
            .await
            .unwrap_or_default();
        let projects = crate::services::content_service::load_all_projects()
            .await
            .unwrap_or_default();

        // Calculate total progress using the learning path service
        if let Some(ref m) = modules {
            let pct = crate::services::learning_path::get_completion_percentage(
                m, &exercises, &projects, &all_progress,
            );
            total_progress.set(pct);

            // Get recommended next item
            let next = crate::services::learning_path::get_next_recommended(
                m, &exercises, &projects, &all_progress,
            );
            recommended.set(next);
        }
        data_loaded.set(true);
    });

    view! {
        <div class="max-w-7xl mx-auto space-y-8">
            // Welcome header with gradient background
            <div class="bg-gradient-to-r from-orange-500 to-orange-700 rounded-2xl p-8 text-white shadow-lg">
                <div class="flex items-center justify-between">
                    <div>
                        <h1 class="text-3xl font-bold mb-2">
                            {move || {
                                let locale = i18n.locale.get();
                                let tr = Translations::get(locale);
                                format!("{} \u{1F980}", tr.dashboard_title)
                            }}
                        </h1>
                        <p class="text-orange-100 text-lg">
                            {move || {
                                let locale = i18n.locale.get();
                                let tr = Translations::get(locale);
                                tr.dashboard_welcome.to_string()
                            }}
                        </p>
                    </div>
                    <div class="hidden md:block">
                        {move || {
                            let pct = total_progress.get();
                            view! { <ProgressRing percent=pct size=120 /> }
                        }}
                    </div>
                </div>
            </div>

            // Recommended Next section (dynamic, from learning path)
            <div class="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-sm border border-gray-200 dark:border-gray-700">
                <h2 class="text-xl font-semibold text-gray-800 dark:text-white mb-4">
                    {move || {
                        let tr = Translations::get(i18n.locale.get());
                        tr.path_recommended_next.to_string()
                    }}
                </h2>
                {move || {
                    if !data_loaded.get() {
                        // Still loading
                        return view! {
                            <div class="flex items-center justify-center p-4">
                                <div class="inline-block w-5 h-5 border-3 border-orange-500 border-t-transparent rounded-full animate-spin"></div>
                                <span class="ml-2 text-gray-500 dark:text-gray-400 text-sm">
                                    {move || match i18n.locale.get() {
                                        Locale::Es => "Cargando...".to_string(),
                                        Locale::En => "Loading...".to_string(),
                                    }}
                                </span>
                            </div>
                        }.into_any();
                    }
                    if let Some(next) = recommended.get() {
                        let title_es = next.title_es.clone();
                        let title_en = next.title_en.clone();
                        let category = next.category.clone();
                        let href = next.href.clone();
                        view! {
                            <div class="flex items-center justify-between p-4 bg-orange-50 dark:bg-orange-900/20 rounded-lg border border-orange-200 dark:border-orange-800">
                                <div class="flex items-center gap-3">
                                    // Category icon
                                    {match category.as_str() {
                                        "lesson" => view! {
                                            <div class="flex-shrink-0 p-2 bg-green-100 dark:bg-green-900/30 rounded-lg">
                                                <svg class="w-5 h-5 text-green-600 dark:text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253" />
                                                </svg>
                                            </div>
                                        }.into_any(),
                                        "exercise" => view! {
                                            <div class="flex-shrink-0 p-2 bg-blue-100 dark:bg-blue-900/30 rounded-lg">
                                                <svg class="w-5 h-5 text-blue-600 dark:text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4" />
                                                </svg>
                                            </div>
                                        }.into_any(),
                                        _ => view! {
                                            <div class="flex-shrink-0 p-2 bg-purple-100 dark:bg-purple-900/30 rounded-lg">
                                                <svg class="w-5 h-5 text-purple-600 dark:text-purple-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
                                                </svg>
                                            </div>
                                        }.into_any(),
                                    }}
                                    <div>
                                        <p class="font-medium text-gray-800 dark:text-gray-200">
                                            {move || match i18n.locale.get() {
                                                Locale::Es => title_es.clone(),
                                                Locale::En => title_en.clone(),
                                            }}
                                        </p>
                                        <p class="text-xs text-gray-500 dark:text-gray-400 mt-0.5 capitalize">
                                            {move || match (i18n.locale.get(), category.as_str()) {
                                                (Locale::Es, "lesson") => "Leccion".to_string(),
                                                (Locale::Es, "exercise") => "Ejercicio".to_string(),
                                                (Locale::Es, "project") => "Proyecto".to_string(),
                                                (Locale::En, "lesson") => "Lesson".to_string(),
                                                (Locale::En, "exercise") => "Exercise".to_string(),
                                                (Locale::En, "project") => "Project".to_string(),
                                                _ => category.clone(),
                                            }}
                                        </p>
                                    </div>
                                </div>
                                <A
                                    href=href
                                    attr:class="px-4 py-2 bg-orange-600 hover:bg-orange-700 text-white rounded-lg font-medium transition-colors shadow-md"
                                >
                                    {move || match i18n.locale.get() {
                                        Locale::Es => "Continuar".to_string(),
                                        Locale::En => "Continue".to_string(),
                                    }}
                                </A>
                            </div>
                        }.into_any()
                    } else {
                        // All completed or still loading
                        view! {
                            <div class="p-4 bg-green-50 dark:bg-green-900/20 rounded-lg border border-green-200 dark:border-green-800 text-center">
                                <p class="text-green-700 dark:text-green-300 font-medium">
                                    {move || {
                                        let tr = Translations::get(i18n.locale.get());
                                        tr.path_all_completed.to_string()
                                    }}
                                </p>
                            </div>
                        }.into_any()
                    }
                }}
            </div>

            // Quick stats cards
            <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                // Lessons completed
                <div class="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-sm border border-gray-200 dark:border-gray-700">
                    <div class="flex items-center">
                        <div class="flex-shrink-0 p-3 bg-green-100 dark:bg-green-900/30 rounded-lg">
                            <svg class="w-6 h-6 text-green-600 dark:text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                            </svg>
                        </div>
                        <div class="ml-4">
                            <p class="text-sm text-gray-500 dark:text-gray-400">
                                {move || match i18n.locale.get() {
                                    Locale::Es => "Lecciones completadas".to_string(),
                                    Locale::En => "Lessons completed".to_string(),
                                }}
                            </p>
                            <p class="text-2xl font-bold text-gray-800 dark:text-white">
                                {move || format!("{}", lessons_completed.get())}
                            </p>
                        </div>
                    </div>
                </div>

                // Exercises done
                <div class="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-sm border border-gray-200 dark:border-gray-700">
                    <div class="flex items-center">
                        <div class="flex-shrink-0 p-3 bg-blue-100 dark:bg-blue-900/30 rounded-lg">
                            <svg class="w-6 h-6 text-blue-600 dark:text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4" />
                            </svg>
                        </div>
                        <div class="ml-4">
                            <p class="text-sm text-gray-500 dark:text-gray-400">
                                {move || match i18n.locale.get() {
                                    Locale::Es => "Ejercicios resueltos".to_string(),
                                    Locale::En => "Exercises done".to_string(),
                                }}
                            </p>
                            <p class="text-2xl font-bold text-gray-800 dark:text-white">
                                {move || format!("{}", exercises_done.get())}
                            </p>
                        </div>
                    </div>
                </div>

                // Projects started
                <div class="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-sm border border-gray-200 dark:border-gray-700">
                    <div class="flex items-center">
                        <div class="flex-shrink-0 p-3 bg-purple-100 dark:bg-purple-900/30 rounded-lg">
                            <svg class="w-6 h-6 text-purple-600 dark:text-purple-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z" />
                            </svg>
                        </div>
                        <div class="ml-4">
                            <p class="text-sm text-gray-500 dark:text-gray-400">
                                {move || match i18n.locale.get() {
                                    Locale::Es => "Proyectos iniciados".to_string(),
                                    Locale::En => "Projects started".to_string(),
                                }}
                            </p>
                            <p class="text-2xl font-bold text-gray-800 dark:text-white">
                                {move || format!("{}", projects_started.get())}
                            </p>
                        </div>
                    </div>
                </div>
            </div>

            // Module progress grid (loaded from TOML)
            <div>
                <h2 class="text-xl font-semibold text-gray-800 dark:text-white mb-4">
                    {move || {
                        let locale = i18n.locale.get();
                        let tr = Translations::get(locale);
                        tr.dashboard_progress.to_string()
                    }}
                </h2>
                {move || {
                    let Some(modules) = modules_data.get() else {
                        return view! {
                            <div class="text-center py-8">
                                <div class="inline-block w-6 h-6 border-4 border-orange-500 border-t-transparent rounded-full animate-spin"></div>
                            </div>
                        }.into_any();
                    };

                    view! {
                        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
                            {modules.into_iter().enumerate().map(|(idx, module)| {
                                let href = format!("/theory/{}", module.id);
                                let total_lessons = module.lessons.len();
                                let title_es = module.title_es.clone();
                                let title_en = module.title_en.clone();

                                // Collect lesson IDs for this module to compute completion
                                let module_lesson_ids: Vec<String> = module.lessons.iter()
                                    .map(|l| l.id.clone())
                                    .collect();

                                view! {
                                    <A
                                        href=href
                                        attr:class="block p-4 bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 hover:border-orange-400 dark:hover:border-orange-500 hover:shadow-md transition-all"
                                    >
                                        <div class="flex items-center justify-between mb-3">
                                            <span class="text-xs font-medium text-gray-400 dark:text-gray-500">
                                                {format!("{:02}", idx + 1)}
                                            </span>
                                            <span class="text-xs text-gray-400">
                                                {move || {
                                                    let locale = i18n.locale.get();
                                                    let tr = Translations::get(locale);
                                                    format!("{} {}", total_lessons, tr.theory_lessons)
                                                }}
                                            </span>
                                        </div>
                                        <h3 class="font-medium text-gray-800 dark:text-gray-200 mb-3">
                                            {move || match i18n.locale.get() {
                                                Locale::Es => title_es.clone(),
                                                Locale::En => title_en.clone(),
                                            }}
                                        </h3>
                                        {move || {
                                            let completed_ids = completed_lesson_ids.get();
                                            let done = module_lesson_ids.iter()
                                                .filter(|id| completed_ids.contains(id))
                                                .count();
                                            let pct = if total_lessons > 0 {
                                                (done as f64 / total_lessons as f64 * 100.0).round() as u32
                                            } else {
                                                0
                                            };
                                            let bar_color = if pct == 100 {
                                                "bg-green-500"
                                            } else if pct > 0 {
                                                "bg-orange-500"
                                            } else {
                                                "bg-gray-300 dark:bg-gray-600"
                                            };
                                            view! {
                                                <div class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
                                                    <div
                                                        class=format!("{bar_color} h-2 rounded-full transition-all duration-500")
                                                        style=format!("width: {}%", pct)
                                                    ></div>
                                                </div>
                                                <p class="text-xs text-gray-400 mt-1.5 text-right">
                                                    {format!("{}%", pct)}
                                                </p>
                                            }
                                        }}
                                    </A>
                                }
                            }).collect::<Vec<_>>()}
                        </div>
                    }.into_any()
                }}
            </div>
        </div>
    }
}
