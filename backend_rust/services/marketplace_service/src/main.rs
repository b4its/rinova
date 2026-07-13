use actix_web::{web, App, HttpServer, middleware};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod handlers;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8090".to_string());
    let addr = format!("{}:{}", host, port);

    tracing::info!("Starting Marketplace Service on {}", addr);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api/v1/marketplace")
                    .route("/components", web::get().to(handlers::list_components))
                    .route("/purchase", web::post().to(handlers::purchase_component))
                    .route("/add-to-library", web::post().to(handlers::add_to_library))
                    .route("/library", web::get().to(handlers::get_library))
            )
            .route("/health", web::get().to(health_check))
    })
    .bind(&addr)?
    .run()
    .await
}

async fn health_check() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "marketplace_service"
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_health_check() {
        let app = test::init_service(App::new().route("/health", web::get().to(health_check))).await;
        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
