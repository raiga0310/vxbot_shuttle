use anyhow::Result;
use sqlx::{FromRow, PgPool};
use tracing::info;

#[derive(FromRow, Debug)]
struct Mode {
    pub mode: String,
}

pub(crate) async fn get(
    pool: &PgPool,
    guild_id: &str,
    user_id: &str,
) -> Result<String, sqlx::Error> {
    let row: Option<Mode> =
        sqlx::query_as("SELECT mode FROM modes WHERE guild_id = $1 AND user_id = $2 LIMIT 1")
            .bind(guild_id)
            .bind(user_id)
            .fetch_optional(pool)
            .await?;
    Ok(row
        .unwrap_or_else(|| Mode {
            mode: "vx".to_string(),
        })
        .mode)
}

pub(crate) async fn set(
    pool: &PgPool,
    mode: &str,
    guild_id: &str,
    user_id: &str,
) -> Result<String, sqlx::Error> {
    info!("db insertion");
    let mut tx = pool.begin().await?;
    sqlx::query(
        r#"INSERT INTO modes 
        (guild_id, user_id, mode) 
        VALUES ($1, $2, $3) 
        ON CONFLICT (guild_id, user_id) 
        DO UPDATE
        SET mode = $3"#,
    )
    .bind(guild_id)
    .bind(user_id)
    .bind(mode)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    info!("after db insertion");

    Ok(mode.to_string())
}
