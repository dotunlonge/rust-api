//! Data models and storage structures
//!
//! This module defines the core data structures used throughout the API,
//! including request/response models and in-memory storage.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Represents a user in the system
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    /// Unique identifier for the user
    pub id: Uuid,
    /// User's full name
    pub name: String,
    /// User's email address
    pub email: String,
    /// Timestamp when the user was created
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    /// Timestamp when the user was last updated
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<Utc>,
}

/// Request payload for creating a new user
#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    /// User's full name
    pub name: String,
    /// User's email address
    pub email: String,
}

/// Request payload for updating an existing user
#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    /// Optional new name for the user
    pub name: Option<String>,
    /// Optional new email for the user
    pub email: Option<String>,
}

/// Response wrapper for user data
#[derive(Debug, Serialize)]
pub struct UserResponse {
    /// The user data
    pub user: User,
}

/// Response wrapper for a list of users
#[derive(Debug, Serialize)]
pub struct UsersResponse {
    /// List of users
    pub users: Vec<User>,
    /// Total count of users
    pub count: usize,
}

/// In-memory storage for users
///
/// In a production environment, this would be replaced with
/// a proper database connection pool.
#[derive(Debug, Default)]
pub struct Storage {
    users: HashMap<Uuid, User>,
}

impl Storage {
    /// Creates a new empty storage instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Retrieves all users from storage
    pub fn get_all(&self) -> Vec<User> {
        self.users.values().cloned().collect()
    }

    /// Retrieves a user by ID
    ///
    /// # Arguments
    ///
    /// * `id` - The UUID of the user to retrieve
    ///
    /// # Returns
    ///
    /// Returns `Some(User)` if found, `None` otherwise
    pub fn get(&self, id: &Uuid) -> Option<User> {
        self.users.get(id).cloned()
    }

    /// Creates a new user in storage
    ///
    /// # Arguments
    ///
    /// * `user` - The user to store
    ///
    /// # Returns
    ///
    /// Returns `true` if the user was created, `false` if a user
    /// with the same ID already exists
    pub fn create(&mut self, user: User) -> bool {
        if self.users.contains_key(&user.id) {
            return false;
        }
        self.users.insert(user.id, user);
        true
    }

    /// Updates an existing user in storage
    ///
    /// # Arguments
    ///
    /// * `id` - The UUID of the user to update
    /// * `updater` - A closure that receives a mutable reference to the user
    ///
    /// # Returns
    ///
    /// Returns `true` if the user was updated, `false` if not found
    pub fn update<F>(&mut self, id: &Uuid, updater: F) -> bool
    where
        F: FnOnce(&mut User),
    {
        if let Some(user) = self.users.get_mut(id) {
            updater(user);
            true
        } else {
            false
        }
    }

    /// Deletes a user from storage
    ///
    /// # Arguments
    ///
    /// * `id` - The UUID of the user to delete
    ///
    /// # Returns
    ///
    /// Returns `true` if the user was deleted, `false` if not found
    pub fn delete(&mut self, id: &Uuid) -> bool {
        self.users.remove(id).is_some()
    }

    /// Checks if a user with the given email exists
    ///
    /// # Arguments
    ///
    /// * `email` - The email address to check
    ///
    /// # Returns
    ///
    /// Returns `true` if a user with this email exists, `false` otherwise
    pub fn email_exists(&self, email: &str) -> bool {
        self.users.values().any(|user| user.email == email)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_user(id: Uuid, name: &str, email: &str) -> User {
        let now = Utc::now();
        User {
            id,
            name: name.to_string(),
            email: email.to_string(),
            created_at: now,
            updated_at: now,
        }
    }

    #[test]
    fn test_storage_create_and_get() {
        let mut storage = Storage::new();
        let user_id = Uuid::new_v4();
        let user = create_test_user(user_id, "Test User", "test@example.com");

        assert!(storage.create(user.clone()));
        assert_eq!(storage.get(&user_id), Some(user));
    }

    #[test]
    fn test_storage_get_all() {
        let mut storage = Storage::new();
        let user1 = create_test_user(Uuid::new_v4(), "User 1", "user1@example.com");
        let user2 = create_test_user(Uuid::new_v4(), "User 2", "user2@example.com");

        storage.create(user1);
        storage.create(user2);

        let all_users = storage.get_all();
        assert_eq!(all_users.len(), 2);
    }

    #[test]
    fn test_storage_update() {
        let mut storage = Storage::new();
        let user_id = Uuid::new_v4();
        let user = create_test_user(user_id, "Original Name", "original@example.com");

        storage.create(user);

        let updated = storage.update(&user_id, |u| {
            u.name = "Updated Name".to_string();
        });

        assert!(updated);
        let updated_user = storage.get(&user_id).unwrap();
        assert_eq!(updated_user.name, "Updated Name");
    }

    #[test]
    fn test_storage_delete() {
        let mut storage = Storage::new();
        let user_id = Uuid::new_v4();
        let user = create_test_user(user_id, "Test User", "test@example.com");

        storage.create(user);
        assert!(storage.get(&user_id).is_some());

        assert!(storage.delete(&user_id));
        assert!(storage.get(&user_id).is_none());
    }

    #[test]
    fn test_storage_email_exists() {
        let mut storage = Storage::new();
        let user = create_test_user(Uuid::new_v4(), "Test User", "test@example.com");

        storage.create(user);
        assert!(storage.email_exists("test@example.com"));
        assert!(!storage.email_exists("nonexistent@example.com"));
    }

    #[test]
    fn test_storage_duplicate_id() {
        let mut storage = Storage::new();
        let user_id = Uuid::new_v4();
        let user1 = create_test_user(user_id, "User 1", "user1@example.com");
        let user2 = create_test_user(user_id, "User 2", "user2@example.com");

        assert!(storage.create(user1));
        assert!(!storage.create(user2)); // Should fail due to duplicate ID
    }
}
