//! GraphQL resolvers for Tardigrade Git module
//!
//! This module implements the GraphQL resolvers for Git operations.

use super::schema::*;
use async_graphql::*;
use std::sync::Arc;
use uuid::Uuid;

use crate::error::GitError;
use crate::models::{
    branch::{Branch, CreateBranchInput, ListBranchesQuery, UpdateBranchInput},
    commit::{Commit, CreateCommitInput, ListCommitsQuery},
    CreateRepositoryInput, ListRepositoriesQuery, PaginatedResponse, Repository, UpdateRepositoryInput,
};
use crate::repository::*;
use crate::AppState;

/// Query resolvers
#[Object]
impl QueryRoot {
    /// Get a repository by ID
    async fn repository(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> Result<Option<RepositoryType>, async_graphql::Error> {
        let state = ctx.data::<Arc<AppState>>()?;
        
        let repo = get_repository(&state.pool, id).await
            .map_err(|e| async_graphql::Error::from(e))?;
        
        Ok(repo.map(RepositoryType::from))
    }

    /// List repositories with pagination
    async fn repositories(
        &self,
        ctx: &Context<'_>,
        input: Option<ListRepositoriesInputType>,
    ) -> Result<PaginatedResponse<RepositoryType>, async_graphql::Error> {
        let state = ctx.data::<Arc<AppState>>()?;
        
        let query = ListRepositoriesQuery {
            owner_id: input.and_then(|i| i.owner_id),
            page: input.map(|i| i.page).unwrap_or(1),
            page_size: input.map(|i| i.page_size).unwrap_or(20),
            search: input.and_then(|i| i.search),
            is_private: input.and_then(|i| i.is_private),
        };
        
        let response = list_repositories(&state.pool, query).await
            .map_err(|e| async_graphql::Error::from(e))?;
        
        let repositories: Vec<RepositoryType> = response.data
            .into_iter()
            .map(RepositoryType::from)
            .collect();
        
        Ok(PaginatedResponse {
            data: repositories,
            page: response.page,
            page_size: response.page_size,
            total: response.total,
            total_pages: response.total_pages,
        })
    }

    /// Get a branch by ID
    async fn branch(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> Result<Option<BranchType>, async_graphql::Error> {
        let state = ctx.data::<Arc<AppState>>()?;
        
        let branch = crate::repository::branch::get_branch(&state.pool, id).await
            .map_err(|e| async_graphql::Error::from(e))?;
        
        Ok(branch.map(BranchType::from))
    }

    /// List branches for a repository
    async fn branches(
        &self,
        ctx: &Context<'_>,
        repository_id: Uuid,
        input: Option<ListBranchesInputType>,
    ) -> Result<PaginatedResponse<BranchType>, async_graphql::Error> {
        let state = ctx.data::<Arc<AppState>>()?;
        
        let query = ListBranchesQuery {
            repository_id,
            page: input.map(|i| i.page).unwrap_or(1),
            page_size: input.map(|i| i.page_size).unwrap_or(20),
            search: None,
        };
        
        let response = crate::repository::branch::list_branches(&state.pool, query).await
            .map_err(|e| async_graphql::Error::from(e))?;
        
        let branches: Vec<BranchType> = response.data
            .into_iter()
            .map(BranchType::from)
            .collect();
        
        Ok(PaginatedResponse {
            data: branches,
            page: response.page,
            page_size: response.page_size,
            total: response.total,
            total_pages: response.total_pages,
        })
    }

    /// Get a commit by ID
    async fn commit(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> Result<Option<CommitType>, async_graphql::Error> {
        let state = ctx.data::<Arc<AppState>>()?;
        
        let commit = crate::repository::commit::get_commit(&state.pool, id).await
            .map_err(|e| async_graphql::Error::from(e))?;
        
        Ok(commit.map(CommitType::from))
    }

    /// List commits for a repository
    async fn commits(
        &self,
        ctx: &Context<'_>,
        repository_id: Uuid,
        input: Option<ListCommitsInputType>,
    ) -> Result<PaginatedResponse<CommitType>, async_graphql::Error> {
        let state = ctx.data::<Arc<AppState>>()?;
        
        let query = ListCommitsQuery {
            repository_id,
            branch_name: input.and_then(|i| i.branch_name),
            page: input.map(|i| i.page).unwrap_or(1),
            page_size: input.map(|i| i.page_size).unwrap_or(20),
            search: input.and_then(|i| i.search),
        };
        
        let response = crate::repository::commit::list_commits(&state.pool, query).await
            .map_err(|e| async_graphql::Error::from(e))?;
        
        let commits: Vec<CommitType> = response.data
            .into_iter()
            .map(CommitType::from)
            .collect();
        
        Ok(PaginatedResponse {
            data: commits,
            page: response.page,
            page_size: response.page_size,
            total: response.total,
            total_pages: response.total_pages,
        })
    }

    /// Get the latest commit for a branch
    async fn latest_commit(
        &self,
        ctx: &Context<'_>,
        repository_id: Uuid,
        branch_name: String,
    ) -> Result<Option<CommitType>, async_graphql::Error> {
        let state = ctx.data::<Arc<AppState>>()?;
        
        let commit = crate::repository::commit::get_latest_commit(&state.pool, repository_id, &branch_name).await
            .map_err(|e| async_graphql::Error::from(e))?;
        
        Ok(commit.map(CommitType::from))
    }

    /// Health check
    async fn health_check(&self) -> Result<String, async_graphql::Error> {
        Ok("healthy".to_string())
    }
}

/// Mutation resolvers
#[Object]
impl MutationRoot {
    /// Create a new repository
    async fn create_repository(
        &self,
        ctx: &Context<'_>,
        input: CreateRepositoryInputType,
    ) -> Result<RepositoryType, async_graphql::Error> {
        let state = ctx.data::<Arc<AppState>>()?;
        
        // For now, use a fixed owner_id
        let owner_id = Uuid::parse_str("00000000-0000-0000-0000-000000000000")
            .map_err(|_| async_graphql::Error::new("Invalid owner ID"))?;
        
        let repo_input = CreateRepositoryInput {
            name: input.name,
            description: input.description,
            is_private: input.is_private,
            default_branch: input.default_branch,
        };
        
        let repo = create_repository(&state.pool, repo_input, owner_id).await
            .map_err(|e| async_graphql::Error::from(e))?;
        
        Ok(RepositoryType::from(repo))
    }

    /// Update a repository
    async fn update_repository(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        input: UpdateRepositoryInputType,
    ) -> Result<RepositoryType, async_graphql::Error> {
        let state = ctx.data::<Arc<AppState>>()?;
        
        // For now, use a fixed owner_id
        let owner_id = Uuid::parse_str("00000000-0000-0000-0000-000000000000")
            .map_err(|_| async_graphql::Error::new("Invalid owner ID"))?;
        
        let repo_input = UpdateRepositoryInput {
            name: input.name,
            description: input.description,
            is_private: input.is_private,
            default_branch: input.default_branch,
        };
        
        let repo = update_repository(&state.pool, id, owner_id, repo_input).await
            .map_err(|e| async_graphql::Error::from(e))?;
        
        Ok(RepositoryType::from(repo))
    }

    /// Delete a repository
    async fn delete_repository(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> Result<bool, async_graphql::Error> {
        let state = ctx.data::<Arc<AppState>>()?;
        
        // For now, use a fixed owner_id
        let owner_id = Uuid::parse_str("00000000-0000-0000-0000-000000000000")
            .map_err(|_| async_graphql::Error::new("Invalid owner ID"))?;
        
        delete_repository(&state.pool, id, owner_id).await
            .map_err(|e| async_graphql::Error::from(e))?;
        
        Ok(true)
    }

    /// Create a new branch
    async fn create_branch(
        &self,
        ctx: &Context<'_>,
        input: CreateBranchInputType,
    ) -> Result<BranchType, async_graphql::Error> {
        let state = ctx.data::<Arc<AppState>>()?;
        
        let branch_input = CreateBranchInput {
            name: input.name,
            commit_hash: input.commit_hash,
            is_default: input.is_default,
        };
        
        let branch = crate::repository::branch::create_branch(
            &state.pool,
            input.repository_id,
            branch_input,
        ).await
        .map_err(|e| async_graphql::Error::from(e))?;
        
        Ok(BranchType::from(branch))
    }

    /// Update a branch
    async fn update_branch(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        input: UpdateBranchInputType,
    ) -> Result<BranchType, async_graphql::Error> {
        let state = ctx.data::<Arc<AppState>>()?;
        
        let branch_input = UpdateBranchInput {
            name: input.name,
            commit_hash: input.commit_hash,
            is_default: input.is_default,
        };
        
        let branch = crate::repository::branch::update_branch(&state.pool, id, branch_input).await
            .map_err(|e| async_graphql::Error::from(e))?;
        
        Ok(BranchType::from(branch))
    }

    /// Delete a branch
    async fn delete_branch(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> Result<bool, async_graphql::Error> {
        let state = ctx.data::<Arc<AppState>>()?;
        
        crate::repository::branch::delete_branch(&state.pool, id).await
            .map_err(|e| async_graphql::Error::from(e))?;
        
        Ok(true)
    }

    /// Create a new commit
    async fn create_commit(
        &self,
        ctx: &Context<'_>,
        input: CreateCommitInputType,
    ) -> Result<CommitType, async_graphql::Error> {
        let state = ctx.data::<Arc<AppState>>()?;
        
        let commit_input = CreateCommitInput {
            hash: input.hash,
            parent_hash: input.parent_hash,
            message: input.message,
            author_name: input.author_name,
            author_email: input.author_email,
            committer_name: input.committer_name,
            committer_email: input.committer_email,
            branch_name: input.branch_name,
        };
        
        let commit = crate::repository::commit::create_commit(
            &state.pool,
            input.repository_id,
            commit_input,
        ).await
        .map_err(|e| async_graphql::Error::from(e))?;
        
        Ok(CommitType::from(commit))
    }
}

/// Subscription resolvers
#[Object]
impl SubscriptionRoot {
    // Subscriptions can be added here for real-time updates
    // For example: repository updates, branch changes, new commits
}

/// Convert GitError to async_graphql::Error
impl From<GitError> for async_graphql::Error {
    fn from(err: GitError) -> Self {
        match err {
            GitError::RepositoryNotFound => async_graphql::Error::new("Repository not found"),
            GitError::RepositoryAlreadyExists(name) => {
                async_graphql::Error::new(format!("Repository '{}' already exists", name))
            }
            GitError::BranchNotFound => async_graphql::Error::new("Branch not found"),
            GitError::BranchAlreadyExists(name) => {
                async_graphql::Error::new(format!("Branch '{}' already exists", name))
            }
            GitError::CannotDeleteDefaultBranch => {
                async_graphql::Error::new("Cannot delete default branch")
            }
            GitError::CommitNotFound => async_graphql::Error::new("Commit not found"),
            GitError::ParentCommitNotFound(hash) => {
                async_graphql::Error::new(format!("Parent commit '{}' not found", hash))
            }
            GitError::PermissionDenied => async_graphql::Error::new("Permission denied"),
            GitError::ValidationError(msg) => async_graphql::Error::new(msg),
            GitError::InvalidInput(msg) => async_graphql::Error::new(msg),
            GitError::InvalidUuid(msg) => async_graphql::Error::new(msg),
            GitError::Database(_) => async_graphql::Error::new("Database error"),
            GitError::InternalError(msg) => async_graphql::Error::new(msg),
            GitError::ConfigError(msg) => async_graphql::Error::new(msg),
        }
    }
}

/// Convert domain models to GraphQL types

impl From<Repository> for RepositoryType {
    fn from(repo: Repository) -> Self {
        Self {
            id: repo.id,
            name: repo.name,
            description: repo.description,
            is_private: repo.is_private,
            owner_id: repo.owner_id,
            default_branch: repo.default_branch,
            created_at: repo.created_at,
            updated_at: repo.updated_at,
        }
    }
}

impl From<Branch> for BranchType {
    fn from(branch: Branch) -> Self {
        Self {
            id: branch.id,
            repository_id: branch.repository_id,
            name: branch.name,
            commit_hash: branch.commit_hash,
            is_default: branch.is_default,
            created_at: branch.created_at,
            updated_at: branch.updated_at,
        }
    }
}

impl From<Commit> for CommitType {
    fn from(commit: Commit) -> Self {
        Self {
            id: commit.id,
            repository_id: commit.repository_id,
            hash: commit.hash,
            parent_hash: commit.parent_hash,
            message: commit.message,
            author_name: commit.author_name,
            author_email: commit.author_email,
            committer_name: commit.committer_name,
            committer_email: commit.committer_email,
            branch_name: commit.branch_name,
            created_at: commit.created_at,
        }
    }
}
