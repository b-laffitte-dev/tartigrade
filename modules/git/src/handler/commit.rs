//! HTTP handlers for commit operations
//!
//! This module provides Axum handlers for managing Git commits.

use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::error::GitError;
use crate::models::{CloneRepositoryRequest, CloneRepositoryResponse, Commit, CreateCommitInput, PaginatedResponse, PushRequest, PushResponse};
use crate::AppState;

/// Query parameters for listing commits
#[derive(Debug, Deserialize)]
pub struct ListCommitsQuery {
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

impl Default for ListCommitsQuery {
    fn default() -> Self {
        Self {
            page: default_page(),
            page_size: default_page_size(),
        }
    }
}

/// Create a new commit in a branch
///
/// POST /repositories/:repository_id/branches/:branch_name/commits
pub async fn create_commit(
    State(state): State<Arc<AppState>>,
    Path((repository_id, branch_name)): Path<(Uuid, String)>,
    Json(input): Json<CreateCommitInput>,
) -> Result<Json<Commit>, GitError> {
    // For now, use fixed committer info for testing
    let committer_id = Uuid::parse_str("00000000-0000-0000-0000-000000000000")
        .map_err(|_| GitError::InternalError("Invalid committer ID".to_string()))?;
    let committer_name = "test-user".to_string();
    let committer_email = "test@example.com".to_string();

    tracing::info!(
        "Creating commit in repository: {} branch: {} with message: {}",
        repository_id,
        branch_name,
        input.message
    );

    let commit = crate::repository::create_commit(
        &state.pool,
        repository_id,
        &branch_name,
        input,
        committer_id,
        &committer_name,
        &committer_email,
    )
    .await?;

    Ok(Json(commit))
}

/// List commits for a branch
///
/// GET /repositories/:repository_id/branches/:branch_name/commits
pub async fn list_commits(
    State(state): State<Arc<AppState>>,
    Path((repository_id, branch_name)): Path<(Uuid, String)>,
    Query(query): Query<ListCommitsQuery>,
) -> Result<Json<PaginatedResponse<Commit>>, GitError> {
    tracing::info!(
        "Listing commits for repository: {} branch: {}",
        repository_id,
        branch_name
    );

    let commits = crate::repository::list_commits(
        &state.pool,
        repository_id,
        &branch_name,
        query.page,
        query.page_size,
    )
    .await?;

    Ok(Json(commits))
}

/// Get a specific commit by ID
///
/// GET /commits/:commit_id
pub async fn get_commit(
    State(state): State<Arc<AppState>>,
    Path(commit_id): Path<Uuid>,
) -> Result<Json<Commit>, GitError> {
    tracing::info!("Getting commit: {}", commit_id);

    let commit = crate::repository::get_commit(&state.pool, commit_id).await?;

    match commit {
        Some(c) => Ok(Json(c)),
        None => Err(GitError::CommitNotFound),
    }
}

/// Clone a repository
///
/// POST /repositories/:repository_id/clone
pub async fn clone_repository(
    State(state): State<Arc<AppState>>,
    Path(repository_id): Path<Uuid>,
    Json(request): Json<CloneRepositoryRequest>,
) -> Result<Json<CloneRepositoryResponse>, GitError> {
    tracing::info!("Cloning repository: {}", repository_id);

    // Use the repository_id from path
    let url = crate::repository::clone_repository(
        &state.pool,
        repository_id,
        request.user_id,
        &request.method,
    )
    .await?;

    Ok(Json(CloneRepositoryResponse {
        url,
        success: true,
        error: None,
    }))
}

/// Push to a repository
///
/// POST /repositories/:repository_id/push
pub async fn push_to_repository(
    State(state): State<Arc<AppState>>,
    Path(repository_id): Path<Uuid>,
    Json(request): Json<PushRequest>,
) -> Result<Json<PushResponse>, GitError> {
    tracing::info!("Pushing to repository: {}", repository_id);

    // Use fixed committer info for now
    let committer_name = "test-user".to_string();
    let committer_email = "test@example.com".to_string();

    // Create commit input from request
    let input = CreateCommitInput {
        message: request.message,
        author_name: request.author_name,
        author_email: request.author_email,
    };

    let commit_hash = crate::repository::push_to_repository(
        &state.pool,
        repository_id,
        &request.branch_name,
        input,
        request.user_id,
        &committer_name,
        &committer_email,
    )
    .await?;

    Ok(Json(PushResponse {
        success: true,
        commit_hash: Some(commit_hash),
        error: None,
    }))
}
