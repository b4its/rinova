use actix_web::{web, App, HttpServer, middleware};
use actix_cors::Cors;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use database::{create_pool, DbPool};
use messaging::ZeroMQPublisher;
use std::env;

mod handlers;
mod models;
mod repository;
mod services;

#[derive(Clone)]
pub struct AppState {
    pub db: DbPool,
    pub zmq_publisher: ZeroMQPublisher,
    pub jwt_secret: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables
    dotenv::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "user_service=debug,actix_web=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Create database pool
    let db = create_pool()
        .await
        .expect("Failed to create database pool");

    tracing::info!("Database connection pool created successfully");

    // Create ZeroMQ publisher
    let zmq_url = env::var("ZMQ_PUBLISHER_URL")
        .unwrap_or_else(|_| "tcp://localhost:5555".to_string());
    let zmq_publisher = ZeroMQPublisher::new(&zmq_url)
        .expect("Failed to create ZeroMQ publisher");

    tracing::info!("ZeroMQ publisher connected to {}", zmq_url);

    // Get JWT secret
    let jwt_secret = env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set");

    // Create app state
    let state = AppState {
        db,
        zmq_publisher,
        jwt_secret,
    };

    // Get server port
    let port = env::var("SERVICE_PORT")
        .unwrap_or_else(|_| "8001".to_string())
        .parse::<u16>()
        .expect("SERVICE_PORT must be a valid port number");

    tracing::info!("Starting User Service on port {}", port);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
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
            .service(handlers::health_check)
            .service(
                web::scope("/api/v1/auth")
                    .route("/register", web::post().to(handlers::register))
                    .route("/login", web::post().to(handlers::login))
                    .route("/verify-email", web::post().to(handlers::verify_email))
                    .route("/resend-verification", web::post().to(handlers::resend_verification))
                    .route("/logout", web::post().to(handlers::logout)),
            )
            .service(
                web::scope("/api/v1/users")
                    .route("/me", web::get().to(handlers::get_current_user))
                    .route("/me", web::put().to(handlers::update_user)),
            )
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
