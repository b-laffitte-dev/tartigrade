//! Modèles de données communs

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Structure de base pour la pagination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    /// Numéro de page (1-based)
    pub page: i64,
    /// Taille de la page
    pub page_size: i64,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 1,
            page_size: 20,
        }
    }
}

impl Pagination {
    /// Calcule l'offset pour les requêtes SQL
    pub fn offset(&self) -> i64 {
        (self.page - 1) * self.page_size
    }

    /// Calcule le limit pour les requêtes SQL
    pub fn limit(&self) -> i64 {
        self.page_size
    }
}

/// Réponse paginée générique
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    /// Données de la page
    pub data: Vec<T>,
    /// Numéro de page
    pub page: i64,
    /// Taille de la page
    pub page_size: i64,
    /// Total d'éléments
    pub total: i64,
    /// Nombre total de pages
    pub total_pages: i64,
}

impl<T> PaginatedResponse<T> {
    /// Crée une nouvelle réponse paginée
    pub fn new(data: Vec<T>, page: i64, page_size: i64, total: i64) -> Self {
        let total_pages = if page_size > 0 {
            (total + page_size - 1) / page_size
        } else {
            0
        };

        Self {
            data,
            page,
            page_size,
            total,
            total_pages,
        }
    }
}

/// Structure de base pour un utilisateur (pour plus tard)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Structure de base pour les métadonnées de timing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timestamps {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Timestamps {
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update(&mut self) {
        self.updated_at = Utc::now();
    }
}

/// Trait pour les entités avec timestamps
pub trait WithTimestamps {
    fn timestamps(&self) -> &Timestamps;
    fn timestamps_mut(&mut self) -> &mut Timestamps;
}
