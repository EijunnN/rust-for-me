mod commands;
pub mod db;

use tauri::Manager;

/// Entry point for the Tauri application. Call this from `main()`.
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // ── Database setup ──
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to resolve app data directory");

            std::fs::create_dir_all(&app_data_dir)
                .expect("Failed to create app data directory");

            let db_path = app_data_dir.join("rust_for_everyone.db");

            let db_state = db::init_db(
                db_path
                    .to_str()
                    .expect("Database path contains invalid UTF-8"),
            )
            .expect("Failed to initialise database");

            app.manage(db_state);

            // ── Content directory setup ──
            // In dev mode, the Tauri binary runs from src-tauri/ so we go up
            // one level to find the project root's content/ directory.
            // In production, content/ is bundled next to the binary.
            let content_dir = {
                let cwd = std::env::current_dir().unwrap_or_default();
                let candidate = cwd.join("content");
                if candidate.exists() {
                    candidate
                } else {
                    // Try parent directory (dev mode: CWD is src-tauri/)
                    let parent_candidate = cwd.parent()
                        .map(|p| p.join("content"))
                        .unwrap_or(candidate);
                    if parent_candidate.exists() {
                        parent_candidate
                    } else {
                        // Fallback: resolve relative to the executable
                        let exe_dir = std::env::current_exe()
                            .ok()
                            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
                            .unwrap_or(cwd);
                        exe_dir.join("content")
                    }
                }
            };

            eprintln!("[rust-for-everyone] Content directory: {}", content_dir.display());
            app.manage(commands::filesystem::ContentDir(content_dir));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::compiler::compile_and_run,
            commands::compiler::compile_and_run_hybrid,
            commands::compiler::clippy_check,
            commands::compiler::check_rust_available,
            commands::progress::get_progress,
            commands::progress::save_progress,
            commands::progress::get_all_progress,
            commands::filesystem::load_content,
            commands::filesystem::list_content_dir,
        ])
        .run(tauri::generate_context!())
        .expect("Error while running Tauri application");
}
