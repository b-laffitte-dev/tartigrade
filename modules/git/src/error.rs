//! Error handling module for Tardigrade Git
//!
//! This module defines custom error types for the Git module.

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use thiserror::Error;

/// Git module error types
#[derive(Error, Debug)]
pub enum GitError {
    /// Database related errors
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    /// Repository not found
    #[error("Repository not found")]
    RepositoryNotFound,

    /// Repository with given name already exists
    #[error("Repository '{0}' already exists")]
    RepositoryAlreadyExists(String),

    /// Permission denied for the operation
    #[error("Permission denied")]
    PermissionDenied,

    /// Validation error with message
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// Invalid input data
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// UUID parsing error
    #[error("Invalid UUID: {0}")]
    InvalidUuid(String),

    /// Internal server error
    #[error("Internal server error: {0}")]
    InternalError(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigError(String),
}

/// Error response for API
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

impl GitError {
    /// Get the HTTP status code for this error
    pub fn status_code(&self) -> StatusCode {
        match self {
            GitError::RepositoryNotFound => StatusCode::NOT_FOUND,
            GitError::RepositoryAlreadyExists(_) => StatusCode::CONFLICT,
            GitError::PermissionDenied => StatusCode::FORBIDDEN,
            GitError::ValidationError(_) => StatusCode::BAD_REQUEST,
            GitError::InvalidInput(_) => StatusCode::BAD_REQUEST,
            GitError::InvalidUuid(_) => StatusCode::BAD_REQUEST,
            GitError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            GitError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            GitError::ConfigError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    /// Create an error response from this error
    pub fn to_error_response(&self) -> ErrorResponse {
        let (error, message, details) = match self {
            GitError::RepositoryNotFound => (
                "not_found".to_string(),
                "Repository not found".to_string(),
                None,
            ),
            GitError::RepositoryAlreadyExists(name) => (
                "conflict".to_string(),
                "Repository already exists".to_string(),
                Some(format!("Repository with name '{}' already exists", name)),
            ),
            GitError::PermissionDenied => (
                "forbidden".to_string(),
                "Permission denied".to_string(),
                None,
            ),
            GitError::ValidationError(msg) => (
                "validation_error".to_string(),
                "Validation error".to_string(),
                Some(msg.clone()),
            ),
            GitError::InvalidInput(msg) => (
                "invalid_input".to_string(),
                "Invalid input".to_string(),
                Some(msg.clone()),
            ),
            GitError::InvalidUuid(msg) => (
                "invalid_uuid".to_string(),
                "Invalid UUID".to_string(),
                Some(msg.clone()),
            ),
            GitError::Database(err) => (
                "database_error".to_string(),
                "Database error".to_string(),
                Some(err.to_string()),
            ),
            GitError::InternalError(msg) => (
                "internal_error".to_string(),
                "Internal server error".to_string(),
                Some(msg.clone()),
            ),
            GitError::ConfigError(msg) => (
                "config_error".to_string(),
                "Configuration error".to_string(),
                Some(msg.clone()),
            ),
        };

        ErrorResponse {
            error,
            message,
            details,
        }
    }
}

impl IntoResponse for GitError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let error_response = self.to_error_response();
        (status, axum::Json(error_response)).into_response()
    }
}

/// Result type alias for Git module operations
pub type GitResult<T> = Result<T, GitError>;
