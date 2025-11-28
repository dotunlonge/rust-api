//! Integration tests for the API
//!
//! These tests verify the API endpoints work correctly end-to-end.

use axum::http::StatusCode;
use rust_api::{handlers, AppState};
use serde_json::json;

fn create_test_state() -> AppState {
    AppState::new()
}

#[tokio::test]
async fn test_health_check() {
    let response = handlers::health_check().await;
    let body = serde_json::to_value(&*response).unwrap();

    assert_eq!(body["status"], "healthy");
    assert_eq!(body["service"], "rust-api");
}

#[tokio::test]
async fn test_create_user() {
    let state = create_test_state();
    let payload = json!({
        "name": "John Doe",
        "email": "john@example.com"
    });

    let response = handlers::create_user(
        axum::extract::State(state),
        axum::Json(serde_json::from_value(payload).unwrap()),
    )
    .await;

    assert!(response.is_ok());
    let (status, body) = response.unwrap();
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(body.user.name, "John Doe");
    assert_eq!(body.user.email, "john@example.com");
}

#[tokio::test]
async fn test_create_user_validation() {
    let state = create_test_state();

    // Test empty name
    let payload = json!({
        "name": "",
        "email": "test@example.com"
    });

    let response = handlers::create_user(
        axum::extract::State(state.clone()),
        axum::Json(serde_json::from_value(payload).unwrap()),
    )
    .await;

    assert!(response.is_err());

    // Test invalid email
    let payload = json!({
        "name": "Test User",
        "email": "invalid-email"
    });

    let response = handlers::create_user(
        axum::extract::State(state),
        axum::Json(serde_json::from_value(payload).unwrap()),
    )
    .await;

    assert!(response.is_err());
}

#[tokio::test]
async fn test_get_user_not_found() {
    let state = create_test_state();
    let user_id = uuid::Uuid::new_v4();

    let response =
        handlers::get_user(axum::extract::Path(user_id), axum::extract::State(state)).await;

    assert!(response.is_err());
}

#[tokio::test]
async fn test_list_users_empty() {
    let state = create_test_state();

    let response = handlers::list_users(axum::extract::State(state)).await;

    assert!(response.is_ok());
    let body = response.unwrap();
    assert_eq!(body.count, 0);
    assert!(body.users.is_empty());
}
