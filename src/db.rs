use anyhow::Result;
use sqlx::{FromRow, PgPool};

#[derive(FromRow)]
struct Mode {
    pub mode: String,
}

pub(crate) async fn get(
    pool: &PgPool,
    guild_id: String,
    user_id: String,
) -> Result<String, sqlx::Error> {
    let row: Mode = sqlx::query_as("SELECT mode FROM modes WHERE guild_id = $1 AND user_id = $2")
        .bind(guild_id.clone())
        .bind(user_id.clone())
        .fetch_one(pool)
        .await?;

    Ok(row.mode)
}

pub(crate) async fn set(
    pool: &PgPool,
    mode: &str,
    guild_id: String,
    user_id: String,
) -> Result<String, sqlx::Error> {
    let row: Mode = sqlx::query_as("SELECT mode FROM modes WHERE guild_id = $1 AND user_id = $2")
        .bind(guild_id.clone())
        .bind(user_id.clone())
        .fetch_one(pool)
        .await?;
    if row.mode.is_empty() {
        sqlx::query("INSERT INTO modes (guild_id, user_id, mode) VALUES ($1, $2, $3)")
            .bind(guild_id)
            .bind(user_id)
            .bind(mode)
            .execute(pool)
            .await?;
    } else {
        sqlx::query("UPDATE modes SET mode = $1 WHERE guild_id = $2, user_id = $3")
            .bind(mode)
            .bind(guild_id)
            .bind(user_id)
            .execute(pool)
            .await?;
    }

    Ok(mode.to_string())
}
