//! Service layer for Tardigrade Git module
//!
//! This module provides a higher-level service interface for repository operations.

use crate::error::GitError;
use crate::models::{
    Branch, CloneRepositoryRequest, CloneRepositoryResponse, Commit, CreateBranchInput, 
    CreateCommitInput, CreateRepositoryInput, ListRepositoriesQuery, PaginatedResponse, 
    PushRequest, PushResponse, Repository, UpdateRepositoryInput,
};
use sqlx::postgres::PgPool;
use uuid::Uuid;

/// Git service for managing repositories
#[derive(Debug, Clone)]
pub struct GitService {
    pool: PgPool,
}

impl GitService {
    /// Create a new GitService instance
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Create a new repository
    pub async fn create_repository(
        &self,
        input: CreateRepositoryInput,
        owner_id: Uuid,
    ) -> Result<Repository, GitError> {
        crate::repository::create_repository(&self.pool, input, owner_id).await
    }

    /// Get a repository by ID (public access)
    pub async fn get_repository(&self, id: Uuid) -> Result<Option<Repository>, GitError> {
        crate::repository::get_repository(&self.pool, id).await
    }

    /// Get a repository by ID with owner check (private access)
    pub async fn get_repository_by_owner(
        &self,
        id: Uuid,
        owner_id: Uuid,
    ) -> Result<Option<Repository>, GitError> {
        crate::repository::get_repository_by_id_and_owner(&self.pool, id, owner_id).await
    }

    /// Get a repository by name and owner
    pub async fn get_repository_by_name(
        &self,
        name: &str,
        owner_id: Uuid,
    ) -> Result<Option<Repository>, GitError> {
        crate::repository::get_repository_by_name_and_owner(&self.pool, name, owner_id).await
    }

    /// List repositories with pagination
    pub async fn list_repositories(
        &self,
        query: ListRepositoriesQuery,
    ) -> Result<PaginatedResponse<Repository>, GitError> {
        crate::repository::list_repositories(&self.pool, query).await
    }

    /// List repositories by owner
    pub async fn list_repositories_by_owner(
        &self,
        owner_id: Uuid,
        page: i32,
        page_size: i32,
    ) -> Result<PaginatedResponse<Repository>, GitError> {
        crate::repository::list_repositories_by_owner(&self.pool, owner_id, page, page_size).await
    }

    /// Update a repository
    pub async fn update_repository(
        &self,
        id: Uuid,
        owner_id: Uuid,
        input: UpdateRepositoryInput,
    ) -> Result<Repository, GitError> {
        crate::repository::update_repository(&self.pool, id, owner_id, input).await
    }

    /// Delete a repository
    pub async fn delete_repository(&self, id: Uuid, owner_id: Uuid) -> Result<(), GitError> {
        crate::repository::delete_repository(&self.pool, id, owner_id).await
    }

    /// Check if a repository exists
    pub async fn repository_exists(&self, id: Uuid) -> Result<bool, GitError> {
        crate::repository::repository_exists(&self.pool, id).await
    }

    /// Check if a repository name exists for an owner
    pub async fn repository_name_exists(
        &self,
        name: &str,
        owner_id: Uuid,
    ) -> Result<bool, GitError> {
        crate::repository::repository_name_exists(&self.pool, name, owner_id).await
    }

    /// Get repository count by owner
    pub async fn count_repositories_by_owner(&self, owner_id: Uuid) -> Result<i64, GitError> {
        crate::repository::count_repositories_by_owner(&self.pool, owner_id).await
    }

    // ============================================================================
    // Branch Operations
    // ============================================================================

    /// Create a new branch in a repository
    pub async fn create_branch(
        &self,
        repository_id: Uuid,
        input: CreateBranchInput,
        owner_id: Uuid,
    ) -> Result<Branch, GitError> {
        crate::repository::create_branch(&self.pool, repository_id, input, owner_id).await
    }

    /// Get a branch by ID
    pub async fn get_branch(&self, id: Uuid) -> Result<Option<Branch>, GitError> {
        crate::repository::get_branch(&self.pool, id).await
    }

    /// Get a branch by repository ID and branch name
    pub async fn get_branch_by_repository_and_name(
        &self,
        repository_id: Uuid,
        name: &str,
    ) -> Result<Option<Branch>, GitError> {
        crate::repository::get_branch_by_repository_and_name(&self.pool, repository_id, name).await
    }

    /// List branches for a repository with pagination
    pub async fn list_branches(
        &self,
        repository_id: Uuid,
        page: i32,
        page_size: i32,
    ) -> Result<PaginatedResponse<Branch>, GitError> {
        crate::repository::list_branches(&self.pool, repository_id, page, page_size).await
    }

    /// Delete a branch from a repository
    pub async fn delete_branch(
        &self,
        repository_id: Uuid,
        branch_name: &str,
        owner_id: Uuid,
    ) -> Result<(), GitError> {
        crate::repository::delete_branch(&self.pool, repository_id, branch_name, owner_id).await
    }

    // ============================================================================
    // Commit Operations
    // ============================================================================

    /// Create a new commit in a branch
    pub async fn create_commit(
        &self,
        repository_id: Uuid,
        branch_name: &str,
        input: CreateCommitInput,
        committer_id: Uuid,
        committer_name: &str,
        committer_email: &str,
    ) -> Result<Commit, GitError> {
        crate::repository::create_commit(
            &self.pool, repository_id, branch_name, input, committer_id, committer_name, committer_email
        ).await
    }

    /// Get a commit by ID
    pub async fn get_commit(&self, id: Uuid) -> Result<Option<Commit>, GitError> {
        crate::repository::get_commit(&self.pool, id).await
    }

    /// Get a commit by hash and repository
    pub async fn get_commit_by_hash(
        &self,
        repository_id: Uuid,
        hash: &str,
    ) -> Result<Option<Commit>, GitError> {
        crate::repository::get_commit_by_hash(&self.pool, repository_id, hash).await
    }

    /// List commits for a repository with pagination
    pub async fn list_commits(
        &self,
        repository_id: Uuid,
        branch_name: &str,
        page: i32,
        page_size: i32,
    ) -> Result<PaginatedResponse<Commit>, GitError> {
        crate::repository::list_commits(&self.pool, repository_id, branch_name, page, page_size).await
    }

    // ============================================================================
    // Clone and Push Operations
    // ============================================================================

    /// Clone a repository - generate clone URL
    pub async fn clone_repository(
        &self,
        request: CloneRepositoryRequest,
    ) -> Result<CloneRepositoryResponse, GitError> {
        let url = crate::repository::clone_repository(
            &self.pool,
            request.repository_id,
            request.user_id,
            &request.method,
        ).await?;

        Ok(CloneRepositoryResponse {
            url,
            success: true,
            error: None,
        })
    }

    /// Push to a repository - create a new commit
    pub async fn push_to_repository(
        &self,
        request: PushRequest,
        committer_name: &str,
        committer_email: &str,
    ) -> Result<PushResponse, GitError> {
        let input = CreateCommitInput {
            message: request.message,
            author_name: request.author_name,
            author_email: request.author_email,
        };

        let commit_hash = crate::repository::push_to_repository(
            &self.pool,
            request.repository_id,
            &request.branch_name,
            input,
            request.user_id,
            committer_name,
            committer_email,
        ).await?;

        Ok(PushResponse {
            success: true,
            commit_hash: Some(commit_hash),
            error: None,
        })
    }
}

/// Trait for Git service operations
#[async_trait::async_trait]
pub trait GitServiceTrait {
    async fn create_repository(
        &self,
        input: CreateRepositoryInput,
        owner_id: Uuid,
    ) -> Result<Repository, GitError>;

    async fn get_repository(&self, id: Uuid) -> Result<Option<Repository>, GitError>;

    async fn get_repository_by_owner(
        &self,
        id: Uuid,
        owner_id: Uuid,
    ) -> Result<Option<Repository>, GitError>;

    async fn list_repositories(
        &self,
        query: ListRepositoriesQuery,
    ) -> Result<PaginatedResponse<Repository>, GitError>;

    async fn update_repository(
        &self,
        id: Uuid,
        owner_id: Uuid,
        input: UpdateRepositoryInput,
    ) -> Result<Repository, GitError>;

    async fn delete_repository(&self, id: Uuid, owner_id: Uuid) -> Result<(), GitError>;

    // Branch operations
    async fn create_branch(
        &self,
        repository_id: Uuid,
        input: CreateBranchInput,
        owner_id: Uuid,
    ) -> Result<Branch, GitError>;

    async fn get_branch(&self, id: Uuid) -> Result<Option<Branch>, GitError>;

    async fn get_branch_by_repository_and_name(
        &self,
        repository_id: Uuid,
        name: &str,
    ) -> Result<Option<Branch>, GitError>;

    async fn list_branches(
        &self,
        repository_id: Uuid,
        page: i32,
        page_size: i32,
    ) -> Result<PaginatedResponse<Branch>, GitError>;

    async fn delete_branch(
        &self,
        repository_id: Uuid,
        branch_name: &str,
        owner_id: Uuid,
    ) -> Result<(), GitError>;

    // Commit operations
    async fn create_commit(
        &self,
        repository_id: Uuid,
        branch_name: &str,
        input: CreateCommitInput,
        committer_id: Uuid,
        committer_name: &str,
        committer_email: &str,
    ) -> Result<Commit, GitError>;

    async fn get_commit(&self, id: Uuid) -> Result<Option<Commit>, GitError>;

    async fn list_commits(
        &self,
        repository_id: Uuid,
        branch_name: &str,
        page: i32,
        page_size: i32,
    ) -> Result<PaginatedResponse<Commit>, GitError>;

    // Clone and push operations
    async fn clone_repository(
        &self,
        request: CloneRepositoryRequest,
    ) -> Result<CloneRepositoryResponse, GitError>;

    async fn push_to_repository(
        &self,
        request: PushRequest,
        committer_name: &str,
        committer_email: &str,
    ) -> Result<PushResponse, GitError>;
}

#[async_trait::async_trait]
impl GitServiceTrait for GitService {
    async fn create_repository(
        &self,
        input: CreateRepositoryInput,
        owner_id: Uuid,
    ) -> Result<Repository, GitError> {
        self.create_repository(input, owner_id).await
    }

    async fn get_repository(&self, id: Uuid) -> Result<Option<Repository>, GitError> {
        self.get_repository(id).await
    }

    async fn get_repository_by_owner(
        &self,
        id: Uuid,
        owner_id: Uuid,
    ) -> Result<Option<Repository>, GitError> {
        self.get_repository_by_owner(id, owner_id).await
    }

    async fn list_repositories(
        &self,
        query: ListRepositoriesQuery,
    ) -> Result<PaginatedResponse<Repository>, GitError> {
        self.list_repositories(query).await
    }

    async fn update_repository(
        &self,
        id: Uuid,
        owner_id: Uuid,
        input: UpdateRepositoryInput,
    ) -> Result<Repository, GitError> {
        self.update_repository(id, owner_id, input).await
    }

    async fn delete_repository(&self, id: Uuid, owner_id: Uuid) -> Result<(), GitError> {
        self.delete_repository(id, owner_id).await
    }

    // Branch operations
    async fn create_branch(
        &self,
        repository_id: Uuid,
        input: CreateBranchInput,
        owner_id: Uuid,
    ) -> Result<Branch, GitError> {
        self.create_branch(repository_id, input, owner_id).await
    }

    async fn get_branch(&self, id: Uuid) -> Result<Option<Branch>, GitError> {
        self.get_branch(id).await
    }

    async fn get_branch_by_repository_and_name(
        &self,
        repository_id: Uuid,
        name: &str,
    ) -> Result<Option<Branch>, GitError> {
        self.get_branch_by_repository_and_name(repository_id, name).await
    }

    async fn list_branches(
        &self,
        repository_id: Uuid,
        page: i32,
        page_size: i32,
    ) -> Result<PaginatedResponse<Branch>, GitError> {
        self.list_branches(repository_id, page, page_size).await
    }

    async fn delete_branch(
        &self,
        repository_id: Uuid,
        branch_name: &str,
        owner_id: Uuid,
    ) -> Result<(), GitError> {
        self.delete_branch(repository_id, branch_name, owner_id).await
    }

    // Commit operations
    async fn create_commit(
        &self,
        repository_id: Uuid,
        branch_name: &str,
        input: CreateCommitInput,
        committer_id: Uuid,
        committer_name: &str,
        committer_email: &str,
    ) -> Result<Commit, GitError> {
        self.create_commit(repository_id, branch_name, input, committer_id, committer_name, committer_email).await
    }

    async fn get_commit(&self, id: Uuid) -> Result<Option<Commit>, GitError> {
        self.get_commit(id).await
    }

    async fn list_commits(
        &self,
        repository_id: Uuid,
        branch_name: &str,
        page: i32,
        page_size: i32,
    ) -> Result<PaginatedResponse<Commit>, GitError> {
        self.list_commits(repository_id, branch_name, page, page_size).await
    }

    // Clone and push operations
    async fn clone_repository(
        &self,
        request: CloneRepositoryRequest,
    ) -> Result<CloneRepositoryResponse, GitError> {
        self.clone_repository(request).await
    }

    async fn push_to_repository(
        &self,
        request: PushRequest,
        committer_name: &str,
        committer_email: &str,
    ) -> Result<PushResponse, GitError> {
        self.push_to_repository(request, committer_name, committer_email).await
    }
}
