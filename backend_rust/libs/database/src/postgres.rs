use sqlx::postgres::{PgPoolOptions, PgPool};
use std::env;

pub type DbPool = PgPool;

pub async fn create_pool() -> Result<DbPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgPoolOptions::new()
        .max_connections(10)
        .min_connections(2)
        .connect(&database_url)
        .await
}

pub async fn create_test_pool() -> Result<DbPool, sqlx::Error> {
    let database_url = env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://rinova:rinova_secret@localhost:5432/rinova_test".to_string());

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}
