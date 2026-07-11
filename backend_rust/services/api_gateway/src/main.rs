use actix_web::{web, App, HttpServer, middleware, HttpResponse, Responder};
use actix_cors::Cors;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::env;

async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "api_gateway"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "api_gateway=debug,actix_web=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let port = env::var("GATEWAY_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("GATEWAY_PORT must be a valid port number");

    tracing::info!("Starting API Gateway on port {}", port);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::permissive()
                    .allowed_origin_fn(|origin, _req_head| {
                        origin.as_bytes().starts_with(b"http://localhost")
                            || origin.as_bytes().starts_with(b"https://rinova.app")
                    })
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![
                        actix_web::http::header::AUTHORIZATION,
                        actix_web::http::header::CONTENT_TYPE,
                    ])
                    .supports_credentials()
                    .max_age(3600),
            )
            .route("/health", web::get().to(health_check))
            .route("/api/v1/{path:.*}", web::route().to(|| async {
                HttpResponse::Ok().json(serde_json::json!({
                    "message": "API Gateway - Route forwarding not implemented yet"
                }))
            }))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
