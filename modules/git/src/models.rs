//! Modèles de données pour le module Git

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Représente un repository Git
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Repository {
    /// Identifiant unique
    pub id: Uuid,
    /// Nom du repository
    pub name: String,
    /// Description (optionnelle)
    pub description: Option<String>,
    /// Repository privé ou public
    pub is_private: bool,
    /// ID du propriétaire (pour plus tard)
    pub owner_id: Option<Uuid>,
    /// Branche par défaut
    pub default_branch: String,
    /// Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl fmt::Display for Repository {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Repository {{ id: {}, name: {}, default_branch: {} }}",
            self.id, self.name, self.default_branch
        )
    }
}

impl Repository {
    /// Crée un nouveau repository
    pub fn new(
        name: impl Into<String>,
        description: Option<String>,
        is_private: bool,
        default_branch: impl Into<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            description,
            is_private,
            owner_id: None,
            default_branch: default_branch.into(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Met à jour les timestamps
    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }

    /// Met à jour le nom
    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
        self.touch();
    }

    /// Met à jour la description
    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
        self.touch();
    }

    /// Met à jour la visibilité
    pub fn set_visibility(&mut self, is_private: bool) {
        self.is_private = is_private;
        self.touch();
    }

    /// Met à jour la branche par défaut
    pub fn set_default_branch(&mut self, branch: impl Into<String>) {
        self.default_branch = branch.into();
        self.touch();
    }
}

/// Données pour créer un repository
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRepositoryInput {
    /// Nom du repository
    pub name: String,
    /// Description (optionnelle)
    pub description: Option<String>,
    /// Repository privé ou public
    #[serde(default = "default_private")]
    pub is_private: bool,
    /// Branche par défaut
    #[serde(default = "default_branch")]
    pub default_branch: String,
}

fn default_private() -> bool {
    false
}

fn default_branch() -> String {
    "main".to_string()
}

impl Default for CreateRepositoryInput {
    fn default() -> Self {
        Self {
            name: String::new(),
            description: None,
            is_private: false,
            default_branch: "main".to_string(),
        }
    }
}

impl CreateRepositoryInput {
    /// Valide les données d'entrée
    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Repository name cannot be empty".to_string());
        }

        if self.name.len() > 100 {
            return Err("Repository name cannot exceed 100 characters".to_string());
        }

        // Validation basique du nom (alphanumérique, -, _, .)
        if !self
            .name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.')
        {
            return Err(
                "Repository name can only contain alphanumeric characters, '-', '_', and '.'"
                    .to_string(),
            );
        }

        if self.default_branch.is_empty() {
            return Err("Default branch cannot be empty".to_string());
        }

        Ok(())
    }

    /// Convertit en Repository avec un ID aléatoire
    pub fn into_repository(self) -> Repository {
        Repository::new(
            self.name,
            self.description,
            self.is_private,
            self.default_branch,
        )
    }
}

/// Données pour mettre à jour un repository
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRepositoryInput {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_private: Option<bool>,
    pub default_branch: Option<String>,
}

/// Représente une branche Git
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Branch {
    /// Identifiant unique
    pub id: Uuid,
    /// ID du repository parent
    pub repository_id: Uuid,
    /// Nom de la branche
    pub name: String,
    /// Hash du dernier commit (optionnel pour le MVP)
    pub commit_hash: Option<String>,
    /// Timestamps
    pub created_at: DateTime<Utc>,
}

impl fmt::Display for Branch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Branch {{ id: {}, repository_id: {}, name: {} }}",
            self.id, self.repository_id, self.name
        )
    }
}

impl Branch {
    /// Crée une nouvelle branche
    pub fn new(repository_id: Uuid, name: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            repository_id,
            name: name.into(),
            commit_hash: None,
            created_at: Utc::now(),
        }
    }

    /// Met à jour le hash du commit
    pub fn set_commit_hash(&mut self, hash: impl Into<String>) {
        self.commit_hash = Some(hash.into());
    }
}

/// Données pour créer une branche
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBranchInput {
    /// Nom de la branche
    pub name: String,
    /// Hash du commit initial (optionnel)
    pub commit_hash: Option<String>,
}

impl Default for CreateBranchInput {
    fn default() -> Self {
        Self {
            name: String::new(),
            commit_hash: None,
        }
    }
}

impl CreateBranchInput {
    /// Valide les données d'entrée
    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Branch name cannot be empty".to_string());
        }

        if self.name.len() > 255 {
            return Err("Branch name cannot exceed 255 characters".to_string());
        }

        Ok(())
    }

    /// Convertit en Branch avec un repository_id
    pub fn into_branch(self, repository_id: Uuid) -> Branch {
        let mut branch = Branch::new(repository_id, self.name);
        if let Some(hash) = self.commit_hash {
            branch.set_commit_hash(hash);
        }
        branch
    }
}

/// Représente un commit (simplifié pour le MVP)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commit {
    /// Hash du commit (clé primaire)
    pub hash: String,
    /// ID du repository
    pub repository_id: Uuid,
    /// Message du commit
    pub message: String,
    /// Auteur (pour plus tard)
    pub author_name: Option<String>,
    /// Email de l'auteur (pour plus tard)
    pub author_email: Option<String>,
    /// Timestamps
    pub created_at: DateTime<Utc>,
}

impl Commit {
    /// Crée un nouveau commit
    pub fn new(
        hash: impl Into<String>,
        repository_id: Uuid,
        message: impl Into<String>,
    ) -> Self {
        Self {
            hash: hash.into(),
            repository_id,
            message: message.into(),
            author_name: None,
            author_email: None,
            created_at: Utc::now(),
        }
    }
}

/// Requête de pagination
#[derive(Debug, Clone, Deserialize)]
pub struct PaginationQuery {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_page_size")]
    pub page_size: i64,
}

fn default_page() -> i64 {
    1
}

fn default_page_size() -> i64 {
    20
}

impl Default for PaginationQuery {
    fn default() -> Self {
        Self {
            page: 1,
            page_size: 20,
        }
    }
}

impl PaginationQuery {
    pub fn offset(&self) -> i64 {
        (self.page - 1) * self.page_size
    }

    pub fn limit(&self) -> i64 {
        self.page_size
    }
}

/// Réponse paginée pour les repositories
pub type RepositoryListResponse = tardigrade_common::PaginatedResponse<Repository>;

/// Réponse paginée pour les branches
pub type BranchListResponse = tardigrade_common::PaginatedResponse<Branch>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repository_creation() {
        let repo = Repository::new(
            "my-repo",
            Some("A test repository".to_string()),
            false,
            "main",
        );

        assert!(!repo.id.is_nil());
        assert_eq!(repo.name, "my-repo");
        assert_eq!(repo.description, Some("A test repository".to_string()));
        assert!(!repo.is_private);
        assert_eq!(repo.default_branch, "main");
    }

    #[test]
    fn test_repository_validation() {
        let input = CreateRepositoryInput {
            name: "".to_string(),
            description: None,
            is_private: false,
            default_branch: "main".to_string(),
        };

        assert!(input.validate().is_err());

        let input = CreateRepositoryInput {
            name: "valid-repo".to_string(),
            description: None,
            is_private: false,
            default_branch: "main".to_string(),
        };

        assert!(input.validate().is_ok());
    }

    #[test]
    fn test_branch_creation() {
        let repo_id = Uuid::new_v4();
        let branch = Branch::new(repo_id, "main");

        assert!(!branch.id.is_nil());
        assert_eq!(branch.repository_id, repo_id);
        assert_eq!(branch.name, "main");
        assert!(branch.commit_hash.is_none());
    }

    #[test]
    fn test_pagination_offset() {
        let query = PaginationQuery {
            page: 2,
            page_size: 10,
        };

        assert_eq!(query.offset(), 10);
        assert_eq!(query.limit(), 10);
    }

    #[test]
    fn test_update_repository_input() {
        let input = UpdateRepositoryInput {
            name: Some("new-name".to_string()),
            description: Some("new description".to_string()),
            is_private: Some(true),
            default_branch: Some("develop".to_string()),
        };

        assert_eq!(input.name, Some("new-name".to_string()));
    }
}
