use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;
use crate::i18n::{use_i18n, Translations};
use crate::i18n::locale::Locale;
use crate::components::common::completion_check::CompletionCheck;
use crate::models::module::Module;
use crate::models::progress::ProgressStatus;

/// Displays the list of all theory modules, or a specific module's lessons
/// when a module_id is present in the URL.
#[component]
pub fn ModuleList() -> impl IntoView {
    let i18n = use_i18n();
    let params = use_params_map();

    // Async-loaded modules
    let modules_data: RwSignal<Option<Vec<Module>>> = RwSignal::new(None);
    let load_error: RwSignal<Option<String>> = RwSignal::new(None);
    // IDs of completed lessons (from progress DB)
    let completed_ids: RwSignal<Vec<String>> = RwSignal::new(Vec::new());

    // Trigger load on mount
    leptos::task::spawn_local(async move {
        match crate::services::content_service::load_all_modules().await {
            Ok(modules) => modules_data.set(Some(modules)),
            Err(e) => load_error.set(Some(e)),
        }
        // Load progress data
        if let Ok(all_progress) = crate::services::progress_service::get_all_progress(None).await {
            let ids: Vec<String> = all_progress
                .iter()
                .filter(|p| p.category == "lesson" && p.status == ProgressStatus::Completed)
                .map(|p| p.id.clone())
                .collect();
            completed_ids.set(ids);
        }
    });

    view! {
        <div class="max-w-7xl mx-auto">
            {move || {
                // Show loading state
                if let Some(err) = load_error.get() {
                    return view! {
                        <div class="text-center py-12">
                            <p class="text-red-500 text-lg">{format!("Error loading modules: {}", err)}</p>
                        </div>
                    }.into_any();
                }

                let Some(modules) = modules_data.get() else {
                    return view! {
                        <div class="text-center py-12">
                            <div class="inline-block w-8 h-8 border-4 border-orange-500 border-t-transparent rounded-full animate-spin"></div>
                            <p class="mt-4 text-gray-500 dark:text-gray-400">
                                {move || match i18n.locale.get() {
                                    Locale::Es => "Cargando modulos...".to_string(),
                                    Locale::En => "Loading modules...".to_string(),
                                }}
                            </p>
                        </div>
                    }.into_any();
                };

                let module_id = params.get().get("module_id").map(|s| s.to_string());

                if let Some(mod_id) = module_id {
                    // Show lessons for a specific module
                    let module = modules.iter().find(|m| m.id == mod_id);

                    if let Some(m) = module {
                        let title_es = m.title_es.clone();
                        let title_en = m.title_en.clone();
                        let desc_es = m.description_es.clone();
                        let desc_en = m.description_en.clone();
                        let icon = m.icon.clone();
                        let lessons = m.lessons.clone();
                        let mod_id_clone = mod_id.clone();

                        view! {
                            <div class="space-y-6">
                                // Back link
                                <A
                                    href="/theory"
                                    attr:class="inline-flex items-center text-gray-600 dark:text-gray-400 hover:text-orange-500 transition-colors"
                                >
                                    <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
                                    </svg>
                                    {move || match i18n.locale.get() {
                                        Locale::Es => "Volver a modulos".to_string(),
                                        Locale::En => "Back to modules".to_string(),
                                    }}
                                </A>

                                // Module header card
                                <div class="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-sm border border-gray-200 dark:border-gray-700">
                                    <div class="flex items-start gap-4">
                                        <span class="text-4xl">{icon.clone()}</span>
                                        <div class="flex-1">
                                            <h1 class="text-2xl font-bold text-gray-800 dark:text-white mb-1">
                                                {move || match i18n.locale.get() {
                                                    Locale::Es => title_es.clone(),
                                                    Locale::En => title_en.clone(),
                                                }}
                                            </h1>
                                            <p class="text-gray-500 dark:text-gray-400 mb-4">
                                                {move || match i18n.locale.get() {
                                                    Locale::Es => desc_es.clone(),
                                                    Locale::En => desc_en.clone(),
                                                }}
                                            </p>
                                        </div>
                                    </div>
                                </div>

                                // Lesson list
                                <div class="space-y-3">
                                    {lessons.into_iter().enumerate().map(|(idx, lesson)| {
                                        let href = format!("/theory/{}/{}", mod_id_clone, lesson.id);
                                        let l_title_es = lesson.title_es.clone();
                                        let l_title_en = lesson.title_en.clone();
                                        let lesson_id = lesson.id.clone();

                                        view! {
                                            <A
                                                href=href
                                                attr:class="flex items-center p-4 bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 hover:border-orange-400 dark:hover:border-orange-500 hover:shadow-md transition-all group"
                                            >
                                                {move || {
                                                    let done = completed_ids.get().contains(&lesson_id);
                                                    view! { <CompletionCheck completed=done /> }
                                                }}
                                                <div class="ml-4 flex-1">
                                                    <div class="flex items-center gap-2">
                                                        <span class="text-xs text-gray-400 font-mono bg-gray-100 dark:bg-gray-700 px-1.5 py-0.5 rounded">
                                                            {format!("{:02}", idx + 1)}
                                                        </span>
                                                        <h3 class="font-medium text-gray-800 dark:text-gray-200 group-hover:text-orange-500 transition-colors">
                                                            {move || match i18n.locale.get() {
                                                                Locale::Es => l_title_es.clone(),
                                                                Locale::En => l_title_en.clone(),
                                                            }}
                                                        </h3>
                                                    </div>
                                                </div>
                                                <svg class="w-5 h-5 text-gray-400 group-hover:text-orange-500 transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
                                                </svg>
                                            </A>
                                        }
                                    }).collect::<Vec<_>>()}
                                </div>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div class="text-center py-12">
                                <p class="text-gray-500 dark:text-gray-400 text-lg mb-4">
                                    {move || match i18n.locale.get() {
                                        Locale::Es => "Modulo no encontrado".to_string(),
                                        Locale::En => "Module not found".to_string(),
                                    }}
                                </p>
                                <A
                                    href="/theory"
                                    attr:class="inline-block px-4 py-2 bg-orange-600 text-white rounded-lg hover:bg-orange-700 transition-colors"
                                >
                                    {move || match i18n.locale.get() {
                                        Locale::Es => "Volver a modulos".to_string(),
                                        Locale::En => "Back to modules".to_string(),
                                    }}
                                </A>
                            </div>
                        }.into_any()
                    }
                } else {
                    // Show the grid of all modules
                    let lesson_count: Vec<usize> = modules.iter().map(|m| m.lessons.len()).collect();

                    view! {
                        <div class="space-y-6">
                            <h1 class="text-2xl font-bold text-gray-800 dark:text-white">
                                {move || {
                                    let locale = i18n.locale.get();
                                    let tr = Translations::get(locale);
                                    tr.theory_title.to_string()
                                }}
                            </h1>

                            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                                {modules.into_iter().enumerate().map(|(idx, module)| {
                                    let href = format!("/theory/{}", module.id);
                                    let title_es = module.title_es.clone();
                                    let title_en = module.title_en.clone();
                                    let desc_es = module.description_es.clone();
                                    let desc_en = module.description_en.clone();
                                    let icon = module.icon.clone();
                                    let lessons = lesson_count[idx];

                                    view! {
                                        <A
                                            href=href
                                            attr:class="block p-6 bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700 hover:border-orange-400 dark:hover:border-orange-500 hover:shadow-lg transition-all group"
                                        >
                                            <div class="flex items-center justify-between mb-3">
                                                <span class="text-2xl">{icon.clone()}</span>
                                                <span class="text-xs font-medium text-gray-400 bg-gray-100 dark:bg-gray-700 px-2 py-1 rounded">
                                                    {format!("{:02}", idx + 1)}
                                                </span>
                                            </div>
                                            <h3 class="text-lg font-semibold text-gray-800 dark:text-gray-200 group-hover:text-orange-500 transition-colors mb-2">
                                                {move || match i18n.locale.get() {
                                                    Locale::Es => title_es.clone(),
                                                    Locale::En => title_en.clone(),
                                                }}
                                            </h3>
                                            <p class="text-sm text-gray-500 dark:text-gray-400 mb-4 line-clamp-2">
                                                {move || match i18n.locale.get() {
                                                    Locale::Es => desc_es.clone(),
                                                    Locale::En => desc_en.clone(),
                                                }}
                                            </p>
                                            <div class="flex items-center justify-between text-xs text-gray-400 mb-2">
                                                <span>
                                                    {move || {
                                                        let locale = i18n.locale.get();
                                                        let tr = Translations::get(locale);
                                                        format!("{} {}", lessons, tr.theory_lessons)
                                                    }}
                                                </span>
                                            </div>
                                            <div class="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
                                                <div
                                                    class="bg-gray-300 dark:bg-gray-600 h-2 rounded-full transition-all duration-500"
                                                    style="width: 0%"
                                                ></div>
                                            </div>
                                        </A>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        </div>
                    }.into_any()
                }
            }}
        </div>
    }
}
