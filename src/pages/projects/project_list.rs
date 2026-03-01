use leptos::prelude::*;
use leptos_router::components::A;
use crate::components::common::difficulty_badge::DifficultyBadge;
use crate::i18n::{use_i18n, Translations};
use crate::i18n::locale::Locale;
use crate::models::project::Project;
use crate::models::progress::UserProgress;

/// Displays a catalog of guided projects loaded from TOML files.
/// Projects with unmet prerequisites show a lock icon and cannot be clicked.
#[component]
pub fn ProjectList() -> impl IntoView {
    let i18n = use_i18n();

    // Async-loaded projects
    let projects_data: RwSignal<Option<Vec<Project>>> = RwSignal::new(None);
    let load_error: RwSignal<Option<String>> = RwSignal::new(None);
    let all_progress: RwSignal<Vec<UserProgress>> = RwSignal::new(Vec::new());

    leptos::task::spawn_local(async move {
        match crate::services::content_service::load_all_projects().await {
            Ok(projects) => projects_data.set(Some(projects)),
            Err(e) => load_error.set(Some(e)),
        }

        if let Ok(progress) = crate::services::progress_service::get_all_progress(None).await {
            all_progress.set(progress);
        }
    });

    view! {
        <div class="max-w-6xl mx-auto space-y-6">
            <div class="flex items-center justify-between">
                <h1 class="text-2xl font-bold text-gray-800 dark:text-white">
                    {move || {
                        let tr = Translations::get(i18n.locale.get());
                        tr.projects_title.to_string()
                    }}
                </h1>
            </div>

            <p class="text-gray-500 dark:text-gray-400">
                {move || match i18n.locale.get() {
                    Locale::Es => "Aprende Rust construyendo proyectos reales paso a paso.".to_string(),
                    Locale::En => "Learn Rust by building real projects step by step.".to_string(),
                }}
            </p>

            {move || {
                if let Some(err) = load_error.get() {
                    return view! {
                        <div class="text-center py-8">
                            <p class="text-red-500">{format!("Error: {}", err)}</p>
                        </div>
                    }.into_any();
                }

                let Some(projects) = projects_data.get() else {
                    return view! {
                        <div class="text-center py-8">
                            <div class="inline-block w-8 h-8 border-4 border-orange-500 border-t-transparent rounded-full animate-spin"></div>
                            <p class="mt-4 text-gray-500 dark:text-gray-400">
                                {move || match i18n.locale.get() {
                                    Locale::Es => "Cargando proyectos...".to_string(),
                                    Locale::En => "Loading projects...".to_string(),
                                }}
                            </p>
                        </div>
                    }.into_any();
                };

                view! {
                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                        {projects.into_iter().map(|project| {
                            let href = format!("/projects/{}", project.id);
                            let title_es = project.title.es.clone();
                            let title_en = project.title.en.clone();
                            let desc_es = project.description.es.clone();
                            let desc_en = project.description.en.clone();
                            let difficulty = project.difficulty.clone();
                            let steps = project.steps.len();
                            let prerequisites = project.prerequisites.clone();

                            let is_locked = {
                                let prereqs = prerequisites.clone();
                                move || {
                                    let progress = all_progress.get();
                                    !crate::services::learning_path::is_unlocked(&prereqs, &progress)
                                }
                            };

                            let prereq_display = prerequisites.clone();

                            view! {
                                <div class="relative">
                                    {move || {
                                        let locked = is_locked();
                                        if locked {
                                            // Locked card - no link, dimmed appearance
                                            let t_es = title_es.clone();
                                            let t_en = title_en.clone();
                                            let d_es = desc_es.clone();
                                            let d_en = desc_en.clone();
                                            let diff = difficulty.clone();
                                            let prereqs = prereq_display.clone();
                                            view! {
                                                <div class="block bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 overflow-hidden opacity-60">
                                                    // Lock overlay badge
                                                    <div class="absolute top-3 right-3 z-10">
                                                        <span class="inline-flex items-center gap-1 px-2 py-1 rounded-full text-xs font-medium bg-gray-200 dark:bg-gray-600 text-gray-600 dark:text-gray-300">
                                                            <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
                                                            </svg>
                                                            {move || {
                                                                let tr = Translations::get(i18n.locale.get());
                                                                tr.path_locked.to_string()
                                                            }}
                                                        </span>
                                                    </div>
                                                    <div class="p-6 pb-4">
                                                        <div class="flex items-start justify-between mb-3">
                                                            <span class="text-3xl opacity-40">{"\u{1F4E6}"}</span>
                                                            <DifficultyBadge difficulty=diff />
                                                        </div>
                                                        <h3 class="text-lg font-semibold text-gray-400 dark:text-gray-500 mb-2">
                                                            {move || match i18n.locale.get() {
                                                                Locale::Es => t_es.clone(),
                                                                Locale::En => t_en.clone(),
                                                            }}
                                                        </h3>
                                                        <p class="text-sm text-gray-400 dark:text-gray-500 line-clamp-3">
                                                            {move || match i18n.locale.get() {
                                                                Locale::Es => d_es.clone(),
                                                                Locale::En => d_en.clone(),
                                                            }}
                                                        </p>
                                                    </div>
                                                    // Prerequisites hint
                                                    <div class="px-6 pb-4">
                                                        <p class="text-xs text-gray-400 dark:text-gray-500">
                                                            {move || {
                                                                let tr = Translations::get(i18n.locale.get());
                                                                format!("{}: {}", tr.path_complete_first, prereqs.join(", "))
                                                            }}
                                                        </p>
                                                    </div>
                                                </div>
                                            }.into_any()
                                        } else {
                                            // Unlocked card - clickable
                                            let t_es = title_es.clone();
                                            let t_en = title_en.clone();
                                            let d_es = desc_es.clone();
                                            let d_en = desc_en.clone();
                                            let diff = difficulty.clone();
                                            let h = href.clone();
                                            view! {
                                                <A
                                                    href=h
                                                    attr:class="block bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 hover:border-orange-400 dark:hover:border-orange-500 hover:shadow-lg transition-all group overflow-hidden"
                                                >
                                                    <div class="p-6 pb-4">
                                                        <div class="flex items-start justify-between mb-3">
                                                            <span class="text-3xl">{"\u{1F4E6}"}</span>
                                                            <DifficultyBadge difficulty=diff />
                                                        </div>
                                                        <h3 class="text-lg font-semibold text-gray-800 dark:text-gray-200 group-hover:text-orange-500 transition-colors mb-2">
                                                            {move || match i18n.locale.get() {
                                                                Locale::Es => t_es.clone(),
                                                                Locale::En => t_en.clone(),
                                                            }}
                                                        </h3>
                                                        <p class="text-sm text-gray-500 dark:text-gray-400 line-clamp-3">
                                                            {move || match i18n.locale.get() {
                                                                Locale::Es => d_es.clone(),
                                                                Locale::En => d_en.clone(),
                                                            }}
                                                        </p>
                                                    </div>
                                                    <div class="px-6 pb-4">
                                                        <div class="flex items-center justify-between text-xs text-gray-400 mb-2">
                                                            <span>
                                                                {move || match i18n.locale.get() {
                                                                    Locale::Es => format!("{} pasos", steps),
                                                                    Locale::En => format!("{} steps", steps),
                                                                }}
                                                            </span>
                                                        </div>
                                                        <div class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-1.5">
                                                            <div
                                                                class="bg-gray-300 dark:bg-gray-600 h-1.5 rounded-full transition-all duration-500"
                                                                style="width: 0%"
                                                            ></div>
                                                        </div>
                                                    </div>
                                                </A>
                                            }.into_any()
                                        }
                                    }}
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                }.into_any()
            }}
        </div>
    }
}
