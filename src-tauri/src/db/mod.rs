pub mod migrations;
pub mod schema;

use std::sync::Mutex;

use rusqlite::Connection;

/// Shared database state managed by Tauri. The `Mutex` ensures that only one
/// command accesses the SQLite connection at a time, which is required because
/// `rusqlite::Connection` is not `Sync`.
pub struct DbState {
    pub conn: Mutex<Connection>,
}

/// Open (or create) the SQLite database at `db_path`, run all migrations, and
/// return the wrapped connection ready for Tauri state management.
pub fn init_db(db_path: &str) -> Result<DbState, rusqlite::Error> {
    let conn = Connection::open(db_path)?;

    // Enable WAL mode for better concurrent read performance.
    conn.pragma_update(None, "journal_mode", "WAL")?;

    migrations::run_migrations(&conn)?;

    Ok(DbState {
        conn: Mutex::new(conn),
    })
}
