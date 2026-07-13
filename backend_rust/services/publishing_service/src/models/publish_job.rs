//! Publish job models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Publish job status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum PublishStatus {
    /// Job is pending
    Pending,
    /// Job is being processed
    Building,
    /// Job is uploading
    Uploading,
    /// Job completed successfully
    Completed,
    /// Job failed
    Failed,
}

impl std::fmt::Display for PublishStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PublishStatus::Pending => write!(f, "pending"),
            PublishStatus::Building => write!(f, "building"),
            PublishStatus::Uploading => write!(f, "uploading"),
            PublishStatus::Completed => write!(f, "completed"),
            PublishStatus::Failed => write!(f, "failed"),
        }
    }
}

/// Publish job record
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PublishJob {
    /// Unique job ID
    pub id: Uuid,
    /// Project ID being published
    pub project_id: Uuid,
    /// User who initiated the publish
    pub user_id: Uuid,
    /// Current status
    pub status: PublishStatus,
    /// Number of retry attempts
    pub retry_count: i32,
    /// Maximum retries allowed
    pub max_retries: i32,
    /// Error message if failed
    pub error_message: Option<String>,
    /// Error details (JSON)
    pub error_details: Option<serde_json::Value>,
    /// Live URL after successful publish
    pub live_url: Option<String>,
    /// Transaction hash from blockchain audit
    pub audit_tx_hash: Option<String>,
    /// When the job was created
    pub created_at: DateTime<Utc>,
    /// When the job started processing
    pub started_at: Option<DateTime<Utc>>,
    /// When the job completed
    pub completed_at: Option<DateTime<Utc>>,
}

impl PublishJob {
    /// Create a new publish job
    pub fn new(project_id: Uuid, user_id: Uuid) -> Self {
        PublishJob {
            id: Uuid::new_v4(),
            project_id,
            user_id,
            status: PublishStatus::Pending,
            retry_count: 0i32,
            max_retries: 3i32,
            error_message: None,
            error_details: None,
            live_url: None,
            audit_tx_hash: None,
            created_at: Utc::now(),
            started_at: None,
            completed_at: None,
        }
    }

    /// Check if the job can be retried
    pub fn can_retry(&self) -> bool {
        self.retry_count < self.max_retries
    }

    /// Get the duration of the job
    pub fn duration(&self) -> Option<chrono::Duration> {
        match (self.started_at, self.completed_at) {
            (Some(start), Some(end)) => Some(end - start),
            (Some(start), None) => Some(Utc::now() - start),
            _ => None,
        }
    }
}

/// Request to publish a project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishRequest {
    /// Project ID to publish
    pub project_id: Uuid,
    /// Skip validation (for re-publish)
    #[serde(default)]
    pub skip_validation: bool,
}

/// Validation result for project completeness
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    /// Whether the project is valid for publishing
    pub is_valid: bool,
    /// List of issues found
    pub issues: Vec<ValidationIssue>,
}

/// Issue found during validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationIssue {
    /// Issue severity
    pub severity: IssueSeverity,
    /// Issue code
    pub code: String,
    /// Human-readable message
    pub message: String,
    /// Affected component (if applicable)
    pub component_id: Option<String>,
}

/// Issue severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IssueSeverity {
    Error,
    Warning,
    Info,
}

/// Publish status response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishStatusResponse {
    pub job: PublishJob,
    pub progress: PublishProgress,
}

/// Publish progress information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishProgress {
    /// Current step
    pub current_step: String,
    /// Progress percentage (0-100)
    pub progress_percent: u8,
    /// Estimated time remaining in seconds
    pub eta_seconds: Option<u64>,
}

/// Build bundle result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildResult {
    /// Bundle size in bytes
    pub size_bytes: u64,
    /// Number of files
    pub file_count: u32,
    /// SHA-256 hash of the bundle
    pub content_hash: String,
    /// Build duration in milliseconds
    pub build_duration_ms: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_publish_job_new() {
        let project_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let job = PublishJob::new(project_id, user_id);

        assert_eq!(job.project_id, project_id);
        assert_eq!(job.status, PublishStatus::Pending);
        assert!(job.can_retry());
        assert_eq!(job.max_retries, 3);
    }

    #[test]
    fn test_publish_job_retry() {
        let mut job = PublishJob::new(Uuid::new_v4(), Uuid::new_v4());
        assert!(job.can_retry());

        job.retry_count = 3i32;
        assert!(!job.can_retry());
    }
}
