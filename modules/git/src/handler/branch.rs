//! HTTP handlers for branch operations
//!
//! This module provides Axum handlers for managing Git branches.

use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::error::GitError;
use crate::models::{Branch, CreateBranchInput, PaginatedResponse};
use crate::AppState;

/// Query parameters for listing branches
#[derive(Debug, Deserialize)]
pub struct ListBranchesQuery {
    /// Page number (1-indexed, default: 1)
    #[serde(default = "default_page")]
    pub page: i32,
    /// Number of items per page (default: 20, max: 100)
    #[serde(default = "default_page_size")]
    pub page_size: i32,
}

fn default_page() -> i32 {
    1
}

fn default_page_size() -> i32 {
    20
}

impl Default for ListBranchesQuery {
    fn default() -> Self {
        Self {
            page: default_page(),
            page_size: default_page_size(),
        }
    }
}

/// Create a new branch in a repository
///
/// POST /repositories/:repository_id/branches
pub async fn create_branch(
    State(state): State<Arc<AppState>>,
    Path(repository_id): Path<Uuid>,
    Json(input): Json<CreateBranchInput>,
) -> Result<Json<Branch>, GitError> {
    // For now, use a fixed owner_id for testing
    // In production, this would come from authentication (JWT, session, etc.)
    let owner_id = Uuid::parse_str("00000000-0000-0000-0000-000000000000")
        .map_err(|_| GitError::InternalError("Invalid owner ID".to_string()))?;

    tracing::info!(
        "Creating branch: {} in repository: {} with owner: {}",
        input.name,
        repository_id,
        owner_id
    );

    let branch = crate::repository::create_branch(&state.pool, repository_id, input, owner_id).await?;

    Ok(Json(branch))
}

/// List branches for a repository
///
/// GET /repositories/:repository_id/branches
pub async fn list_branches(
    State(state): State<Arc<AppState>>,
    Path(repository_id): Path<Uuid>,
    Query(query): Query<ListBranchesQuery>,
) -> Result<Json<PaginatedResponse<Branch>>, GitError> {
    tracing::info!("Listing branches for repository: {}", repository_id);

    let branches = crate::repository::list_branches(
        &state.pool,
        repository_id,
        query.page,
        query.page_size,
    )
    .await?;

    Ok(Json(branches))
}

/// Get a specific branch by name
///
/// GET /repositories/:repository_id/branches/:branch_name
pub async fn get_branch(
    State(state): State<Arc<AppState>>,
    Path((repository_id, branch_name)): Path<(Uuid, String)>,
) -> Result<Json<Branch>, GitError> {
    tracing::info!(
        "Getting branch: {} from repository: {}",
        branch_name,
        repository_id
    );

    let branch = crate::repository::get_branch_by_repository_and_name(
        &state.pool,
        repository_id,
        &branch_name,
    )
    .await?;

    match branch {
        Some(b) => Ok(Json(b)),
        None => Err(GitError::BranchNotFound),
    }
}

/// Delete a branch from a repository
///
/// DELETE /repositories/:repository_id/branches/:branch_name
pub async fn delete_branch(
    State(state): State<Arc<AppState>>,
    Path((repository_id, branch_name)): Path<(Uuid, String)>,
) -> Result<Json<()>, GitError> {
    // For now, use a fixed owner_id for testing
    let owner_id = Uuid::parse_str("00000000-0000-0000-0000-000000000000")
        .map_err(|_| GitError::InternalError("Invalid owner ID".to_string()))?;

    tracing::info!(
        "Deleting branch: {} from repository: {} with owner: {}",
        branch_name,
        repository_id,
        owner_id
    );

    crate::repository::delete_branch(&state.pool, repository_id, &branch_name, owner_id).await?;

    Ok(Json(()))
}
