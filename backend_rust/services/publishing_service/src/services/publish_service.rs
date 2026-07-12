//! Publish service for orchestrating the publish workflow

use anyhow::{Context, Result};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{PublishJob, PublishRequest, PublishStatus, PublishStatusResponse, PublishProgress};
use super::{BuildService, DomainService};

/// Publish service for orchestrating the publish workflow
pub struct PublishService {
    db: PgPool,
    build_service: BuildService,
    domain_service: DomainService,
}

impl PublishService {
    /// Create a new publish service
    pub fn new(db: PgPool) -> Self {
        PublishService {
            db,
            build_service: BuildService::new(),
            domain_service: DomainService::new(),
        }
    }

    /// Start a new publish job
    pub async fn start_publish(&self, request: PublishRequest, user_id: Uuid) -> Result<PublishJob> {
        tracing::info!("Starting publish for project: {}", request.project_id);

        // Create job record
        let mut job = PublishJob::new(request.project_id, user_id);

        // Save job to database
        let job = self.create_job(&job).await?;

        // Start async processing
        let job_id = job.id;
        let db = self.db.clone();
        let build_service = BuildService::new();

        tokio::spawn(async move {
            if let Err(e) = Self::process_publish_job(job_id, request.project_id, db, build_service).await {
                tracing::error!("Publish job {} failed: {}", job_id, e);
            }
        });

        Ok(job)
    }

    /// Process publish job (runs in background)
    async fn process_publish_job(
        job_id: Uuid,
        project_id: Uuid,
        db: PgPool,
        build_service: BuildService,
    ) -> Result<()> {
        // Update status to building
        Self::update_job_status(&db, job_id, PublishStatus::Building).await?;

        // Validate project
        let validation = build_service.validate_project(project_id).await?;
        if !validation.is_valid {
            let errors: Vec<String> = validation
                .issues
                .iter()
                .filter(|i| i.severity == crate::models::IssueSeverity::Error)
                .map(|i| i.message.clone())
                .collect();

            Self::mark_job_failed(&db, job_id, &errors.join(", ")).await?;
            return Ok(());
        }

        // Build bundle with retry logic
        let mut retry_count = 0;
        let max_retries = 3;
        let mut last_error = None;

        while retry_count < max_retries {
            match build_service.build_bundle(project_id).await {
                Ok(build_result) => {
                    // Update status to uploading
                    Self::update_job_status(&db, job_id, PublishStatus::Uploading).await?;

                    // Upload bundle (placeholder)
                    let bundle = vec![]; // Would contain actual bundle
                    match build_service.upload_bundle(project_id, bundle).await {
                        Ok(live_url) => {
                            // Mark job as completed
                            Self::mark_job_completed(&db, job_id, &live_url, &build_result.content_hash)
                                .await?;
                            return Ok(());
                        }
                        Err(e) => {
                            last_error = Some(e);
                            retry_count += 1;
                            if retry_count < max_retries {
                                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                            }
                        }
                    }
                }
                Err(e) => {
                    last_error = Some(e);
                    retry_count += 1;
                    if retry_count < max_retries {
                        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                    }
                }
            }
        }

        // All retries failed
        if let Some(e) = last_error {
            Self::mark_job_failed(&db, job_id, &e.to_string()).await?;
        }

        Ok(())
    }

    /// Get publish job status
    pub async fn get_status(&self, job_id: Uuid) -> Result<Option<PublishStatusResponse>> {
        let job = sqlx::query_as::<_, PublishJob>(
            "SELECT * FROM publish_jobs WHERE id = $1",
        )
        .bind(job_id)
        .fetch_optional(&self.db)
        .await?
        .context("Failed to fetch job")?;

        Ok(job.map(|j| {
            let progress = match j.status {
                PublishStatus::Pending => PublishProgress {
                    current_step: "Waiting in queue".to_string(),
                    progress_percent: 0,
                    eta_seconds: Some(60),
                },
                PublishStatus::Building => PublishProgress {
                    current_step: "Building website bundle".to_string(),
                    progress_percent: 30,
                    eta_seconds: Some(45),
                },
                PublishStatus::Uploading => PublishProgress {
                    current_step: "Uploading to hosting".to_string(),
                    progress_percent: 70,
                    eta_seconds: Some(20),
                },
                PublishStatus::Completed => PublishProgress {
                    current_step: "Published successfully".to_string(),
                    progress_percent: 100,
                    eta_seconds: None,
                },
                PublishStatus::Failed => PublishProgress {
                    current_step: "Publish failed".to_string(),
                    progress_percent: 0,
                    eta_seconds: None,
                },
            };

            PublishStatusResponse { job: j, progress }
        }))
    }

    /// Create job in database
    async fn create_job(&self, job: &PublishJob) -> Result<PublishJob> {
        // Note: This is a simplified version. In production, would use proper SQL
        Ok(job.clone())
    }

    /// Update job status
    async fn update_job_status(db: &PgPool, job_id: Uuid, status: PublishStatus) -> Result<()> {
        tracing::info!("Updating job {} status to {:?}", job_id, status);
        // Note: Would execute UPDATE query
        Ok(())
    }

    /// Mark job as completed
    async fn mark_job_completed(
        db: &PgPool,
        job_id: Uuid,
        live_url: &str,
        audit_hash: &str,
    ) -> Result<()> {
        tracing::info!("Job {} completed: {}", job_id, live_url);
        // Note: Would execute UPDATE query
        Ok(())
    }

    /// Mark job as failed
    async fn mark_job_failed(db: &PgPool, job_id: Uuid, error: &str) -> Result<()> {
        tracing::error!("Job {} failed: {}", job_id, error);
        // Note: Would execute UPDATE query
        Ok(())
    }

    /// Cancel a publish job
    pub async fn cancel_job(&self, job_id: Uuid) -> Result<()> {
        tracing::info!("Cancelling job: {}", job_id);
        // Note: Would execute UPDATE query to mark as cancelled
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_publish_progress() {
        let progress = PublishProgress {
            current_step: "Building".to_string(),
            progress_percent: 30,
            eta_seconds: Some(45),
        };

        assert_eq!(progress.progress_percent, 30);
    }
}
