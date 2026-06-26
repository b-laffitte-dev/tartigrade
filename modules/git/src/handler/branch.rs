//! Branch HTTP handlers for Tardigrade Git module
//!
//! This module implements the HTTP handlers for branch operations.

use crate::error::GitError;
use crate::models::branch::{
    Branch, CreateBranchInput, ListBranchesQuery, PaginatedResponse, UpdateBranchInput,
};
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Json, Router};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

use crate::AppState;

/// Create a new branch
pub async fn create_branch_handler(
    State(state): State<Arc<AppState>>,
    Path(repository_id): Path<Uuid>,
    Json(input): Json<CreateBranchInput>,
) -> Result<Json<Branch>, GitError> {
    tracing::info!(
        "Creating branch: {} for repository: {}",
        input.name,
        repository_id
    );

    let branch = crate::repository::branch::create_branch(&state.pool, repository_id, input).await?;

    Ok(Json(branch))
}

/// Get a branch by ID
pub async fn get_branch_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Branch>, GitError> {
    tracing::info!("Getting branch: {}", id);

    let branch = crate::repository::branch::get_branch(&state.pool, id)
        .await?
        .ok_or(GitError::BranchNotFound)?;

    Ok(Json(branch))
}

/// Get a branch by name and repository ID
pub async fn get_branch_by_name_handler(
    State(state): State<Arc<AppState>>,
    Path((repository_id, name)): Path<(Uuid, String)>,
) -> Result<Json<Branch>, GitError> {
    tracing::info!("Getting branch: {} for repository: {}", name, repository_id);

    let branch = crate::repository::branch::get_branch_by_name(&state.pool, repository_id, &name)
        .await?
        .ok_or(GitError::BranchNotFound)?;

    Ok(Json(branch))
}

/// List branches for a repository with pagination
pub async fn list_branches_handler(
    State(state): State<Arc<AppState>>,
    Path(repository_id): Path<Uuid>,
    Query(query): Query<ListBranchesQuery>,
) -> Result<Json<PaginatedResponse<Branch>>, GitError> {
    tracing::info!(
        "Listing branches for repository: {}, page={}, page_size={}",
        repository_id,
        query.page,
        query.page_size
    );

    // Override repository_id from path
    let mut query = query;
    query.repository_id = repository_id;

    let response = crate::repository::branch::list_branches(&state.pool, query).await?;

    Ok(Json(response))
}

/// Update a branch
pub async fn update_branch_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateBranchInput>,
) -> Result<Json<Branch>, GitError> {
    tracing::info!("Updating branch: {}", id);

    let branch = crate::repository::branch::update_branch(&state.pool, id, input).await?;

    Ok(Json(branch))
}

/// Delete a branch
pub async fn delete_branch_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, GitError> {
    tracing::info!("Deleting branch: {}", id);

    crate::repository::branch::delete_branch(&state.pool, id).await?;

    Ok(Json(json!({
        "success": true,
        "message": "Branch deleted successfully"
    })))
}

/// Get the default branch for a repository
pub async fn get_default_branch_handler(
    State(state): State<Arc<AppState>>,
    Path(repository_id): Path<Uuid>,
) -> Result<Json<Branch>, GitError> {
    tracing::info!("Getting default branch for repository: {}", repository_id);

    let branch = crate::repository::branch::get_default_branch(&state.pool, repository_id)
        .await?
        .ok_or(GitError::BranchNotFound)?;

    Ok(Json(branch))
}
