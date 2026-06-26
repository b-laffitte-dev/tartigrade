//! GraphQL schema definitions for Tardigrade Git module
//!
//! This module defines the GraphQL types and schema for Git operations.

use async_graphql::*;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Repository GraphQL type
#[derive(Debug, Clone, SimpleObject)]
pub struct RepositoryType {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub is_private: bool,
    pub owner_id: Uuid,
    pub default_branch: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Branch GraphQL type
#[derive(Debug, Clone, SimpleObject)]
pub struct BranchType {
    pub id: Uuid,
    pub repository_id: Uuid,
    pub name: String,
    pub commit_hash: String,
    pub is_default: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Commit GraphQL type
#[derive(Debug, Clone, SimpleObject)]
pub struct CommitType {
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

/// Paginated response wrapper for GraphQL
#[derive(Debug, Clone, SimpleObject)]
pub struct PaginatedResponse<T: SimpleObject> {
    pub data: Vec<T>,
    pub page: i32,
    pub page_size: i32,
    pub total: i64,
    pub total_pages: i32,
}

/// Input for creating a repository
#[derive(Debug, Clone, InputObject)]
pub struct CreateRepositoryInputType {
    pub name: String,
    pub description: Option<String>,
    pub is_private: bool,
    pub default_branch: String,
}

/// Input for updating a repository
#[derive(Debug, Clone, InputObject)]
pub struct UpdateRepositoryInputType {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_private: Option<bool>,
    pub default_branch: Option<String>,
}

/// Input for listing repositories
#[derive(Debug, Clone, InputObject)]
pub struct ListRepositoriesInputType {
    pub owner_id: Option<Uuid>,
    pub page: Option<i32>,
    pub page_size: Option<i32>,
    pub search: Option<String>,
    pub is_private: Option<bool>,
}

/// Input for creating a branch
#[derive(Debug, Clone, InputObject)]
pub struct CreateBranchInputType {
    pub repository_id: Uuid,
    pub name: String,
    pub commit_hash: String,
    pub is_default: bool,
}

/// Input for updating a branch
#[derive(Debug, Clone, InputObject)]
pub struct UpdateBranchInputType {
    pub name: Option<String>,
    pub commit_hash: Option<String>,
    pub is_default: Option<bool>,
}

/// Input for listing branches
#[derive(Debug, Clone, InputObject)]
pub struct ListBranchesInputType {
    pub repository_id: Uuid,
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

/// Input for creating a commit
#[derive(Debug, Clone, InputObject)]
pub struct CreateCommitInputType {
    pub repository_id: Uuid,
    pub hash: String,
    pub parent_hash: Option<String>,
    pub message: String,
    pub author_name: String,
    pub author_email: String,
    pub committer_name: Option<String>,
    pub committer_email: Option<String>,
    pub branch_name: String,
}

/// Input for listing commits
#[derive(Debug, Clone, InputObject)]
pub struct ListCommitsInputType {
    pub repository_id: Uuid,
    pub branch_name: Option<String>,
    pub page: Option<i32>,
    pub page_size: Option<i32>,
    pub search: Option<String>,
}

/// Query root for Git GraphQL operations
#[derive(Debug, Clone, Default)]
pub struct QueryRoot;

/// Mutation root for Git GraphQL operations
#[derive(Debug, Clone, Default)]
pub struct MutationRoot;

/// Subscription root for Git GraphQL operations
#[derive(Debug, Clone, Default)]
pub struct SubscriptionRoot;

/// Git GraphQL Schema
#[derive(Debug, Clone)]
pub struct GitSchema;

impl GitSchema {
    /// Create a new Git GraphQL schema
    pub fn new() -> Schema<QueryRoot, MutationRoot, SubscriptionRoot> {
        Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
            .finish()
    }
}
