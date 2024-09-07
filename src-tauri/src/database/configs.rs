use crate::database::schemas::Config;
use sqlx::SqlitePool;

pub async fn get_config_by_id(pool: &SqlitePool, id: i64) -> anyhow::Result<Config> {
    let config = sqlx::query_as::<sqlx::Sqlite, Config>("SELECT * FROM configs WHERE id = ?;")
        .bind(id)
        .fetch_one(pool)
        .await?;
    Ok(config)
}

pub async fn get_configs(pool: &SqlitePool) -> anyhow::Result<Vec<Config>> {
    let configs = sqlx::query_as::<sqlx::Sqlite, Config>("SELECT * FROM configs;")
        .fetch_all(pool)
        .await?;
    Ok(configs)
}

pub async fn insert_config(pool: &SqlitePool, config: Config) -> anyhow::Result<i64> {
    let id = sqlx::query("INSERT INTO configs (temperature, num_ctx, frequency_penalty, presence_penalty) VALUES (?, ?, ?, ?);")
        .bind(config.temperature)
        .bind(config.num_ctx)
        .bind(config.frequency_penalty)
        .bind(config.presence_penalty)
        .execute(pool)
        .await?
        .last_insert_rowid();
    Ok(id)
}

pub async fn update_config(pool: &SqlitePool, config: Config) -> anyhow::Result<()> {
    sqlx::query("UPDATE configs SET temperature = ?, num_ctx = ?, frequency_penalty = ?, presence_penalty = ? WHERE id = ?;")
        .bind(config.temperature)
        .bind(config.num_ctx)
        .bind(config.frequency_penalty)
        .bind(config.presence_penalty)
        .bind(config.id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete_config(pool: &SqlitePool, id: i64) -> anyhow::Result<()> {
    sqlx::query("DELETE FROM configs WHERE id = ?;")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
