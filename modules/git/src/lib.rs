//! Tardigrade Git Module
//!
//! This module provides Git repository management functionality for Tardigrade-CI.
//! It includes models, repository operations, and HTTP handlers for managing Git repositories.

#![allow(dead_code)]
#![allow(unused_imports)]

pub mod config;
pub mod error;
pub mod models;
pub mod repository;
pub mod service;

pub mod handler;
pub mod routes;

// Re-export main types for easier access
pub use config::*;
pub use error::*;
pub use models::*;
pub use repository::*;
pub use service::*;

use std::sync::Arc;

/// Application state shared across all handlers
#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::postgres::PgPool,
}

impl AppState {
    /// Create a new AppState instance
    pub fn new(pool: sqlx::postgres::PgPool) -> Self {
        Self { pool }
    }
}

/// Create a new AppState with Arc wrapper for thread-safe sharing
pub fn create_app_state(pool: sqlx::postgres::PgPool) -> Arc<AppState> {
    Arc::new(AppState::new(pool))
}
