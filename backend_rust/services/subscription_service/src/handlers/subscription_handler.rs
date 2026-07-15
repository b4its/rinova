//! Subscription HTTP handlers

use actix_web::{web, HttpResponse};
use uuid::Uuid;

use crate::models::{FeatureFlags, PlanDetails, PlanLimits, PlanType, Subscription, SubscriptionChangeRequest, SubscriptionType};
use crate::repository::SubscriptionRepository;
use crate::services::SubscriptionService;

/// Response for subscription plans list
#[derive(Debug, serde::Serialize)]
pub struct PlansResponse {
    pub plans: Vec<PlanDetails>,
}

/// Response for current subscription
#[derive(Debug, serde::Serialize)]
pub struct SubscriptionResponse {
    pub subscription: Subscription,
    pub limits: crate::models::PlanLimits,
    pub features: crate::models::FeatureFlags,
}

/// Response for subscription upgrade/downgrade
#[derive(Debug, serde::Serialize)]
pub struct SubscriptionChangeResponse {
    pub subscription: Subscription,
    pub message: String,
}

/// Error response
#[derive(Debug, serde::Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub code: String,
}

/// GET /api/v1/subscriptions/plans - List all available plans
pub async fn list_plans() -> HttpResponse {
    let plans = PlanDetails::all_plans();
    HttpResponse::Ok().json(PlansResponse { plans })
}

/// GET /api/v1/subscriptions/plans/personal - List personal plans
pub async fn list_personal_plans() -> HttpResponse {
    let plans = PlanDetails::all_plans_for(&SubscriptionType::Personal);
    HttpResponse::Ok().json(PlansResponse { plans })
}

/// GET /api/v1/subscriptions/plans/workspace - List workspace/company plans
pub async fn list_workspace_plans() -> HttpResponse {
    let plans = PlanDetails::all_plans_for(&SubscriptionType::Workspace);
    HttpResponse::Ok().json(PlansResponse { plans })
}

/// Request body for creating freemium subscription
#[derive(Debug, serde::Deserialize)]
pub struct CreateFreemiumRequest {
    pub user_id: Uuid,
    pub workspace_id: Option<Uuid>,
}

/// POST /api/v1/subscriptions/freemium - Create freemium subscription (personal)
pub async fn create_freemium_subscription(
    repo: web::Data<SubscriptionRepository>,
    body: web::Json<CreateFreemiumRequest>,
) -> HttpResponse {
    let user_id = body.user_id;

    // Check if user already has a personal subscription
    match repo.get_personal_by_user_id(user_id).await {
        Ok(Some(existing)) => {
            let limits = existing.limits();
            let features = existing.features();
            return HttpResponse::Ok().json(SubscriptionResponse {
                subscription: existing,
                limits,
                features,
            });
        }
        Ok(None) => {}
        Err(e) => {
            tracing::error!("Failed to check existing subscription: {}", e);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal server error".to_string(),
                code: "INTERNAL_ERROR".to_string(),
            });
        }
    }

    // Create freemium subscription
    match repo.create_freemium(user_id).await {
        Ok(subscription) => {
            let limits = subscription.limits();
            let features = subscription.features();
            HttpResponse::Created().json(SubscriptionResponse {
                subscription,
                limits,
                features,
            })
        }
        Err(e) => {
            tracing::error!("Failed to create freemium subscription: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Failed to create subscription".to_string(),
                code: "CREATE_FAILED".to_string(),
            })
        }
    }
}

/// POST /api/v1/subscriptions/workspace/freemium - Create workspace freemium subscription
pub async fn create_workspace_freemium_subscription(
    repo: web::Data<SubscriptionRepository>,
    body: web::Json<CreateFreemiumRequest>,
) -> HttpResponse {
    let user_id = body.user_id;
    let workspace_id = match body.workspace_id {
        Some(id) => id,
        None => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: "workspace_id is required for workspace subscription".to_string(),
                code: "MISSING_WORKSPACE_ID".to_string(),
            });
        }
    };

    // Check if workspace already has a subscription
    match repo.get_by_workspace_id(workspace_id).await {
        Ok(Some(existing)) => {
            let limits = existing.limits();
            let features = existing.features();
            return HttpResponse::Ok().json(SubscriptionResponse {
                subscription: existing,
                limits,
                features,
            });
        }
        Ok(None) => {}
        Err(e) => {
            tracing::error!("Failed to check workspace subscription: {}", e);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal server error".to_string(),
                code: "INTERNAL_ERROR".to_string(),
            });
        }
    }

    // Create workspace freemium subscription
    match repo.create_workspace_freemium(user_id, workspace_id).await {
        Ok(subscription) => {
            let limits = subscription.limits();
            let features = subscription.features();
            HttpResponse::Created().json(SubscriptionResponse {
                subscription,
                limits,
                features,
            })
        }
        Err(e) => {
            tracing::error!("Failed to create workspace freemium subscription: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Failed to create workspace subscription".to_string(),
                code: "CREATE_FAILED".to_string(),
            })
        }
    }
}

/// GET /api/v1/subscriptions/me - Get personal subscription by user ID
pub async fn get_subscription(
    repo: web::Data<SubscriptionRepository>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let user_id = path.into_inner();

    match repo.get_personal_by_user_id(user_id).await {
        Ok(Some(subscription)) => {
            let limits = subscription.limits();
            let features = subscription.features();
            HttpResponse::Ok().json(SubscriptionResponse {
                subscription,
                limits,
                features,
            })
        }
        Ok(None) => HttpResponse::NotFound().json(ErrorResponse {
            error: "Personal subscription not found".to_string(),
            code: "SUB_NOT_FOUND".to_string(),
        }),
        Err(e) => {
            tracing::error!("Failed to get subscription: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal server error".to_string(),
                code: "INTERNAL_ERROR".to_string(),
            })
        }
    }
}

/// GET /api/v1/subscriptions/workspace/{workspace_id} - Get workspace subscription
pub async fn get_workspace_subscription(
    repo: web::Data<SubscriptionRepository>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let workspace_id = path.into_inner();

    match repo.get_by_workspace_id(workspace_id).await {
        Ok(Some(subscription)) => {
            let limits = subscription.limits();
            let features = subscription.features();
            HttpResponse::Ok().json(SubscriptionResponse {
                subscription,
                limits,
                features,
            })
        }
        Ok(None) => HttpResponse::NotFound().json(ErrorResponse {
            error: "Workspace subscription not found".to_string(),
            code: "SUB_NOT_FOUND".to_string(),
        }),
        Err(e) => {
            tracing::error!("Failed to get workspace subscription: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal server error".to_string(),
                code: "INTERNAL_ERROR".to_string(),
            })
        }
    }
}

/// GET /api/v1/subscriptions/me/effective-plan - Get effective plan for a user
pub async fn get_effective_plan(
    svc: web::Data<SubscriptionService>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let user_id = path.into_inner();

    match svc.get_effective_plan(user_id).await {
        Ok(plan) => {
            HttpResponse::Ok().json(serde_json::json!({
                "plan_type": plan,
                "limits": PlanLimits::for_plan(&plan),
                "features": FeatureFlags::for_plan(&plan),
            }))
        }
        Err(e) => {
            tracing::error!("Failed to get effective plan: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal server error".to_string(),
                code: "INTERNAL_ERROR".to_string(),
            })
        }
    }
}

/// POST /api/v1/subscriptions/{user_id}/upgrade - Upgrade subscription plan
pub async fn upgrade_subscription(
    repo: web::Data<SubscriptionRepository>,
    _svc: web::Data<SubscriptionService>,
    path: web::Path<Uuid>,
    body: web::Json<SubscriptionChangeRequest>,
) -> HttpResponse {
    let user_id = path.into_inner();
    let request = body.into_inner();

    // Get current subscription (personal or workspace)
    let current_subscription = if let Some(wid) = request.workspace_id {
        match repo.get_by_workspace_id(wid).await {
            Ok(Some(sub)) => sub,
            Ok(None) => {
                return HttpResponse::NotFound().json(ErrorResponse {
                    error: "Workspace subscription not found".to_string(),
                    code: "SUB_NOT_FOUND".to_string(),
                })
            }
            Err(e) => {
                tracing::error!("Failed to get workspace subscription: {}", e);
                return HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Internal server error".to_string(),
                    code: "INTERNAL_ERROR".to_string(),
                });
            }
        }
    } else {
        match repo.get_personal_by_user_id(user_id).await {
            Ok(Some(sub)) => sub,
            Ok(None) => {
                return HttpResponse::NotFound().json(ErrorResponse {
                    error: "Personal subscription not found".to_string(),
                    code: "SUB_NOT_FOUND".to_string(),
                })
            }
            Err(e) => {
                tracing::error!("Failed to get subscription: {}", e);
                return HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Internal server error".to_string(),
                    code: "INTERNAL_ERROR".to_string(),
                });
            }
        }
    };

    // Validate upgrade path
    if !current_subscription.plan_type.can_upgrade_to(&request.target_plan) {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: format!(
                "Cannot upgrade from {} to {}",
                current_subscription.plan_type, request.target_plan
            ),
            code: "INVALID_UPGRADE".to_string(),
        });
    }

    // Determine the correct price
    let price_cents = if current_subscription.workspace_id.is_some() {
        request.target_plan.price_cents_workspace()
    } else {
        request.target_plan.price_cents()
    };

    // Update subscription plan
    match repo.update_plan(current_subscription.id, &request.target_plan).await {
        Ok(updated_subscription) => {
            // Record the transaction on-chain (best-effort, non-blocking).
            crate::services::record_subscription_tx(
                &user_id.to_string(),
                &updated_subscription.id.to_string(),
                request.target_plan.display_name(),
                "upgrade",
                price_cents as i64,
            )
            .await;

            HttpResponse::Ok().json(SubscriptionChangeResponse {
                subscription: updated_subscription,
                message: format!(
                    "Successfully upgraded to {} plan",
                    request.target_plan.display_name()
                ),
            })
        }
        Err(e) => {
            tracing::error!("Failed to upgrade subscription: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Failed to upgrade subscription".to_string(),
                code: "UPGRADE_FAILED".to_string(),
            })
        }
    }
}

/// POST /api/v1/subscriptions/downgrade - Downgrade subscription plan
pub async fn downgrade_subscription(
    repo: web::Data<SubscriptionRepository>,
    path: web::Path<Uuid>,
    body: web::Json<SubscriptionChangeRequest>,
) -> HttpResponse {
    let user_id = path.into_inner();
    let request = body.into_inner();

    // Get current subscription (personal or workspace)
    let current_subscription = if let Some(wid) = request.workspace_id {
        match repo.get_by_workspace_id(wid).await {
            Ok(Some(sub)) => sub,
            Ok(None) => {
                return HttpResponse::NotFound().json(ErrorResponse {
                    error: "Workspace subscription not found".to_string(),
                    code: "SUB_NOT_FOUND".to_string(),
                })
            }
            Err(e) => {
                tracing::error!("Failed to get workspace subscription: {}", e);
                return HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Internal server error".to_string(),
                    code: "INTERNAL_ERROR".to_string(),
                });
            }
        }
    } else {
        match repo.get_personal_by_user_id(user_id).await {
            Ok(Some(sub)) => sub,
            Ok(None) => {
                return HttpResponse::NotFound().json(ErrorResponse {
                    error: "Personal subscription not found".to_string(),
                    code: "SUB_NOT_FOUND".to_string(),
                })
            }
            Err(e) => {
                tracing::error!("Failed to get subscription: {}", e);
                return HttpResponse::InternalServerError().json(ErrorResponse {
                    error: "Internal server error".to_string(),
                    code: "INTERNAL_ERROR".to_string(),
                });
            }
        }
    };

    // Validate downgrade path
    if !current_subscription.plan_type.can_downgrade_to(&request.target_plan) {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: format!(
                "Cannot downgrade from {} to {}",
                current_subscription.plan_type, request.target_plan
            ),
            code: "INVALID_DOWNGRADE".to_string(),
        });
    }

    // Check project limit for downgrade
    let limits = crate::models::PlanLimits::for_plan(&request.target_plan);
    match repo.get_active_project_count(user_id).await {
        Ok(project_count) => {
            if limits.is_project_limit_exceeded(project_count) {
                return HttpResponse::BadRequest().json(ErrorResponse {
                    error: format!(
                        "Cannot downgrade: you have {} active projects, but {} plan only allows {}",
                        project_count,
                        request.target_plan.display_name(),
                        limits.max_projects
                    ),
                    code: "PROJECT_LIMIT_EXCEEDED".to_string(),
                });
            }
        }
        Err(e) => {
            tracing::error!("Failed to check project count: {}", e);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Failed to verify project limits".to_string(),
                code: "INTERNAL_ERROR".to_string(),
            });
        }
    }

    // Update subscription plan (will take effect at next billing period)
    match repo.update_plan(current_subscription.id, &request.target_plan).await {
        Ok(updated_subscription) => {
            HttpResponse::Ok().json(SubscriptionChangeResponse {
                subscription: updated_subscription,
                message: format!(
                    "Downgrade to {} plan scheduled for next billing period",
                    request.target_plan.display_name()
                ),
            })
        }
        Err(e) => {
            tracing::error!("Failed to downgrade subscription: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Failed to downgrade subscription".to_string(),
                code: "DOWNGRADE_FAILED".to_string(),
            })
        }
    }
}

/// POST /api/v1/subscriptions/:id/cancel - Cancel subscription
pub async fn cancel_subscription(
    repo: web::Data<SubscriptionRepository>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let subscription_id = path.into_inner();

    match repo.cancel(subscription_id).await {
        Ok(subscription) => {
            HttpResponse::Ok().json(SubscriptionChangeResponse {
                subscription,
                message: "Subscription canceled. You will have access until the end of your billing period.".to_string(),
            })
        }
        Err(e) => {
            tracing::error!("Failed to cancel subscription: {}", e);
            HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Failed to cancel subscription".to_string(),
                code: "CANCEL_FAILED".to_string(),
            })
        }
    }
}

/// Check if user can create a new project based on subscription limits
#[derive(Debug, serde::Serialize)]
pub struct ProjectLimitCheck {
    pub can_create: bool,
    pub current_count: i32,
    pub max_projects: i32,
    pub plan_type: PlanType,
}

/// GET /api/v1/subscriptions/:user_id/project-limit - Check project creation limit
pub async fn check_project_limit(
    repo: web::Data<SubscriptionRepository>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let user_id = path.into_inner();

    // Get subscription
    let subscription = match repo.get_by_user_id(user_id).await {
        Ok(Some(sub)) => sub,
        Ok(None) => {
            return HttpResponse::NotFound().json(ErrorResponse {
                error: "Subscription not found".to_string(),
                code: "SUB_NOT_FOUND".to_string(),
            })
        }
        Err(e) => {
            tracing::error!("Failed to get subscription: {}", e);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Internal server error".to_string(),
                code: "INTERNAL_ERROR".to_string(),
            })
        }
    };

    // Get current project count
    let current_count = match repo.get_active_project_count(user_id).await {
        Ok(count) => count,
        Err(e) => {
            tracing::error!("Failed to get project count: {}", e);
            return HttpResponse::InternalServerError().json(ErrorResponse {
                error: "Failed to check project count".to_string(),
                code: "INTERNAL_ERROR".to_string(),
            });
        }
    };

    let limits = subscription.limits();
    let can_create = !limits.is_project_limit_exceeded(current_count);

    HttpResponse::Ok().json(ProjectLimitCheck {
        can_create,
        current_count,
        max_projects: limits.max_projects,
        plan_type: subscription.plan_type,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plans_response_serialization() {
        let plans = PlanDetails::all_plans();
        let response = PlansResponse { plans };
        
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("freemium"));
        assert!(json.contains("enterprise"));
        assert!(json.contains("exclusive"));
    }
}
