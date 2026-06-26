//! Commit HTTP handlers for Tardigrade Git module
//!
//! This module implements the HTTP handlers for commit operations.

use crate::error::GitError;
use crate::models::commit::{Commit, CreateCommitInput, ListCommitsQuery, PaginatedResponse};
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Json, Router};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

use crate::AppState;

/// Create a new commit
pub async fn create_commit_handler(
    State(state): State<Arc<AppState>>,
    Path(repository_id): Path<Uuid>,
    Json(input): Json<CreateCommitInput>,
) -> Result<Json<Commit>, GitError> {
    tracing::info!(
        "Creating commit: {} for repository: {}",
        input.hash,
        repository_id
    );

    let commit = crate::repository::commit::create_commit(&state.pool, repository_id, input).await?;

    Ok(Json(commit))
}

/// Get a commit by ID
pub async fn get_commit_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Commit>, GitError> {
    tracing::info!("Getting commit: {}", id);

    let commit = crate::repository::commit::get_commit(&state.pool, id)
        .await?
        .ok_or(GitError::CommitNotFound)?;

    Ok(Json(commit))
}

/// Get a commit by hash and repository ID
pub async fn get_commit_by_hash_handler(
    State(state): State<Arc<AppState>>,
    Path((repository_id, hash)): Path<(Uuid, String)>,
) -> Result<Json<Commit>, GitError> {
    tracing::info!("Getting commit: {} for repository: {}", hash, repository_id);

    let commit = crate::repository::commit::get_commit_by_hash(&state.pool, repository_id, &hash)
        .await?
        .ok_or(GitError::CommitNotFound)?;

    Ok(Json(commit))
}

/// List commits for a repository with pagination
pub async fn list_commits_handler(
    State(state): State<Arc<AppState>>,
    Path(repository_id): Path<Uuid>,
    Query(query): Query<ListCommitsQuery>,
) -> Result<Json<PaginatedResponse<Commit>>, GitError> {
    tracing::info!(
        "Listing commits for repository: {}, page={}, page_size={}",
        repository_id,
        query.page,
        query.page_size
    );

    // Override repository_id from path
    let mut query = query;
    query.repository_id = repository_id;

    let response = crate::repository::commit::list_commits(&state.pool, query).await?;

    Ok(Json(response))
}

/// List commits for a specific branch
pub async fn list_commits_by_branch_handler(
    State(state): State<Arc<AppState>>,
    Path((repository_id, branch_name)): Path<(Uuid, String)>,
    Query(query): Query<ListCommitsQuery>,
) -> Result<Json<PaginatedResponse<Commit>>, GitError> {
    tracing::info!(
        "Listing commits for repository: {} and branch: {}, page={}, page_size={}",
        repository_id,
        branch_name,
        query.page,
        query.page_size
    );

    // Override repository_id and branch_name from path
    let mut query = query;
    query.repository_id = repository_id;
    query.branch_name = Some(branch_name);

    let response = crate::repository::commit::list_commits(&state.pool, query).await?;

    Ok(Json(response))
}

/// Get the latest commit for a branch
pub async fn get_latest_commit_handler(
    State(state): State<Arc<AppState>>,
    Path((repository_id, branch_name)): Path<(Uuid, String)>,
) -> Result<Json<Commit>, GitError> {
    tracing::info!(
        "Getting latest commit for repository: {} and branch: {}",
        repository_id,
        branch_name
    );

    let commit = crate::repository::commit::get_latest_commit(&state.pool, repository_id, &branch_name)
        .await?
        .ok_or(GitError::CommitNotFound)?;

    Ok(Json(commit))
}
