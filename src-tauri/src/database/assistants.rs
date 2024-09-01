use crate::database::schemas::Assistant;
use chrono::SecondsFormat;
use sqlx::SqlitePool;

pub async fn insert_assistant(pool: &SqlitePool, assistant: Assistant) -> anyhow::Result<i64> {
    let id = sqlx::query(
        r#"
        INSERT INTO assistants (name, instructions, config_id, created_at)
        VALUES (?, ?, ?, ?);
        "#,
    )
    .bind(assistant.name)
    .bind(assistant.instructions)
    .bind(assistant.config_id)
    .bind(
        assistant
            .created_at
            .to_rfc3339_opts(SecondsFormat::Secs, true),
    )
    .execute(pool)
    .await?
    .last_insert_rowid();
    Ok(id)
}

pub async fn get_assistants(pool: &SqlitePool) -> anyhow::Result<Vec<Assistant>> {
    let assistants = sqlx::query_as("SELECT * FROM assistants")
        .fetch_all(pool)
        .await?;
    Ok(assistants)
}

pub async fn get_assistant_by_id(pool: &SqlitePool, id: i64) -> anyhow::Result<Assistant> {
    let assistant =
        sqlx::query_as::<sqlx::Sqlite, Assistant>("SELECT * FROM assistants WHERE id = ?;")
            .bind(id)
            .fetch_one(pool)
            .await?;
    Ok(assistant)
}

pub async fn update_assistant(pool: &SqlitePool, assistant: Assistant) -> anyhow::Result<()> {
    sqlx::query(
        r#"
        UPDATE assistants
        SET name = ?, instructions = ?, config_id = ?
        WHERE id = ?;
        "#,
    )
    .bind(assistant.name)
    .bind(assistant.instructions)
    .bind(assistant.config_id)
    .bind(assistant.id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete_assistant(pool: &SqlitePool, id: i64) -> anyhow::Result<()> {
    sqlx::query("DELETE FROM assistants WHERE id = ?;")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
