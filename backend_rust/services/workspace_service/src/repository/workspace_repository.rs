use sqlx::{PgPool, query_as, FromRow};
use shared::types::{Workspace, WorkspaceType, WorkspaceMember, Role, InvitationStatus};
use uuid::Uuid;

// Internal struct for database mapping with sqlx::FromRow
#[derive(Debug, FromRow)]
struct WorkspaceRow {
    id: Uuid,
    name: String,
    #[sqlx(rename = "type")]
    workspace_type: String,
    owner_id: Uuid,
    logo_url: Option<String>,
    settings: serde_json::Value,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<WorkspaceRow> for Workspace {
    fn from(row: WorkspaceRow) -> Self {
        Workspace {
            id: row.id,
            name: row.name,
            workspace_type: match row.workspace_type.to_lowercase().as_str() {
                "personal" => WorkspaceType::Personal,
                "company" => WorkspaceType::Company,
                _ => WorkspaceType::Personal,
            },
            owner_id: row.owner_id,
            logo_url: row.logo_url,
            settings: row.settings,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

// Internal struct for workspace member database mapping
#[derive(Debug, FromRow)]
struct WorkspaceMemberRow {
    id: Uuid,
    workspace_id: Uuid,
    user_id: Uuid,
    role: String,
    invitation_status: String,
    invited_at: chrono::DateTime<chrono::Utc>,
    joined_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<WorkspaceMemberRow> for WorkspaceMember {
    fn from(row: WorkspaceMemberRow) -> Self {
        WorkspaceMember {
            id: row.id,
            workspace_id: row.workspace_id,
            user_id: row.user_id,
            role: match row.role.to_lowercase().as_str() {
                "owner" => Role::Owner,
                "admin" => Role::Admin,
                "member" => Role::Member,
                _ => Role::Member,
            },
            invitation_status: match row.invitation_status.to_lowercase().as_str() {
                "pending" => InvitationStatus::Pending,
                "accepted" => InvitationStatus::Accepted,
                "declined" => InvitationStatus::Declined,
                "expired" => InvitationStatus::Expired,
                _ => InvitationStatus::Pending,
            },
            invited_at: row.invited_at,
            joined_at: row.joined_at,
        }
    }
}

/// Find a workspace by ID
pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Workspace>, sqlx::Error> {
    sqlx::query_as::<_, WorkspaceRow>(
        r#"
        SELECT id, name, type, owner_id, logo_url, settings, created_at, updated_at
        FROM workspaces
        WHERE id = $1
        "#
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map(|opt| opt.map(|row| row.into()))
}

/// Find all workspaces accessible by a user (owned or member of)
pub async fn find_by_user_id(pool: &PgPool, user_id: Uuid) -> Result<Vec<Workspace>, sqlx::Error> {
    sqlx::query_as::<_, WorkspaceRow>(
        r#"
        SELECT DISTINCT w.id, w.name, w.type, w.owner_id, w.logo_url, w.settings, w.created_at, w.updated_at
        FROM workspaces w
        LEFT JOIN workspace_members wm ON w.id = wm.workspace_id
        WHERE w.owner_id = $1 OR wm.user_id = $1
        "#
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .map(|rows| rows.into_iter().map(|row| row.into()).collect())
}

/// Find personal workspace for a user
pub async fn find_personal_workspace(pool: &PgPool, user_id: Uuid) -> Result<Option<Workspace>, sqlx::Error> {
    sqlx::query_as::<_, WorkspaceRow>(
        r#"
        SELECT id, name, type, owner_id, logo_url, settings, created_at, updated_at
        FROM workspaces
        WHERE owner_id = $1 AND type = 'personal'
        "#
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map(|opt| opt.map(|row| row.into()))
}

/// Create a new workspace
pub async fn create(
    pool: &PgPool,
    name: &str,
    workspace_type: WorkspaceType,
    owner_id: Uuid,
    logo_url: Option<&str>,
) -> Result<Workspace, sqlx::Error> {
    let type_str = match workspace_type {
        WorkspaceType::Personal => "personal",
        WorkspaceType::Company => "company",
    };
    
    sqlx::query_as::<_, WorkspaceRow>(
        r#"
        INSERT INTO workspaces (name, type, owner_id, logo_url)
        VALUES ($1, $2, $3, $4)
        RETURNING id, name, type, owner_id, logo_url, settings, created_at, updated_at
        "#
    )
    .bind(name)
    .bind(type_str)
    .bind(owner_id)
    .bind(logo_url)
    .fetch_one(pool)
    .await
    .map(|row| row.into())
}

/// Update workspace details
pub async fn update(
    pool: &PgPool,
    id: Uuid,
    name: Option<&str>,
    logo_url: Option<&str>,
    settings: Option<&serde_json::Value>,
) -> Result<Workspace, sqlx::Error> {
    sqlx::query_as::<_, WorkspaceRow>(
        r#"
        UPDATE workspaces
        SET name = COALESCE($2, name),
            logo_url = COALESCE($3, logo_url),
            settings = COALESCE($4, settings),
            updated_at = NOW()
        WHERE id = $1
        RETURNING id, name, type, owner_id, logo_url, settings, created_at, updated_at
        "#
    )
    .bind(id)
    .bind(name)
    .bind(logo_url)
    .bind(settings)
    .fetch_one(pool)
    .await
    .map(|row| row.into())
}

/// Delete a workspace
pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        DELETE FROM workspaces
        WHERE id = $1
        "#
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Check if user is workspace owner
pub async fn is_owner(pool: &PgPool, workspace_id: Uuid, user_id: Uuid) -> Result<bool, sqlx::Error> {
    #[derive(Debug, FromRow)]
    struct CountResult {
        count: Option<i64>,
    }
    
    let result = sqlx::query_as::<_, CountResult>(
        r#"
        SELECT COUNT(*) as count
        FROM workspaces
        WHERE id = $1 AND owner_id = $2
        "#
    )
    .bind(workspace_id)
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(result.count.unwrap_or(0) > 0)
}

/// Get user's role in a workspace
pub async fn get_user_role(pool: &PgPool, workspace_id: Uuid, user_id: Uuid) -> Result<Option<Role>, sqlx::Error> {
    #[derive(Debug, FromRow)]
    struct OwnerResult {
        owner_id: Uuid,
    }
    
    // First check if user is owner
    let owner_result = sqlx::query_as::<_, OwnerResult>(
        r#"
        SELECT owner_id FROM workspaces WHERE id = $1
        "#
    )
    .bind(workspace_id)
    .fetch_optional(pool)
    .await?;

    if let Some(row) = owner_result {
        if row.owner_id == user_id {
            return Ok(Some(Role::Owner));
        }
    }

    // Check workspace_members for role
    #[derive(Debug, FromRow)]
    struct RoleResult {
        role: String,
    }
    
    let result = sqlx::query_as::<_, RoleResult>(
        r#"
        SELECT role
        FROM workspace_members
        WHERE workspace_id = $1 AND user_id = $2 AND invitation_status = 'accepted'
        "#
    )
    .bind(workspace_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    Ok(result.map(|r| match r.role.to_lowercase().as_str() {
        "owner" => Role::Owner,
        "admin" => Role::Admin,
        "member" => Role::Member,
        _ => Role::Member,
    }))
}

// Workspace Member operations

/// Find workspace member by ID
pub async fn find_member_by_id(pool: &PgPool, workspace_id: Uuid, user_id: Uuid) -> Result<Option<WorkspaceMember>, sqlx::Error> {
    sqlx::query_as::<_, WorkspaceMemberRow>(
        r#"
        SELECT id, workspace_id, user_id, role, invitation_status, invited_at, joined_at
        FROM workspace_members
        WHERE workspace_id = $1 AND user_id = $2
        "#
    )
    .bind(workspace_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map(|opt| opt.map(|row| row.into()))
}

/// List all members of a workspace
pub async fn list_members(pool: &PgPool, workspace_id: Uuid) -> Result<Vec<WorkspaceMember>, sqlx::Error> {
    sqlx::query_as::<_, WorkspaceMemberRow>(
        r#"
        SELECT id, workspace_id, user_id, role, invitation_status, invited_at, joined_at
        FROM workspace_members
        WHERE workspace_id = $1
        ORDER BY joined_at DESC NULLS LAST, invited_at DESC
        "#
    )
    .bind(workspace_id)
    .fetch_all(pool)
    .await
    .map(|rows| rows.into_iter().map(|row| row.into()).collect())
}

/// Count members in a workspace
pub async fn count_members(pool: &PgPool, workspace_id: Uuid) -> Result<i64, sqlx::Error> {
    #[derive(Debug, FromRow)]
    struct CountResult {
        count: Option<i64>,
    }
    
    let result = sqlx::query_as::<_, CountResult>(
        r#"
        SELECT COUNT(*) as count
        FROM workspace_members
        WHERE workspace_id = $1 AND invitation_status = 'accepted'
        "#
    )
    .bind(workspace_id)
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
    let role_str = match role {
        Role::Owner => "owner",
        Role::Admin => "admin",
        Role::Member => "member",
    };
    
    sqlx::query_as::<_, WorkspaceMemberRow>(
        r#"
        INSERT INTO workspace_members (workspace_id, user_id, role, invitation_status)
        VALUES ($1, $2, $3, 'pending')
        RETURNING id, workspace_id, user_id, role, invitation_status, invited_at, joined_at
        "#
    )
    .bind(workspace_id)
    .bind(user_id)
    .bind(role_str)
    .fetch_one(pool)
    .await
    .map(|row| row.into())
}

/// Accept an invitation
pub async fn accept_invitation(pool: &PgPool, workspace_id: Uuid, user_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE workspace_members
        SET invitation_status = 'accepted', joined_at = NOW()
        WHERE workspace_id = $1 AND user_id = $2
        "#
    )
    .bind(workspace_id)
    .bind(user_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Decline an invitation
pub async fn decline_invitation(pool: &PgPool, workspace_id: Uuid, user_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE workspace_members
        SET invitation_status = 'declined'
        WHERE workspace_id = $1 AND user_id = $2
        "#
    )
    .bind(workspace_id)
    .bind(user_id)
    .execute(pool)
    .await?;

    Ok(())
}

/// Mark expired invitations
pub async fn mark_expired_invitations(pool: &PgPool) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
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
    sqlx::query(
        r#"
        DELETE FROM workspace_members
        WHERE workspace_id = $1 AND user_id = $2
        "#
    )
    .bind(workspace_id)
    .bind(user_id)
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
    let role_str = match role {
        Role::Owner => "owner",
        Role::Admin => "admin",
        Role::Member => "member",
    };
    
    sqlx::query_as::<_, WorkspaceMemberRow>(
        r#"
        UPDATE workspace_members
        SET role = $3
        WHERE workspace_id = $1 AND user_id = $2
        RETURNING id, workspace_id, user_id, role, invitation_status, invited_at, joined_at
        "#
    )
    .bind(workspace_id)
    .bind(user_id)
    .bind(role_str)
    .fetch_one(pool)
    .await
    .map(|row| row.into())
}

// User lookup (temporary - should use user service client in production)

/// User info for lookups
#[derive(Debug, FromRow)]
pub struct UserInfo {
    pub id: Uuid,
    pub email: String,
}

/// Find user by email
pub async fn find_user_by_email(pool: &PgPool, email: &str) -> Result<Option<UserInfo>, sqlx::Error> {
    sqlx::query_as::<_, UserInfo>(
        r#"
        SELECT id, email
        FROM users
        WHERE email = $1
        "#
    )
    .bind(email)
    .fetch_optional(pool)
    .await
}
