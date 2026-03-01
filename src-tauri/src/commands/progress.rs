use rusqlite::params;
use rusqlite::OptionalExtension;
use serde::{Deserialize, Serialize};

use crate::db::DbState;

/// A single progress record for a lesson, exercise, or project.
#[derive(Debug, Serialize, Deserialize)]
pub struct Progress {
    pub id: String,
    pub category: String,
    pub status: String,
    pub score: i32,
    pub attempts: i32,
    pub completed_at: Option<String>,
    pub updated_at: String,
}

/// Retrieve a single progress entry by its `id`.
/// The `category` parameter is accepted for API consistency but not used in the query.
#[tauri::command]
pub async fn get_progress(
    id: String,
    #[allow(unused_variables)] category: Option<String>,
    state: tauri::State<'_, DbState>,
) -> Result<Option<Progress>, String> {
    let conn = state.conn.lock().map_err(|e| format!("Lock error: {e}"))?;

    let mut stmt = conn
        .prepare(
            "SELECT id, category, status, score, attempts, completed_at, updated_at \
             FROM user_progress WHERE id = ?1",
        )
        .map_err(|e| format!("Query error: {e}"))?;

    let result = stmt
        .query_row(params![id], |row| {
            Ok(Progress {
                id: row.get(0)?,
                category: row.get(1)?,
                status: row.get(2)?,
                score: row.get(3)?,
                attempts: row.get(4)?,
                completed_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        })
        .optional()
        .map_err(|e| format!("Query error: {e}"))?;

    Ok(result)
}

/// Create or update a progress entry. Uses SQLite `INSERT OR REPLACE` so
/// calling this with the same `id` will overwrite the previous record. The
/// `attempts` counter is incremented automatically on conflict.
#[tauri::command]
pub async fn save_progress(
    id: String,
    category: String,
    status: String,
    score: i32,
    state: tauri::State<'_, DbState>,
) -> Result<(), String> {
    let conn = state.conn.lock().map_err(|e| format!("Lock error: {e}"))?;

    let now = chrono::Utc::now().to_rfc3339();

    let completed_at: Option<String> = if status == "completed" {
        Some(now.clone())
    } else {
        None
    };

    conn.execute(
        "INSERT INTO user_progress (id, category, status, score, attempts, completed_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, 1, ?5, ?6)
         ON CONFLICT(id) DO UPDATE SET
             category = excluded.category,
             status = excluded.status,
             score = excluded.score,
             attempts = attempts + 1,
             completed_at = COALESCE(excluded.completed_at, user_progress.completed_at),
             updated_at = excluded.updated_at",
        params![id, category, status, score, completed_at, now],
    )
    .map_err(|e| format!("Insert error: {e}"))?;

    Ok(())
}

/// Return every progress record in the database.
#[tauri::command]
pub async fn get_all_progress(
    state: tauri::State<'_, DbState>,
) -> Result<Vec<Progress>, String> {
    let conn = state.conn.lock().map_err(|e| format!("Lock error: {e}"))?;

    let mut stmt = conn
        .prepare(
            "SELECT id, category, status, score, attempts, completed_at, updated_at \
             FROM user_progress ORDER BY updated_at DESC",
        )
        .map_err(|e| format!("Query error: {e}"))?;

    let rows = stmt
        .query_map([], |row| {
            Ok(Progress {
                id: row.get(0)?,
                category: row.get(1)?,
                status: row.get(2)?,
                score: row.get(3)?,
                attempts: row.get(4)?,
                completed_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        })
        .map_err(|e| format!("Query error: {e}"))?;

    let mut progress_list = Vec::new();
    for row in rows {
        progress_list.push(row.map_err(|e| format!("Row error: {e}"))?);
    }

    Ok(progress_list)
}

