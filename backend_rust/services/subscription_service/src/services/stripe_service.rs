//! Stripe payment integration service

use anyhow::{Context, Result};
use stripe::{
    Client as StripeClient, CreateCustomer, CreateSubscription,
    Customer, Subscription as StripeSubscription, SubscriptionStatus as StripeSubscriptionStatus,
    UpdateSubscription, SubscriptionId, CustomerId,
};

use crate::models::{PlanType, SubscriptionStatus};

/// Stripe payment service for subscription billing
pub struct StripeService {
    client: StripeClient,
}

impl StripeService {
    /// Create a new Stripe service
    pub fn new(secret_key: &str) -> Self {
        StripeService {
            client: StripeClient::new(secret_key),
        }
    }

    /// Create a Stripe customer for a user
    pub async fn create_customer(&self, email: &str, name: Option<&str>) -> Result<String> {
        let customer = Customer::create(
            &self.client,
            CreateCustomer {
                email: Some(email),
                name,
                ..Default::default()
            },
        )
        .await
        .context("Failed to create Stripe customer")?;

        Ok(customer.id.to_string())
    }

    /// Create a subscription for a customer
    pub async fn create_subscription(
        &self,
        customer_id: &str,
        plan: &PlanType,
        payment_method_id: Option<&str>,
    ) -> Result<String> {
        let price_id = Self::get_price_id(plan).context("No price ID for this plan")?;
        let customer: CustomerId = customer_id.parse().context("Invalid customer ID")?;

        let mut create_sub = CreateSubscription::new(customer);
        create_sub.items = Some(vec![stripe::CreateSubscriptionItems {
            price: Some(price_id.to_string()),
            ..Default::default()
        }]);
        
        if let Some(pm_id) = payment_method_id {
            create_sub.default_payment_method = Some(pm_id);
        }

        let subscription = StripeSubscription::create(&self.client, create_sub)
            .await
            .context("Failed to create Stripe subscription")?;

        Ok(subscription.id.to_string())
    }

    /// Get subscription from Stripe
    pub async fn get_subscription(&self, subscription_id: &str) -> Result<StripeSubscription> {
        let sub_id: SubscriptionId = subscription_id.parse().context("Invalid subscription ID")?;
        let subscription = StripeSubscription::retrieve(&self.client, &sub_id, &[])
            .await
            .context("Failed to retrieve Stripe subscription")?;

        Ok(subscription)
    }

    /// Cancel a subscription in Stripe
    pub async fn cancel_subscription(&self, subscription_id: &str) -> Result<()> {
        let sub_id: SubscriptionId = subscription_id.parse().context("Invalid subscription ID")?;
        StripeSubscription::update(
            &self.client,
            &sub_id,
            UpdateSubscription {
                cancel_at_period_end: Some(true),
                ..Default::default()
            },
        )
        .await
        .context("Failed to cancel Stripe subscription")?;

        Ok(())
    }

    /// Map Stripe subscription status to our status
    pub fn map_status(status: StripeSubscriptionStatus) -> SubscriptionStatus {
        match status {
            StripeSubscriptionStatus::Active => SubscriptionStatus::Active,
            StripeSubscriptionStatus::Canceled => SubscriptionStatus::Canceled,
            StripeSubscriptionStatus::PastDue => SubscriptionStatus::PastDue,
            StripeSubscriptionStatus::Trialing => SubscriptionStatus::Trialing,
            StripeSubscriptionStatus::Unpaid => SubscriptionStatus::Expired,
            _ => SubscriptionStatus::Active,
        }
    }

    /// Get Stripe price ID for a plan
    fn get_price_id(plan: &PlanType) -> Option<&'static str> {
        match plan {
            PlanType::Freemium => None,
            PlanType::Enterprise => Some("price_enterprise_monthly"),
            PlanType::Exclusive => Some("price_exclusive_monthly"),
        }
    }
}

/// Webhook event data from Stripe
#[derive(Debug, serde::Deserialize)]
pub struct StripeWebhookEvent {
    #[serde(rename = "type")]
    pub event_type: String,
    pub data: StripeEventData,
}

#[derive(Debug, serde::Deserialize)]
pub struct StripeEventData {
    pub object: serde_json::Value,
}

/// Parsed subscription from webhook
#[derive(Debug)]
pub struct WebhookSubscriptionData {
    pub stripe_subscription_id: String,
    pub stripe_customer_id: String,
    pub status: SubscriptionStatus,
    pub current_period_start: i64,
    pub current_period_end: i64,
}

impl StripeService {
    /// Handle webhook event from Stripe
    pub fn parse_subscription_webhook(
        &self,
        event: &StripeWebhookEvent,
    ) -> Result<Option<WebhookSubscriptionData>> {
        match event.event_type.as_str() {
            "customer.subscription.created"
            | "customer.subscription.updated"
            | "customer.subscription.deleted" => {
                let obj = &event.data.object;

                let stripe_subscription_id =
                    obj["id"].as_str().context("Missing subscription ID")?.to_string();

                let stripe_customer_id =
                    obj["customer"].as_str().context("Missing customer ID")?.to_string();

                let status_str = obj["status"].as_str().unwrap_or("active");

                let status = match status_str {
                    "active" => SubscriptionStatus::Active,
                    "canceled" => SubscriptionStatus::Canceled,
                    "past_due" => SubscriptionStatus::PastDue,
                    "trialing" => SubscriptionStatus::Trialing,
                    "unpaid" => SubscriptionStatus::Expired,
                    _ => SubscriptionStatus::Active,
                };

                let current_period_start = obj["current_period_start"].as_i64().unwrap_or(0);

                let current_period_end = obj["current_period_end"].as_i64().unwrap_or(0);

                Ok(Some(WebhookSubscriptionData {
                    stripe_subscription_id,
                    stripe_customer_id,
                    status,
                    current_period_start,
                    current_period_end,
                }))
            }
            _ => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_stripe_status() {
        assert_eq!(
            StripeService::map_status(StripeSubscriptionStatus::Active),
            SubscriptionStatus::Active
        );
        assert_eq!(
            StripeService::map_status(StripeSubscriptionStatus::Canceled),
            SubscriptionStatus::Canceled
        );
        assert_eq!(
            StripeService::map_status(StripeSubscriptionStatus::PastDue),
            SubscriptionStatus::PastDue
        );
        assert_eq!(
            StripeService::map_status(StripeSubscriptionStatus::Trialing),
            SubscriptionStatus::Trialing
        );
    }

    #[test]
    fn test_get_price_id() {
        assert!(StripeService::get_price_id(&PlanType::Freemium).is_none());
        assert!(StripeService::get_price_id(&PlanType::Enterprise).is_some());
        assert!(StripeService::get_price_id(&PlanType::Exclusive).is_some());
    }
}
