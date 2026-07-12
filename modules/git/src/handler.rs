//! Handlers HTTP pour le module Git

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    error::GitError,
    models::*,
    service::{BranchService, RepositoryService},
    DbPool,
};

/// État partagé pour les handlers
#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: DbPool,
    pub repository_service: Arc<RepositoryService>,
    pub branch_service: Arc<BranchService>,
}

impl AppState {
    /// Crée un nouvel AppState
    pub fn new(pool: DbPool) -> Self {
        let repository_service = Arc::new(RepositoryService::new(pool.clone()));
        let branch_service = Arc::new(BranchService::new(pool.clone()));

        Self {
            pool,
            repository_service,
            branch_service,
        }
    }
}

/// Handler pour créer un repository
pub async fn create_repository(
    State(state): State<AppState>,
    Json(input): Json<CreateRepositoryInput>,
) -> Result<Json<Repository>, GitError> {
    let repository = state.repository_service.create_repository(input).await?;
    Ok(Json(repository))
}

/// Handler pour récupérer un repository par ID
pub async fn get_repository(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Repository>, GitError> {
    let repository = state
        .repository_service
        .get_repository(id)
        .await?
        .ok_or_else(|| GitError::repository_not_found(id.to_string()))?;

    Ok(Json(repository))
}

/// Handler pour lister les repositories
pub async fn list_repositories(
    State(state): State<AppState>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<RepositoryListResponse>, GitError> {
    let response = state
        .repository_service
        .list_repositories(pagination)
        .await?;

    Ok(Json(response))
}

/// Handler pour mettre à jour un repository
pub async fn update_repository(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(update): Json<UpdateRepositoryInput>,
) -> Result<Json<Repository>, GitError> {
    let repository = state
        .repository_service
        .update_repository(
            id,
            update.name,
            update.description,
            update.is_private,
            update.default_branch,
        )
        .await?;

    Ok(Json(repository))
}

/// Handler pour supprimer un repository
pub async fn delete_repository(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl axum::response::IntoResponse, GitError> {
    state.repository_service.delete_repository(id).await?;
    Ok((StatusCode::NO_CONTENT, ()))
}

/// Handler pour créer une branche
pub async fn create_branch(
    State(state): State<AppState>,
    Path(repository_id): Path<Uuid>,
    Json(input): Json<CreateBranchInput>,
) -> Result<Json<Branch>, GitError> {
    let branch = state
        .branch_service
        .create_branch(repository_id, input)
        .await?;

    Ok(Json(branch))
}

/// Handler pour récupérer une branche par ID
pub async fn get_branch(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Branch>, GitError> {
    let branch = state
        .branch_service
        .get_branch(id)
        .await?
        .ok_or_else(|| GitError::branch_not_found(id.to_string(), "unknown".to_string()))?;

    Ok(Json(branch))
}

/// Handler pour lister les branches d'un repository
pub async fn list_branches(
    State(state): State<AppState>,
    Path(repository_id): Path<Uuid>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<BranchListResponse>, GitError> {
    let response = state
        .branch_service
        .list_branches(repository_id, pagination)
        .await?;

    Ok(Json(response))
}

/// Handler pour supprimer une branche
pub async fn delete_branch(
    State(state): State<AppState>,
    Path((repository_id, id)): Path<(Uuid, Uuid)>,
) -> Result<impl axum::response::IntoResponse, GitError> {
    state
        .branch_service
        .delete_branch(id, repository_id)
        .await?;

    Ok((StatusCode::NO_CONTENT, ()))
}

/// Handler pour la santé de l'API
pub async fn health_check() -> Result<Json<serde_json::Value>, GitError> {
    let response = serde_json::json!({
        "status": "healthy",
        "module": "git",
        "timestamp": chrono::Utc::now().to_rfc3339()
    });

    Ok(Json(response))
}

/// Handler pour les informations de l'API
pub async fn api_info() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "name": crate::NAME,
        "version": crate::VERSION,
        "description": "Tardigrade-CI Git Module API",
        "endpoints": {
            "repositories": {
                "POST /repositories": "Create a new repository",
                "GET /repositories": "List all repositories",
                "GET /repositories/{id}": "Get a repository by ID",
                "PUT /repositories/{id}": "Update a repository",
                "DELETE /repositories/{id}": "Delete a repository",
            },
            "branches": {
                "POST /repositories/{id}/branches": "Create a new branch",
                "GET /repositories/{id}/branches": "List branches of a repository",
                "GET /branches/{id}": "Get a branch by ID",
                "DELETE /repositories/{id}/branches/{branch_id}": "Delete a branch",
            },
            "health": {
                "GET /health": "Health check",
                "GET /": "API information",
            }
        }
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_health_check_handler() {
        let response = health_check().await.unwrap();
        let json = response.0 ;
        assert_eq!(json["status"], "healthy");
    }

}
