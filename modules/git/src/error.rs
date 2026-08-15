//! Gestion des erreurs spécifiques au module Git

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;

/// Type d'erreur pour le module Git
#[derive(Error, Debug, Serialize)]
pub enum GitError {
    /// Repository non trouvé
    #[error("Repository '{0}' not found")]
    RepositoryNotFound(String),

    /// Repository déjà existant
    #[error("Repository '{0}' already exists")]
    RepositoryAlreadyExists(String),

    /// Branche non trouvée
    #[error("Branch '{0}' not found in repository '{1}'")]
    BranchNotFound(String, String),

    /// Branche déjà existante
    #[error("Branch '{0}' already exists in repository '{1}'")]
    BranchAlreadyExists(String, String),

    /// Erreur de validation
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// Erreur de base de données
    #[error("Database error: {0}")]
    DatabaseError(String),

    /// Erreur interne
    #[error("Internal server error")]
    InternalError,
}

impl GitError {
    /// Crée une erreur RepositoryNotFound
    pub fn repository_not_found(id: impl Into<String>) -> Self {
        Self::RepositoryNotFound(id.into())
    }

    /// Crée une erreur RepositoryAlreadyExists
    pub fn repository_exists(name: impl Into<String>) -> Self {
        Self::RepositoryAlreadyExists(name.into())
    }

    /// Crée une erreur BranchNotFound
    pub fn branch_not_found(branch: impl Into<String>, repo: impl Into<String>) -> Self {
        Self::BranchNotFound(branch.into(), repo.into())
    }

    /// Crée une erreur BranchAlreadyExists
    pub fn branch_exists(branch: impl Into<String>, repo: impl Into<String>) -> Self {
        Self::BranchAlreadyExists(branch.into(), repo.into())
    }

    /// Crée une erreur de validation
    pub fn validation(msg: impl Into<String>) -> Self {
        Self::ValidationError(msg.into())
    }

    /// Crée une erreur de base de données
    pub fn database(msg: impl Into<String>) -> Self {
        Self::DatabaseError(msg.into())
    }
}

/// Conversion depuis sqlx::Error
impl From<sqlx::Error> for GitError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => Self::repository_not_found("unknown"),
            _ => Self::database(err.to_string()),
        }
    }
}

/// Conversion depuis tardigrade_common::TardigradeError
impl From<tardigrade_common::TardigradeError> for GitError {
    fn from(err: tardigrade_common::TardigradeError) -> Self {
        match err {
            tardigrade_common::TardigradeError::NotFound(resource) => {
                Self::RepositoryNotFound(resource)
            }
            tardigrade_common::TardigradeError::Conflict(resource) => {
                Self::RepositoryAlreadyExists(resource)
            }
            tardigrade_common::TardigradeError::Validation(msg) => Self::ValidationError(msg),
            tardigrade_common::TardigradeError::Database(msg) => Self::DatabaseError(msg),
            _ => Self::InternalError,
        }
    }
}

/// Implémentation de IntoResponse pour retourner des erreurs HTTP appropriées
impl IntoResponse for GitError {
    fn into_response(self) -> Response {
        let status = match self {
            Self::RepositoryNotFound(_) => StatusCode::NOT_FOUND,
            Self::RepositoryAlreadyExists(_) => StatusCode::CONFLICT,
            Self::BranchNotFound(_, _) => StatusCode::NOT_FOUND,
            Self::BranchAlreadyExists(_, _) => StatusCode::CONFLICT,
            Self::ValidationError(_) => StatusCode::BAD_REQUEST,
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = serde_json::json!({
            "error": self.to_string(),
            "status": status.as_u16(),
        });

        (status, axum::Json(body)).into_response()
    }
}

/// Type de résultat pour le module Git
pub type GitResult<T> = Result<T, GitError>;
