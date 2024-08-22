use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqliteRow, FromRow, Row};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Config {
    pub id: i64,
    pub temperature: f64,
    pub num_ctx: i64,
    pub frequency_penalty: f64,
    pub presence_penalty: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct History {
    pub id: i64,
    pub file_id: String,
    pub name: String,
    pub assistant_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Assistant {
    pub id: i64,
    pub name: String,
    pub instructions: String,
    pub config_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl<'r> FromRow<'r, SqliteRow> for Config {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get::<i64, _>(0)?,
            temperature: row.try_get::<f64, _>(1)?,
            num_ctx: row.try_get::<i64, _>(2)?,
            frequency_penalty: row.try_get::<f64, _>(3)?,
            presence_penalty: row.try_get::<f64, _>(4)?,
        })
    }
}

impl<'r> FromRow<'r, SqliteRow> for History {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        let created_at_str: String = row.try_get(4)?;
        let updated_at_str: String = row.try_get(5)?;
        let created_at = DateTime::parse_from_rfc3339(&created_at_str)
            .unwrap()
            .with_timezone(&Utc);
        let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)
            .unwrap()
            .with_timezone(&Utc);

        Ok(Self {
            id: row.try_get::<i64, _>(0)?,
            file_id: row.try_get::<String, _>(1)?,
            name: row.try_get::<String, _>(2)?,
            assistant_id: row.try_get::<i64, _>(3)?,
            created_at,
            updated_at,
        })
    }
}

impl<'r> FromRow<'r, SqliteRow> for Assistant {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        let created_at_str: String = row.try_get(4)?;
        let updated_at_str: String = row.try_get(5)?;
        let created_at = DateTime::parse_from_rfc3339(&created_at_str)
            .unwrap()
            .with_timezone(&Utc);
        let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)
            .unwrap()
            .with_timezone(&Utc);

        Ok(Self {
            id: row.try_get::<i64, _>(0)?,
            name: row.try_get::<String, _>(1)?,
            instructions: row.try_get::<String, _>(2)?,
            config_id: row.try_get::<i64, _>(3)?,
            created_at,
            updated_at,
        })
    }
}
