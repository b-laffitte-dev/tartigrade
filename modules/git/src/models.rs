//! Data models for Tardigrade Git module
//!
//! This module defines the data structures used throughout the Git module.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use uuid::Uuid;

// Re-export branch and commit models
pub mod branch;
pub mod commit;

pub use branch::*;
pub use commit::*;

/// Repository entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Repository {
    /// Unique identifier
    pub id: Uuid,
    /// Repository name
    pub name: String,
    /// Repository description (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether the repository is private
    pub is_private: bool,
    /// Owner user ID
    pub owner_id: Uuid,
    /// Default branch name
    pub default_branch: String,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl Repository {
    /// Create a new Repository instance
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: Uuid,
        name: String,
        description: Option<String>,
        is_private: bool,
        owner_id: Uuid,
        default_branch: String,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            name,
            description,
            is_private,
            owner_id,
            default_branch,
            created_at,
            updated_at,
        }
    }

    /// Update the repository with new values
    pub fn update(&mut self, input: &UpdateRepositoryInput) {
        if let Some(name) = &input.name {
            self.name = name.clone();
        }
        if let Some(description) = &input.description {
            self.description = Some(description.clone());
        }
        if let Some(is_private) = input.is_private {
            self.is_private = is_private;
        }
        if let Some(default_branch) = &input.default_branch {
            self.default_branch = default_branch.clone();
        }
        self.updated_at = Utc::now();
    }
}

/// Input for creating a new repository
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateRepositoryInput {
    /// Repository name (required)
    pub name: String,
    /// Repository description (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether the repository is private (default: false)
    #[serde(default)]
    pub is_private: bool,
    /// Default branch name (default: "main")
    #[serde(default = "default_default_branch")]
    pub default_branch: String,
}

impl Default for CreateRepositoryInput {
    fn default() -> Self {
        Self {
            name: String::new(),
            description: None,
            is_private: false,
            default_branch: default_default_branch(),
        }
    }
}

fn default_default_branch() -> String {
    "main".to_string()
}

impl CreateRepositoryInput {
    /// Validate the input
    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Repository name cannot be empty".to_string());
        }

        if self.name.len() > 255 {
            return Err("Repository name cannot exceed 255 characters".to_string());
        }

        if !is_valid_repo_name(&self.name) {
            return Err(
                "Repository name can only contain alphanumeric characters, hyphens, underscores, and dots"
                    .to_string(),
            );
        }

        if let Some(desc) = &self.description {
            if desc.len() > 5000 {
                return Err("Repository description cannot exceed 5000 characters".to_string());
            }
        }

        if !is_valid_branch_name(&self.default_branch) {
            return Err("Default branch name is invalid".to_string());
        }

        Ok(())
    }
}

/// Input for updating a repository
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct UpdateRepositoryInput {
    /// New repository name (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// New repository description (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// New privacy setting (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
    /// New default branch name (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_branch: Option<String>,
}

/// Paginated response wrapper
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PaginatedResponse<T> {
    /// Data items
    pub data: Vec<T>,
    /// Current page number (1-indexed)
    pub page: i32,
    /// Number of items per page
    pub page_size: i32,
    /// Total number of items
    pub total: i64,
    /// Total number of pages
    pub total_pages: i32,
}

impl<T> PaginatedResponse<T> {
    /// Create a new paginated response
    pub fn new(data: Vec<T>, page: i32, page_size: i32, total: i64) -> Self {
        let total_pages = if page_size > 0 {
            ((total + page_size as i64 - 1) / page_size as i64) as i32
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

    /// Check if there are more pages
    pub fn has_more_pages(&self) -> bool {
        self.page < self.total_pages
    }

    /// Check if there are previous pages
    pub fn has_previous_pages(&self) -> bool {
        self.page > 1
    }
}

/// Query parameters for listing repositories
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ListRepositoriesQuery {
    /// Filter by owner ID (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<Uuid>,
    /// Page number (1-indexed, default: 1)
    #[serde(default = "default_page")]
    pub page: i32,
    /// Number of items per page (default: 20, max: 100)
    #[serde(default = "default_page_size")]
    pub page_size: i32,
    /// Search term for repository name (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// Filter by visibility (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_private: Option<bool>,
}

impl Default for ListRepositoriesQuery {
    fn default() -> Self {
        Self {
            owner_id: None,
            page: default_page(),
            page_size: default_page_size(),
            search: None,
            is_private: None,
        }
    }
}

fn default_page() -> i32 {
    1
}

fn default_page_size() -> i32 {
    20
}

impl ListRepositoriesQuery {
    /// Get the offset for SQL queries
    pub fn offset(&self) -> i64 {
        let page = self.page.max(1);
        let page_size = self.page_size.clamp(1, 100);
        (page - 1) as i64 * page_size as i64
    }

    /// Get the limit for SQL queries
    pub fn limit(&self) -> i64 {
        self.page_size.clamp(1, 100) as i64
    }
}

/// Database representation of a repository (from SQL queries)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbRepository {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub is_private: bool,
    pub owner_id: Uuid,
    pub default_branch: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl sqlx::FromRow<'_, sqlx::postgres::PgRow> for DbRepository {
    fn from_row(row: &sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            is_private: row.get("is_private"),
            owner_id: row.get("owner_id"),
            default_branch: row.get("default_branch"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }
}

impl From<DbRepository> for Repository {
    fn from(db_repo: DbRepository) -> Self {
        Repository::new(
            db_repo.id,
            db_repo.name,
            db_repo.description,
            db_repo.is_private,
            db_repo.owner_id,
            db_repo.default_branch,
            db_repo.created_at,
            db_repo.updated_at,
        )
    }
}

/// Helper function to validate repository name
fn is_valid_repo_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    name.chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.')
}

/// Helper function to validate branch name
fn is_valid_branch_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    name.chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '/' || c == '.')
        && !name.starts_with('/')
        && !name.ends_with('/')
        && !name.contains("//")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_repository_input_validation() {
        let mut input = CreateRepositoryInput::default();
        input.name = "my-repo".to_string();
        assert!(input.validate().is_ok());

        let mut input = CreateRepositoryInput::default();
        input.name = "".to_string();
        assert!(input.validate().is_err());

        let mut input = CreateRepositoryInput::default();
        input.name = "a".repeat(256);
        assert!(input.validate().is_err());

        let mut input = CreateRepositoryInput::default();
        input.name = "invalid@name".to_string();
        assert!(input.validate().is_err());
    }

    #[test]
    fn test_paginated_response() {
        let data: Vec<i32> = vec![1, 2, 3, 4, 5];
        let response = PaginatedResponse::new(data.clone(), 1, 5, 10);

        assert_eq!(response.data, data);
        assert_eq!(response.page, 1);
        assert_eq!(response.page_size, 5);
        assert_eq!(response.total, 10);
        assert_eq!(response.total_pages, 2);
        assert!(response.has_more_pages());
        assert!(!response.has_previous_pages());
    }

    #[test]
    fn test_list_repositories_query_offset_limit() {
        let query = ListRepositoriesQuery::default();
        assert_eq!(query.offset(), 0);
        assert_eq!(query.limit(), 20);

        let query = ListRepositoriesQuery {
            page: 2,
            page_size: 10,
            ..Default::default()
        };
        assert_eq!(query.offset(), 10);
        assert_eq!(query.limit(), 10);
    }
}
