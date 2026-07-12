//! Project model and related types

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

use super::ProjectStatus;

/// Project model matching the database schema
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Project {
    /// Unique project ID
    pub id: Uuid,
    /// Workspace ID this project belongs to
    pub workspace_id: Uuid,
    /// User ID who owns this project
    pub owner_id: Uuid,
    /// Project name
    pub name: String,
    /// Project status
    pub status: ProjectStatus,
    /// Project metadata (theme, settings, etc.)
    pub metadata: serde_json::Value,
    /// When this project was last published
    pub last_published_at: Option<DateTime<Utc>>,
    /// When this project was created
    pub created_at: DateTime<Utc>,
    /// When this project was last updated
    pub updated_at: DateTime<Utc>,
}

impl Project {
    /// Create a new project
    pub fn new(workspace_id: Uuid, owner_id: Uuid, name: String) -> Self {
        let now = Utc::now();
        Project {
            id: Uuid::new_v4(),
            workspace_id,
            owner_id,
            name,
            status: ProjectStatus::Draft,
            metadata: serde_json::json!({}),
            last_published_at: None,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Request to create a new project
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateProjectRequest {
    /// Project name
    #[validate(length(min = 1, max = 255, message = "Project name must be 1-255 characters"))]
    pub name: String,
    /// Workspace ID
    pub workspace_id: Uuid,
    /// Initial project metadata
    #[serde(default)]
    pub metadata: Option<serde_json::Value>,
}

/// Request to update a project
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateProjectRequest {
    /// New project name
    #[validate(length(min = 1, max = 255, message = "Project name must be 1-255 characters"))]
    pub name: Option<String>,
    /// New project status
    pub status: Option<ProjectStatus>,
    /// New project metadata
    pub metadata: Option<serde_json::Value>,
}

/// Project list query parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectListQuery {
    /// Filter by workspace ID
    pub workspace_id: Option<Uuid>,
    /// Filter by status
    pub status: Option<ProjectStatus>,
    /// Page number (1-based)
    #[serde(default = "default_page")]
    pub page: i32,
    /// Page size
    #[serde(default = "default_page_size")]
    pub page_size: i32,
}

fn default_page() -> i32 {
    1
}

fn default_page_size() -> i32 {
    20
}

impl Default for ProjectListQuery {
    fn default() -> Self {
        ProjectListQuery {
            workspace_id: None,
            status: None,
            page: 1,
            page_size: 20,
        }
    }
}

/// Paginated project list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectListResponse {
    pub projects: Vec<Project>,
    pub total: i64,
    pub page: i32,
    pub page_size: i32,
    pub total_pages: i32,
}

/// Published site information
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PublishedSite {
    /// Unique published site ID
    pub id: Uuid,
    /// Project ID
    pub project_id: Uuid,
    /// Subdomain for the site
    pub subdomain: String,
    /// Custom domain (if configured)
    pub custom_domain: Option<String>,
    /// SSL certificate status
    pub ssl_status: String,
    /// When this site was published
    pub published_at: DateTime<Utc>,
}

impl PublishedSite {
    /// Get the live URL for this published site
    pub fn live_url(&self) -> String {
        if let Some(ref domain) = self.custom_domain {
            format!("https://{}", domain)
        } else {
            format!("https://{}.rinova.app", self.subdomain)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_new() {
        let workspace_id = Uuid::new_v4();
        let owner_id = Uuid::new_v4();
        let project = Project::new(workspace_id, owner_id, "Test Project".to_string());

        assert_eq!(project.name, "Test Project");
        assert_eq!(project.status, ProjectStatus::Draft);
        assert_eq!(project.workspace_id, workspace_id);
        assert_eq!(project.owner_id, owner_id);
    }

    #[test]
    fn test_published_site_live_url() {
        let site = PublishedSite {
            id: Uuid::new_v4(),
            project_id: Uuid::new_v4(),
            subdomain: "my-project".to_string(),
            custom_domain: None,
            ssl_status: "active".to_string(),
            published_at: Utc::now(),
        };

        assert_eq!(site.live_url(), "https://my-project.rinova.app");
    }

    #[test]
    fn test_published_site_custom_domain_url() {
        let site = PublishedSite {
            id: Uuid::new_v4(),
            project_id: Uuid::new_v4(),
            subdomain: "my-project".to_string(),
            custom_domain: Some("custom.com".to_string()),
            ssl_status: "active".to_string(),
            published_at: Utc::now(),
        };

        assert_eq!(site.live_url(), "https://custom.com");
    }
}
