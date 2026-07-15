use sqlx::{PgPool, query_as};
use shared::types::{User, AccountType};
use uuid::Uuid;

pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
    query_as::<_, User>(
        r#"
        SELECT id, email, password_hash, full_name, 
               account_type, role,
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
               account_type, role,
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
                  account_type, role,
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
                  account_type, role,
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
    let mut tx = pool.begin().await?;

    let result: (Uuid,) = sqlx::query_as(
        r#"
        INSERT INTO workspaces (name, type, owner_id, settings)
        VALUES ('Personal Workspace', 'personal', $1, '{}')
        RETURNING id
        "#
    )
    .bind(user_id)
    .fetch_one(&mut *tx)
    .await?;

    let workspace_id = result.0;

    // Add owner as workspace member with 'owner' role
    sqlx::query(
        r#"
        INSERT INTO workspace_members (workspace_id, user_id, role, invitation_status, joined_at)
        VALUES ($1, $2, 'owner', 'accepted', NOW())
        ON CONFLICT (workspace_id, user_id) DO NOTHING
        "#
    )
    .bind(workspace_id)
    .bind(user_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(workspace_id)
}

pub async fn create_company_workspace(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Uuid, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let result: (Uuid,) = sqlx::query_as(
        r#"
        INSERT INTO workspaces (name, type, owner_id, settings)
        VALUES ('Company Workspace', 'company', $1, '{}')
        RETURNING id
        "#
    )
    .bind(user_id)
    .fetch_one(&mut *tx)
    .await?;

    let workspace_id = result.0;

    // Add owner as workspace member with 'owner' role
    sqlx::query(
        r#"
        INSERT INTO workspace_members (workspace_id, user_id, role, invitation_status, joined_at)
        VALUES ($1, $2, 'owner', 'accepted', NOW())
        ON CONFLICT (workspace_id, user_id) DO NOTHING
        "#
    )
    .bind(workspace_id)
    .bind(user_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(workspace_id)
}

pub async fn list_all(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    query_as::<_, User>(
        r#"
        SELECT id, email, password_hash, full_name,
               account_type, role,
               email_verified_at, created_at, updated_at
        FROM users
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(pool)
    .await
}

pub async fn update_role(
    pool: &PgPool,
    id: Uuid,
    role: &str,
) -> Result<User, sqlx::Error> {
    query_as::<_, User>(
        r#"
        UPDATE users
        SET role = $2
        WHERE id = $1
        RETURNING id, email, password_hash, full_name,
                  account_type, role,
                  email_verified_at, created_at, updated_at
        "#
    )
    .bind(id)
    .bind(role)
    .fetch_one(pool)
    .await
}

pub async fn delete_user(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        r#"
        DELETE FROM users
        WHERE id = $1
        "#
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

/// Get the user's personal subscription plan (where workspace_id IS NULL)
pub async fn get_personal_plan(pool: &PgPool, user_id: Uuid) -> Result<Option<String>, sqlx::Error> {
    let plan: Option<(String,)> = query_as(
        r#"
        SELECT plan_type FROM subscriptions
        WHERE user_id = $1 AND workspace_id IS NULL AND status = 'active'
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(plan.map(|(p,)| p))
}

/// Get the highest workspace subscription plan for a user
pub async fn get_highest_workspace_plan(pool: &PgPool, user_id: Uuid) -> Result<Option<String>, sqlx::Error> {
    let plan: Option<(String,)> = query_as(
        r#"
        SELECT s.plan_type
        FROM workspace_members wm
        JOIN subscriptions s ON s.workspace_id = wm.workspace_id
        WHERE wm.user_id = $1
          AND wm.invitation_status = 'accepted'
          AND s.status = 'active'
        ORDER BY
            CASE s.plan_type
                WHEN 'exclusive' THEN 3
                WHEN 'enterprise' THEN 2
                WHEN 'freemium' THEN 1
            END DESC
        LIMIT 1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(plan.map(|(p,)| p))
}
