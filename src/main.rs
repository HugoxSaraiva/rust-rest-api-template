use crate::{config::Config, http};
use anyhow::{Context, Ok};
use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_env()?;
    let db = PgPoolOptions::new()
        .max_connections(50)
        .connect(&db_url)
        .await
        .context("Failed to connect to DATABASE_URL")?;

    http::serve(config, db).await?;

    Ok(())
}
