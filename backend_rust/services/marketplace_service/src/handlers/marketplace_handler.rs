use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::*;

/// GET /api/v1/marketplace/components - List marketplace components
pub async fn list_components(
    query: web::Query<ListComponentsQuery>,
) -> HttpResponse {
    // Mock response for now - in production, fetch from database
    let components = vec![
        MarketplaceComponent {
            id: Uuid::new_v4(),
            name: "Hero Section".to_string(),
            description: "Modern hero section with gradient background".to_string(),
            category: ComponentCategory::Layout,
            price_cents: 999,
            currency: "USD".to_string(),
            preview_url: "https://cdn.rinova.app/components/hero/preview".to_string(),
            thumbnail_url: "https://cdn.rinova.app/components/hero/thumb.png".to_string(),
            ipfs_hash: "QmHero123".to_string(),
            creator_id: Uuid::new_v4(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            downloads: 150,
            rating: 4.5,
            review_count: 12,
            is_premium: false,
            tags: vec!["hero".to_string(), "landing".to_string()],
        },
    ];

    HttpResponse::Ok().json(ListComponentsResponse {
        components,
        total: 1,
        page: query.page.unwrap_or(1),
        per_page: query.per_page.unwrap_or(20),
    })
}

/// POST /api/v1/marketplace/purchase - Purchase a component
pub async fn purchase_component(
    body: web::Json<PurchaseRequest>,
) -> HttpResponse {
    // Mock response - in production, integrate with Stripe and blockchain
    HttpResponse::Ok().json(PurchaseResponse {
        purchase_id: Uuid::new_v4(),
        component_id: body.component_id,
        status: "completed".to_string(),
        blockchain_tx_hash: Some("0x123abc...".to_string()),
    })
}

/// POST /api/v1/marketplace/add-to-library - Add component to user library (premium users)
pub async fn add_to_library(
    body: web::Json<AddToLibraryRequest>,
) -> HttpResponse {
    HttpResponse::Ok().json(AddToLibraryResponse {
        success: true,
        component_id: body.component_id,
        added_at: chrono::Utc::now(),
    })
}

/// GET /api/v1/marketplace/library - Get user's component library
pub async fn get_library() -> HttpResponse {
    HttpResponse::Ok().json(UserLibrary {
        user_id: Uuid::new_v4(),
        components: vec![],
        updated_at: chrono::Utc::now(),
    })
}

// Request/Response types
#[derive(Debug, Deserialize)]
pub struct ListComponentsQuery {
    pub category: Option<String>,
    pub min_price: Option<u32>,
    pub max_price: Option<u32>,
    pub is_premium: Option<bool>,
    pub search: Option<String>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct ListComponentsResponse {
    pub components: Vec<MarketplaceComponent>,
    pub total: u32,
    pub page: u32,
    pub per_page: u32,
}

#[derive(Debug, Deserialize)]
pub struct PurchaseRequest {
    pub component_id: Uuid,
    pub user_id: Uuid,
    pub payment_method_id: String,
}

#[derive(Debug, Serialize)]
pub struct PurchaseResponse {
    pub purchase_id: Uuid,
    pub component_id: Uuid,
    pub status: String,
    pub blockchain_tx_hash: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AddToLibraryRequest {
    pub component_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct AddToLibraryResponse {
    pub success: bool,
    pub component_id: Uuid,
    pub added_at: chrono::DateTime<chrono::Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_components_query_defaults() {
        let query = ListComponentsQuery {
            category: None,
            min_price: None,
            max_price: None,
            is_premium: None,
            search: None,
            page: None,
            per_page: None,
        };

        assert!(query.category.is_none());
        assert!(query.page.is_none());
    }
}
