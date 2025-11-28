//! # Rust API
//!
//! A clean, production-ready REST API built with Rust and Axum.
//! This API demonstrates best practices for error handling, documentation,
//! and maintainable code structure.

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

use rust_api::{handlers, AppState};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for structured logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_api=debug,tower_http=debug".into()),
        )
        .init();

    let app_state = AppState::new();

    // Build the application router
    let app = Router::new()
        .route("/", get(handlers::health_check))
        .route("/api/v1/users", get(handlers::list_users))
        .route("/api/v1/users", post(handlers::create_user))
        .route("/api/v1/users/:id", get(handlers::get_user))
        .route("/api/v1/users/:id", put(handlers::update_user))
        .route("/api/v1/users/:id", delete(handlers::delete_user))
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    // Bind to address and start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
