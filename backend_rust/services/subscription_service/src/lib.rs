//! Subscription Service Library
//! 
//! This crate provides subscription management functionality including:
//! - Plan types and limits configuration
//! - Feature flags for plan-based access control
//! - Subscription model and repository
//! - Stripe payment integration
//! - HTTP handlers for subscription endpoints

pub mod handlers;
pub mod models;
pub mod repository;
pub mod services;

pub use models::{Feature, FeatureFlags, PlanDetails, PlanLimits, PlanType, Subscription, SubscriptionStatus};
pub use repository::SubscriptionRepository;
pub use services::{StripeService, SubscriptionService};
