//! Subscription repository for database operations

use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{PlanType, Subscription, SubscriptionStatus};

/// Repository for subscription CRUD operations
#[derive(Clone)]
pub struct SubscriptionRepository {
    pool: PgPool,
}

impl SubscriptionRepository {
    /// Create a new subscription repository
    pub fn new(pool: PgPool) -> Self {
        SubscriptionRepository { pool }
    }

    /// Create a new freemium subscription for a user (personal)
    pub async fn create_freemium(&self, user_id: Uuid) -> Result<Subscription> {
        let subscription = sqlx::query_as::<_, Subscription>(
            r#"
            INSERT INTO subscriptions (user_id, plan_type, status)
            VALUES ($1, 'freemium', 'active')
            RETURNING *
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(subscription)
    }

    /// Create a new freemium subscription for a workspace (company)
    pub async fn create_workspace_freemium(&self, user_id: Uuid, workspace_id: Uuid) -> Result<Subscription> {
        let subscription = sqlx::query_as::<_, Subscription>(
            r#"
            INSERT INTO subscriptions (user_id, workspace_id, plan_type, status)
            VALUES ($1, $2, 'freemium', 'active')
            RETURNING *
            "#,
        )
        .bind(user_id)
        .bind(workspace_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(subscription)
    }

    /// Get subscription by user ID
    pub async fn get_by_user_id(&self, user_id: Uuid) -> Result<Option<Subscription>> {
        let subscription = sqlx::query_as::<_, Subscription>(
            r#"
            SELECT * FROM subscriptions WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(subscription)
    }

    /// Get personal subscription for a user (where workspace_id IS NULL)
    pub async fn get_personal_by_user_id(&self, user_id: Uuid) -> Result<Option<Subscription>> {
        let subscription = sqlx::query_as::<_, Subscription>(
            r#"
            SELECT * FROM subscriptions WHERE user_id = $1 AND workspace_id IS NULL
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(subscription)
    }

    /// Get subscription by workspace ID (company/workspace subscriptions)
    pub async fn get_by_workspace_id(&self, workspace_id: Uuid) -> Result<Option<Subscription>> {
        let subscription = sqlx::query_as::<_, Subscription>(
            r#"
            SELECT * FROM subscriptions WHERE workspace_id = $1
            "#,
        )
        .bind(workspace_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(subscription)
    }

    /// Get subscription by ID
    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<Subscription>> {
        let subscription = sqlx::query_as::<_, Subscription>(
            r#"
            SELECT * FROM subscriptions WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(subscription)
    }

    /// Get subscription by Stripe subscription ID
    pub async fn get_by_stripe_id(&self, stripe_subscription_id: &str) -> Result<Option<Subscription>> {
        let subscription = sqlx::query_as::<_, Subscription>(
            r#"
            SELECT * FROM subscriptions WHERE stripe_subscription_id = $1
            "#,
        )
        .bind(stripe_subscription_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(subscription)
    }

    /// Update subscription plan
    pub async fn update_plan(&self, id: Uuid, plan_type: &PlanType) -> Result<Subscription> {
        let plan_str = plan_type.to_string();
        let subscription = sqlx::query_as::<_, Subscription>(
            r#"
            UPDATE subscriptions
            SET plan_type = $2, updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(&plan_str)
        .fetch_one(&self.pool)
        .await?;

        Ok(subscription)
    }

    /// Update subscription status
    pub async fn update_status(&self, id: Uuid, status: SubscriptionStatus) -> Result<Subscription> {
        let status_str = match status {
            SubscriptionStatus::Active => "active",
            SubscriptionStatus::Canceled => "canceled",
            SubscriptionStatus::Expired => "expired",
            SubscriptionStatus::PastDue => "past_due",
            SubscriptionStatus::Trialing => "trialing",
        };

        let subscription = sqlx::query_as::<_, Subscription>(
            r#"
            UPDATE subscriptions
            SET status = $2, updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(status_str)
        .fetch_one(&self.pool)
        .await?;

        Ok(subscription)
    }

    /// Update Stripe subscription ID
    pub async fn update_stripe_id(
        &self,
        id: Uuid,
        stripe_subscription_id: &str,
        stripe_customer_id: Option<&str>,
    ) -> Result<Subscription> {
        let subscription = sqlx::query_as::<_, Subscription>(
            r#"
            UPDATE subscriptions
            SET stripe_subscription_id = $2,
                stripe_customer_id = COALESCE($3, stripe_customer_id),
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(stripe_subscription_id)
        .bind(stripe_customer_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(subscription)
    }

    /// Update billing period
    pub async fn update_billing_period(
        &self,
        id: Uuid,
        period_start: chrono::DateTime<chrono::Utc>,
        period_end: chrono::DateTime<chrono::Utc>,
    ) -> Result<Subscription> {
        let subscription = sqlx::query_as::<_, Subscription>(
            r#"
            UPDATE subscriptions
            SET current_period_start = $2,
                current_period_end = $3,
                updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(period_start)
        .bind(period_end)
        .fetch_one(&self.pool)
        .await?;

        Ok(subscription)
    }

    /// Cancel subscription (will remain active until period end)
    pub async fn cancel(&self, id: Uuid) -> Result<Subscription> {
        let subscription = sqlx::query_as::<_, Subscription>(
            r#"
            UPDATE subscriptions
            SET status = 'canceled', updated_at = NOW()
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        Ok(subscription)
    }

    /// Delete subscription (for account deletion)
    pub async fn delete(&self, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM subscriptions WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Get the highest active workspace plan for a user (across all workspace memberships)
    pub async fn get_highest_workspace_plan_for_user(&self, user_id: Uuid) -> Result<Option<String>> {
        let plan: Option<(String,)> = sqlx::query_as(
            r#"
            SELECT s.plan_type
            FROM workspace_members wm
            JOIN subscriptions s ON s.workspace_id = wm.workspace_id
            WHERE wm.user_id = $1
              AND wm.invitation_status = 'accepted'
              AND s.status = 'active'
            ORDER BY
                CASE s.plan_type
                    WHEN 'exclusive' THEN 3
                    WHEN 'enterprise' THEN 2
                    WHEN 'freemium' THEN 1
                END DESC
            LIMIT 1
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(plan.map(|(p,)| p))
    }

    /// Get count of active projects for a user (for limit checking)
    pub async fn get_active_project_count(&self, user_id: Uuid) -> Result<i32> {
        let count: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*) FROM projects p
            JOIN workspaces w ON p.workspace_id = w.id
            WHERE w.owner_id = $1 AND p.status != 'archived'
            "#,
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(count as i32)
    }

    /// List all subscriptions that are expiring soon (for notifications)
    pub async fn list_expiring_soon(&self, days: i32) -> Result<Vec<Subscription>> {
        let subscriptions = sqlx::query_as::<_, Subscription>(
            r#"
            SELECT * FROM subscriptions
            WHERE status = 'active'
            AND current_period_end IS NOT NULL
            AND current_period_end <= NOW() + INTERVAL '1 day' * $1
            AND current_period_end > NOW()
            "#,
        )
        .bind(days)
        .fetch_all(&self.pool)
        .await?;

        Ok(subscriptions)
    }

    /// Update expired subscriptions (batch operation)
    pub async fn update_expired(&self) -> Result<u64> {
        let result = sqlx::query(
            r#"
            UPDATE subscriptions
            SET status = 'expired', updated_at = NOW()
            WHERE status = 'active'
            AND current_period_end IS NOT NULL
            AND current_period_end < NOW()
            "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Integration tests require a test database
    // These would be implemented with sqlx::test macro
    // when running against a real test database
}
