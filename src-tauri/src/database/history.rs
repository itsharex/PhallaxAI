use crate::database::schemas::History;
use chrono::SecondsFormat;
use sqlx::SqlitePool;

pub async fn insert_history(pool: &SqlitePool, history: History) -> Result<(), sqlx::Error> {
    sqlx::query(
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

    Ok(())
}

pub async fn get_history(pool: &SqlitePool) -> Result<Vec<History>, sqlx::Error> {
    sqlx::query_as("SELECT * FROM history")
        .fetch_all(pool)
        .await
}

pub async fn get_history_by_id(pool: &SqlitePool, id: i64) -> Result<History, sqlx::Error> {
    sqlx::query_as::<sqlx::Sqlite, History>("SELECT * FROM history WHERE id = ?;")
        .bind(id)
        .fetch_one(pool)
        .await
}

pub async fn update_history(pool: &SqlitePool, history: History) -> Result<(), sqlx::Error> {
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

pub async fn delete_history(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM history WHERE id = ?;")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}
