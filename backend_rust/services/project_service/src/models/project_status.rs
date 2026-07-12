//! Project status enum

use serde::{Deserialize, Serialize};
use sqlx::Type;

/// Project status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum ProjectStatus {
    /// Project is in draft state
    Draft,
    /// Project is published and live
    Published,
    /// Project is archived
    Archived,
}

impl Default for ProjectStatus {
    fn default() -> Self {
        ProjectStatus::Draft
    }
}

impl std::fmt::Display for ProjectStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectStatus::Draft => write!(f, "draft"),
            ProjectStatus::Published => write!(f, "published"),
            ProjectStatus::Archived => write!(f, "archived"),
        }
    }
}
