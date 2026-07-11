//! Plan type definitions and limits configuration

use serde::{Deserialize, Serialize};
use std::fmt;

/// Subscription plan types available in the platform
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum PlanType {
    /// Free tier with limited features
    Freemium,
    /// Mid-tier with advanced editor and marketplace access
    Enterprise,
    /// Premium tier with hosting, custom domain, and analytics
    Exclusive,
}

impl fmt::Display for PlanType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlanType::Freemium => write!(f, "freemium"),
            PlanType::Enterprise => write!(f, "enterprise"),
            PlanType::Exclusive => write!(f, "exclusive"),
        }
    }
}

impl Default for PlanType {
    fn default() -> Self {
        PlanType::Freemium
    }
}

impl PlanType {
    /// Parse plan type from string (case-insensitive)
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "freemium" => Some(PlanType::Freemium),
            "enterprise" => Some(PlanType::Enterprise),
            "exclusive" => Some(PlanType::Exclusive),
            _ => None,
        }
    }

    /// Get plan display name for UI
    pub fn display_name(&self) -> &'static str {
        match self {
            PlanType::Freemium => "Freemium",
            PlanType::Enterprise => "Enterprise",
            PlanType::Exclusive => "Exclusive",
        }
    }

    /// Get plan price in USD cents per month
    pub fn price_cents(&self) -> u32 {
        match self {
            PlanType::Freemium => 0,
            PlanType::Enterprise => 29_00, // $29/month
            PlanType::Exclusive => 79_00,  // $79/month
        }
    }

    /// Get plan price in USD per month (as f64)
    pub fn price_usd(&self) -> f64 {
        self.price_cents() as f64 / 100.0
    }

    /// Get the Stripe product ID for this plan
    pub fn stripe_product_id(&self) -> Option<&'static str> {
        match self {
            PlanType::Freemium => None, // No Stripe product for free tier
            PlanType::Enterprise => Some("prod_enterprise"),
            PlanType::Exclusive => Some("prod_exclusive"),
        }
    }

    /// Check if this plan can be upgraded to the target plan
    pub fn can_upgrade_to(&self, target: &PlanType) -> bool {
        matches!(
            (self, target),
            (PlanType::Freemium, PlanType::Enterprise)
                | (PlanType::Freemium, PlanType::Exclusive)
                | (PlanType::Enterprise, PlanType::Exclusive)
        )
    }

    /// Check if this plan can be downgraded to the target plan
    pub fn can_downgrade_to(&self, target: &PlanType) -> bool {
        target.can_upgrade_to(self)
    }
}

/// Plan limits configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanLimits {
    /// Maximum number of active projects allowed
    pub max_projects: i32,
    /// Maximum number of team members in workspace
    pub max_team_members: i32,
    /// Maximum storage in megabytes
    pub max_storage_mb: i32,
    /// Rate limit per minute for API requests
    pub rate_limit_per_minute: i32,
    /// Maximum custom domains per project
    pub max_custom_domains: i32,
    /// Maximum CSS size in kilobytes
    pub max_css_size_kb: i32,
}

impl PlanLimits {
    /// Get limits for a specific plan type
    pub fn for_plan(plan: &PlanType) -> Self {
        match plan {
            PlanType::Freemium => PlanLimits {
                max_projects: 2,
                max_team_members: 1,
                max_storage_mb: 100,
                rate_limit_per_minute: 100,
                max_custom_domains: 0,
                max_css_size_kb: 0,
            },
            PlanType::Enterprise => PlanLimits {
                max_projects: 10,
                max_team_members: 10,
                max_storage_mb: 5_000,
                rate_limit_per_minute: 1_000,
                max_custom_domains: 0,
                max_css_size_kb: 100,
            },
            PlanType::Exclusive => PlanLimits {
                max_projects: i32::MAX, // Unlimited
                max_team_members: 50,
                max_storage_mb: 50_000,
                rate_limit_per_minute: 5_000,
                max_custom_domains: 5,
                max_css_size_kb: 100,
            },
        }
    }

    /// Check if a given count exceeds the project limit
    pub fn is_project_limit_exceeded(&self, current_count: i32) -> bool {
        current_count >= self.max_projects
    }

    /// Check if a given count exceeds the team member limit
    pub fn is_team_member_limit_exceeded(&self, current_count: i32) -> bool {
        current_count >= self.max_team_members
    }

    /// Check if a given storage size exceeds the limit
    pub fn is_storage_limit_exceeded(&self, current_mb: i32) -> bool {
        current_mb >= self.max_storage_mb
    }

    /// Check if rate limit is exceeded
    pub fn is_rate_limit_exceeded(&self, requests_in_minute: i32) -> bool {
        requests_in_minute >= self.rate_limit_per_minute
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plan_type_display() {
        assert_eq!(PlanType::Freemium.to_string(), "freemium");
        assert_eq!(PlanType::Enterprise.to_string(), "enterprise");
        assert_eq!(PlanType::Exclusive.to_string(), "exclusive");
    }

    #[test]
    fn test_plan_upgrade_path() {
        assert!(PlanType::Freemium.can_upgrade_to(&PlanType::Enterprise));
        assert!(PlanType::Freemium.can_upgrade_to(&PlanType::Exclusive));
        assert!(PlanType::Enterprise.can_upgrade_to(&PlanType::Exclusive));
        
        assert!(!PlanType::Enterprise.can_upgrade_to(&PlanType::Freemium));
        assert!(!PlanType::Exclusive.can_upgrade_to(&PlanType::Enterprise));
    }

    #[test]
    fn test_plan_limits_freemium() {
        let limits = PlanLimits::for_plan(&PlanType::Freemium);
        assert_eq!(limits.max_projects, 2);
        assert_eq!(limits.rate_limit_per_minute, 100);
        assert_eq!(limits.max_custom_domains, 0);
    }

    #[test]
    fn test_plan_limits_enterprise() {
        let limits = PlanLimits::for_plan(&PlanType::Enterprise);
        assert_eq!(limits.max_projects, 10);
        assert_eq!(limits.rate_limit_per_minute, 1_000);
        assert_eq!(limits.max_custom_domains, 0);
    }

    #[test]
    fn test_plan_limits_exclusive() {
        let limits = PlanLimits::for_plan(&PlanType::Exclusive);
        assert!(limits.max_projects > 1000); // Effectively unlimited
        assert_eq!(limits.rate_limit_per_minute, 5_000);
        assert_eq!(limits.max_custom_domains, 5);
    }
}
