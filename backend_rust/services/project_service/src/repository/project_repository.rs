//! Project repository for PostgreSQL operations

use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{Project, ProjectListQuery, ProjectListResponse, ProjectStatus, PublishedSite, UpdateProjectRequest};

/// Repository for project CRUD operations
pub struct ProjectRepository {
    pool: PgPool,
}

impl ProjectRepository {
    /// Create a new project repository
    pub fn new(pool: PgPool) -> Self {
        ProjectRepository { pool }
    }

    /// Create a new project
    pub async fn create(&self, workspace_id: Uuid, owner_id: Uuid, name: &str, metadata: Option<serde_json::Value>) -> Result<Project> {
        let metadata = metadata.unwrap_or(serde_json::json!({}));
        
        let project = sqlx::query_as::<_, Project>(
            r#"
            INSERT INTO projects (workspace_id, owner_id, name, status, metadata)
            VALUES ($1, $2, $3, 'draft', $4)
            RETURNING *
            "#,
        )
        .bind(workspace_id)
        .bind(owner_id)
        .bind(name)
        .bind(&metadata)
        .fetch_one(&self.pool)
        .await?;

        Ok(project)
    }

    /// Get project by ID
    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<Project>> {
        let project = sqlx::query_as::<_, Project>(
            "SELECT * FROM projects WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(project)
    }

    /// List projects with filtering and pagination
    pub async fn list(&self, query: ProjectListQuery, user_id: Uuid) -> Result<ProjectListResponse> {
        // Build the WHERE clause based on filters
        let mut conditions = vec![
            "p.owner_id = $1 OR EXISTS (
                SELECT 1 FROM workspace_members wm
                WHERE wm.workspace_id = p.workspace_id
                AND wm.user_id = $1
                AND wm.invitation_status = 'accepted'
            )".to_string(),
        ];

        let mut param_count = 2;

        if let Some(workspace_id) = query.workspace_id {
            conditions.push(format!("p.workspace_id = ${}", param_count));
            param_count += 1;
        }

        if let Some(status) = &query.status {
            conditions.push(format!("p.status = ${}", param_count));
            param_count += 1;
        }

        let where_clause = conditions.join(" AND ");

        // Count total
        let count_query = format!(
            "SELECT COUNT(*) FROM projects p WHERE {}",
            where_clause
        );

        let total: i64 = sqlx::query_scalar(&count_query)
            .bind(user_id)
            .bind(query.workspace_id)
            .bind(query.status.map(|s| s.to_string()))
            .fetch_one(&self.pool)
            .await?;

        // Calculate pagination
        let offset = (query.page - 1) * query.page_size;
        let total_pages = ((total as f64) / (query.page_size as f64)).ceil() as i32;

        // Fetch projects
        let data_query = format!(
            "SELECT p.* FROM projects p WHERE {} ORDER BY p.updated_at DESC LIMIT ${} OFFSET ${}",
            where_clause, param_count, param_count + 1
        );

        let projects = sqlx::query_as::<_, Project>(&data_query)
            .bind(user_id)
            .bind(query.workspace_id)
            .bind(query.status.map(|s| s.to_string()))
            .bind(query.page_size)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?;

        Ok(ProjectListResponse {
            projects,
            total,
            page: query.page,
            page_size: query.page_size,
            total_pages,
        })
    }

    /// Update a project
    pub async fn update(&self, id: Uuid, request: UpdateProjectRequest) -> Result<Project> {
        let mut updates = vec!["updated_at = NOW()"];
        let mut param_count = 2;

        if request.name.is_some() {
            updates.push(&format!("name = ${}", param_count));
            param_count += 1;
        }

        if request.status.is_some() {
            updates.push(&format!("status = ${}", param_count));
            param_count += 1;
        }

        if request.metadata.is_some() {
            updates.push(&format!("metadata = ${}", param_count));
            param_count += 1;
        }

        let set_clause = updates.join(", ");

        let query = format!(
            "UPDATE projects SET {} WHERE id = $1 RETURNING *",
            set_clause
        );

        let mut q = sqlx::query_as::<_, Project>(&query).bind(id);

        if let Some(ref name) = request.name {
            q = q.bind(name);
        }

        if let Some(status) = &request.status {
            q = q.bind(status.to_string());
        }

        if let Some(ref metadata) = request.metadata {
            q = q.bind(metadata);
        }

        let project = q.fetch_one(&self.pool).await?;

        Ok(project)
    }

    /// Delete a project
    pub async fn delete(&self, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM projects WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Archive a project
    pub async fn archive(&self, id: Uuid) -> Result<Project> {
        let project = sqlx::query_as::<_, Project>(
            r#"
            UPDATE projects
            SET status = 'archived', updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(project)
    }

    /// Get published site by project ID
    pub async fn get_published_site(&self, project_id: Uuid) -> Result<Option<PublishedSite>> {
        let site = sqlx::query_as::<_, PublishedSite>(
            "SELECT * FROM published_sites WHERE project_id = $1",
        )
        .bind(project_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(site)
    }

    /// Create published site
    pub async fn create_published_site(&self, project_id: Uuid, subdomain: String) -> Result<PublishedSite> {
        let site = sqlx::query_as::<_, PublishedSite>(
            r#"
            INSERT INTO published_sites (project_id, subdomain, ssl_status)
            VALUES ($1, $2, 'pending')
            RETURNING *
            "#,
        )
        .bind(project_id)
        .bind(&subdomain)
        .fetch_one(&self.pool)
        .await?;

        Ok(site)
    }

    /// Update project publish timestamp
    pub async fn mark_published(&self, id: Uuid) -> Result<Project> {
        let project = sqlx::query_as::<_, Project>(
            r#"
            UPDATE projects
            SET status = 'published', 
                last_published_at = NOW(), 
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(project)
    }

    /// Check if user has access to project
    pub async fn check_access(&self, project_id: Uuid, user_id: Uuid) -> Result<bool> {
        let has_access: bool = sqlx::query_scalar(
            r#"
            SELECT EXISTS (
                SELECT 1 FROM projects p
                WHERE p.id = $1
                AND (p.owner_id = $2 OR EXISTS (
                    SELECT 1 FROM workspace_members wm
                    WHERE wm.workspace_id = p.workspace_id
                    AND wm.user_id = $2
                    AND wm.invitation_status = 'accepted'
                ))
            )
            "#,
        )
        .bind(project_id)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(has_access)
    }

    /// Get active project count for a user in a workspace
    pub async fn get_active_project_count(&self, user_id: Uuid, workspace_id: Option<Uuid>) -> Result<i32> {
        let count: i64 = if let Some(ws_id) = workspace_id {
            sqlx::query_scalar(
                r#"
                SELECT COUNT(*) FROM projects p
                WHERE p.workspace_id = $1 
                AND p.owner_id = $2 
                AND p.status != 'archived'
                "#,
            )
            .bind(ws_id)
            .bind(user_id)
            .fetch_one(&self.pool)
            .await?
        } else {
            sqlx::query_scalar(
                r#"
                SELECT COUNT(*) FROM projects p
                WHERE p.owner_id = $1 
                AND p.status != 'archived'
                "#,
            )
            .bind(user_id)
            .fetch_one(&self.pool)
            .await?
        };

        Ok(count as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Integration tests require a test database
    // These would be implemented with sqlx::test macro
}
