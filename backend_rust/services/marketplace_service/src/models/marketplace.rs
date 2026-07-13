use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Marketplace component for sale
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplaceComponent {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub category: ComponentCategory,
    pub price_cents: u32,
    pub currency: String,
    pub preview_url: String,
    pub thumbnail_url: String,
    pub ipfs_hash: String,
    pub creator_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub downloads: u32,
    pub rating: f32,
    pub review_count: u32,
    pub is_premium: bool,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "component_category", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum ComponentCategory {
    Layout,
    Navigation,
    Form,
    Media,
    Typography,
    Animation,
    Ecommerce,
    Social,
    Seo,
    Custom,
}

/// Component purchase record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentPurchase {
    pub id: Uuid,
    pub component_id: Uuid,
    pub user_id: Uuid,
    pub price_cents: u32,
    pub currency: String,
    pub stripe_payment_id: Option<String>,
    pub blockchain_tx_hash: Option<String>,
    pub purchased_at: DateTime<Utc>,
}

/// User's component library
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLibrary {
    pub user_id: Uuid,
    pub components: Vec<LibraryItem>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryItem {
    pub component_id: Uuid,
    pub added_at: DateTime<Utc>,
}

/// Component review
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentReview {
    pub id: Uuid,
    pub component_id: Uuid,
    pub user_id: Uuid,
    pub rating: u8,
    pub comment: String,
    pub created_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marketplace_component_creation() {
        let component = MarketplaceComponent {
            id: Uuid::new_v4(),
            name: "Test Component".to_string(),
            description: "A test component".to_string(),
            category: ComponentCategory::Layout,
            price_cents: 999,
            currency: "USD".to_string(),
            preview_url: "https://example.com/preview".to_string(),
            thumbnail_url: "https://example.com/thumb".to_string(),
            ipfs_hash: "QmTest123".to_string(),
            creator_id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            downloads: 0,
            rating: 0.0,
            review_count: 0,
            is_premium: false,
            tags: vec!["test".to_string()],
        };

        assert_eq!(component.name, "Test Component");
        assert_eq!(component.price_cents, 999);
    }
}
