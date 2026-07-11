use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use validator::Validate;
use shared::errors::ServiceError;
use shared::types::{Workspace, WorkspaceType, Role, InvitationStatus};
use crate::{AppState, models::*, repository};
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct WorkspaceResponse {
    pub id: Uuid,
    pub name: String,
    #[serde(rename = "type")]
    pub workspace_type: WorkspaceType,
    pub owner_id: Uuid,
    pub logo_url: Option<String>,
    pub settings: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<Workspace> for WorkspaceResponse {
    fn from(workspace: Workspace) -> Self {
        Self {
            id: workspace.id,
            name: workspace.name,
            workspace_type: workspace.workspace_type,
            owner_id: workspace.owner_id,
            logo_url: workspace.logo_url,
            settings: workspace.settings,
            created_at: workspace.created_at,
            updated_at: workspace.updated_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct WorkspaceListResponse {
    pub workspaces: Vec<WorkspaceWithRole>,
}

#[derive(Debug, Serialize)]
pub struct MessageResponse {
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct UserIdPath {
    pub id: Uuid,
    pub user_id: Uuid,
}

/// Health check endpoint
#[actix_web::get("/health")]
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "workspace_service"
    }))
}

/// List all workspaces accessible by the current user
pub async fn list_workspaces(
    state: web::Data<AppState>,
    query: web::Query<UserQuery>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = query.user_id
        .ok_or_else(|| ServiceError::AuthError("User ID is required".to_string()))?;

    let workspaces = repository::find_by_user_id(&state.db, user_id).await?;

    let workspaces_with_roles: Vec<WorkspaceWithRole> = futures::future::join_all(
        workspaces.into_iter().map(|w| {
            let db = state.db.clone();
            async move {
                let role = repository::get_user_role(&db, w.id, user_id).await
                    .unwrap_or(None)
                    .unwrap_or(Role::Member);
                let member_count = repository::count_members(&db, w.id).await.unwrap_or(1);

                WorkspaceWithRole {
                    id: w.id,
                    name: w.name,
                    workspace_type: w.workspace_type,
                    owner_id: w.owner_id,
                    logo_url: w.logo_url,
                    settings: w.settings,
                    member_count,
                    current_user_role: role,
                    created_at: w.created_at,
                    updated_at: w.updated_at,
                }
            }
        })
    ).await;

    Ok(HttpResponse::Ok().json(WorkspaceListResponse {
        workspaces: workspaces_with_roles,
    }))
}

#[derive(Debug, Deserialize)]
pub struct UserQuery {
    pub user_id: Option<Uuid>,
}

/// Get a specific workspace by ID
pub async fn get_workspace(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    query: web::Query<UserQuery>,
) -> Result<HttpResponse, ServiceError> {
    let workspace_id = path.into_inner();
    let user_id = query.user_id
        .ok_or_else(|| ServiceError::AuthError("User ID is required".to_string()))?;

    // Check access
    let role = repository::get_user_role(&state.db, workspace_id, user_id).await?
        .ok_or_else(|| ServiceError::ForbiddenError("Access denied to this workspace".to_string()))?;

    let workspace = repository::find_by_id(&state.db, workspace_id).await?
        .ok_or_else(|| ServiceError::NotFoundError("Workspace not found".to_string()))?;

    let member_count = repository::count_members(&state.db, workspace_id).await.unwrap_or(1);

    Ok(HttpResponse::Ok().json(WorkspaceWithRole {
        id: workspace.id,
        name: workspace.name,
        workspace_type: workspace.workspace_type,
        owner_id: workspace.owner_id,
        logo_url: workspace.logo_url,
        settings: workspace.settings,
        member_count,
        current_user_role: role,
        created_at: workspace.created_at,
        updated_at: workspace.updated_at,
    }))
}

/// Create a new workspace
pub async fn create_workspace(
    state: web::Data<AppState>,
    body: web::Json<CreateWorkspaceRequest>,
    query: web::Query<UserQuery>,
) -> Result<HttpResponse, ServiceError> {
    body.validate()
        .map_err(|e| ServiceError::ValidationError(e.to_string()))?;

    let user_id = query.user_id
        .ok_or_else(|| ServiceError::AuthError("User ID is required".to_string()))?;

    let workspace_type = match body.workspace_type.as_deref() {
        Some("company") => WorkspaceType::Company,
        _ => WorkspaceType::Personal,
    };

    let workspace = repository::create(
        &state.db,
        &body.name,
        workspace_type.clone(),
        user_id,
        body.logo_url.as_deref(),
    ).await?;

    // Publish workspace created event
    state.zmq_publisher.publish("workspace.created", serde_json::json!({
        "workspace_id": workspace.id.to_string(),
        "owner_id": user_id.to_string(),
        "name": &body.name,
        "type": workspace_type,
    })).await;

    Ok(HttpResponse::Created().json(WorkspaceResponse::from(workspace)))
}

/// Update a workspace
pub async fn update_workspace(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateWorkspaceRequest>,
    query: web::Query<UserQuery>,
) -> Result<HttpResponse, ServiceError> {
    body.validate()
        .map_err(|e| ServiceError::ValidationError(e.to_string()))?;

    let workspace_id = path.into_inner();
    let user_id = query.user_id
        .ok_or_else(|| ServiceError::AuthError("User ID is required".to_string()))?;

    // Check if user has permission (owner or admin)
    let role = repository::get_user_role(&state.db, workspace_id, user_id).await?
        .ok_or_else(|| ServiceError::ForbiddenError("Access denied to this workspace".to_string()))?;

    if !role.can_manage_members() {
        return Err(ServiceError::ForbiddenError("You don't have permission to update this workspace".to_string()));
    }

    let workspace = repository::update(
        &state.db,
        workspace_id,
        body.name.as_deref(),
        body.logo_url.as_deref(),
        body.settings.as_ref(),
    ).await?;

    Ok(HttpResponse::Ok().json(WorkspaceResponse::from(workspace)))
}

/// Delete a workspace
pub async fn delete_workspace(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    query: web::Query<UserQuery>,
) -> Result<HttpResponse, ServiceError> {
    let workspace_id = path.into_inner();
    let user_id = query.user_id
        .ok_or_else(|| ServiceError::AuthError("User ID is required".to_string()))?;

    // Only owner can delete workspace
    let is_owner = repository::is_owner(&state.db, workspace_id, user_id).await?;
    if !is_owner {
        return Err(ServiceError::ForbiddenError("Only workspace owner can delete the workspace".to_string()));
    }

    repository::delete(&state.db, workspace_id).await?;

    // Publish workspace deleted event
    state.zmq_publisher.publish("workspace.deleted", serde_json::json!({
        "workspace_id": workspace_id.to_string(),
    })).await;

    Ok(HttpResponse::Ok().json(MessageResponse {
        message: "Workspace deleted successfully".to_string(),
    }))
}

/// Invite a member to workspace
pub async fn invite_member(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: web::Json<InviteMemberRequest>,
    query: web::Query<UserQuery>,
) -> Result<HttpResponse, ServiceError> {
    body.validate()
        .map_err(|e| ServiceError::ValidationError(e.to_string()))?;

    let workspace_id = path.into_inner();
    let inviter_id = query.user_id
        .ok_or_else(|| ServiceError::AuthError("User ID is required".to_string()))?;

    // Check if inviter has permission
    let role = repository::get_user_role(&state.db, workspace_id, inviter_id).await?
        .ok_or_else(|| ServiceError::ForbiddenError("Access denied to this workspace".to_string()))?;

    if !role.can_manage_members() {
        return Err(ServiceError::ForbiddenError("You don't have permission to invite members".to_string()));
    }

    // Get invited user ID by email
    let invited_user = repository::find_user_by_email(&state.db, &body.email).await?
        .ok_or_else(|| ServiceError::NotFoundError("User not found".to_string()))?;

    // Check if already a member
    if repository::find_member_by_id(&state.db, workspace_id, invited_user.id).await?.is_some() {
        return Err(ServiceError::ValidationError("User is already a member or has a pending invitation".to_string()));
    }

    let invited_role = match body.role.as_deref() {
        Some("admin") => Role::Admin,
        Some("member") => Role::Member,
        _ => Role::Member,
    };

    let _member = repository::create_invitation(
        &state.db,
        workspace_id,
        invited_user.id,
        invited_role.clone(),
    ).await?;

    // Publish member invited event
    state.zmq_publisher.publish("workspace.member_invited", serde_json::json!({
        "workspace_id": workspace_id.to_string(),
        "user_id": invited_user.id.to_string(),
        "email": &body.email,
        "role": invited_role,
    })).await;

    Ok(HttpResponse::Created().json(MessageResponse {
        message: "Invitation sent successfully".to_string(),
    }))
}

/// Join a workspace (accept invitation)
pub async fn join_workspace(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    _body: web::Json<JoinWorkspaceRequest>,
    query: web::Query<UserQuery>,
) -> Result<HttpResponse, ServiceError> {
    let workspace_id = path.into_inner();
    let user_id = query.user_id
        .ok_or_else(|| ServiceError::AuthError("User ID is required".to_string()))?;

    // Verify invitation exists and is pending
    let member = repository::find_member_by_id(&state.db, workspace_id, user_id).await?
        .ok_or_else(|| ServiceError::NotFoundError("No pending invitation found".to_string()))?;

    if member.invitation_status != InvitationStatus::Pending {
        return Err(ServiceError::ValidationError("Invitation is no longer valid".to_string()));
    }

    // Accept invitation
    repository::accept_invitation(&state.db, workspace_id, user_id).await?;

    // Publish member joined event
    state.zmq_publisher.publish("workspace.member_joined", serde_json::json!({
        "workspace_id": workspace_id.to_string(),
        "user_id": user_id.to_string(),
    })).await;

    Ok(HttpResponse::Ok().json(MessageResponse {
        message: "Successfully joined workspace".to_string(),
    }))
}

/// List members of a workspace
pub async fn list_members(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    query: web::Query<UserQuery>,
) -> Result<HttpResponse, ServiceError> {
    let workspace_id = path.into_inner();
    let user_id = query.user_id
        .ok_or_else(|| ServiceError::AuthError("User ID is required".to_string()))?;

    // Check access
    let _ = repository::get_user_role(&state.db, workspace_id, user_id).await?
        .ok_or_else(|| ServiceError::ForbiddenError("Access denied to this workspace".to_string()))?;

    let members = repository::list_members(&state.db, workspace_id).await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "members": members
    })))
}

/// Remove a member from workspace
pub async fn remove_member(
    state: web::Data<AppState>,
    path: web::Path<UserIdPath>,
    query: web::Query<UserQuery>,
) -> Result<HttpResponse, ServiceError> {
    let paths = path.into_inner();
    let workspace_id = paths.id;
    let member_user_id = paths.user_id;
    let requester_id = query.user_id
        .ok_or_else(|| ServiceError::AuthError("User ID is required".to_string()))?;

    // Check if requester has permission
    let role = repository::get_user_role(&state.db, workspace_id, requester_id).await?
        .ok_or_else(|| ServiceError::ForbiddenError("Access denied to this workspace".to_string()))?;

    // Can't remove owner
    if repository::is_owner(&state.db, workspace_id, member_user_id).await? {
        return Err(ServiceError::ForbiddenError("Cannot remove the workspace owner".to_string()));
    }

    // Owner/admin can remove anyone, members can only remove themselves
    if !role.can_manage_members() && member_user_id != requester_id {
        return Err(ServiceError::ForbiddenError("You don't have permission to remove this member".to_string()));
    }

    repository::remove_member(&state.db, workspace_id, member_user_id).await?;

    // Publish member removed event
    state.zmq_publisher.publish("workspace.member_removed", serde_json::json!({
        "workspace_id": workspace_id.to_string(),
        "user_id": member_user_id.to_string(),
    })).await;

    Ok(HttpResponse::Ok().json(MessageResponse {
        message: "Member removed successfully".to_string(),
    }))
}
