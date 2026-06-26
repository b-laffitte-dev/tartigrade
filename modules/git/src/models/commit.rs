//! Commit models for Tardigrade Git module
//!
//! This module defines the data structures for Git commits.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use uuid::Uuid;

/// Commit entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Commit {
    /// Unique identifier
    pub id: Uuid,
    /// Repository ID this commit belongs to
    pub repository_id: Uuid,
    /// Commit hash (SHA-1)
    pub hash: String,
    /// Parent commit hash (optional, for first commit)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_hash: Option<String>,
    /// Commit message
    pub message: String,
    /// Author name
    pub author_name: String,
    /// Author email
    pub author_email: String,
    /// Committer name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub committer_name: Option<String>,
    /// Committer email
    #[serde(skip_serializing_if = "Option::is_none")]
    pub committer_email: Option<String>,
    /// Branch name this commit belongs to
    pub branch_name: String,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
}

impl Commit {
    /// Create a new Commit instance
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: Uuid,
        repository_id: Uuid,
        hash: String,
        parent_hash: Option<String>,
        message: String,
        author_name: String,
        author_email: String,
        committer_name: Option<String>,
        committer_email: Option<String>,
        branch_name: String,
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            repository_id,
            hash,
            parent_hash,
            message,
            author_name,
            author_email,
            committer_name,
            committer_email,
            branch_name,
            created_at,
        }
    }
}

/// Input for creating a new commit
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateCommitInput {
    /// Commit hash (required)
    pub hash: String,
    /// Parent commit hash (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_hash: Option<String>,
    /// Commit message (required)
    pub message: String,
    /// Author name (required)
    pub author_name: String,
    /// Author email (required)
    pub author_email: String,
    /// Committer name (optional, defaults to author_name)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub committer_name: Option<String>,
    /// Committer email (optional, defaults to author_email)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub committer_email: Option<String>,
    /// Branch name (required)
    pub branch_name: String,
}

impl Default for CreateCommitInput {
    fn default() -> Self {
        Self {
            hash: String::new(),
            parent_hash: None,
            message: String::new(),
            author_name: String::new(),
            author_email: String::new(),
            committer_name: None,
            committer_email: None,
            branch_name: String::new(),
        }
    }
}

impl CreateCommitInput {
    /// Validate the input
    pub fn validate(&self) -> Result<(), String> {
        if self.hash.is_empty() {
            return Err("Commit hash cannot be empty".to_string());
        }

        if self.hash.len() != 40 {
            return Err("Commit hash must be 40 characters (SHA-1)".to_string());
        }

        if !self.hash.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err("Commit hash must contain only hexadecimal characters".to_string());
        }

        if self.message.is_empty() {
            return Err("Commit message cannot be empty".to_string());
        }

        if self.message.len() > 10000 {
            return Err("Commit message cannot exceed 10000 characters".to_string());
        }

        if self.author_name.is_empty() {
            return Err("Author name cannot be empty".to_string());
        }

        if self.author_email.is_empty() {
            return Err("Author email cannot be empty".to_string());
        }

        if !is_valid_email(&self.author_email) {
            return Err("Author email is invalid".to_string());
        }

        if let Some(ref committer_email) = self.committer_email {
            if !is_valid_email(committer_email) {
                return Err("Committer email is invalid".to_string());
            }
        }

        if self.branch_name.is_empty() {
            return Err("Branch name cannot be empty".to_string());
        }

        if !crate::models::branch::is_valid_branch_name(&self.branch_name) {
            return Err("Branch name is invalid".to_string());
        }

        Ok(())
    }
}

/// Query parameters for listing commits
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ListCommitsQuery {
    /// Filter by repository ID (required)
    pub repository_id: Uuid,
    /// Filter by branch name (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_name: Option<String>,
    /// Page number (1-indexed, default: 1)
    #[serde(default = "default_page")]
    pub page: i32,
    /// Number of items per page (default: 20, max: 100)
    #[serde(default = "default_page_size")]
    pub page_size: i32,
    /// Search term for commit message (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
}

impl Default for ListCommitsQuery {
    fn default() -> Self {
        Self {
            repository_id: Uuid::nil(),
            branch_name: None,
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

impl ListCommitsQuery {
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

/// Database representation of a commit (from SQL queries)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbCommit {
    pub id: Uuid,
    pub repository_id: Uuid,
    pub hash: String,
    pub parent_hash: Option<String>,
    pub message: String,
    pub author_name: String,
    pub author_email: String,
    pub committer_name: Option<String>,
    pub committer_email: Option<String>,
    pub branch_name: String,
    pub created_at: DateTime<Utc>,
}

impl sqlx::FromRow<'_, sqlx::postgres::PgRow> for DbCommit {
    fn from_row(row: &sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.get("id"),
            repository_id: row.get("repository_id"),
            hash: row.get("hash"),
            parent_hash: row.get("parent_hash"),
            message: row.get("message"),
            author_name: row.get("author_name"),
            author_email: row.get("author_email"),
            committer_name: row.get("committer_name"),
            committer_email: row.get("committer_email"),
            branch_name: row.get("branch_name"),
            created_at: row.get("created_at"),
        })
    }
}

impl From<DbCommit> for Commit {
    fn from(db_commit: DbCommit) -> Self {
        Commit::new(
            db_commit.id,
            db_commit.repository_id,
            db_commit.hash,
            db_commit.parent_hash,
            db_commit.message,
            db_commit.author_name,
            db_commit.author_email,
            db_commit.committer_name,
            db_commit.committer_email,
            db_commit.branch_name,
            db_commit.created_at,
        )
    }
}

/// Helper function to validate email format
fn is_valid_email(email: &str) -> bool {
    if email.is_empty() {
        return false;
    }

    // Simple email validation - for production, use a proper email validation library
    email.contains('@') && email.contains('.') && !email.starts_with('@') && !email.ends_with('@')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_commit_input_validation() {
        let mut input = CreateCommitInput::default();
        input.hash = "a".repeat(40);
        input.message = "Initial commit".to_string();
        input.author_name = "John Doe".to_string();
        input.author_email = "john@example.com".to_string();
        input.branch_name = "main".to_string();
        assert!(input.validate().is_ok());

        let mut input = CreateCommitInput::default();
        input.hash = "".to_string();
        assert!(input.validate().is_err());

        let mut input = CreateCommitInput::default();
        input.hash = "abc".to_string(); // Too short
        assert!(input.validate().is_err());

        let mut input = CreateCommitInput::default();
        input.hash = "abc123".to_string(); // Invalid length
        assert!(input.validate().is_err());

        let mut input = CreateCommitInput::default();
        input.hash = "a".repeat(40);
        input.message = "".to_string();
        assert!(input.validate().is_err());

        let mut input = CreateCommitInput::default();
        input.hash = "a".repeat(40);
        input.message = "Valid message".to_string();
        input.author_email = "invalid-email".to_string();
        assert!(input.validate().is_err());
    }

    #[test]
    fn test_list_commits_query_offset_limit() {
        let query = ListCommitsQuery {
            repository_id: Uuid::new_v4(),
            branch_name: None,
            page: 1,
            page_size: 20,
            search: None,
        };
        assert_eq!(query.offset(), 0);
        assert_eq!(query.limit(), 20);

        let query = ListCommitsQuery {
            repository_id: Uuid::new_v4(),
            branch_name: Some("main".to_string()),
            page: 2,
            page_size: 10,
            search: None,
        };
        assert_eq!(query.offset(), 10);
        assert_eq!(query.limit(), 10);
    }
}
