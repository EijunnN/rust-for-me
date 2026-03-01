use leptos::prelude::*;
use leptos_router::components::A;
use crate::components::common::difficulty_badge::DifficultyBadge;
use crate::components::common::completion_check::CompletionCheck;
use crate::i18n::{use_i18n, Translations};
use crate::i18n::locale::Locale;
use crate::models::exercise::{Exercise, ExerciseType};
use crate::models::progress::{ProgressStatus, UserProgress};

/// Displays a filterable list of all practice exercises loaded from TOML files.
/// Exercises with unmet prerequisites show a lock icon instead of the completion check.
#[component]
pub fn ExerciseList() -> impl IntoView {
    let i18n = use_i18n();
    let (filter_difficulty, set_filter_difficulty) = signal(String::from("all"));
    let (filter_type, set_filter_type) = signal(String::from("all"));

    // Async-loaded exercises
    let exercises_data: RwSignal<Option<Vec<Exercise>>> = RwSignal::new(None);
    let load_error: RwSignal<Option<String>> = RwSignal::new(None);
    let all_progress: RwSignal<Vec<UserProgress>> = RwSignal::new(Vec::new());

    leptos::task::spawn_local(async move {
        match crate::services::content_service::load_all_exercises().await {
            Ok(exercises) => exercises_data.set(Some(exercises)),
            Err(e) => load_error.set(Some(e)),
        }

        if let Ok(progress) = crate::services::progress_service::get_all_progress(None).await {
            all_progress.set(progress);
        }
    });

    // Difficulty filter options
    let filter_options = vec![
        ("all", "Todos", "All"),
        ("beginner", "Principiante", "Beginner"),
        ("intermediate", "Intermedio", "Intermediate"),
        ("advanced", "Avanzado", "Advanced"),
    ];

    // Exercise type filter options
    let type_filter_options = vec![
        ("all", "Todos los tipos", "All types"),
        ("write_code", "Escribir Codigo", "Write Code"),
        ("fix_bug", "Corregir Error", "Fix the Bug"),
        ("predict_output", "Predecir Salida", "Predict Output"),
    ];

    view! {
        <div class="max-w-4xl mx-auto space-y-6">
            <h1 class="text-2xl font-bold text-gray-800 dark:text-white">
                {move || {
                    let tr = Translations::get(i18n.locale.get());
                    tr.practice_title.to_string()
                }}
            </h1>

            // Filter bar
            <div class="space-y-3 p-4 bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700">
                // Difficulty filter
                <div class="flex flex-wrap items-center gap-2">
                    <span class="text-sm text-gray-500 dark:text-gray-400 mr-2">
                        {move || match i18n.locale.get() {
                            Locale::Es => "Filtrar:".to_string(),
                            Locale::En => "Filter:".to_string(),
                        }}
                    </span>
                    {filter_options.into_iter().map(|(value, label_es, label_en)| {
                        let value_str = value.to_string();
                        let value_for_click = value.to_string();
                        view! {
                            <button
                                class=move || {
                                    let base = "px-3 py-1.5 rounded-lg text-sm font-medium transition-colors";
                                    if filter_difficulty.get() == value_str {
                                        format!("{base} bg-orange-500 text-white")
                                    } else {
                                        format!("{base} bg-gray-100 dark:bg-gray-700 text-gray-600 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-600")
                                    }
                                }
                                on:click={
                                    let v = value_for_click.clone();
                                    move |_| set_filter_difficulty.set(v.clone())
                                }
                            >
                                {move || match i18n.locale.get() {
                                    Locale::Es => label_es.to_string(),
                                    Locale::En => label_en.to_string(),
                                }}
                            </button>
                        }
                    }).collect::<Vec<_>>()}
                </div>

                // Exercise type filter
                <div class="flex flex-wrap items-center gap-2">
                    <span class="text-sm text-gray-500 dark:text-gray-400 mr-2">
                        {move || match i18n.locale.get() {
                            Locale::Es => "Tipo:".to_string(),
                            Locale::En => "Type:".to_string(),
                        }}
                    </span>
                    {type_filter_options.into_iter().map(|(value, label_es, label_en)| {
                        let value_str = value.to_string();
                        let value_for_click = value.to_string();
                        view! {
                            <button
                                class=move || {
                                    let base = "px-3 py-1.5 rounded-lg text-sm font-medium transition-colors";
                                    if filter_type.get() == value_str {
                                        format!("{base} bg-orange-500 text-white")
                                    } else {
                                        format!("{base} bg-gray-100 dark:bg-gray-700 text-gray-600 dark:text-gray-300 hover:bg-gray-200 dark:hover:bg-gray-600")
                                    }
                                }
                                on:click={
                                    let v = value_for_click.clone();
                                    move |_| set_filter_type.set(v.clone())
                                }
                            >
                                {move || match i18n.locale.get() {
                                    Locale::Es => label_es.to_string(),
                                    Locale::En => label_en.to_string(),
                                }}
                            </button>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>

            // Exercise list
            {move || {
                if let Some(err) = load_error.get() {
                    return view! {
                        <div class="text-center py-8">
                            <p class="text-red-500">{format!("Error: {}", err)}</p>
                        </div>
                    }.into_any();
                }

                let Some(exercises) = exercises_data.get() else {
                    return view! {
                        <div class="text-center py-8">
                            <div class="inline-block w-8 h-8 border-4 border-orange-500 border-t-transparent rounded-full animate-spin"></div>
                            <p class="mt-4 text-gray-500 dark:text-gray-400">
                                {move || match i18n.locale.get() {
                                    Locale::Es => "Cargando ejercicios...".to_string(),
                                    Locale::En => "Loading exercises...".to_string(),
                                }}
                            </p>
                        </div>
                    }.into_any();
                };

                view! {
                    <div class="grid grid-cols-1 gap-3">
                        {exercises.into_iter().map(|exercise| {
                            let id = exercise.meta.id.clone();
                            let title_es = exercise.title.es.clone();
                            let title_en = exercise.title.en.clone();
                            let module = exercise.meta.module.clone();
                            let difficulty = exercise.meta.difficulty.clone();
                            let difficulty_filter = exercise.meta.difficulty.clone();
                            let exercise_type = exercise.meta.exercise_type.clone();
                            let prerequisites = exercise.meta.prerequisites.clone();
                            let href = format!("/practice/{}", id);
                            let exercise_id = id.clone();

                            let type_filter_key = match &exercise_type {
                                ExerciseType::WriteCode => "write_code".to_string(),
                                ExerciseType::FixBug => "fix_bug".to_string(),
                                ExerciseType::PredictOutput => "predict_output".to_string(),
                            };

                            let badge_classes = exercise_type.badge_classes().to_string();
                            let type_label_es = exercise_type.label_es().to_string();
                            let type_label_en = exercise_type.label_en().to_string();
                            let is_fix_bug = exercise_type == ExerciseType::FixBug;
                            let is_predict = exercise_type == ExerciseType::PredictOutput;

                            // Check if exercise is locked due to prerequisites
                            let is_locked = {
                                let prereqs = prerequisites.clone();
                                move || {
                                    let progress = all_progress.get();
                                    !crate::services::learning_path::is_unlocked(&prereqs, &progress)
                                }
                            };

                            // Check if exercise is completed
                            let is_completed = {
                                let eid = exercise_id.clone();
                                move || {
                                    let progress = all_progress.get();
                                    progress.iter().any(|p| {
                                        p.id == eid && p.category == "exercise" && p.status == ProgressStatus::Completed
                                    })
                                }
                            };

                            view! {
                                <div
                                    class=move || {
                                        let diff_filter = filter_difficulty.get();
                                        let t_filter = filter_type.get();
                                        let diff_match = diff_filter == "all" || diff_filter == difficulty_filter;
                                        let type_match = t_filter == "all" || t_filter == type_filter_key;
                                        if diff_match && type_match {
                                            "block".to_string()
                                        } else {
                                            "hidden".to_string()
                                        }
                                    }
                                >
                                    {move || {
                                        let locked = is_locked();
                                        let completed = is_completed();
                                        let t_es = title_es.clone();
                                        let t_en = title_en.clone();
                                        let tl_es = type_label_es.clone();
                                        let tl_en = type_label_en.clone();
                                        let bc = badge_classes.clone();
                                        let mod_name = module.clone();
                                        let diff = difficulty.clone();

                                        if locked {
                                            // Locked exercise row
                                            view! {
                                                <div class="flex items-center justify-between p-4 bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 opacity-50 cursor-not-allowed">
                                                    <div class="flex items-center gap-4">
                                                        // Lock icon instead of completion check
                                                        <div class="w-6 h-6 flex items-center justify-center text-gray-400">
                                                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
                                                            </svg>
                                                        </div>
                                                        <div>
                                                            <div class="flex items-center gap-2">
                                                                <h3 class="font-medium text-gray-400 dark:text-gray-500">
                                                                    {move || match i18n.locale.get() {
                                                                        Locale::Es => t_es.clone(),
                                                                        Locale::En => t_en.clone(),
                                                                    }}
                                                                </h3>
                                                                <span class={format!("inline-flex items-center gap-1 px-2 py-0.5 rounded-full text-xs font-medium opacity-50 {}", bc)}>
                                                                    {move || match i18n.locale.get() {
                                                                        Locale::Es => tl_es.clone(),
                                                                        Locale::En => tl_en.clone(),
                                                                    }}
                                                                </span>
                                                            </div>
                                                            <p class="text-xs text-gray-400 dark:text-gray-500 mt-0.5">
                                                                {mod_name}
                                                            </p>
                                                        </div>
                                                    </div>
                                                    <div class="flex items-center gap-3">
                                                        <DifficultyBadge difficulty=diff />
                                                        <span class="text-xs text-gray-400">
                                                            {move || {
                                                                let tr = Translations::get(i18n.locale.get());
                                                                tr.path_locked.to_string()
                                                            }}
                                                        </span>
                                                    </div>
                                                </div>
                                            }.into_any()
                                        } else {
                                            // Unlocked exercise row
                                            let h = href.clone();
                                            view! {
                                                <A
                                                    href=h
                                                    attr:class="flex items-center justify-between p-4 bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700 hover:border-orange-400 dark:hover:border-orange-500 hover:shadow-md transition-all group"
                                                >
                                                    <div class="flex items-center gap-4">
                                                        <CompletionCheck completed=completed />
                                                        <div>
                                                            <div class="flex items-center gap-2">
                                                                <h3 class="font-medium text-gray-800 dark:text-gray-200 group-hover:text-orange-500 transition-colors">
                                                                    {move || match i18n.locale.get() {
                                                                        Locale::Es => t_es.clone(),
                                                                        Locale::En => t_en.clone(),
                                                                    }}
                                                                </h3>
                                                                <span class={format!("inline-flex items-center gap-1 px-2 py-0.5 rounded-full text-xs font-medium {}", bc)}>
                                                                    {if is_fix_bug {
                                                                        view! {
                                                                            <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.066 2.573c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.573 1.066c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.066-2.573c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
                                                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                                                                            </svg>
                                                                        }.into_any()
                                                                    } else if is_predict {
                                                                        view! {
                                                                            <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8.228 9c.549-1.165 2.03-2 3.772-2 2.21 0 4 1.343 4 3 0 1.4-1.278 2.575-3.006 2.907-.542.104-.994.54-.994 1.093m0 3h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                                                                            </svg>
                                                                        }.into_any()
                                                                    } else {
                                                                        view! {
                                                                            <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4" />
                                                                            </svg>
                                                                        }.into_any()
                                                                    }}
                                                                    {move || match i18n.locale.get() {
                                                                        Locale::Es => tl_es.clone(),
                                                                        Locale::En => tl_en.clone(),
                                                                    }}
                                                                </span>
                                                            </div>
                                                            <p class="text-xs text-gray-400 dark:text-gray-500 mt-0.5">
                                                                {mod_name}
                                                            </p>
                                                        </div>
                                                    </div>
                                                    <div class="flex items-center gap-3">
                                                        <DifficultyBadge difficulty=diff />
                                                        <svg class="w-5 h-5 text-gray-400 group-hover:text-orange-500 transition-colors" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
                                                        </svg>
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
