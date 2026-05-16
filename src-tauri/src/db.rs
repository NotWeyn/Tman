use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct TranslationHistory {
    pub id: i64,
    pub original_text: String,
    pub translated_text: String,
    pub source_lang: String,
    pub target_lang: String,
    pub timestamp: String,
}

pub async fn init_db(db_url: &str) -> Result<SqlitePool, String> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
        .map_err(|e| format!("Failed to connect to db: {}", e))?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            original_text TEXT NOT NULL,
            translated_text TEXT NOT NULL,
            source_lang TEXT NOT NULL,
            target_lang TEXT NOT NULL,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
        );"
    )
    .execute(&pool)
    .await
    .map_err(|e| format!("Failed to create table: {}", e))?;

    Ok(pool)
}

pub async fn insert_history(pool: &SqlitePool, original: &str, translated: &str, target: &str, max_records: u32) -> Result<(), String> {
    sqlx::query(
        "INSERT INTO history (original_text, translated_text, source_lang, target_lang) VALUES (?, ?, ?, ?)"
    )
    .bind(original)
    .bind(translated)
    .bind("auto")
    .bind(target)
    .execute(pool)
    .await
    .map_err(|e| format!("Failed to insert history: {}", e))?;

    if max_records > 0 {
        // Keep only the latest `max_records` rows
        sqlx::query(
            "DELETE FROM history WHERE id NOT IN (SELECT id FROM history ORDER BY timestamp DESC LIMIT ?)"
        )
        .bind(max_records)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to prune history: {}", e))?;
    }

    Ok(())
}

pub async fn get_history(pool: &SqlitePool) -> Result<Vec<TranslationHistory>, String> {
    let history = sqlx::query_as::<_, TranslationHistory>(
        "SELECT * FROM history ORDER BY timestamp DESC LIMIT 50"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Failed to fetch history: {}", e))?;

    Ok(history)
}

pub async fn get_cached_translation(pool: &SqlitePool, original: &str, target: &str) -> Result<Option<String>, String> {
    let row: Option<(String,)> = sqlx::query_as(
        "SELECT translated_text FROM history WHERE original_text = ? AND target_lang = ? ORDER BY timestamp DESC LIMIT 1"
    )
    .bind(original)
    .bind(target)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Failed to query cache: {}", e))?;

    Ok(row.map(|r| r.0))
}

pub async fn clear_history(pool: &SqlitePool) -> Result<(), String> {
    sqlx::query("DELETE FROM history")
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to clear history: {}", e))?;
    Ok(())
}
