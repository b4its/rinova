use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;
use shared::types::{WorkspaceType, Role};

/// Request to create a new workspace
#[derive(Debug, Deserialize, Validate)]
pub struct CreateWorkspaceRequest {
    #[validate(length(min = 1, max = 255, message = "Name must be between 1 and 255 characters"))]
    pub name: String,
    pub workspace_type: Option<String>,
    pub logo_url: Option<String>,
}

/// Request to update a workspace
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateWorkspaceRequest {
    #[validate(length(min = 1, max = 255, message = "Name must be between 1 and 255 characters"))]
    pub name: Option<String>,
    pub logo_url: Option<String>,
    pub settings: Option<serde_json::Value>,
}

/// Request to invite a member to a workspace
#[derive(Debug, Deserialize, Validate)]
pub struct InviteMemberRequest {
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    pub role: Option<String>,
}

/// Request to join a workspace
#[derive(Debug, Deserialize)]
pub struct JoinWorkspaceRequest {
    pub invitation_token: String,
}

/// Workspace with member count and current user's role
#[derive(Debug, Serialize)]
pub struct WorkspaceWithRole {
    pub id: Uuid,
    pub name: String,
    #[serde(rename = "type")]
    pub workspace_type: WorkspaceType,
    pub owner_id: Uuid,
    pub logo_url: Option<String>,
    pub settings: serde_json::Value,
    pub member_count: i64,
    pub current_user_role: Role,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Workspace invitation with expiration info
#[derive(Debug, Serialize)]
pub struct WorkspaceInvitation {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub workspace_name: String,
    pub email: String,
    pub role: Role,
    pub invited_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub is_expired: bool,
}

/// JWT Claims for workspace context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceClaims {
    pub sub: String,
    pub email: String,
    pub exp: usize,
    pub iat: usize,
    pub workspace_id: Option<String>,
    pub role: Option<String>,
}
