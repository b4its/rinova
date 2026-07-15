//! Subscription model and status management

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use super::{FeatureFlags, PlanLimits, PlanType};

/// Subscription status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum SubscriptionStatus {
    /// Subscription is active and usable
    Active,
    /// Subscription was canceled but still in period
    Canceled,
    /// Subscription has expired
    Expired,
    /// Payment failed, subscription past due
    PastDue,
    /// Subscription is in trial period
    Trialing,
}

impl Default for SubscriptionStatus {
    fn default() -> Self {
        SubscriptionStatus::Active
    }
}

impl SubscriptionStatus {
    /// Check if the subscription is usable
    pub fn is_usable(&self) -> bool {
        matches!(self, SubscriptionStatus::Active | SubscriptionStatus::Trialing)
    }

    /// Check if the subscription needs attention (payment issues)
    pub fn needs_attention(&self) -> bool {
        matches!(self, SubscriptionStatus::PastDue | SubscriptionStatus::Expired)
    }
}

/// Subscription model matching the database schema
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Subscription {
    /// Unique subscription ID
    pub id: Uuid,
    /// User ID who owns this subscription
    pub user_id: Uuid,
    /// Workspace ID (if this is a workspace subscription)
    pub workspace_id: Option<Uuid>,
    /// Current plan type
    pub plan_type: PlanType,
    /// Subscription status
    pub status: SubscriptionStatus,
    /// Start of current billing period
    pub current_period_start: Option<DateTime<Utc>>,
    /// End of current billing period
    pub current_period_end: Option<DateTime<Utc>>,
    /// Stripe subscription ID for payment gateway
    pub stripe_subscription_id: Option<String>,
    /// Stripe customer ID
    pub stripe_customer_id: Option<String>,
    /// When this subscription was created
    pub created_at: DateTime<Utc>,
    /// When this subscription was last updated
    pub updated_at: DateTime<Utc>,
}

impl Subscription {
    /// Create a new freemium subscription
    pub fn new_freemium(user_id: Uuid, workspace_id: Option<Uuid>) -> Self {
        let now = Utc::now();
        Subscription {
            id: Uuid::new_v4(),
            user_id,
            workspace_id,
            plan_type: PlanType::Freemium,
            status: SubscriptionStatus::Active,
            current_period_start: Some(now),
            current_period_end: None, // No end for freemium
            stripe_subscription_id: None,
            stripe_customer_id: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Get the plan limits for this subscription
    pub fn limits(&self) -> PlanLimits {
        PlanLimits::for_plan(&self.plan_type)
    }

    /// Get the feature flags for this subscription
    pub fn features(&self) -> FeatureFlags {
        FeatureFlags::for_plan(&self.plan_type)
    }

    /// Check if the subscription is currently usable
    pub fn is_usable(&self) -> bool {
        self.status.is_usable()
    }

    /// Check if the subscription period has ended
    pub fn is_period_ended(&self) -> bool {
        if let Some(end) = self.current_period_end {
            return Utc::now() > end;
        }
        false
    }

    /// Check if a specific feature is enabled for this subscription
    pub fn has_feature(&self, feature: super::Feature) -> bool {
        self.features().is_enabled(feature)
    }

    /// Get days remaining in current period
    pub fn days_remaining(&self) -> Option<i64> {
        self.current_period_end.map(|end| {
            let now = Utc::now();
            let remaining = (end - now).num_days();
            remaining.max(0)
        })
    }

    /// Check if this is a paid subscription
    pub fn is_paid(&self) -> bool {
        !matches!(self.plan_type, PlanType::Freemium)
    }

    /// Check if this is a workspace subscription
    pub fn is_workspace_subscription(&self) -> bool {
        self.workspace_id.is_some()
    }

    /// Get the effective price in cents (personal vs workspace)
    pub fn effective_price_cents(&self) -> u32 {
        if self.is_workspace_subscription() {
            self.plan_type.price_cents_workspace()
        } else {
            self.plan_type.price_cents()
        }
    }
}

/// Subscription type (personal vs workspace/company)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SubscriptionType {
    Personal,
    Workspace,
}

impl Default for SubscriptionType {
    fn default() -> Self {
        SubscriptionType::Personal
    }
}

/// Request body for creating/upgrading a subscription (supports workspace)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSubscriptionRequest {
    pub user_id: Uuid,
    pub workspace_id: Option<Uuid>,
    pub plan_type: Option<PlanType>,
}

/// Plan details for display (used in pricing page)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanDetails {
    /// Plan type identifier
    pub plan_type: PlanType,
    /// Display name for UI
    pub name: String,
    /// Price in USD cents per month
    pub price_cents: u32,
    /// Price in USD per month (for display)
    pub price_usd: f64,
    /// Billing period description
    pub billing_period: String,
    /// Feature list for this plan
    pub features: Vec<String>,
    /// Plan limits
    pub limits: PlanLimits,
    /// Feature flags
    pub feature_flags: FeatureFlags,
    /// Whether this is the most popular plan
    pub is_popular: bool,
    /// Call-to-action button text
    pub cta_text: String,
}

impl PlanDetails {
    /// Get all available plans for a given subscription type
    pub fn all_plans_for(sub_type: &SubscriptionType) -> Vec<PlanDetails> {
        match sub_type {
            SubscriptionType::Personal => vec![
                Self::freemium_personal(),
                Self::enterprise_personal(),
                Self::exclusive_personal(),
            ],
            SubscriptionType::Workspace => vec![
                Self::freemium_workspace(),
                Self::enterprise_workspace(),
                Self::exclusive_workspace(),
            ],
        }
    }

    /// Get all available plans as details (defaults to personal)
    pub fn all_plans() -> Vec<PlanDetails> {
        Self::all_plans_for(&SubscriptionType::Personal)
    }

    /// Freemium plan details (personal)
    pub fn freemium_personal() -> Self {
        let features = FeatureFlags::for_freemium();
        let limits = PlanLimits::for_plan(&PlanType::Freemium);

        PlanDetails {
            plan_type: PlanType::Freemium,
            name: "Freemium".to_string(),
            price_cents: 0,
            price_usd: 0.0,
            billing_period: "Free forever".to_string(),
            features: vec![
                "Up to 2 projects".to_string(),
                "Basic components".to_string(),
                "Export to ZIP".to_string(),
                "Community support".to_string(),
            ],
            limits,
            feature_flags: features,
            is_popular: false,
            cta_text: "Get Started".to_string(),
        }
    }

    /// Freemium plan details (workspace/company)
    pub fn freemium_workspace() -> Self {
        let mut p = Self::freemium_personal();
        p.name = "Workspace Free".to_string();
        p.cta_text = "Create Workspace".to_string();
        p
    }

    /// Enterprise plan details (personal)
    pub fn enterprise_personal() -> Self {
        let features = FeatureFlags::for_enterprise();
        let limits = PlanLimits::for_plan(&PlanType::Enterprise);

        PlanDetails {
            plan_type: PlanType::Enterprise,
            name: "Enterprise".to_string(),
            price_cents: 29_00,
            price_usd: 29.0,
            billing_period: "per month".to_string(),
            features: vec![
                "Up to 10 projects".to_string(),
                "Animation Editor".to_string(),
                "Custom CSS Editor".to_string(),
                "SEO Tools".to_string(),
                "Responsive Design".to_string(),
                "Asset Marketplace access".to_string(),
                "Premium Components included".to_string(),
                "Team Collaboration (up to 10)".to_string(),
                "Priority support".to_string(),
            ],
            limits,
            feature_flags: features,
            is_popular: true,
            cta_text: "Start Free Trial".to_string(),
        }
    }

    /// Enterprise plan details (workspace/company)
    pub fn enterprise_workspace() -> Self {
        PlanDetails {
            plan_type: PlanType::Enterprise,
            name: "Workspace Enterprise".to_string(),
            price_cents: 99_00,
            price_usd: 99.0,
            billing_period: "per month".to_string(),
            features: vec![
                "Up to 10 projects".to_string(),
                "Animation Editor".to_string(),
                "Custom CSS Editor".to_string(),
                "SEO Tools".to_string(),
                "Responsive Design".to_string(),
                "Asset Marketplace access".to_string(),
                "Premium Components included".to_string(),
                "Team Collaboration (up to 10)".to_string(),
                "Workspace-wide subscription".to_string(),
                "Priority support".to_string(),
            ],
            limits: PlanLimits::for_plan(&PlanType::Enterprise),
            feature_flags: FeatureFlags::for_enterprise(),
            is_popular: true,
            cta_text: "Start Free Trial".to_string(),
        }
    }

    /// Exclusive plan details (personal)
    pub fn exclusive_personal() -> Self {
        let features = FeatureFlags::for_exclusive();
        let limits = PlanLimits::for_plan(&PlanType::Exclusive);

        PlanDetails {
            plan_type: PlanType::Exclusive,
            name: "Exclusive".to_string(),
            price_cents: 79_00,
            price_usd: 79.0,
            billing_period: "per month".to_string(),
            features: vec![
                "Unlimited projects".to_string(),
                "Everything in Enterprise".to_string(),
                "One-Click Publish".to_string(),
                "Custom Domain (up to 5)".to_string(),
                "Analytics Dashboard".to_string(),
                "SSL Certificate included".to_string(),
                "50GB Storage".to_string(),
                "Team Collaboration (up to 50)".to_string(),
                "Dedicated support".to_string(),
            ],
            limits,
            feature_flags: features,
            is_popular: false,
            cta_text: "Get Exclusive".to_string(),
        }
    }

    /// Exclusive plan details (workspace/company)
    pub fn exclusive_workspace() -> Self {
        PlanDetails {
            plan_type: PlanType::Exclusive,
            name: "Workspace Exclusive".to_string(),
            price_cents: 199_00,
            price_usd: 199.0,
            billing_period: "per month".to_string(),
            features: vec![
                "Unlimited projects".to_string(),
                "Everything in Enterprise".to_string(),
                "One-Click Publish".to_string(),
                "Custom Domain (up to 5)".to_string(),
                "Analytics Dashboard".to_string(),
                "SSL Certificate included".to_string(),
                "50GB Storage".to_string(),
                "Team Collaboration (up to 50)".to_string(),
                "Workspace-wide subscription".to_string(),
                "Dedicated support".to_string(),
            ],
            limits: PlanLimits::for_plan(&PlanType::Exclusive),
            feature_flags: FeatureFlags::for_exclusive(),
            is_popular: false,
            cta_text: "Get Exclusive".to_string(),
        }
    }
}

/// Request to upgrade/downgrade a subscription
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionChangeRequest {
    /// Target plan type
    pub target_plan: PlanType,
    /// Stripe payment method ID (for upgrades)
    pub payment_method_id: Option<String>,
    /// Workspace ID (if upgrading a workspace subscription)
    pub workspace_id: Option<Uuid>,
}

/// Subscription comparison result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionComparison {
    /// Current plan
    pub current_plan: PlanType,
    /// Target plan for comparison
    pub target_plan: PlanType,
    /// Price difference in cents (positive = increase)
    pub price_difference_cents: i32,
    /// Features that would be gained
    pub features_gained: Vec<String>,
    /// Features that would be lost (for downgrades)
    pub features_lost: Vec<String>,
    /// Limits comparison
    pub limits_difference: LimitsDifference,
}

/// Difference in limits between plans
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitsDifference {
    /// Project limit change
    pub max_projects_change: i32,
    /// Storage limit change in MB
    pub max_storage_mb_change: i32,
    /// Rate limit change
    pub rate_limit_change: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subscription_new_freemium() {
        let user_id = Uuid::new_v4();
        let sub = Subscription::new_freemium(user_id, None);

        assert_eq!(sub.user_id, user_id);
        assert_eq!(sub.plan_type, PlanType::Freemium);
        assert_eq!(sub.status, SubscriptionStatus::Active);
        assert!(sub.is_usable());
        assert!(!sub.is_paid());
        assert!(sub.workspace_id.is_none());
    }

    #[test]
    fn test_subscription_new_workspace_freemium() {
        let user_id = Uuid::new_v4();
        let workspace_id = Uuid::new_v4();
        let sub = Subscription::new_freemium(user_id, Some(workspace_id));

        assert_eq!(sub.user_id, user_id);
        assert_eq!(sub.workspace_id, Some(workspace_id));
        assert!(sub.is_workspace_subscription());
    }

    #[test]
    fn test_subscription_features() {
        let user_id = Uuid::new_v4();
        let sub = Subscription::new_freemium(user_id, None);

        let features = sub.features();
        assert!(!features.animation_editor);
        assert!(!features.custom_css);
        assert!(features.export_zip);
    }

    #[test]
    fn test_subscription_limits() {
        let user_id = Uuid::new_v4();
        let sub = Subscription::new_freemium(user_id, None);

        let limits = sub.limits();
        assert_eq!(limits.max_projects, 2);
        assert_eq!(limits.rate_limit_per_minute, 100);
    }

    #[test]
    fn test_status_is_usable() {
        assert!(SubscriptionStatus::Active.is_usable());
        assert!(SubscriptionStatus::Trialing.is_usable());
        assert!(!SubscriptionStatus::Canceled.is_usable());
        assert!(!SubscriptionStatus::Expired.is_usable());
        assert!(!SubscriptionStatus::PastDue.is_usable());
    }

    #[test]
    fn test_plan_details_all_plans() {
        let plans = PlanDetails::all_plans();
        assert_eq!(plans.len(), 3);

        assert_eq!(plans[0].plan_type, PlanType::Freemium);
        assert_eq!(plans[1].plan_type, PlanType::Enterprise);
        assert_eq!(plans[2].plan_type, PlanType::Exclusive);
    }

    #[test]
    fn test_workspace_plans() {
        let plans = PlanDetails::all_plans_for(&SubscriptionType::Workspace);
        assert_eq!(plans.len(), 3);

        // Workspace plans should be more expensive
        assert!(plans[1].price_cents > 29_00); // Enterprise workspace > $29
        assert!(plans[2].price_cents > 79_00); // Exclusive workspace > $79
    }
}
