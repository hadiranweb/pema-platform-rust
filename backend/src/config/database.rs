use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;
use crate::config::settings::Settings;

pub async fn establish_connection(settings: &Settings) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(settings.database.max_connections)
        .min_connections(settings.database.min_connections)
        .acquire_timeout(Duration::from_secs(settings.database.acquire_timeout_seconds))
        .connect(&settings.database.url)
        .await
}

