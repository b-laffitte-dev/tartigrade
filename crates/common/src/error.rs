//! Module de gestion des erreurs communes

use serde::Serialize;
use thiserror::Error;

/// Type d'erreur de base pour les modules Tardigrade
#[derive(Error, Debug, Serialize)]
pub enum TardigradeError {
    /// Erreur de base de données
    #[error("Database error: {0}")]
    Database(String),

    /// Erreur de configuration
    #[error("Configuration error: {0}")]
    Configuration(String),

    /// Erreur de validation
    #[error("Validation error: {0}")]
    Validation(String),

    /// Ressource non trouvée
    #[error("Not found: {0}")]
    NotFound(String),

    /// Conflit (ressource déjà existante)
    #[error("Conflict: {0}")]
    Conflict(String),

    /// Erreur interne du serveur
    #[error("Internal server error")]
    InternalServerError,

    /// Erreur d'authentification (pour plus tard)
    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    /// Erreur de permission (pour plus tard)
    #[error("Forbidden: {0}")]
    Forbidden(String),
}

impl TardigradeError {
    /// Crée une nouvelle erreur de base de données
    pub fn database(msg: impl Into<String>) -> Self {
        Self::Database(msg.into())
    }

    /// Crée une nouvelle erreur de configuration
    pub fn configuration(msg: impl Into<String>) -> Self {
        Self::Configuration(msg.into())
    }

    /// Crée une nouvelle erreur de validation
    pub fn validation(msg: impl Into<String>) -> Self {
        Self::Validation(msg.into())
    }

    /// Crée une nouvelle erreur NotFound
    pub fn not_found(resource: impl Into<String>) -> Self {
        Self::NotFound(format!("{} not found", resource.into()))
    }

    /// Crée une nouvelle erreur Conflict
    pub fn conflict(resource: impl Into<String>) -> Self {
        Self::Conflict(format!("{} already exists", resource.into()))
    }
}

/// Conversion depuis sqlx::Error
impl From<sqlx::Error> for TardigradeError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => Self::not_found("Database row"),
            _ => Self::database(err.to_string()),
        }
    }
}

/// Conversion depuis std::io::Error
impl From<std::io::Error> for TardigradeError {
    fn from(err: std::io::Error) -> Self {
        Self::InternalServerError
    }
}

/// Conversion depuis serde_json::Error
impl From<serde_json::Error> for TardigradeError {
    fn from(err: serde_json::Error) -> Self {
        Self::Validation(err.to_string())
    }
}

/// Conversion depuis uuid::Error
impl From<uuid::Error> for TardigradeError {
    fn from(err: uuid::Error) -> Self {
        Self::Validation(err.to_string())
    }
}

/// Type pour les résultats avec TardigradeError
pub type TardigradeResult<T> = Result<T, TardigradeError>;
