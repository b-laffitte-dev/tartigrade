//! GraphQL schema for Tardigrade Git module
//!
//! This module defines the GraphQL types, queries, and mutations for the Git API.

use async_graphql::*;
use axum::{Extension, Router};
use axum::response::IntoResponse;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::GitError;
use crate::models::{Branch, Commit, CreateBranchInput, CreateCommitInput, CreateRepositoryInput, PaginatedResponse, Repository, UpdateRepositoryInput};
use crate::service::GitService;

// Re-export types for convenience
pub use async_graphql::Schema;
pub use async_graphql::EmptySubscription;

// ============================================================================
// GraphQL Types
// ============================================================================

/// Repository type for GraphQL
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
#[graphql(name = "Repository")]
pub struct GraphqlRepository {
    pub id: ID,
    pub name: String,
    pub description: Option<String>,
    pub is_private: bool,
    pub owner_id: ID,
    pub default_branch: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Repository> for GraphqlRepository {
    fn from(repo: Repository) -> Self {
        Self {
            id: ID::from(repo.id.to_string()),
            name: repo.name,
            description: repo.description,
            is_private: repo.is_private,
            owner_id: ID::from(repo.owner_id.to_string()),
            default_branch: repo.default_branch,
            created_at: repo.created_at.to_rfc3339(),
            updated_at: repo.updated_at.to_rfc3339(),
        }
    }
}

/// Branch type for GraphQL
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
#[graphql(name = "Branch")]
pub struct GraphqlBranch {
    pub id: ID,
    pub repository_id: ID,
    pub name: String,
    pub commit_hash: String,
    pub created_at: String,
}

impl From<Branch> for GraphqlBranch {
    fn from(branch: Branch) -> Self {
        Self {
            id: ID::from(branch.id.to_string()),
            repository_id: ID::from(branch.repository_id.to_string()),
            name: branch.name,
            commit_hash: branch.commit_hash,
            created_at: branch.created_at.to_rfc3339(),
        }
    }
}

/// Commit type for GraphQL
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
#[graphql(name = "Commit")]
pub struct GraphqlCommit {
    pub id: ID,
    pub repository_id: ID,
    pub hash: String,
    pub message: String,
    pub author_name: String,
    pub author_email: String,
    pub committer_name: String,
    pub committer_email: String,
    pub created_at: String,
}

impl From<Commit> for GraphqlCommit {
    fn from(commit: Commit) -> Self {
        Self {
            id: ID::from(commit.id.to_string()),
            repository_id: ID::from(commit.repository_id.to_string()),
            hash: commit.hash,
            message: commit.message,
            author_name: commit.author_name,
            author_email: commit.author_email,
            committer_name: commit.committer_name,
            committer_email: commit.committer_email,
            created_at: commit.created_at.to_rfc3339(),
        }
    }
}

/// Paginated response type for GraphQL
#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
#[graphql(name = "PaginatedResponse")]
pub struct GraphqlPaginatedResponse<T: OutputType> {
    pub data: Vec<T>,
    pub page: i32,
    pub page_size: i32,
    pub total: i64,
    pub total_pages: i32,
}

impl<T: OutputType + From<U>, U> From<PaginatedResponse<U>> for GraphqlPaginatedResponse<T> {
    fn from(paginated: PaginatedResponse<U>) -> Self {
        Self {
            data: paginated.data.into_iter().map(|item| item.into()).collect(),
            page: paginated.page,
            page_size: paginated.page_size,
            total: paginated.total,
            total_pages: paginated.total_pages,
        }
    }
}

// ============================================================================
// Input Types
// ============================================================================

/// Input for creating a repository
#[derive(Debug, Clone, Serialize, Deserialize, InputObject)]
#[graphql(name = "CreateRepositoryInput")]
pub struct GraphqlCreateRepositoryInput {
    pub name: String,
    pub description: Option<String>,
    #[graphql(default = false)]
    pub is_private: bool,
    #[graphql(default = "main")]
    pub default_branch: String,
}

impl From<GraphqlCreateRepositoryInput> for CreateRepositoryInput {
    fn from(input: GraphqlCreateRepositoryInput) -> Self {
        Self {
            name: input.name,
            description: input.description,
            is_private: input.is_private,
            default_branch: input.default_branch,
        }
    }
}

/// Input for updating a repository
#[derive(Debug, Clone, Serialize, Deserialize, InputObject)]
#[graphql(name = "UpdateRepositoryInput")]
pub struct GraphqlUpdateRepositoryInput {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_private: Option<bool>,
    pub default_branch: Option<String>,
}

impl From<GraphqlUpdateRepositoryInput> for UpdateRepositoryInput {
    fn from(input: GraphqlUpdateRepositoryInput) -> Self {
        Self {
            name: input.name,
            description: input.description,
            is_private: input.is_private,
            default_branch: input.default_branch,
        }
    }
}

/// Input for creating a branch
#[derive(Debug, Clone, Serialize, Deserialize, InputObject)]
#[graphql(name = "CreateBranchInput")]
pub struct GraphqlCreateBranchInput {
    pub name: String,
    pub commit_hash: Option<String>,
}

impl From<GraphqlCreateBranchInput> for CreateBranchInput {
    fn from(input: GraphqlCreateBranchInput) -> Self {
        Self {
            name: input.name,
            commit_hash: input.commit_hash,
        }
    }
}

/// Input for creating a commit
#[derive(Debug, Clone, Serialize, Deserialize, InputObject)]
#[graphql(name = "CreateCommitInput")]
pub struct GraphqlCreateCommitInput {
    pub message: String,
    pub author_name: String,
    pub author_email: String,
}

impl From<GraphqlCreateCommitInput> for CreateCommitInput {
    fn from(input: GraphqlCreateCommitInput) -> Self {
        Self {
            message: input.message,
            author_name: input.author_name,
            author_email: input.author_email,
        }
    }
}

// ============================================================================
// Query Root
// ============================================================================

/// GraphQL Query root
#[derive(Debug, Default)]
pub struct Query;

#[Object]
impl Query {
    /// Get a repository by ID
    async fn repository(
        &self,
        ctx: &Context<'_>,
        id: ID,
    ) -> Result<Option<GraphqlRepository>> {
        let service = ctx.data::<GitService>()?;
        let uuid = Uuid::parse_str(&id).map_err(|_| async_graphql::Error::new("Invalid UUID"))?;
        let repo = service.get_repository(uuid).await.map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(repo.map(|r| r.into()))
    }

    /// List repositories with optional filtering and pagination
    async fn repositories(
        &self,
        ctx: &Context<'_>,
        owner_id: Option<ID>,
        page: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<GraphqlPaginatedResponse<GraphqlRepository>> {
        let service = ctx.data::<GitService>()?;
        
        let owner_uuid = owner_id.map(|id| Uuid::parse_str(&id)).transpose()
            .map_err(|_| async_graphql::Error::new("Invalid owner UUID"))?;
        
        let query = crate::models::ListRepositoriesQuery {
            owner_id: owner_uuid,
            page: page.unwrap_or(1),
            page_size: page_size.unwrap_or(20),
            search: None,
            is_private: None,
        };
        
        let paginated = service.list_repositories(query).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(paginated.into())
    }

    /// Get a branch by ID
    async fn branch(
        &self,
        ctx: &Context<'_>,
        id: ID,
    ) -> Result<Option<GraphqlBranch>> {
        let service = ctx.data::<GitService>()?;
        let uuid = Uuid::parse_str(&id).map_err(|_| async_graphql::Error::new("Invalid UUID"))?;
        let branch = service.get_branch(uuid).await.map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(branch.map(|b| b.into()))
    }

    /// List branches for a repository
    async fn branches(
        &self,
        ctx: &Context<'_>,
        repository_id: ID,
        page: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<GraphqlPaginatedResponse<GraphqlBranch>> {
        let service = ctx.data::<GitService>()?;
        let repo_uuid = Uuid::parse_str(&repository_id).map_err(|_| async_graphql::Error::new("Invalid UUID"))?;
        let paginated = service.list_branches(repo_uuid, page.unwrap_or(1), page_size.unwrap_or(20)).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(paginated.into())
    }

    /// Get a commit by ID
    async fn commit(
        &self,
        ctx: &Context<'_>,
        id: ID,
    ) -> Result<Option<GraphqlCommit>> {
        let service = ctx.data::<GitService>()?;
        let uuid = Uuid::parse_str(&id).map_err(|_| async_graphql::Error::new("Invalid UUID"))?;
        let commit = service.get_commit(uuid).await.map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(commit.map(|c| c.into()))
    }

    /// List commits for a repository and branch
    async fn commits(
        &self,
        ctx: &Context<'_>,
        repository_id: ID,
        branch_name: String,
        page: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<GraphqlPaginatedResponse<GraphqlCommit>> {
        let service = ctx.data::<GitService>()?;
        let repo_uuid = Uuid::parse_str(&repository_id).map_err(|_| async_graphql::Error::new("Invalid UUID"))?;
        let paginated = service.list_commits(repo_uuid, &branch_name, page.unwrap_or(1), page_size.unwrap_or(20)).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(paginated.into())
    }
}

// ============================================================================
// Mutation Root
// ============================================================================

/// GraphQL Mutation root
#[derive(Debug, Default)]
pub struct Mutation;

#[Object]
impl Mutation {
    /// Create a new repository
    async fn create_repository(
        &self,
        ctx: &Context<'_>,
        input: GraphqlCreateRepositoryInput,
    ) -> Result<GraphqlRepository> {
        let service = ctx.data::<GitService>()?;
        // For now, use a fixed owner_id
        let owner_id = Uuid::parse_str("00000000-0000-0000-0000-000000000000").map_err(|_| async_graphql::Error::new("Invalid owner UUID"))?;
        let repo = service.create_repository(input.into(), owner_id).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(repo.into())
    }

    /// Update a repository
    async fn update_repository(
        &self,
        ctx: &Context<'_>,
        id: ID,
        input: GraphqlUpdateRepositoryInput,
    ) -> Result<GraphqlRepository> {
        let service = ctx.data::<GitService>()?;
        let uuid = Uuid::parse_str(&id).map_err(|_| async_graphql::Error::new("Invalid UUID"))?;
        let owner_id = Uuid::parse_str("00000000-0000-0000-0000-000000000000").map_err(|_| async_graphql::Error::new("Invalid owner UUID"))?;
        let repo = service.update_repository(uuid, owner_id, input.into()).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(repo.into())
    }

    /// Delete a repository
    async fn delete_repository(
        &self,
        ctx: &Context<'_>,
        id: ID,
    ) -> Result<bool> {
        let service = ctx.data::<GitService>()?;
        let uuid = Uuid::parse_str(&id).map_err(|_| async_graphql::Error::new("Invalid UUID"))?;
        let owner_id = Uuid::parse_str("00000000-0000-0000-0000-000000000000").map_err(|_| async_graphql::Error::new("Invalid owner UUID"))?;
        service.delete_repository(uuid, owner_id).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(true)
    }

    /// Create a new branch
    async fn create_branch(
        &self,
        ctx: &Context<'_>,
        repository_id: ID,
        input: GraphqlCreateBranchInput,
    ) -> Result<GraphqlBranch> {
        let service = ctx.data::<GitService>()?;
        let repo_uuid = Uuid::parse_str(&repository_id).map_err(|_| async_graphql::Error::new("Invalid UUID"))?;
        let owner_id = Uuid::parse_str("00000000-0000-0000-0000-000000000000").map_err(|_| async_graphql::Error::new("Invalid owner UUID"))?;
        let branch = service.create_branch(repo_uuid, input.into(), owner_id).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(branch.into())
    }

    /// Delete a branch
    async fn delete_branch(
        &self,
        ctx: &Context<'_>,
        repository_id: ID,
        branch_name: String,
    ) -> Result<bool> {
        let service = ctx.data::<GitService>()?;
        let repo_uuid = Uuid::parse_str(&repository_id).map_err(|_| async_graphql::Error::new("Invalid UUID"))?;
        let owner_id = Uuid::parse_str("00000000-0000-0000-0000-000000000000").map_err(|_| async_graphql::Error::new("Invalid owner UUID"))?;
        service.delete_branch(repo_uuid, &branch_name, owner_id).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(true)
    }

    /// Create a new commit
    async fn create_commit(
        &self,
        ctx: &Context<'_>,
        repository_id: ID,
        branch_name: String,
        input: GraphqlCreateCommitInput,
    ) -> Result<GraphqlCommit> {
        let service = ctx.data::<GitService>()?;
        let repo_uuid = Uuid::parse_str(&repository_id).map_err(|_| async_graphql::Error::new("Invalid UUID"))?;
        let committer_id = Uuid::parse_str("00000000-0000-0000-0000-000000000000").map_err(|_| async_graphql::Error::new("Invalid UUID"))?;
        let committer_name = "test-user".to_string();
        let committer_email = "test@example.com".to_string();
        let commit = service.create_commit(
            repo_uuid,
            &branch_name,
            input.into(),
            committer_id,
            &committer_name,
            &committer_email,
        ).await.map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(commit.into())
    }
}

// ============================================================================
// Schema
// ============================================================================

/// GraphQL Schema type
pub type GitSchema = Schema<Query, Mutation, EmptySubscription>;

/// Create a new GraphQL schema
pub fn create_schema(pool: sqlx::postgres::PgPool) -> GitSchema {
    let service = GitService::new(pool);
    
    Schema::build(Query, Mutation, EmptySubscription)
        .data(service)
        .finish()
}

/// Create a GraphQL router for Axum
pub fn create_graphql_router(pool: sqlx::postgres::PgPool) -> Router {
    use async_graphql_axum::GraphQL;
    
    let schema = create_schema(pool);

    Router::new()
        .route(
            "/graphql",
            axum::routing::get(graphql_playground),
        )
        .route_service("/graphql", GraphQL::new(schema))
}

/// GraphQL playground handler for GET requests
async fn graphql_playground() -> impl IntoResponse {
    // Simple HTML playground
    axum::response::Html(
        r#"<html>
            <head>
                <title>GraphQL Playground</title>
            </head>
            <body>
                <h1>GraphQL Playground</h1>
                <p>Use a GraphQL client like Postman, Insomnia, or GraphiQL to send POST requests to /graphql</p>
            </body>
        </html>"#.to_string(),
    )
}
