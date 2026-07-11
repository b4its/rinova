use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub full_name: Option<String>,
    pub account_type: AccountType,
    pub email_verified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "lowercase")]
pub enum AccountType {
    Personal,
    Company,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub id: Uuid,
    pub name: String,
    #[serde(rename = "type")]
    pub workspace_type: WorkspaceType,
    pub owner_id: Uuid,
    pub logo_url: Option<String>,
    pub settings: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "lowercase")]
pub enum WorkspaceType {
    Personal,
    Company,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceMember {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub user_id: Uuid,
    pub role: Role,
    pub invitation_status: InvitationStatus,
    pub invited_at: DateTime<Utc>,
    pub joined_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "lowercase")]
pub enum Role {
    Owner,
    Admin,
    Member,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "lowercase")]
pub enum InvitationStatus {
    Pending,
    Accepted,
    Declined,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    pub id: Uuid,
    pub user_id: Uuid,
    pub plan_type: PlanType,
    pub status: SubscriptionStatus,
    pub current_period_start: Option<DateTime<Utc>>,
    pub current_period_end: Option<DateTime<Utc>>,
    pub stripe_subscription_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "lowercase")]
pub enum PlanType {
    Freemium,
    Enterprise,
    Exclusive,
}

impl PlanType {
    pub fn max_projects(&self) -> i32 {
        match self {
            PlanType::Freemium => 2,
            PlanType::Enterprise => 10,
            PlanType::Exclusive => i32::MAX,
        }
    }

    pub fn rate_limit_per_minute(&self) -> i32 {
        match self {
            PlanType::Freemium => 100,
            PlanType::Enterprise => 1000,
            PlanType::Exclusive => 5000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "lowercase")]
pub enum SubscriptionStatus {
    Active,
    Canceled,
    Expired,
    PastDue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub exp: usize,
    pub iat: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub owner_id: Uuid,
    pub name: String,
    pub status: ProjectStatus,
    pub metadata: serde_json::Value,
    pub last_published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "lowercase")]
pub enum ProjectStatus {
    Draft,
    Published,
    Archived,
}
