use actix_web::{web, HttpResponse};
use uuid::Uuid;

use crate::repository::{MarketplaceRepository, PaginationQuery};

fn ok_json<T: serde::Serialize>(data: T) -> HttpResponse {
    HttpResponse::Ok().json(data)
}

fn created_json<T: serde::Serialize>(data: T) -> HttpResponse {
    HttpResponse::Created().json(data)
}

fn not_found(msg: &str) -> HttpResponse {
    HttpResponse::NotFound().json(serde_json::json!({
        "error": msg,
        "code": "NOT_FOUND"
    }))
}

fn bad_request(msg: &str) -> HttpResponse {
    HttpResponse::BadRequest().json(serde_json::json!({
        "error": msg,
        "code": "BAD_REQUEST"
    }))
}

fn internal_error(e: impl std::fmt::Display) -> HttpResponse {
    tracing::error!("{}", e);
    HttpResponse::InternalServerError().json(serde_json::json!({
        "error": "Internal server error",
        "code": "INTERNAL_ERROR"
    }))
}

// --- Marketplace Categories ---

pub async fn list_marketplace_categories(
    repo: web::Data<MarketplaceRepository>,
    query: web::Query<PaginationQuery>,
) -> HttpResponse {
    match repo.list_categories(&query).await {
        Ok(resp) => ok_json(resp),
        Err(e) => internal_error(e),
    }
}

pub async fn create_marketplace_category(
    repo: web::Data<MarketplaceRepository>,
    body: web::Json<serde_json::Value>,
) -> HttpResponse {
    let name = match body.get("name").and_then(|v| v.as_str()) {
        Some(n) if !n.trim().is_empty() => n.trim(),
        _ => return bad_request("Name is required"),
    };
    match repo.create_category(name).await {
        Ok(cat) => created_json(cat),
        Err(e) => internal_error(e),
    }
}

pub async fn update_marketplace_category(
    repo: web::Data<MarketplaceRepository>,
    path: web::Path<Uuid>,
    body: web::Json<serde_json::Value>,
) -> HttpResponse {
    let id = path.into_inner();
    let name = match body.get("name").and_then(|v| v.as_str()) {
        Some(n) if !n.trim().is_empty() => n.trim(),
        _ => return bad_request("Name is required"),
    };
    match repo.update_category(id, name).await {
        Ok(Some(cat)) => ok_json(cat),
        Ok(None) => not_found("Category not found"),
        Err(e) => internal_error(e),
    }
}

pub async fn delete_marketplace_category(
    repo: web::Data<MarketplaceRepository>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let id = path.into_inner();
    match repo.delete_category(id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => not_found("Category not found"),
        Err(e) => internal_error(e),
    }
}

// --- Marketplace Products ---

#[derive(serde::Deserialize)]
pub struct ProductListQuery {
    pub page: Option<i32>,
    pub page_size: Option<i32>,
    pub search: Option<String>,
    pub category_id: Option<Uuid>,
}

pub async fn list_marketplace_products(
    repo: web::Data<MarketplaceRepository>,
    query: web::Query<ProductListQuery>,
) -> HttpResponse {
    let pq = PaginationQuery {
        page: query.page,
        page_size: query.page_size,
        search: query.search.clone(),
    };
    match repo.list_products(&pq, query.category_id).await {
        Ok(resp) => ok_json(resp),
        Err(e) => internal_error(e),
    }
}

pub async fn create_marketplace_product(
    repo: web::Data<MarketplaceRepository>,
    body: web::Json<serde_json::Value>,
) -> HttpResponse {
    let name = match body.get("name").and_then(|v| v.as_str()) {
        Some(n) if !n.trim().is_empty() => n.trim(),
        _ => return bad_request("Name is required"),
    };
    let category_id = match body.get("category_id").and_then(|v| v.as_str()).and_then(|s| Uuid::parse_str(s).ok()) {
        Some(id) => id,
        _ => return bad_request("Valid category_id is required"),
    };
    let descriptions = body.get("descriptions").and_then(|v| v.as_str()).unwrap_or("");
    let visual = body.get("visual").and_then(|v| v.as_str()).unwrap_or("");
    let price = body.get("price").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
    let html_code = body.get("html_code").and_then(|v| v.as_str());

    match repo.create_product(name, category_id, descriptions, visual, price, html_code).await {
        Ok(prod) => created_json(prod),
        Err(e) => internal_error(e),
    }
}

pub async fn get_marketplace_product(
    repo: web::Data<MarketplaceRepository>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let id = path.into_inner();
    match repo.get_product_by_id(id).await {
        Ok(Some(prod)) => ok_json(prod),
        Ok(None) => not_found("Product not found"),
        Err(e) => internal_error(e),
    }
}

pub async fn update_marketplace_product(
    repo: web::Data<MarketplaceRepository>,
    path: web::Path<Uuid>,
    body: web::Json<serde_json::Value>,
) -> HttpResponse {
    let id = path.into_inner();
    let name = match body.get("name").and_then(|v| v.as_str()) {
        Some(n) if !n.trim().is_empty() => n.trim(),
        _ => return bad_request("Name is required"),
    };
    let category_id = match body.get("category_id").and_then(|v| v.as_str()).and_then(|s| Uuid::parse_str(s).ok()) {
        Some(id) => id,
        _ => return bad_request("Valid category_id is required"),
    };
    let descriptions = body.get("descriptions").and_then(|v| v.as_str()).unwrap_or("");
    let visual = body.get("visual").and_then(|v| v.as_str()).unwrap_or("");
    let price = body.get("price").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
    let html_code = body.get("html_code").and_then(|v| v.as_str());

    match repo.update_product(id, name, category_id, descriptions, visual, price, html_code).await {
        Ok(Some(prod)) => ok_json(prod),
        Ok(None) => not_found("Product not found"),
        Err(e) => internal_error(e),
    }
}

pub async fn delete_marketplace_product(
    repo: web::Data<MarketplaceRepository>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let id = path.into_inner();
    match repo.delete_product(id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => not_found("Product not found"),
        Err(e) => internal_error(e),
    }
}

// --- Project Categories ---

pub async fn list_project_categories(
    repo: web::Data<MarketplaceRepository>,
    query: web::Query<PaginationQuery>,
) -> HttpResponse {
    match repo.list_project_categories(&query).await {
        Ok(resp) => ok_json(resp),
        Err(e) => internal_error(e),
    }
}

pub async fn create_project_category(
    repo: web::Data<MarketplaceRepository>,
    body: web::Json<serde_json::Value>,
) -> HttpResponse {
    let name = match body.get("name").and_then(|v| v.as_str()) {
        Some(n) if !n.trim().is_empty() => n.trim(),
        _ => return bad_request("Name is required"),
    };
    match repo.create_project_category(name).await {
        Ok(cat) => created_json(cat),
        Err(e) => internal_error(e),
    }
}

pub async fn update_project_category(
    repo: web::Data<MarketplaceRepository>,
    path: web::Path<Uuid>,
    body: web::Json<serde_json::Value>,
) -> HttpResponse {
    let id = path.into_inner();
    let name = match body.get("name").and_then(|v| v.as_str()) {
        Some(n) if !n.trim().is_empty() => n.trim(),
        _ => return bad_request("Name is required"),
    };
    match repo.update_project_category(id, name).await {
        Ok(Some(cat)) => ok_json(cat),
        Ok(None) => not_found("Project category not found"),
        Err(e) => internal_error(e),
    }
}

pub async fn delete_project_category(
    repo: web::Data<MarketplaceRepository>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let id = path.into_inner();
    match repo.delete_project_category(id).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => not_found("Project category not found"),
        Err(e) => internal_error(e),
    }
}
