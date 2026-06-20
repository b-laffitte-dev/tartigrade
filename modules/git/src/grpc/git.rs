//! gRPC service implementation for Git module
//!
//! This module provides a gRPC service for Git operations.
//! It uses Tonic for gRPC implementation.
//!
//! NOTE: For full implementation, you need to generate protobuf code:
//! 1. Install protoc: brew install protobuf
//! 2. Install tonic-build: cargo install tonic-build
//! 3. Generate code: cargo build --features grpc
//!
//! The proto file is at: proto/git.proto

use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tonic::{Request, Response, Status};
use tower::Service;
use uuid::Uuid;

use crate::error::GitError;
use crate::models::{
    Branch, CloneRepositoryRequest, CloneRepositoryResponse, Commit, CreateBranchInput,
    CreateCommitInput, PaginatedResponse, PushRequest, PushResponse, Repository,
};
use crate::service::GitService;

/// Convert GitError to tonic Status
pub fn to_status(error: GitError) -> Status {
    match error {
        GitError::RepositoryNotFound => Status::not_found("Repository not found"),
        GitError::RepositoryAlreadyExists(name) => {
            Status::already_exists(format!("Repository '{}' already exists", name))
        }
        GitError::BranchNotFound => Status::not_found("Branch not found"),
        GitError::BranchAlreadyExists(name) => {
            Status::already_exists(format!("Branch '{}' already exists", name))
        }
        GitError::CannotDeleteDefaultBranch => {
            Status::invalid_argument("Cannot delete default branch")
        }
        GitError::CommitNotFound => Status::not_found("Commit not found"),
        GitError::CommitAlreadyExists(hash) => {
            Status::already_exists(format!("Commit with hash '{}' already exists", hash))
        }
        GitError::PermissionDenied => Status::permission_denied("Permission denied"),
        GitError::ValidationError(msg) => Status::invalid_argument(msg),
        GitError::InvalidInput(msg) => Status::invalid_argument(msg),
        GitError::InvalidUuid(msg) => Status::invalid_argument(msg),
        _ => Status::internal("Internal server error"),
    }
}

/// Helper to convert Uuid to String
pub fn uuid_to_string(uuid: Uuid) -> String {
    uuid.to_string()
}

/// Helper to convert String to Uuid
pub fn string_to_uuid(s: &str) -> Result<Uuid, Status> {
    Uuid::parse_str(s).map_err(|_| Status::invalid_argument(format!("Invalid UUID: {}", s)))
}

/// gRPC Service implementation that wraps GitService
#[derive(Debug, Clone)]
pub struct GitGrpcService {
    service: Arc<GitService>,
}

impl GitGrpcService {
    /// Create a new GitGrpcService instance
    pub fn new(service: Arc<GitService>) -> Self {
        Self { service }
    }

    /// Get reference to the underlying GitService
    pub fn git_service(&self) -> &Arc<GitService> {
        &self.service
    }

    // ============================================================================
    // Repository operations
    // ============================================================================

    /// Create a repository via gRPC
    pub async fn create_repository_grpc(
        &self,
        name: String,
        description: Option<String>,
        is_private: bool,
        default_branch: String,
        owner_id: String,
    ) -> Result<Repository, Status> {
        let owner_uuid = string_to_uuid(&owner_id)?;
        let input = crate::models::CreateRepositoryInput {
            name,
            description,
            is_private,
            default_branch,
        };
        self.service
            .create_repository(input, owner_uuid)
            .await
            .map_err(to_status)
    }

    /// Get a repository via gRPC
    pub async fn get_repository_grpc(
        &self,
        repository_id: String,
    ) -> Result<Option<Repository>, Status> {
        let repo_id = string_to_uuid(&repository_id)?;
        self.service
            .get_repository(repo_id)
            .await
            .map_err(to_status)
    }

    /// List repositories via gRPC
    pub async fn list_repositories_grpc(
        &self,
        owner_id: Option<String>,
        page: i32,
        page_size: i32,
    ) -> Result<PaginatedResponse<Repository>, Status> {
        let owner_uuid = owner_id.map(|s| string_to_uuid(&s)).transpose()?;
        let query = crate::models::ListRepositoriesQuery {
            owner_id: owner_uuid,
            page,
            page_size,
            search: None,
            is_private: None,
        };
        self.service
            .list_repositories(query)
            .await
            .map_err(to_status)
    }

    // ============================================================================
    // Branch operations
    // ============================================================================

    /// Create a branch via gRPC
    pub async fn create_branch_grpc(
        &self,
        repository_id: String,
        name: String,
        commit_hash: Option<String>,
        owner_id: String,
    ) -> Result<Branch, Status> {
        let repo_id = string_to_uuid(&repository_id)?;
        let owner_uuid = string_to_uuid(&owner_id)?;
        let input = CreateBranchInput { name, commit_hash };
        self.service
            .create_branch(repo_id, input, owner_uuid)
            .await
            .map_err(to_status)
    }

    /// Get a branch via gRPC
    pub async fn get_branch_grpc(
        &self,
        branch_id: String,
    ) -> Result<Option<Branch>, Status> {
        let branch_id_uuid = string_to_uuid(&branch_id)?;
        self.service
            .get_branch(branch_id_uuid)
            .await
            .map_err(to_status)
    }

    /// List branches via gRPC
    pub async fn list_branches_grpc(
        &self,
        repository_id: String,
        page: i32,
        page_size: i32,
    ) -> Result<PaginatedResponse<Branch>, Status> {
        let repo_id = string_to_uuid(&repository_id)?;
        self.service
            .list_branches(repo_id, page, page_size)
            .await
            .map_err(to_status)
    }

    /// Delete a branch via gRPC
    pub async fn delete_branch_grpc(
        &self,
        repository_id: String,
        branch_name: String,
        owner_id: String,
    ) -> Result<(), Status> {
        let repo_id = string_to_uuid(&repository_id)?;
        let owner_uuid = string_to_uuid(&owner_id)?;
        self.service
            .delete_branch(repo_id, &branch_name, owner_uuid)
            .await
            .map_err(to_status)
    }

    // ============================================================================
    // Commit operations
    // ============================================================================

    /// Create a commit via gRPC
    pub async fn create_commit_grpc(
        &self,
        repository_id: String,
        branch_name: String,
        message: String,
        author_name: String,
        author_email: String,
        committer_id: String,
        committer_name: String,
        committer_email: String,
    ) -> Result<Commit, Status> {
        let repo_id = string_to_uuid(&repository_id)?;
        let committer_uuid = string_to_uuid(&committer_id)?;
        let input = CreateCommitInput {
            message,
            author_name,
            author_email,
        };
        self.service
            .create_commit(
                repo_id,
                &branch_name,
                input,
                committer_uuid,
                &committer_name,
                &committer_email,
            )
            .await
            .map_err(to_status)
    }

    /// Get a commit via gRPC
    pub async fn get_commit_grpc(
        &self,
        commit_id: String,
    ) -> Result<Option<Commit>, Status> {
        let commit_id_uuid = string_to_uuid(&commit_id)?;
        self.service
            .get_commit(commit_id_uuid)
            .await
            .map_err(to_status)
    }

    /// List commits via gRPC
    pub async fn list_commits_grpc(
        &self,
        repository_id: String,
        branch_name: String,
        page: i32,
        page_size: i32,
    ) -> Result<PaginatedResponse<Commit>, Status> {
        let repo_id = string_to_uuid(&repository_id)?;
        self.service
            .list_commits(repo_id, &branch_name, page, page_size)
            .await
            .map_err(to_status)
    }

    // ============================================================================
    // Clone and Push operations
    // ============================================================================

    /// Clone a repository via gRPC
    pub async fn clone_repository_grpc(
        &self,
        repository_id: String,
        user_id: String,
        method: String,
    ) -> Result<CloneRepositoryResponse, Status> {
        let repo_id = string_to_uuid(&repository_id)?;
        let user_uuid = string_to_uuid(&user_id)?;
        let request = CloneRepositoryRequest {
            repository_id: repo_id,
            user_id: user_uuid,
            method,
        };
        self.service
            .clone_repository(request)
            .await
            .map_err(to_status)
    }

    /// Push to a repository via gRPC
    pub async fn push_to_repository_grpc(
        &self,
        repository_id: String,
        branch_name: String,
        message: String,
        author_name: String,
        author_email: String,
        user_id: String,
    ) -> Result<PushResponse, Status> {
        let repo_id = string_to_uuid(&repository_id)?;
        let user_uuid = string_to_uuid(&user_id)?;
        let request = PushRequest {
            repository_id: repo_id,
            branch_name,
            message,
            author_name,
            author_email,
            user_id: user_uuid,
        };
        // Use default committer info for gRPC
        let committer_name = "grpc-user".to_string();
        let committer_email = "grpc@example.com".to_string();
        self.service
            .push_to_repository(request, &committer_name, &committer_email)
            .await
            .map_err(to_status)
    }
}

// Re-export for convenience
// pub use GitGrpcService;
