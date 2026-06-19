//! Repository HTTP handlers for Tardigrade Git module
//!
//! This module implements the HTTP handlers for repository operations.

use crate::error::GitError;
use crate::models::{
    CreateRepositoryInput, ListRepositoriesQuery, PaginatedResponse, Repository,
    UpdateRepositoryInput,
};
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Json, Router};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

use crate::AppState;

/// Create a new repository
pub async fn create_repository_handler(
    State(state): State<Arc<AppState>>,
    Json(input): Json<CreateRepositoryInput>,
) -> Result<Json<Repository>, GitError> {
    // For now, use a fixed owner_id for testing
    // In production, this would come from authentication (JWT, session, etc.)
    let owner_id = Uuid::parse_str("00000000-0000-0000-0000-000000000000")
        .map_err(|_| GitError::InternalError("Invalid owner ID".to_string()))?;

    // In a real implementation, we would extract the owner_id from the request context
    // For example, from a JWT claim or session
    // Let's allow it to be passed via a header or use the fixed one for now

    tracing::info!(
        "Creating repository: {} with owner: {}",
        input.name,
        owner_id
    );

    let repo = crate::repository::create_repository(&state.pool, input, owner_id).await?;

    Ok(Json(repo))
}

/// Get a repository by ID
pub async fn get_repository_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Repository>, GitError> {
    tracing::info!("Getting repository: {}", id);

    let repo = crate::repository::get_repository(&state.pool, id)
        .await?
        .ok_or(GitError::RepositoryNotFound)?;

    Ok(Json(repo))
}

/// Get a repository by ID with owner check (for authenticated users)
pub async fn get_repository_by_owner_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Repository>, GitError> {
    // For now, use a fixed owner_id
    let owner_id = Uuid::parse_str("00000000-0000-0000-0000-000000000000")
        .map_err(|_| GitError::InternalError("Invalid owner ID".to_string()))?;

    tracing::info!("Getting repository: {} for owner: {}", id, owner_id);

    let repo = crate::repository::get_repository_by_id_and_owner(&state.pool, id, owner_id)
        .await?
        .ok_or(GitError::RepositoryNotFound)?;

    Ok(Json(repo))
}

/// List repositories with pagination
pub async fn list_repositories_handler(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ListRepositoriesQuery>,
) -> Result<Json<PaginatedResponse<Repository>>, GitError> {
    tracing::info!(
        "Listing repositories: page={}, page_size={}, owner_id={:?}",
        query.page,
        query.page_size,
        query.owner_id
    );

    let response = crate::repository::list_repositories(&state.pool, query).await?;

    Ok(Json(response))
}

/// Update a repository
pub async fn update_repository_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateRepositoryInput>,
) -> Result<Json<Repository>, GitError> {
    // For now, use a fixed owner_id
    let owner_id = Uuid::parse_str("00000000-0000-0000-0000-000000000000")
        .map_err(|_| GitError::InternalError("Invalid owner ID".to_string()))?;

    tracing::info!("Updating repository: {} with owner: {}", id, owner_id);

    let repo = crate::repository::update_repository(&state.pool, id, owner_id, input).await?;

    Ok(Json(repo))
}

/// Delete a repository
pub async fn delete_repository_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, GitError> {
    // For now, use a fixed owner_id
    let owner_id = Uuid::parse_str("00000000-0000-0000-0000-000000000000")
        .map_err(|_| GitError::InternalError("Invalid owner ID".to_string()))?;

    tracing::info!("Deleting repository: {} with owner: {}", id, owner_id);

    crate::repository::delete_repository(&state.pool, id, owner_id).await?;

    Ok(Json(json!({
        "success": true,
        "message": "Repository deleted successfully"
    })))
}

/// Health check handler
pub async fn health_check_handler() -> impl IntoResponse {
    Json(json!({
        "status": "healthy",
        "module": "git",
        "version": "0.1.0",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

/// Get API version and info
pub async fn get_api_info_handler() -> impl IntoResponse {
    Json(json!({
        "name": "Tardigrade Git API",
        "version": "0.1.0",
        "description": "Git repository management API for Tardigrade-CI",
        "endpoints": {
            "POST /repositories": "Create a new repository",
            "GET /repositories": "List repositories (with pagination)",
            "GET /repositories/:id": "Get a repository by ID",
            "PUT /repositories/:id": "Update a repository",
            "DELETE /repositories/:id": "Delete a repository",
            "GET /health": "Health check"
        }
    }))
}
