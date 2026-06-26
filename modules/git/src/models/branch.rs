//! Branch models for Tardigrade Git module
//!
//! This module defines the data structures for Git branches.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use uuid::Uuid;

/// Branch entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Branch {
    /// Unique identifier
    pub id: Uuid,
    /// Repository ID this branch belongs to
    pub repository_id: Uuid,
    /// Branch name
    pub name: String,
    /// Commit hash this branch points to
    pub commit_hash: String,
    /// Whether this is the default branch
    pub is_default: bool,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl Branch {
    /// Create a new Branch instance
    pub fn new(
        id: Uuid,
        repository_id: Uuid,
        name: String,
        commit_hash: String,
        is_default: bool,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            repository_id,
            name,
            commit_hash,
            is_default,
            created_at,
            updated_at,
        }
    }

    /// Update the branch with new values
    pub fn update(&mut self, input: &UpdateBranchInput) {
        if let Some(name) = &input.name {
            self.name = name.clone();
        }
        if let Some(commit_hash) = &input.commit_hash {
            self.commit_hash = commit_hash.clone();
        }
        if let Some(is_default) = input.is_default {
            self.is_default = is_default;
        }
        self.updated_at = Utc::now();
    }
}

/// Input for creating a new branch
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateBranchInput {
    /// Branch name (required)
    pub name: String,
    /// Commit hash this branch points to (required)
    pub commit_hash: String,
    /// Whether this is the default branch (default: false)
    #[serde(default)]
    pub is_default: bool,
}

impl Default for CreateBranchInput {
    fn default() -> Self {
        Self {
            name: String::new(),
            commit_hash: String::new(),
            is_default: false,
        }
    }
}

impl CreateBranchInput {
    /// Validate the input
    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Branch name cannot be empty".to_string());
        }

        if self.name.len() > 255 {
            return Err("Branch name cannot exceed 255 characters".to_string());
        }

        if !is_valid_branch_name(&self.name) {
            return Err(
                "Branch name can only contain alphanumeric characters, hyphens, underscores, slashes, and dots"
                    .to_string(),
            );
        }

        if self.commit_hash.is_empty() {
            return Err("Commit hash cannot be empty".to_string());
        }

        if self.commit_hash.len() > 64 {
            return Err("Commit hash cannot exceed 64 characters".to_string());
        }

        Ok(())
    }
}

/// Input for updating a branch
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct UpdateBranchInput {
    /// New branch name (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// New commit hash (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_hash: Option<String>,
    /// New default branch flag (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
}

/// Query parameters for listing branches
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ListBranchesQuery {
    /// Filter by repository ID (required)
    pub repository_id: Uuid,
    /// Page number (1-indexed, default: 1)
    #[serde(default = "default_page")]
    pub page: i32,
    /// Number of items per page (default: 20, max: 100)
    #[serde(default = "default_page_size")]
    pub page_size: i32,
    /// Search term for branch name (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
}

impl Default for ListBranchesQuery {
    fn default() -> Self {
        Self {
            repository_id: Uuid::nil(),
            page: default_page(),
            page_size: default_page_size(),
            search: None,
        }
    }
}

fn default_page() -> i32 {
    1
}

fn default_page_size() -> i32 {
    20
}

impl ListBranchesQuery {
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

/// Database representation of a branch (from SQL queries)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbBranch {
    pub id: Uuid,
    pub repository_id: Uuid,
    pub name: String,
    pub commit_hash: String,
    pub is_default: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl sqlx::FromRow<'_, sqlx::postgres::PgRow> for DbBranch {
    fn from_row(row: &sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.get("id"),
            repository_id: row.get("repository_id"),
            name: row.get("name"),
            commit_hash: row.get("commit_hash"),
            is_default: row.get("is_default"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }
}

impl From<DbBranch> for Branch {
    fn from(db_branch: DbBranch) -> Self {
        Branch::new(
            db_branch.id,
            db_branch.repository_id,
            db_branch.name,
            db_branch.commit_hash,
            db_branch.is_default,
            db_branch.created_at,
            db_branch.updated_at,
        )
    }
}

/// Helper function to validate branch name
pub fn is_valid_branch_name(name: &str) -> bool {
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
    fn test_create_branch_input_validation() {
        let mut input = CreateBranchInput::default();
        input.name = "main".to_string();
        input.commit_hash = "abc123".to_string();
        assert!(input.validate().is_ok());

        let mut input = CreateBranchInput::default();
        input.name = "".to_string();
        input.commit_hash = "abc123".to_string();
        assert!(input.validate().is_err());

        let mut input = CreateBranchInput::default();
        input.name = "invalid@name".to_string();
        input.commit_hash = "abc123".to_string();
        assert!(input.validate().is_err());

        let mut input = CreateBranchInput::default();
        input.name = "main".to_string();
        input.commit_hash = "".to_string();
        assert!(input.validate().is_err());
    }

    #[test]
    fn test_list_branches_query_offset_limit() {
        let query = ListBranchesQuery {
            repository_id: Uuid::new_v4(),
            page: 1,
            page_size: 20,
            search: None,
        };
        assert_eq!(query.offset(), 0);
        assert_eq!(query.limit(), 20);

        let query = ListBranchesQuery {
            repository_id: Uuid::new_v4(),
            page: 2,
            page_size: 10,
            search: None,
        };
        assert_eq!(query.offset(), 10);
        assert_eq!(query.limit(), 10);
    }

    #[test]
    fn test_valid_branch_names() {
        assert!(is_valid_branch_name("main"));
        assert!(is_valid_branch_name("feature/new-feature"));
        assert!(is_valid_branch_name("fix_bug-123"));
        assert!(is_valid_branch_name("v1.0.0"));
        assert!(!is_valid_branch_name(""));
        assert!(!is_valid_branch_name("/main"));
        assert!(!is_valid_branch_name("main/"));
        assert!(!is_valid_branch_name("feature//new"));
        assert!(!is_valid_branch_name("invalid@name"));
    }
}
