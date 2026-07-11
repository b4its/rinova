use sqlx::{PgPool, query_as};
use shared::types::{Workspace, WorkspaceType, WorkspaceMember, Role, InvitationStatus};
use uuid::Uuid;

/// Find a workspace by ID
pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Workspace>, sqlx::Error> {
    query_as!(
        Workspace,
        r#"
        SELECT id, name, type as "workspace_type: WorkspaceType", owner_id, logo_url, 
               settings, created_at, updated_at
        FROM workspaces
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}

/// Find all workspaces accessible by a user (owned or member of)
pub async fn find_by_user_id(pool: &PgPool, user_id: Uuid) -> Result<Vec<Workspace>, sqlx::Error> {
    query_as!(
        Workspace,
        r#"
        SELECT DISTINCT w.id, w.name, w.type as "workspace_type: WorkspaceType", 
               w.owner_id, w.logo_url, w.settings, w.created_at, w.updated_at
        FROM workspaces w
        LEFT JOIN workspace_members wm ON w.id = wm.workspace_id
        WHERE w.owner_id = $1 OR wm.user_id = $1
        "#,
        user_id
    )
    .fetch_all(pool)
    .await
}

/// Find personal workspace for a user
pub async fn find_personal_workspace(pool: &PgPool, user_id: Uuid) -> Result<Option<Workspace>, sqlx::Error> {
    query_as!(
        Workspace,
        r#"
        SELECT id, name, type as "workspace_type: WorkspaceType", owner_id, logo_url, 
               settings, created_at, updated_at
        FROM workspaces
        WHERE owner_id = $1 AND type = 'personal'
        "#,
        user_id
    )
    .fetch_optional(pool)
    .await
}

/// Create a new workspace
pub async fn create(
    pool: &PgPool,
    name: &str,
    workspace_type: WorkspaceType,
    owner_id: Uuid,
    logo_url: Option<&str>,
) -> Result<Workspace, sqlx::Error> {
    query_as!(
        Workspace,
        r#"
        INSERT INTO workspaces (name, type, owner_id, logo_url)
        VALUES ($1, $2, $3, $4)
        RETURNING id, name, type as "workspace_type: WorkspaceType", owner_id, logo_url, 
                  settings, created_at, updated_at
        "#,
        name,
        workspace_type as _,
        owner_id,
        logo_url
    )
    .fetch_one(pool)
    .await
}

/// Update workspace details
pub async fn update(
    pool: &PgPool,
    id: Uuid,
    name: Option<&str>,
    logo_url: Option<&str>,
    settings: Option<&serde_json::Value>,
) -> Result<Workspace, sqlx::Error> {
    query_as!(
        Workspace,
        r#"
        UPDATE workspaces
        SET name = COALESCE($2, name),
            logo_url = COALESCE($3, logo_url),
            settings = COALESCE($4, settings),
            updated_at = NOW()
        WHERE id = $1
        RETURNING id, name, type as "workspace_type: WorkspaceType", owner_id, logo_url, 
                  settings, created_at, updated_at
        "#,
        id,
        name,
        logo_url,
        settings
    )
    .fetch_one(pool)
    .await
}

/// Delete a workspace
pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM workspaces
        WHERE id = $1
        "#,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Check if user is workspace owner
pub async fn is_owner(pool: &PgPool, workspace_id: Uuid, user_id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        SELECT COUNT(*) as count
        FROM workspaces
        WHERE id = $1 AND owner_id = $2
        "#,
        workspace_id,
        user_id
    )
    .fetch_one(pool)
    .await?;

    Ok(result.count.unwrap_or(0) > 0)
}

/// Get user's role in a workspace
pub async fn get_user_role(pool: &PgPool, workspace_id: Uuid, user_id: Uuid) -> Result<Option<Role>, sqlx::Error> {
    // First check if user is owner
    let owner_result = sqlx::query!(
        r#"
        SELECT owner_id FROM workspaces WHERE id = $1
        "#,
        workspace_id
    )
    .fetch_optional(pool)
    .await?;

    if let Some(row) = owner_result {
        if row.owner_id == user_id {
            return Ok(Some(Role::Owner));
        }
    }

    // Check workspace_members for role
    let result = sqlx::query!(
        r#"
        SELECT role as "role: Role"
        FROM workspace_members
        WHERE workspace_id = $1 AND user_id = $2 AND invitation_status = 'accepted'
        "#,
        workspace_id,
        user_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(result.map(|r| r.role))
}

// Workspace Member operations

/// Find workspace member by ID
pub async fn find_member_by_id(pool: &PgPool, workspace_id: Uuid, user_id: Uuid) -> Result<Option<WorkspaceMember>, sqlx::Error> {
    query_as!(
        WorkspaceMember,
        r#"
        SELECT id, workspace_id, user_id, role as "role: Role",
               invitation_status as "invitation_status: InvitationStatus",
               invited_at, joined_at
        FROM workspace_members
        WHERE workspace_id = $1 AND user_id = $2
        "#,
        workspace_id,
        user_id
    )
    .fetch_optional(pool)
    .await
}

/// List all members of a workspace
pub async fn list_members(pool: &PgPool, workspace_id: Uuid) -> Result<Vec<WorkspaceMember>, sqlx::Error> {
    query_as!(
        WorkspaceMember,
        r#"
        SELECT id, workspace_id, user_id, role as "role: Role",
               invitation_status as "invitation_status: InvitationStatus",
               invited_at, joined_at
        FROM workspace_members
        WHERE workspace_id = $1
        ORDER BY joined_at DESC NULLS LAST, invited_at DESC
        "#,
        workspace_id
    )
    .fetch_all(pool)
    .await
}

/// Count members in a workspace
pub async fn count_members(pool: &PgPool, workspace_id: Uuid) -> Result<i64, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        SELECT COUNT(*) as count
        FROM workspace_members
        WHERE workspace_id = $1 AND invitation_status = 'accepted'
        "#,
        workspace_id
    )
    .fetch_one(pool)
    .await?;

    Ok(result.count.unwrap_or(0))
}

/// Create a workspace member invitation
pub async fn create_invitation(
    pool: &PgPool,
    workspace_id: Uuid,
    user_id: Uuid,
    role: Role,
) -> Result<WorkspaceMember, sqlx::Error> {
    query_as!(
        WorkspaceMember,
        r#"
        INSERT INTO workspace_members (workspace_id, user_id, role, invitation_status)
        VALUES ($1, $2, $3, 'pending')
        RETURNING id, workspace_id, user_id, role as "role: Role",
                  invitation_status as "invitation_status: InvitationStatus",
                  invited_at, joined_at
        "#,
        workspace_id,
        user_id,
        role as _
    )
    .fetch_one(pool)
    .await
}

/// Accept an invitation
pub async fn accept_invitation(pool: &PgPool, workspace_id: Uuid, user_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE workspace_members
        SET invitation_status = 'accepted', joined_at = NOW()
        WHERE workspace_id = $1 AND user_id = $2
        "#,
        workspace_id,
        user_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Decline an invitation
pub async fn decline_invitation(pool: &PgPool, workspace_id: Uuid, user_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE workspace_members
        SET invitation_status = 'declined'
        WHERE workspace_id = $1 AND user_id = $2
        "#,
        workspace_id,
        user_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Mark expired invitations
pub async fn mark_expired_invitations(pool: &PgPool) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        UPDATE workspace_members
        SET invitation_status = 'expired'
        WHERE invitation_status = 'pending'
          AND invited_at < NOW() - INTERVAL '7 days'
        "#
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}

/// Remove a member from workspace
pub async fn remove_member(pool: &PgPool, workspace_id: Uuid, user_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM workspace_members
        WHERE workspace_id = $1 AND user_id = $2
        "#,
        workspace_id,
        user_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Update member role
pub async fn update_member_role(
    pool: &PgPool,
    workspace_id: Uuid,
    user_id: Uuid,
    role: Role,
) -> Result<WorkspaceMember, sqlx::Error> {
    query_as!(
        WorkspaceMember,
        r#"
        UPDATE workspace_members
        SET role = $3
        WHERE workspace_id = $1 AND user_id = $2
        RETURNING id, workspace_id, user_id, role as "role: Role",
                  invitation_status as "invitation_status: InvitationStatus",
                  invited_at, joined_at
        "#,
        workspace_id,
        user_id,
        role as _
    )
    .fetch_one(pool)
    .await
}

// User lookup (temporary - should use user service client in production)

/// User info for lookups
pub struct UserInfo {
    pub id: Uuid,
    pub email: String,
}

/// Find user by email
pub async fn find_user_by_email(pool: &PgPool, email: &str) -> Result<Option<UserInfo>, sqlx::Error> {
    query_as!(
        UserInfo,
        r#"
        SELECT id, email
        FROM users
        WHERE email = $1
        "#,
        email
    )
    .fetch_optional(pool)
    .await
}
