//! Request handlers for API endpoints
//!
//! This module contains all the HTTP request handlers that process
//! incoming requests and return appropriate responses.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use uuid::Uuid;

use crate::error::ApiError;
use crate::models::{CreateUserRequest, UpdateUserRequest, User, UserResponse, UsersResponse};
use crate::AppState;

/// Health check endpoint
///
/// Returns a simple status message to verify the API is running.
/// Useful for monitoring and load balancer health checks.
///
/// # Returns
///
/// Returns a JSON response with status information
pub async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "rust-api",
        "timestamp": Utc::now().timestamp()
    }))
}

/// Lists all users in the system
///
/// # Arguments
///
/// * `State(state)` - Application state containing the storage
///
/// # Returns
///
/// Returns a JSON response containing all users and the total count
pub async fn list_users(State(state): State<AppState>) -> Result<Json<UsersResponse>, ApiError> {
    let storage = state.storage.read().await;
    let users = storage.get_all();

    Ok(Json(UsersResponse {
        count: users.len(),
        users,
    }))
}

/// Retrieves a specific user by ID
///
/// # Arguments
///
/// * `Path(id)` - The UUID of the user to retrieve
/// * `State(state)` - Application state containing the storage
///
/// # Returns
///
/// Returns the user if found, or a 404 error if not found
pub async fn get_user(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<UserResponse>, ApiError> {
    let storage = state.storage.read().await;

    let user = storage
        .get(&id)
        .ok_or_else(|| ApiError::NotFound(format!("User with id {} not found", id)))?;

    Ok(Json(UserResponse { user }))
}

/// Creates a new user
///
/// Validates the input and creates a new user with a generated UUID.
/// Returns an error if the email is already in use.
///
/// # Arguments
///
/// * `State(state)` - Application state containing the storage
/// * `Json(payload)` - The user creation request payload
///
/// # Returns
///
/// Returns the created user with a 201 status code, or an error
/// if validation fails or the email is already in use
pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>), ApiError> {
    // Validate input
    if payload.name.trim().is_empty() {
        return Err(ApiError::BadRequest("Name cannot be empty".to_string()));
    }

    if payload.email.trim().is_empty() {
        return Err(ApiError::BadRequest("Email cannot be empty".to_string()));
    }

    // Basic email validation
    if !payload.email.contains('@') {
        return Err(ApiError::BadRequest("Invalid email format".to_string()));
    }

    let mut storage = state.storage.write().await;

    // Check if email already exists
    if storage.email_exists(&payload.email) {
        return Err(ApiError::Conflict(format!(
            "User with email {} already exists",
            payload.email
        )));
    }

    // Create new user
    let now = Utc::now();
    let user = User {
        id: Uuid::new_v4(),
        name: payload.name.trim().to_string(),
        email: payload.email.trim().to_lowercase(),
        created_at: now,
        updated_at: now,
    };

    // Store the user
    if !storage.create(user.clone()) {
        return Err(ApiError::Internal(
            "Failed to create user due to ID collision".to_string(),
        ));
    }

    Ok((StatusCode::CREATED, Json(UserResponse { user })))
}

/// Updates an existing user
///
/// Updates the specified fields of a user. Only provided fields
/// are updated; omitted fields remain unchanged.
///
/// # Arguments
///
/// * `Path(id)` - The UUID of the user to update
/// * `State(state)` - Application state containing the storage
/// * `Json(payload)` - The user update request payload
///
/// # Returns
///
/// Returns the updated user, or a 404 error if not found
pub async fn update_user(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserResponse>, ApiError> {
    let mut storage = state.storage.write().await;

    // Validate that user exists
    if storage.get(&id).is_none() {
        return Err(ApiError::NotFound(format!("User with id {} not found", id)));
    }

    // Validate email if provided
    if let Some(ref email) = payload.email {
        if email.trim().is_empty() {
            return Err(ApiError::BadRequest("Email cannot be empty".to_string()));
        }
        if !email.contains('@') {
            return Err(ApiError::BadRequest("Invalid email format".to_string()));
        }

        // Check if email is already in use by another user
        let email_lower = email.trim().to_lowercase();
        if let Some(existing_user) = storage.get_all().iter().find(|u| u.email == email_lower) {
            if existing_user.id != id {
                return Err(ApiError::Conflict(format!(
                    "Email {} is already in use",
                    email
                )));
            }
        }
    }

    // Validate name if provided
    if let Some(ref name) = payload.name {
        if name.trim().is_empty() {
            return Err(ApiError::BadRequest("Name cannot be empty".to_string()));
        }
    }

    // Update the user
    let updated_user = storage
        .update(&id, |user| {
            if let Some(name) = &payload.name {
                user.name = name.trim().to_string();
            }
            if let Some(email) = &payload.email {
                user.email = email.trim().to_lowercase();
            }
            user.updated_at = Utc::now();
        })
        .then(|| storage.get(&id))
        .flatten()
        .ok_or_else(|| ApiError::Internal("Failed to update user".to_string()))?;

    Ok(Json(UserResponse { user: updated_user }))
}

/// Deletes a user from the system
///
/// # Arguments
///
/// * `Path(id)` - The UUID of the user to delete
/// * `State(state)` - Application state containing the storage
///
/// # Returns
///
/// Returns a 204 No Content status on success, or a 404 error if not found
pub async fn delete_user(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<StatusCode, ApiError> {
    let mut storage = state.storage.write().await;

    if !storage.delete(&id) {
        return Err(ApiError::NotFound(format!("User with id {} not found", id)));
    }

    Ok(StatusCode::NO_CONTENT)
}
