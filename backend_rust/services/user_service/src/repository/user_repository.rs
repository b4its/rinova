use sqlx::{PgPool, query_as};
use shared::types::{User, AccountType};
use uuid::Uuid;

pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
    query_as::<_, User>(
        r#"
        SELECT id, email, password_hash, full_name, 
               account_type,
               email_verified_at, created_at, updated_at
        FROM users
        WHERE email = $1
        "#
    )
    .bind(email)
    .fetch_optional(pool)
    .await
}

pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>, sqlx::Error> {
    query_as::<_, User>(
        r#"
        SELECT id, email, password_hash, full_name,
               account_type,
               email_verified_at, created_at, updated_at
        FROM users
        WHERE id = $1
        "#
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn create(
    pool: &PgPool,
    email: &str,
    password_hash: &str,
    account_type: AccountType,
) -> Result<User, sqlx::Error> {
    query_as::<_, User>(
        r#"
        INSERT INTO users (email, password_hash, account_type)
        VALUES ($1, $2, $3)
        RETURNING id, email, password_hash, full_name,
                  account_type,
                  email_verified_at, created_at, updated_at
        "#
    )
    .bind(email)
    .bind(password_hash)
    .bind(account_type.to_string())
    .fetch_one(pool)
    .await
}

pub async fn update(
    pool: &PgPool,
    id: Uuid,
    full_name: Option<&str>,
) -> Result<User, sqlx::Error> {
    query_as::<_, User>(
        r#"
        UPDATE users
        SET full_name = COALESCE($2, full_name)
        WHERE id = $1
        RETURNING id, email, password_hash, full_name,
                  account_type,
                  email_verified_at, created_at, updated_at
        "#
    )
    .bind(id)
    .bind(full_name)
    .fetch_one(pool)
    .await
}

pub async fn verify_email(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE users
        SET email_verified_at = NOW()
        WHERE id = $1
        "#
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn check_email_exists(pool: &PgPool, email: &str) -> Result<bool, sqlx::Error> {
    let result: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*) as count
        FROM users
        WHERE email = $1
        "#
    )
    .bind(email)
    .fetch_one(pool)
    .await?;

    Ok(result.0 > 0)
}

pub async fn create_personal_workspace(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Uuid, sqlx::Error> {
    let result: (Uuid,) = sqlx::query_as(
        r#"
        INSERT INTO workspaces (name, type, owner_id, settings)
        VALUES ('Personal Workspace', 'personal', $1, '{}')
        RETURNING id
        "#
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(result.0)
}
