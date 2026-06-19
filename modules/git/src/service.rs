//! Service layer for Tardigrade Git module
//!
//! This module provides a higher-level service interface for repository operations.

use crate::error::GitError;
use crate::models::{
    CreateRepositoryInput, ListRepositoriesQuery, PaginatedResponse, Repository,
    UpdateRepositoryInput,
};
use sqlx::postgres::PgPool;
use uuid::Uuid;

/// Git service for managing repositories
#[derive(Clone)]
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
}
