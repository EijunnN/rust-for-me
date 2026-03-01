/// SQL statements for creating the database schema.

pub const CREATE_USER_PROGRESS: &str = r#"
CREATE TABLE IF NOT EXISTS user_progress (
    id TEXT PRIMARY KEY,
    category TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'locked',
    score INTEGER DEFAULT 0,
    attempts INTEGER DEFAULT 0,
    completed_at TEXT,
    updated_at TEXT NOT NULL
);
"#;

pub const CREATE_USER_SETTINGS: &str = r#"
CREATE TABLE IF NOT EXISTS user_settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);
"#;

pub const CREATE_USER_NOTES: &str = r#"
CREATE TABLE IF NOT EXISTS user_notes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    content_id TEXT NOT NULL,
    note TEXT NOT NULL,
    created_at TEXT NOT NULL
);
"#;
