use actix_web::{web, App, HttpServer, middleware, HttpResponse, Responder};
use actix_cors::Cors;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::env;

async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "project_service"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "project_service=debug,actix_web=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let port = env::var("SERVICE_PORT")
        .unwrap_or_else(|_| "8003".to_string())
        .parse::<u16>()
        .expect("SERVICE_PORT must be a valid port number");

    tracing::info!("Starting Project Service on port {}", port);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(Cors::permissive())
            .route("/health", web::get().to(health_check))
            .route("/api/v1/projects", web::get().to(|| async {
                HttpResponse::Ok().json(serde_json::json!({
                    "projects": [],
                    "message": "Project Service - Not implemented yet"
                }))
            }))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
