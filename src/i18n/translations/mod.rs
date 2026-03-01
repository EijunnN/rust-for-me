pub mod en;
pub mod es;

use super::locale::Locale;

pub struct Translations;

impl Translations {
    pub fn get(locale: Locale) -> &'static TranslationSet {
        match locale {
            Locale::Es => &es::TRANSLATIONS,
            Locale::En => &en::TRANSLATIONS,
        }
    }
}

#[allow(dead_code)]
pub struct TranslationSet {
    // Navigation
    pub nav_dashboard: &'static str,
    pub nav_theory: &'static str,
    pub nav_practice: &'static str,
    pub nav_projects: &'static str,
    pub nav_settings: &'static str,
    // Dashboard
    pub dashboard_title: &'static str,
    pub dashboard_welcome: &'static str,
    pub dashboard_continue: &'static str,
    pub dashboard_progress: &'static str,
    // Theory
    pub theory_title: &'static str,
    pub theory_lessons: &'static str,
    // Practice
    pub practice_title: &'static str,
    pub practice_exercises: &'static str,
    // Projects
    pub projects_title: &'static str,
    // Settings
    pub settings_title: &'static str,
    pub settings_language: &'static str,
    pub settings_theme: &'static str,
    pub settings_rust_path: &'static str,
    // Common
    pub common_next: &'static str,
    pub common_previous: &'static str,
    pub common_run: &'static str,
    pub common_reset: &'static str,
    pub common_hint: &'static str,
    pub common_solution: &'static str,
    pub common_beginner: &'static str,
    pub common_intermediate: &'static str,
    pub common_advanced: &'static str,

    // Output panel
    pub output_title: &'static str,
    pub output_compiling: &'static str,
    pub output_compiling_running: &'static str,
    pub output_click_run: &'static str,

    // Success panel
    pub success_title: &'static str,
    pub success_compiled_ok: &'static str,
    pub success_exercise_passed: &'static str,
    pub success_exercise_failed: &'static str,
    pub success_expected: &'static str,
    pub success_got: &'static str,
    pub success_keep_going: &'static str,

    // Error explainer
    pub error_what: &'static str,
    pub error_why: &'static str,
    pub error_fix: &'static str,
    pub error_show_explanation: &'static str,
    pub error_hide_explanation: &'static str,

    // E0382 - use of moved value
    pub error_e0382_what: &'static str,
    pub error_e0382_why: &'static str,
    pub error_e0382_fix: &'static str,

    // E0502 - cannot borrow as mutable
    pub error_e0502_what: &'static str,
    pub error_e0502_why: &'static str,
    pub error_e0502_fix: &'static str,

    // E0308 - mismatched types
    pub error_e0308_what: &'static str,
    pub error_e0308_why: &'static str,
    pub error_e0308_fix: &'static str,

    // E0425 - cannot find value
    pub error_e0425_what: &'static str,
    pub error_e0425_why: &'static str,
    pub error_e0425_fix: &'static str,

    // E0384 - cannot assign twice to immutable variable
    pub error_e0384_what: &'static str,
    pub error_e0384_why: &'static str,
    pub error_e0384_fix: &'static str,

    // E0106 - missing lifetime specifier
    pub error_e0106_what: &'static str,
    pub error_e0106_why: &'static str,
    pub error_e0106_fix: &'static str,

    // Exercise types
    pub exercise_type_write_code: &'static str,
    pub exercise_type_fix_bug: &'static str,
    pub exercise_type_predict_output: &'static str,
    pub exercise_fix_instructions: &'static str,
    pub exercise_predict_instructions: &'static str,
    pub exercise_compiler_error: &'static str,
    pub exercise_predict_check: &'static str,
    pub exercise_predict_correct: &'static str,
    pub exercise_predict_incorrect: &'static str,
    pub exercise_predict_select: &'static str,
    pub exercise_expected_output: &'static str,
    pub exercise_show_hint: &'static str,

    // Learning path / prerequisites
    pub path_locked: &'static str,
    pub path_prerequisites: &'static str,
    pub path_complete_first: &'static str,
    pub path_recommended_next: &'static str,
    pub path_all_completed: &'static str,

    // Playground
    pub playground_title: &'static str,
    pub playground_clear: &'static str,
    pub playground_tooltip: &'static str,
    pub playground_clippy: &'static str,
    pub playground_mode_remote: &'static str,
    pub playground_mode_local: &'static str,
    pub playground_no_rust_hint: &'static str,
}
