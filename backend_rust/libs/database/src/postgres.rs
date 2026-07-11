use sqlx::postgres::{PgPoolOptions, PgPool};
use sqlx::migrate::Migrator;
use std::env;

pub type DbPool = PgPool;

/// Migrator for running database migrations
pub static MIGRATOR: Migrator = sqlx::migrate!();

/// Creates a database connection pool
pub async fn create_pool() -> Result<DbPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgPoolOptions::new()
        .max_connections(10)
        .min_connections(2)
        .connect(&database_url)
        .await
}

/// Creates a database connection pool with migrations
pub async fn create_pool_with_migrations() -> Result<DbPool, sqlx::Error> {
    let pool = create_pool().await?;
    
    tracing::info!("Running database migrations...");
    MIGRATOR.run(&pool).await?;
    tracing::info!("Database migrations completed successfully");
    
    Ok(pool)
}

/// Creates a test database connection pool
pub async fn create_test_pool() -> Result<DbPool, sqlx::Error> {
    let database_url = env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://rinova:rinova_secret@localhost:5432/rinova_test".to_string());

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}

/// Creates a test database connection pool with migrations
pub async fn create_test_pool_with_migrations() -> Result<DbPool, sqlx::Error> {
    let pool = create_test_pool().await?;
    
    tracing::info!("Running test database migrations...");
    MIGRATOR.run(&pool).await?;
    tracing::info!("Test database migrations completed successfully");
    
    Ok(pool)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migrator_loads() {
        // Verify the migrator is properly loaded
        let migrations: Vec<_> = MIGRATOR.iter().collect();
        assert!(!migrations.is_empty(), "Migrator should have migrations");
    }
}
