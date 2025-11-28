//! # Rust API Library
//!
//! This library module exposes the core components of the API
//! for use in tests and as a library.

pub mod error;
pub mod handlers;
pub mod models;

pub use crate::models::Storage;

/// Application state shared across all handlers
#[derive(Clone)]
pub struct AppState {
    /// In-memory storage for demonstration purposes
    /// In production, this would be a database connection pool
    pub storage: std::sync::Arc<tokio::sync::RwLock<models::Storage>>,
}

impl AppState {
    /// Creates a new application state with empty storage
    pub fn new() -> Self {
        Self {
            storage: std::sync::Arc::new(tokio::sync::RwLock::new(models::Storage::default())),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
