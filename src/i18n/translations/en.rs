use super::TranslationSet;

pub static TRANSLATIONS: TranslationSet = TranslationSet {
    // Navigation
    nav_dashboard: "Dashboard",
    nav_theory: "Theory",
    nav_practice: "Practice",
    nav_projects: "Projects",
    nav_settings: "Settings",
    // Dashboard
    dashboard_title: "Welcome to Rust for Everyone",
    dashboard_welcome: "Learn Rust interactively",
    dashboard_continue: "Continue learning",
    dashboard_progress: "Your progress",
    // Theory
    theory_title: "Theory Modules",
    theory_lessons: "lessons",
    // Practice
    practice_title: "Practice Exercises",
    practice_exercises: "exercises",
    // Projects
    projects_title: "Guided Projects",
    // Settings
    settings_title: "Settings",
    settings_language: "Language",
    settings_theme: "Theme",
    settings_rust_path: "Rust Path",
    // Common
    common_next: "Next",
    common_previous: "Previous",
    common_run: "Run",
    common_reset: "Reset",
    common_hint: "Hint",
    common_solution: "Solution",
    common_beginner: "Beginner",
    common_intermediate: "Intermediate",
    common_advanced: "Advanced",

    // Output panel
    output_title: "Output",
    output_compiling: "Compiling...",
    output_compiling_running: "Compiling and running...",
    output_click_run: "Click Run to execute your code",

    // Success panel
    success_title: "Success!",
    success_compiled_ok: "Your code compiled and ran successfully!",
    success_exercise_passed: "Output matches expected! Exercise completed!",
    success_exercise_failed: "Output doesn't match expected.",
    success_expected: "Expected",
    success_got: "Got",
    success_keep_going: "Great job! Keep going!",

    // Error explainer
    error_what: "What this error means",
    error_why: "Why Rust prevents this",
    error_fix: "How to fix it",
    error_show_explanation: "Show explanation",
    error_hide_explanation: "Hide explanation",

    // E0382
    error_e0382_what: "You tried to use a variable after its value was moved to another variable. In Rust, each value has exactly one owner. When you assign a value to a new variable or pass it to a function, the original variable can no longer be used.",
    error_e0382_why: "Rust's ownership system prevents use-after-free bugs. If two variables could use the same heap data, one might free it while the other still references it, causing crashes or security vulnerabilities.",
    error_e0382_fix: "You can: (1) Clone the value with .clone() if you need two copies, (2) Use a reference (&value) to borrow instead of moving, or (3) Restructure your code so the value is only used in one place.",

    // E0502
    error_e0502_what: "You tried to borrow a value as immutable (&) while it's already borrowed as mutable (&mut), or vice versa. Rust doesn't allow mixing mutable and immutable borrows at the same time.",
    error_e0502_why: "If you could read a value while something else is changing it, you might see inconsistent or partially updated data. This rule prevents data races and ensures references always point to valid data.",
    error_e0502_fix: "You can: (1) Finish using the mutable borrow before creating an immutable one, (2) Use separate scopes with {} to limit borrow lifetimes, or (3) Consider using Cell or RefCell for interior mutability.",

    // E0308
    error_e0308_what: "The compiler expected one type but found a different one. For example, a function expects an i32 but you passed a String, or a variable was declared as one type but assigned a different type.",
    error_e0308_why: "Rust's strong type system catches type errors at compile time rather than at runtime. This prevents bugs that would be hard to find later, like accidentally treating text as a number.",
    error_e0308_fix: "You can: (1) Change the value to match the expected type, (2) Use type conversion like .into(), as, or parse(), (3) Fix the function signature or variable annotation to match what you actually want.",

    // E0425
    error_e0425_what: "You used a variable or function name that the compiler can't find in the current scope. This usually means the name is misspelled, not yet declared, or declared in a different scope.",
    error_e0425_why: "Rust requires all names to be defined before use. This catches typos and ensures you're referencing something that actually exists, preventing runtime errors from undefined variables.",
    error_e0425_fix: "You can: (1) Check for typos in the variable name, (2) Make sure the variable is declared before it's used, (3) Check if the variable is defined inside a different block {} and move it to the right scope, or (4) Import the name with 'use' if it's from another module.",

    // E0384
    error_e0384_what: "You tried to change the value of a variable that wasn't declared as mutable. In Rust, variables are immutable by default -- you can't change them once assigned.",
    error_e0384_why: "Immutable variables prevent accidental changes to data. When you see a variable without 'mut', you know its value won't change, making the code easier to reason about and less prone to bugs.",
    error_e0384_fix: "Add 'mut' to the variable declaration: change 'let x = 5;' to 'let mut x = 5;'. Only add mut if you truly need to change the value -- keeping variables immutable when possible is good practice.",

    // E0106
    error_e0106_what: "A function or struct that uses references (&) is missing a lifetime annotation. Rust needs to know how long each reference is valid to ensure memory safety.",
    error_e0106_why: "Lifetimes tell Rust how long a reference stays valid. Without them, the compiler can't verify that references don't outlive the data they point to, which could lead to dangling references.",
    error_e0106_fix: "You can: (1) Add lifetime annotations like <'a> to your function or struct, (2) Use owned types (String instead of &str) to avoid references entirely, or (3) Let the compiler's lifetime elision rules handle it by simplifying your function signature.",

    // Exercise types
    exercise_type_write_code: "Write Code",
    exercise_type_fix_bug: "Fix the Bug",
    exercise_type_predict_output: "Predict Output",
    exercise_fix_instructions: "This code has a bug. Read the compiler error message and fix the code so it compiles and produces the expected output.",
    exercise_predict_instructions: "Look at the following code. Without running it, predict what the output will be. Then verify your answer.",
    exercise_compiler_error: "Compiler error",
    exercise_predict_check: "Check answer",
    exercise_predict_correct: "Correct! Your prediction was right.",
    exercise_predict_incorrect: "Incorrect. Run the code to see the actual output.",
    exercise_predict_select: "Select your prediction:",
    exercise_expected_output: "Expected output:",
    exercise_show_hint: "Show hint",

    // Learning path / prerequisites
    path_locked: "Locked",
    path_prerequisites: "Prerequisites",
    path_complete_first: "Complete these first",
    path_recommended_next: "Recommended Next",
    path_all_completed: "All content completed! Great job!",

    // Playground
    playground_title: "Rust Playground",
    playground_clear: "Clear",
    playground_tooltip: "Playground",
    playground_clippy: "Clippy",
    playground_mode_remote: "Remote",
    playground_mode_local: "Local",
    playground_no_rust_hint: "No Rust installed and no internet. Install Rust from rustup.rs or check your connection.",
};
