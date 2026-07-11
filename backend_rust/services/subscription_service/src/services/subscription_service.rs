//! Subscription business logic service

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::models::{Feature, FeatureFlags, PlanLimits, PlanType, Subscription, SubscriptionStatus};
use crate::repository::SubscriptionRepository;
use crate::services::StripeService;

/// Service for subscription business logic
pub struct SubscriptionService {
    repo: SubscriptionRepository,
    stripe: Option<StripeService>,
}

impl SubscriptionService {
    /// Create a new subscription service
    pub fn new(repo: SubscriptionRepository, stripe: Option<StripeService>) -> Self {
        SubscriptionService { repo, stripe }
    }

    /// Create a freemium subscription for a new user
    pub async fn create_freemium_subscription(&self, user_id: Uuid) -> Result<Subscription> {
        self.repo
            .create_freemium(user_id)
            .await
            .context("Failed to create freemium subscription")
    }

    /// Get subscription for a user
    pub async fn get_subscription(&self, user_id: Uuid) -> Result<Option<Subscription>> {
        self.repo
            .get_by_user_id(user_id)
            .await
            .context("Failed to get subscription")
    }

    /// Check if user can use a specific feature
    pub async fn has_feature(&self, user_id: Uuid, feature: Feature) -> Result<bool> {
        let subscription = self
            .repo
            .get_by_user_id(user_id)
            .await
            .context("Failed to get subscription")?;

        match subscription {
            Some(sub) if sub.is_usable() => Ok(sub.has_feature(feature)),
            _ => Ok(false),
        }
    }

    /// Check if user has reached project limit
    pub async fn can_create_project(&self, user_id: Uuid) -> Result<bool> {
        let subscription = self
            .repo
            .get_by_user_id(user_id)
            .await
            .context("Failed to get subscription")?;

        match subscription {
            Some(sub) if sub.is_usable() => {
                let limits = sub.limits();
                let current_count = self
                    .repo
                    .get_active_project_count(user_id)
                    .await
                    .context("Failed to get project count")?;
                Ok(!limits.is_project_limit_exceeded(current_count))
            }
            _ => Ok(false),
        }
    }

    /// Upgrade subscription to a new plan
    pub async fn upgrade_plan(
        &self,
        user_id: Uuid,
        target_plan: PlanType,
        payment_method_id: Option<&str>,
    ) -> Result<Subscription> {
        // Get current subscription
        let current = self
            .repo
            .get_by_user_id(user_id)
            .await
            .context("Failed to get subscription")?
            .context("Subscription not found")?;

        // Validate upgrade path
        if !current.plan_type.can_upgrade_to(&target_plan) {
            anyhow::bail!("Cannot upgrade from {} to {}", current.plan_type, target_plan);
        }

        // Create Stripe subscription if needed
        let stripe_subscription_id = if let Some(ref stripe) = self.stripe {
            let customer_id = match current.stripe_customer_id {
                Some(ref id) => id.clone(),
                None => {
                    // Create customer (would need email from user service)
                    // For now, we'll use a placeholder
                    stripe.create_customer("user@example.com", None).await?
                }
            };

            Some(stripe.create_subscription(&customer_id, &target_plan, payment_method_id).await?)
        } else {
            None
        };

        // Update subscription in database
        let updated = if let Some(stripe_id) = stripe_subscription_id {
            self.repo
                .update_stripe_id(current.id, &stripe_id, None)
                .await
                .context("Failed to update Stripe ID")?;
            self.repo.update_plan(current.id, &target_plan).await?
        } else {
            self.repo.update_plan(current.id, &target_plan).await?
        };

        Ok(updated)
    }

    /// Downgrade subscription to a lower plan
    pub async fn downgrade_plan(&self, user_id: Uuid, target_plan: PlanType) -> Result<Subscription> {
        // Get current subscription
        let current = self
            .repo
            .get_by_user_id(user_id)
            .await
            .context("Failed to get subscription")?
            .context("Subscription not found")?;

        // Validate downgrade path
        if !current.plan_type.can_downgrade_to(&target_plan) {
            anyhow::bail!("Cannot downgrade from {} to {}", current.plan_type, target_plan);
        }

        // Check project limit
        let limits = PlanLimits::for_plan(&target_plan);
        let current_count = self
            .repo
            .get_active_project_count(user_id)
            .await
            .context("Failed to get project count")?;

        if limits.is_project_limit_exceeded(current_count) {
            anyhow::bail!(
                "Cannot downgrade: you have {} active projects, but {} plan only allows {}",
                current_count,
                target_plan.display_name(),
                limits.max_projects
            );
        }

        // Cancel Stripe subscription (if any)
        if let (Some(ref stripe), Some(ref stripe_id)) = (&self.stripe, current.stripe_subscription_id)
        {
            stripe.cancel_subscription(stripe_id).await?;
        }

        // Update subscription in database
        let updated = self
            .repo
            .update_plan(current.id, &target_plan)
            .await
            .context("Failed to update plan")?;

        Ok(updated)
    }

    /// Handle Stripe webhook event
    pub async fn handle_stripe_webhook(
        &self,
        stripe_subscription_id: &str,
        status: SubscriptionStatus,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
    ) -> Result<Subscription> {
        // Find subscription by Stripe ID
        let subscription = self
            .repo
            .get_by_stripe_id(stripe_subscription_id)
            .await
            .context("Failed to get subscription by Stripe ID")?
            .context("Subscription not found for Stripe ID")?;

        // Update subscription status and period
        let updated = self
            .repo
            .update_billing_period(subscription.id, period_start, period_end)
            .await
            .context("Failed to update billing period")?;

        let updated = self
            .repo
            .update_status(updated.id, status)
            .await
            .context("Failed to update status")?;

        Ok(updated)
    }

    /// Get subscription limits for a user
    pub async fn get_limits(&self, user_id: Uuid) -> Result<Option<PlanLimits>> {
        let subscription = self
            .repo
            .get_by_user_id(user_id)
            .await
            .context("Failed to get subscription")?;

        Ok(subscription.map(|s| s.limits()))
    }

    /// Get feature flags for a user
    pub async fn get_features(&self, user_id: Uuid) -> Result<Option<FeatureFlags>> {
        let subscription = self
            .repo
            .get_by_user_id(user_id)
            .await
            .context("Failed to get subscription")?;

        Ok(subscription.map(|s| s.features()))
    }

    /// Process expired subscriptions (cleanup job)
    pub async fn process_expired(&self) -> Result<u64> {
        self.repo
            .update_expired()
            .await
            .context("Failed to update expired subscriptions")
    }

    /// Cancel subscription
    pub async fn cancel(&self, user_id: Uuid) -> Result<Subscription> {
        let subscription = self
            .repo
            .get_by_user_id(user_id)
            .await
            .context("Failed to get subscription")?
            .context("Subscription not found")?;

        // Cancel in Stripe if applicable
        if let (Some(ref stripe), Some(ref stripe_id)) =
            (&self.stripe, subscription.stripe_subscription_id)
        {
            stripe.cancel_subscription(stripe_id).await?;
        }

        // Cancel in database
        self.repo
            .cancel(subscription.id)
            .await
            .context("Failed to cancel subscription")
    }
}

#[cfg(test)]
mod tests {
    // Unit tests would use mock repository
}
