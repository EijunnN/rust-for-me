use rusqlite::Connection;

use super::schema;

/// Run all database migrations. Each migration is idempotent thanks to
/// `CREATE TABLE IF NOT EXISTS`, so this is safe to call on every startup.
pub fn run_migrations(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute_batch(schema::CREATE_USER_PROGRESS)?;
    conn.execute_batch(schema::CREATE_USER_SETTINGS)?;
    conn.execute_batch(schema::CREATE_USER_NOTES)?;
    Ok(())
}
