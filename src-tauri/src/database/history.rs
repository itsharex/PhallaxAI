use crate::database::schemas::History;
use chrono::SecondsFormat;
use sqlx::SqlitePool;

/// Insert a new history into the database
pub async fn insert_history(pool: &SqlitePool, history: History) -> anyhow::Result<i64> {
    let res = sqlx::query(
        r#"
        INSERT INTO history (file_id, name, assistant_id, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?);
        "#,
    )
    .bind(history.file_id)
    .bind(history.name)
    .bind(history.assistant_id)
    .bind(
        history
            .created_at
            .to_rfc3339_opts(SecondsFormat::Secs, true),
    )
    .bind(
        history
            .updated_at
            .to_rfc3339_opts(SecondsFormat::Secs, true),
    )
    .execute(pool)
    .await?;

    Ok(res.last_insert_rowid())
}

/// Get all history from the database
pub async fn get_history(pool: &SqlitePool) -> anyhow::Result<Vec<History>> {
    let history = sqlx::query_as("SELECT * FROM history")
        .fetch_all(pool)
        .await?;

    Ok(history)
}

/// Get a history by its ID
pub async fn get_history_by_id(pool: &SqlitePool, id: i64) -> anyhow::Result<History> {
    let history = sqlx::query_as::<sqlx::Sqlite, History>("SELECT * FROM history WHERE id = ?;")
        .bind(id)
        .fetch_one(pool)
        .await?;

    Ok(history)
}

/// Get a history by its name
pub async fn update_history(pool: &SqlitePool, history: History) -> anyhow::Result<()> {
    sqlx::query(
        r#"
        UPDATE history
        SET file_id = ?, name = ?, assistant_id = ?, updated_at = ?
        WHERE id = ?;
        "#,
    )
    .bind(history.file_id)
    .bind(history.name)
    .bind(history.assistant_id)
    .bind(chrono::Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true))
    .bind(history.id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Delete a history by its ID
pub async fn delete_history(pool: &SqlitePool, id: i64) -> anyhow::Result<()> {
    sqlx::query("DELETE FROM history WHERE id = ?;")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}
