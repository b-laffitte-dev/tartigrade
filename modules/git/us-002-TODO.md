# 📌 TODO - US-002 : Cloner/pusher du code

**User Story** : [US-002](https://github.com/b-laffitte-dev/tartigrade/issues/15) - Cloner/pusher du code (8 pts)
**Branche** : `sprint-1/us-002-clone-push`
**Assigné à** : @b-laffitte-dev
**Statut** : ⏳ À démarrer (dépend de l'US-001)
**Dépendances** : [US-001](https://github.com/b-laffitte-dev/tartigrade/issues/14) (Créer un repository Git)

---

## 🎯 Objectif
Implémenter les fonctionnalités de **clonage** et de **push** pour les repositories Git, avec :
- Clonage via **SSH/HTTPS**.
- Push de code vers un repository.
- Gestion des **permissions** (propriétaire ou collaborateur).
- Gestion des **erreurs** (repository introuvable, permissions insuffisantes).

---

## ⚠️ Dépendances
> **⚠️ Cette US dépend de l'US-001 !**
> Avant de commencer, assurez-vous que :
> - [ ] Les modèles `Repository`, `Branch`, `Commit` sont implémentés (T-003).
> - [ ] La connexion PostgreSQL est configurée (T-004).
> - [ ] Les opérations CRUD pour les repositories fonctionnent (T-005).
> - [ ] Les routes Axum de base sont en place (T-008).

---

## ✅ Tâches à compléter

### 🔹 T-009 : gRPC pour Git Module (6h) - [Issue #9](https://github.com/b-laffitte-dev/tartigrade/issues/9)
**Objectif** : Implémenter une API gRPC interne pour le module Git (utilisée pour la communication entre services).

#### ✅ Critères d'acceptation
- [ ] Définir un **fichier `.proto`** pour le service Git gRPC.
- [ ] Implémenter le **serveur gRPC** avec Tonic.
- [ ] Intégrer le service gRPC avec le service Git existant.
- [ ] Écrire des **tests** pour les endpoints gRPC.
- [ ] Documenter le service gRPC (commentaires dans le `.proto`).

#### 📂 Fichiers à créer/modifier
```
modules/git/
├── proto/
│   └── git.proto          # Définition du service gRPC
├── src/
│   ├── grpc/
│   │   ├── mod.rs         # Module gRPC
│   │   ├── git.rs         # Implémentation du service gRPC
│   │   └── server.rs      # Serveur gRPC
│   └── main.rs            # Intégration du serveur gRPC
└── Cargo.toml            # Ajouter les dépendances Tonic et Prost
```

#### 📦 Dépendances à ajouter dans `Cargo.toml`
```toml
[dependencies]
# gRPC
tonic = "0.10"
prost = "0.12"
prost-types = "0.12"
tokio-stream = "0.1"

[build-dependencies]
tonic-build = "0.10"
```

#### 📄 Exemple de fichier `git.proto`
```protobuf
syntax = "proto3";

package git;

service GitService {
  // Clone a repository (not implemented in gRPC, but can be used for internal communication)
  rpc CloneRepository (CloneRepositoryRequest) returns (CloneRepositoryResponse);
  
  // Push code to a repository
  rpc PushToRepository (PushRequest) returns (PushResponse);
  
  // Create a branch (reused from US-001)
  rpc CreateBranch (CreateBranchRequest) returns (Branch);
  
  // List branches
  rpc ListBranches (ListBranchesRequest) returns (ListBranchesResponse);
  
  // Create a commit
  rpc CreateCommit (CreateCommitRequest) returns (Commit);
  
  // List commits
  rpc ListCommits (ListCommitsRequest) returns (ListCommitsResponse);
}

message CloneRepositoryRequest {
  string repository_id = 1;
  string user_id = 2;  // For permission checks
}

message CloneRepositoryResponse {
  string url = 1;  // SSH or HTTPS URL
  bool success = 2;
  string error = 3;  // Error message if any
}

message PushRequest {
  string repository_id = 1;
  string branch_name = 2;
  bytes content = 3;  // Git objects (simplified for example)
  string user_id = 4;
}

message PushResponse {
  bool success = 1;
  string commit_hash = 2;
  string error = 3;
}

message CreateBranchRequest {
  string repository_id = 1;
  string name = 2;
  string commit_hash = 3;  // Initial commit hash
  string user_id = 4;
}

message ListBranchesRequest {
  string repository_id = 1;
  int32 page = 2;
  int32 page_size = 3;
}

message ListBranchesResponse {
  repeated Branch branches = 1;
  int32 total = 2;
  int32 page = 3;
  int32 page_size = 4;
}

message CreateCommitRequest {
  string repository_id = 1;
  string branch_name = 2;
  string message = 3;
  bytes content = 4;  // Git objects
  string author_name = 5;
  string author_email = 6;
  string user_id = 7;  // For permission checks
}

message ListCommitsRequest {
  string repository_id = 1;
  string branch_name = 2;
  int32 page = 3;
  int32 page_size = 4;
}

message ListCommitsResponse {
  repeated Commit commits = 1;
  int32 total = 2;
  int32 page = 3;
  int32 page_size = 4;
}

// Reuse models from models.rs
message Branch {
  string id = 1;
  string repository_id = 2;
  string name = 3;
  string commit_hash = 4;
  string created_at = 5;
}

message Commit {
  string id = 1;
  string repository_id = 2;
  string hash = 3;
  string message = 4;
  string author_name = 5;
  string author_email = 6;
  string committer_name = 7;
  string committer_email = 8;
  string created_at = 9;
}
```

#### 💻 Exemple de code pour le serveur gRPC (`grpc/git.rs`)
```rust
use tonic::{Request, Response, Status};
use prost::Message;

use crate::models::{Branch, Commit, CreateBranchInput, CreateCommitInput, PaginatedResponse};
use crate::repository::{GitRepository, GitError};
use crate::grpc::git::{GitServiceServer, CloneRepositoryRequest, CloneRepositoryResponse, PushRequest, PushResponse};

pub struct GitGrpcService {
    repository: GitRepository,
}

impl GitGrpcService {
    pub fn new(repository: GitRepository) -> Self {
        Self { repository }
    }
}

#[tonic::async_trait]
impl GitService for GitGrpcService {
    async fn clone_repository(
        &self,
        request: Request<CloneRepositoryRequest>,
    ) -> Result<Response<CloneRepositoryResponse>, Status> {
        let req = request.into_inner();
        
        // Check permissions (simplified)
        if !self.repository.has_permission(&req.repository_id, &req.user_id).await? {
            return Err(Status::permission_denied("Permission denied"));
        }
        
        // Generate clone URL (simplified)
        let repo = self.repository.get_repository(&req.repository_id).await?;
        let clone_url = format!("git@github.com:tardigrade-ci/{}.git", repo.name);
        
        Ok(Response::new(CloneRepositoryResponse {
            url: clone_url,
            success: true,
            error: String::new(),
        }))
    }
    
    async fn push_to_repository(
        &self,
        request: Request<PushRequest>,
    ) -> Result<Response<PushResponse>, Status> {
        let req = request.into_inner();
        
        // Check permissions
        if !self.repository.has_permission(&req.repository_id, &req.user_id).await? {
            return Err(Status::permission_denied("Permission denied"));
        }
        
        // Validate branch exists
        let branch = self.repository.get_branch(&req.repository_id, &req.branch_name).await?;
        if branch.is_none() {
            return Err(Status::not_found("Branch not found"));
        }
        
        // Create commit (simplified)
        let commit_hash = self.repository.create_commit(
            &req.repository_id,
            &req.branch_name,
            &req.message,
            &req.content,
            &req.author_name,
            &req.author_email,
            &req.user_id,
        ).await?;
        
        Ok(Response::new(PushResponse {
            success: true,
            commit_hash,
            error: String::new(),
        }))
    }
}

// Helper trait for GitRepository (to be implemented)
#[async_trait]
pub trait GitRepository {
    async fn has_permission(&self, repository_id: &str, user_id: &str) -> Result<bool, GitError>;
    async fn get_repository(&self, id: &str) -> Result<Repository, GitError>;
    async fn get_branch(&self, repository_id: &str, name: &str) -> Result<Option<Branch>, GitError>;
    async fn create_commit(
        &self,
        repository_id: &str,
        branch_name: &str,
        message: &str,
        content: &[u8],
        author_name: &str,
        author_email: &str,
        user_id: &str,
    ) -> Result<String, GitError>;
}
```

#### 🔧 Configuration dans `main.rs`
```rust
mod grpc;

use grpc::git::GitGrpcService;
use tonic::transport::Server;

#[tokio::main]
async fn main() {
    // Initialize repository
    let repository = GitRepository::new(pool).await;
    
    // Start gRPC server
    let grpc_service = GitGrpcService::new(repository);
    let grpc_server = Server::builder()
        .add_service(GitServiceServer::new(grpc_service))
        .serve_with_incoming(tokio_stream::wrappers::TcpListenerStream::new(listener));
    
    // Run gRPC server in a separate task
    tokio::spawn(async move {
        grpc_server.await.unwrap();
    });
    
    // Start HTTP server (Axum) as before
    // ...
}
```

#### 🧪 Tests pour gRPC
```rust
#[tokio::test]
async fn test_grpc_clone_repository() {
    let repository = MockGitRepository::new();
    let service = GitGrpcService::new(repository);
    
    let request = Request::new(CloneRepositoryRequest {
        repository_id: "repo-123".to_string(),
        user_id: "user-456".to_string(),
    });
    
    let response = service.clone_repository(request).await.unwrap();
    assert!(response.into_inner().success);
}
```

---

### 🔹 T-010 : Intégration API Gateway (4h) - [Issue #10](https://github.com/b-laffitte-dev/tartigrade/issues/10)
**Objectif** : Intégrer le module Git avec l'**API Gateway** du projet (pour centraliser les requêtes).

#### ✅ Critères d'acceptation
- [ ] Les routes du module Git sont **exposées via l'API Gateway**.
- [ ] Les requêtes sont **redirigées** vers le module Git.
- [ ] Gestion centralisée des **erreurs** (500, 404, 400).
- [ ] **Logging** des requêtes/réponses.
- [ ] **Metrics** (optionnel : nombre de requêtes, temps de réponse).

#### 📂 Fichiers à créer/modifier
```
modules/git/
├── src/
│   └── gateway.rs       # Intégration avec l'API Gateway
└── Cargo.toml           # Ajouter les dépendances pour l'API Gateway
```

#### 💻 Exemple de code pour `gateway.rs`
```rust
use axum::{Router, routing::{get, post, delete}, Json, extract::{Path, State}};
use std::sync::Arc;

use crate::handler::repository::*;
use crate::routes;

pub struct AppState {
    pub pool: PgPool,
    // Ajouter d'autres dépendances si nécessaire
}

pub fn create_gateway_router(pool: PgPool) -> Router {
    let state = Arc::new(AppState { pool });
    
    // Réutiliser les routes existantes
    let git_router = routes::create_router(pool.clone());
    
    // Ajouter un middleware pour le logging
    Router::new()
        .nest("/git", git_router)
        .layer(tracing::info_span("git-gateway"))
}

// Exemple d'intégration avec un middleware de logging
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;

pub async fn logging_middleware<B>(request: Request<B>, next: Next<B>) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();
    
    tracing::info!("Request: {} {}", method, uri);
    
    let response = next.run(request).await;
    
    let status = response.status();
    tracing::info!("Response: {} {}", status, uri);
    
    response
}
```

#### 🔧 Intégration dans `main.rs`
```rust
use gateway::create_gateway_router;

#[tokio::main]
async fn main() {
    let pool = create_pool().await;
    
    // Créer le router de l'API Gateway
    let app = create_gateway_router(pool)
        .layer(tracing::info_span("tardigrade-gateway"));
    
    // Démarrer le serveur
    axum::Server::bind(&"0.0.0.0:3001".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

---

### 🔹 T-011 : Schéma GraphQL (4h) - [Issue #11](https://github.com/b-laffitte-dev/tartigrade/issues/11)
**Objectif** : Définir un **schéma GraphQL** pour le module Git, afin de permettre au frontend (React) d'interagir avec le backend.

#### ✅ Critères d'acceptation
- [ ] Définir les **types GraphQL** pour `Repository`, `Branch`, `Commit`.
- [ ] Implémenter les **queries** :
  - `repository(id: ID!)` → `Repository`
  - `repositories(ownerId: ID, page: Int, pageSize: Int)` → `PaginatedResponse<Repository>`
  - `branches(repositoryId: ID!, page: Int, pageSize: Int)` → `PaginatedResponse<Branch>`
  - `commits(repositoryId: ID!, branchName: String!, page: Int, pageSize: Int)` → `PaginatedResponse<Commit>`
- [ ] Implémenter les **mutations** :
  - `createRepository(input: CreateRepositoryInput!)` → `Repository`
  - `deleteRepository(id: ID!)` → `Boolean`
  - `createBranch(repositoryId: ID!, input: CreateBranchInput!)` → `Branch`
  - `createCommit(repositoryId: ID!, branchName: String!, input: CreateCommitInput!)` → `Commit`
- [ ] Intégrer avec **Apollo Server** (ou un autre serveur GraphQL).
- [ ] Écrire des **tests** pour le schéma GraphQL.

#### 📂 Fichiers à créer/modifier
```
modules/git/
├── src/
│   ├── graphql/
│   │   ├── mod.rs         # Module GraphQL
│   │   ├── schema.rs      # Définition du schéma
│   │   └── resolvers.rs    # Résolveurs GraphQL
│   └── main.rs            # Intégration du serveur GraphQL
└── Cargo.toml            # Ajouter les dépendances GraphQL
```

#### 📦 Dépendances à ajouter dans `Cargo.toml`
```toml
[dependencies]
# GraphQL
async-graphql = "6.0"
async-graphql-axum = "6.0"
```

#### 📄 Exemple de schéma GraphQL (`graphql/schema.rs`)
```rust
use async_graphql::{Object, Schema, Query, Mutation, InputObject, SimpleObject, ID};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

// Types GraphQL
#[derive(SimpleObject, Serialize, Deserialize, Debug)]
pub struct Repository {
    pub id: ID,
    pub name: String,
    pub description: Option<String>,
    pub is_private: bool,
    pub owner_id: ID,
    pub default_branch: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(SimpleObject, Serialize, Deserialize, Debug)]
pub struct Branch {
    pub id: ID,
    pub repository_id: ID,
    pub name: String,
    pub commit_hash: String,
    pub created_at: String,
}

#[derive(SimpleObject, Serialize, Deserialize, Debug)]
pub struct Commit {
    pub id: ID,
    pub repository_id: ID,
    pub hash: String,
    pub message: String,
    pub author_name: String,
    pub author_email: String,
    pub created_at: String,
}

#[derive(SimpleObject, Serialize, Deserialize, Debug)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub page: i32,
    pub page_size: i32,
    pub total: i64,
    pub total_pages: i32,
}

// Inputs
#[derive(InputObject, Serialize, Deserialize, Debug)]
pub struct CreateRepositoryInput {
    pub name: String,
    pub description: Option<String>,
    pub is_private: bool,
    pub default_branch: Option<String>,
}

#[derive(InputObject, Serialize, Deserialize, Debug)]
pub struct CreateBranchInput {
    pub name: String,
    pub commit_hash: Option<String>,
}

#[derive(InputObject, Serialize, Deserialize, Debug)]
pub struct CreateCommitInput {
    pub message: String,
    pub author_name: String,
    pub author_email: String,
}

// Query
#[derive(Default)]
pub struct Query;

#[Object]
impl Query {
    async fn repository(&self, ctx: &Context<'_>, id: ID) -> Result<Option<Repository>> {
        let repository = ctx.data::<GitRepository>().unwrap();
        repository.get_repository(&id.to_string()).await.map_err(|e| e.into())
    }
    
    async fn repositories(
        &self,
        ctx: &Context<'_>,
        owner_id: Option<ID>,
        page: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<PaginatedResponse<Repository>> {
        let repository = ctx.data::<GitRepository>().unwrap();
        let owner_id = owner_id.map(|id| id.to_string());
        repository.list_repositories(owner_id.as_deref(), page.unwrap_or(1), page_size.unwrap_or(20)).await.map_err(|e| e.into())
    }
    
    async fn branches(
        &self,
        ctx: &Context<'_>,
        repository_id: ID,
        page: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<PaginatedResponse<Branch>> {
        let repository = ctx.data::<GitRepository>().unwrap();
        repository.list_branches(&repository_id.to_string(), page.unwrap_or(1), page_size.unwrap_or(20)).await.map_err(|e| e.into())
    }
    
    async fn commits(
        &self,
        ctx: &Context<'_>,
        repository_id: ID,
        branch_name: String,
        page: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<PaginatedResponse<Commit>> {
        let repository = ctx.data::<GitRepository>().unwrap();
        repository.list_commits(&repository_id.to_string(), &branch_name, page.unwrap_or(1), page_size.unwrap_or(20)).await.map_err(|e| e.into())
    }
}

// Mutation
#[derive(Default)]
pub struct Mutation;

#[Object]
impl Mutation {
    async fn create_repository(
        &self,
        ctx: &Context<'_>,
        input: CreateRepositoryInput,
    ) -> Result<Repository> {
        let repository = ctx.data::<GitRepository>().unwrap();
        let owner_id = ctx.data::<String>().unwrap().clone(); // User ID from context
        repository.create_repository(&owner_id, input).await.map_err(|e| e.into())
    }
    
    async fn delete_repository(&self, ctx: &Context<'_>, id: ID) -> Result<bool> {
        let repository = ctx.data::<GitRepository>().unwrap();
        let owner_id = ctx.data::<String>().unwrap().clone();
        repository.delete_repository(&id.to_string(), &owner_id).await.map_err(|e| e.into())?;
        Ok(true)
    }
    
    async fn create_branch(
        &self,
        ctx: &Context<'_>,
        repository_id: ID,
        input: CreateBranchInput,
    ) -> Result<Branch> {
        let repository = ctx.data::<GitRepository>().unwrap();
        repository.create_branch(&repository_id.to_string(), input).await.map_err(|e| e.into())
    }
    
    async fn create_commit(
        &self,
        ctx: &Context<'_>,
        repository_id: ID,
        branch_name: String,
        input: CreateCommitInput,
    ) -> Result<Commit> {
        let repository = ctx.data::<GitRepository>().unwrap();
        let user_id = ctx.data::<String>().unwrap().clone();
        repository.create_commit(
            &repository_id.to_string(),
            &branch_name,
            &input.message,
            &input.author_name,
            &input.author_email,
            &user_id,
        ).await.map_err(|e| e.into())
    }
}

// Schéma GraphQL
pub type GitSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn create_schema() -> GitSchema {
    Schema::build(Query, Mutation, async_graphql::EmptySubscription)
        .data(GitRepository::new(pool)) // Injecter le repository
        .data("user_id".to_string()) // Injecter l'ID de l'utilisateur (à remplacer par l'auth)
        .finish()
}
```

#### 🔧 Intégration avec Axum (`main.rs`)
```rust
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};

#[tokio::main]
async fn main() {
    let pool = create_pool().await;
    let schema = create_schema();
    
    let app = Router::new()
        .route("/graphql", post(graphql_handler))
        .route("/graphql", get(graphql_playground))
        .layer(Extension(schema));
    
    axum::Server::bind(&"0.0.0.0:3001".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn graphql_handler(
    schema: Extension<GitSchema>,
    request: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(request.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Response::builder()
        .header("Content-Type", "text/html")
        .body(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
        .unwrap()
}
```

---

### 🔹 T-012 : Gestion branches (6h) - [Issue #12](https://github.com/b-laffitte-dev/tartigrade/issues/12)
**Objectif** : Implémenter la **création**, **liste**, et **suppression** des branches pour un repository.

#### ✅ Critères d'acceptation
- [ ] Un utilisateur peut **créer une branche** dans un repository.
- [ ] Un utilisateur peut **lister les branches** d'un repository (avec pagination).
- [ ] Un utilisateur peut **supprimer une branche** (sauf la branche par défaut).
- [ ] **Gestion des erreurs** :
  - `BranchNotFound` si la branche n'existe pas.
  - `BranchAlreadyExists` si la branche existe déjà.
  - `PermissionDenied` si l'utilisateur n'a pas les droits.
  - `CannotDeleteDefaultBranch` si on essaie de supprimer la branche par défaut.
- [ ] **Tests unitaires** pour toutes les fonctions.

#### 📂 Fichiers à modifier
- `modules/git/src/models.rs` (ajouter/modifier `Branch`)
- `modules/git/src/repository.rs` (ajouter les fonctions pour les branches)
- `modules/git/src/service.rs` (ajouter la logique métier)
- `modules/git/src/handler/branch.rs` (nouveau fichier)
- `modules/git/src/routes.rs` (ajouter les routes pour les branches)

#### 💻 Exemple de code pour `repository.rs` (branches)
```rust
// Ajouter à GitError dans error.rs
#[derive(Error, Debug)]
pub enum GitError {
    // ... autres erreurs
    #[error("Branch not found")]
    BranchNotFound,
    #[error("Branch '{0}' already exists")]
    BranchAlreadyExists(String),
    #[error("Cannot delete default branch")]
    CannotDeleteDefaultBranch,
}

// Ajouter à repository.rs
pub async fn create_branch(
    &self,
    repository_id: &Uuid,
    input: CreateBranchInput,
    owner_id: &Uuid,
) -> Result<Branch, GitError> {
    // Vérifier que le repository existe et appartient à l'utilisateur
    let repository = self.get_repository(*repository_id).await?;
    if repository.is_none() {
        return Err(GitError::RepositoryNotFound);
    }
    let repository = repository.unwrap();
    if repository.owner_id != *owner_id {
        return Err(GitError::PermissionDenied);
    }
    
    // Vérifier que la branche n'existe pas déjà
    let existing_branch = self.get_branch(*repository_id, &input.name).await?;
    if existing_branch.is_some() {
        return Err(GitError::BranchAlreadyExists(input.name));
    }
    
    // Créer la branche en base de données
    let branch = sqlx::query_as!(
        Branch,
        r#"
        INSERT INTO branches (repository_id, name, commit_hash)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
        repository_id,
        input.name,
        input.commit_hash.unwrap_or_else(|| repository.default_branch.clone())
    )
    .fetch_one(&self.pool)
    .await?;
    
    Ok(branch)
}

pub async fn list_branches(
    &self,
    repository_id: Uuid,
    page: i32,
    page_size: i32,
) -> Result<PaginatedResponse<Branch>, GitError> {
    let offset = (page - 1) * page_size;
    
    let branches = sqlx::query_as!(
        Branch,
        r#"
        SELECT * FROM branches
        WHERE repository_id = $1
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
        "#,
        repository_id,
        page_size,
        offset
    )
    .fetch_all(&self.pool)
    .await?;
    
    let total = sqlx::query_scalar!(
        i64,
        r#"SELECT COUNT(*) FROM branches WHERE repository_id = $1"#,
        repository_id
    )
    .fetch_one(&self.pool)
    .await?;
    
    let total_pages = (total as f64 / page_size as f64).ceil() as i32;
    
    Ok(PaginatedResponse {
        data: branches,
        page,
        page_size,
        total,
        total_pages,
    })
}

pub async fn delete_branch(
    &self,
    repository_id: Uuid,
    branch_name: String,
    owner_id: Uuid,
) -> Result<(), GitError> {
    // Vérifier que le repository existe et appartient à l'utilisateur
    let repository = self.get_repository(repository_id).await?;
    if repository.is_none() {
        return Err(GitError::RepositoryNotFound);
    }
    let repository = repository.unwrap();
    if repository.owner_id != owner_id {
        return Err(GitError::PermissionDenied);
    }
    
    // Vérifier que la branche existe
    let branch = self.get_branch(repository_id, &branch_name).await?;
    if branch.is_none() {
        return Err(GitError::BranchNotFound);
    }
    
    // Vérifier qu'on ne supprime pas la branche par défaut
    if branch_name == repository.default_branch {
        return Err(GitError::CannotDeleteDefaultBranch);
    }
    
    // Supprimer la branche
    sqlx::query!(
        r#"DELETE FROM branches WHERE repository_id = $1 AND name = $2"#,
        repository_id,
        branch_name
    )
    .execute(&self.pool)
    .await?;
    
    Ok(())
}
```

#### 💻 Exemple de code pour `handler/branch.rs`
```rust
use axum::{Json, extract::{Path, State, Query}};
use serde::Deserialize;

use crate::models::{Branch, CreateBranchInput, PaginatedResponse};
use crate::service::GitService;
use crate::error::GitError;

#[derive(Deserialize)]
pub struct PaginationQuery {
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

pub async fn create_branch(
    State(service): State<GitService>,
    Path(repository_id): Path<Uuid>,
    Json(input): Json<CreateBranchInput>,
    owner_id: Uuid, // À extraire du token JWT
) -> Result<Json<Branch>, GitError> {
    let branch = service.create_branch(repository_id, input, owner_id).await?;
    Ok(Json(branch))
}

pub async fn list_branches(
    State(service): State<GitService>,
    Path(repository_id): Path<Uuid>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<PaginatedResponse<Branch>>, GitError> {
    let page = pagination.page.unwrap_or(1);
    let page_size = pagination.page_size.unwrap_or(20);
    let branches = service.list_branches(repository_id, page, page_size).await?;
    Ok(Json(branches))
}

pub async fn delete_branch(
    State(service): State<GitService>,
    Path((repository_id, branch_name)): Path<(Uuid, String)>,
    owner_id: Uuid,
) -> Result<Json<()>, GitError> {
    service.delete_branch(repository_id, branch_name, owner_id).await?;
    Ok(Json(()))
}
```

#### 🔧 Ajouter les routes dans `routes.rs`
```rust
use crate::handler::branch::*;

pub fn create_router(pool: PgPool) -> Router {
    let service = GitService::new(pool);
    
    Router::new()
        .nest("/repositories", repository_router(service.clone()))
        .nest("/repositories/:repository_id/branches", branches_router(service))
}

pub fn branches_router(service: GitService) -> Router {
    Router::new()
        .route("/", post(create_branch).get(list_branches))
        .route("/:branch_name", delete(delete_branch))
        .layer(Extension(service))
}
```

---

### 🔹 T-013 : Commits basiques (6h) - [Issue #13](https://github.com/b-laffitte-dev/tartigrade/issues/13)
**Objectif** : Implémenter la **création** et la **liste** des commits pour une branche.

#### ✅ Critères d'acceptation
- [ ] Un utilisateur peut **créer un commit** dans une branche.
- [ ] Un utilisateur peut **lister les commits** d'une branche (avec pagination).
- [ ] **Stockage des métadonnées** :
  - Hash du commit (généré ou fourni).
  - Message du commit.
  - Auteur (nom, email).
  - Date de création.
- [ ] **Gestion des erreurs** :
  - `BranchNotFound` si la branche n'existe pas.
  - `PermissionDenied` si l'utilisateur n'a pas les droits.
- [ ] **Tests unitaires** pour toutes les fonctions.

#### 📂 Fichiers à modifier
- `modules/git/src/models.rs` (ajouter/modifier `Commit`)
- `modules/git/src/repository.rs` (ajouter les fonctions pour les commits)
- `modules/git/src/service.rs` (ajouter la logique métier)
- `modules/git/src/handler/commit.rs` (nouveau fichier)
- `modules/git/src/routes.rs` (ajouter les routes pour les commits)

#### 💻 Exemple de code pour `repository.rs` (commits)
```rust
// Ajouter à GitError dans error.rs
#[derive(Error, Debug)]
pub enum GitError {
    // ... autres erreurs
    #[error("Commit not found")]
    CommitNotFound,
}

// Ajouter à repository.rs
pub async fn create_commit(
    &self,
    repository_id: Uuid,
    branch_name: String,
    message: String,
    author_name: String,
    author_email: String,
    committer_id: Uuid,
) -> Result<Commit, GitError> {
    // Vérifier que le repository existe
    let repository = self.get_repository(repository_id).await?;
    if repository.is_none() {
        return Err(GitError::RepositoryNotFound);
    }
    
    // Vérifier que la branche existe
    let branch = self.get_branch(repository_id, &branch_name).await?;
    if branch.is_none() {
        return Err(GitError::BranchNotFound);
    }
    
    // Vérifier les permissions (simplifié : seul le propriétaire peut commiter)
    let repository = repository.unwrap();
    if repository.owner_id != committer_id {
        return Err(GitError::PermissionDenied);
    }
    
    // Générer un hash de commit (simplifié : UUID)
    let commit_hash = Uuid::new_v4().to_string();
    
    // Créer le commit en base de données
    let commit = sqlx::query_as!(
        Commit,
        r#"
        INSERT INTO commits (repository_id, hash, message, author_name, author_email, committer_id)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#,
        repository_id,
        commit_hash,
        message,
        author_name,
        author_email,
        committer_id
    )
    .fetch_one(&self.pool)
    .await?;
    
    // Mettre à jour la branche avec le nouveau commit hash
    sqlx::query!(
        r#"UPDATE branches SET commit_hash = $1 WHERE repository_id = $2 AND name = $3"#,
        commit_hash,
        repository_id,
        branch_name
    )
    .execute(&self.pool)
    .await?;
    
    Ok(commit)
}

pub async fn list_commits(
    &self,
    repository_id: Uuid,
    branch_name: String,
    page: i32,
    page_size: i32,
) -> Result<PaginatedResponse<Commit>, GitError> {
    // Vérifier que le repository existe
    let repository = self.get_repository(repository_id).await?;
    if repository.is_none() {
        return Err(GitError::RepositoryNotFound);
    }
    
    // Vérifier que la branche existe
    let branch = self.get_branch(repository_id, &branch_name).await?;
    if branch.is_none() {
        return Err(GitError::BranchNotFound);
    }
    
    let offset = (page - 1) * page_size;
    
    let commits = sqlx::query_as!(
        Commit,
        r#"
        SELECT * FROM commits
        WHERE repository_id = $1 AND branch_name = $2
        ORDER BY created_at DESC
        LIMIT $3 OFFSET $4
        "#,
        repository_id,
        branch_name,
        page_size,
        offset
    )
    .fetch_all(&self.pool)
    .await?;
    
    let total = sqlx::query_scalar!(
        i64,
        r#"SELECT COUNT(*) FROM commits WHERE repository_id = $1 AND branch_name = $2"#,
        repository_id,
        branch_name
    )
    .fetch_one(&self.pool)
    .await?;
    
    let total_pages = (total as f64 / page_size as f64).ceil() as i32;
    
    Ok(PaginatedResponse {
        data: commits,
        page,
        page_size,
        total,
        total_pages,
    })
}
```

#### 💻 Exemple de code pour `handler/commit.rs`
```rust
use axum::{Json, extract::{Path, State, Query}};
use serde::Deserialize;

use crate::models::{Commit, CreateCommitInput, PaginatedResponse};
use crate::service::GitService;
use crate::error::GitError;

#[derive(Deserialize)]
pub struct CommitQuery {
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

pub async fn create_commit(
    State(service): State<GitService>,
    Path((repository_id, branch_name)): Path<(Uuid, String)>,
    Json(input): Json<CreateCommitInput>,
    committer_id: Uuid, // À extraire du token JWT
) -> Result<Json<Commit>, GitError> {
    let commit = service.create_commit(
        repository_id,
        branch_name,
        input.message,
        input.author_name,
        input.author_email,
        committer_id,
    ).await?;
    Ok(Json(commit))
}

pub async fn list_commits(
    State(service): State<GitService>,
    Path((repository_id, branch_name)): Path<(Uuid, String)>,
    Query(pagination): Query<CommitQuery>,
) -> Result<Json<PaginatedResponse<Commit>>, GitError> {
    let page = pagination.page.unwrap_or(1);
    let page_size = pagination.page_size.unwrap_or(20);
    let commits = service.list_commits(repository_id, branch_name, page, page_size).await?;
    Ok(Json(commits))
}
```

#### 🔧 Ajouter les routes dans `routes.rs`
```rust
use crate::handler::commit::*;

pub fn branches_router(service: GitService) -> Router {
    Router::new()
        .route("/", post(create_branch).get(list_branches))
        .route("/:branch_name", delete(delete_branch))
        .nest("/:branch_name/commits", commits_router(service.clone()))
        .layer(Extension(service))
}

pub fn commits_router(service: GitService) -> Router {
    Router::new()
        .route("/", post(create_commit).get(list_commits))
        .layer(Extension(service))
}
```

---

## 📅 Planning suggéré pour l'US-002
| **Jour**  | **Tâches** | **Durée** | **Priorité** | **Dépendances** |
|-----------|------------|-----------|--------------|-----------------|
| Jour 1    | T-009 (gRPC) | 6h | ⭐⭐⭐ | US-001 |
| Jour 2    | T-011 (GraphQL) + T-010 (API Gateway) | 8h | ⭐⭐⭐ | US-001 |
| Jour 3    | T-012 (Gestion branches) | 6h | ⭐⭐⭐ | US-001 |
| Jour 4    | T-013 (Commits basiques) | 6h | ⭐⭐⭐ | US-001 + T-012 |

---

## 🔥 Commandes utiles

### Build et test
```bash
# Build
cargo build

# Check (vérification des types)
cargo check

# Lint (Clippy)
cargo clippy -- -D warnings

# Format
cargo fmt

# Tests unitaires
cargo test --lib

# Tests avec couverture
cargo tarpaulin --lib

# Générer le code gRPC (si modification du .proto)
cargo build --features grpc

# Démarrer le serveur GraphQL
cargo run --bin tardigrade-git-graphql
```

### PostgreSQL
```bash
# Appliquer les migrations
sqlx migrate run

# Vérifier les tables
psql -h localhost -U postgres -d tardigrade -c "\dt"
```

### gRPC
```bash
# Générer le code à partir du .proto
protoc --rust_out=src/grpc --grpc_out=src/grpc --plugin=protoc-gen-grpc=`which grpc_rust_plugin` proto/git.proto

# Démarrer le serveur gRPC
cargo run --bin tardigrade-git-grpc
```

---

## 📚 Ressources
- [Documentation Tonic (gRPC)](https://docs.rs/tonic/latest/tonic/)
- [Documentation async-graphql](https://docs.rs/async-graphql/latest/async_graphql/)
- [Documentation SQLx](https://docs.rs/sqlx/latest/sqlx/)
- [SPRINT-1-PLAN.md](https://github.com/b-laffitte-dev/tartigrade/blob/main/SPRINT-1-PLAN.md)
- [ARCHITECTURE.md](https://github.com/b-laffitte-dev/tartigrade/blob/main/ARCHITECTURE.md)

---

## ✅ Checklist avant de merger
- [ ] Toutes les tâches (T-009 à T-013) sont complétées.
- [ ] `cargo build` passe sans erreur.
- [ ] `cargo clippy -- -D warnings` passe sans warning.
- [ ] `cargo test --lib` passe avec >80% de couverture.
- [ ] Les endpoints gRPC fonctionnent (test avec `grpcurl` ou un client gRPC).
- [ ] Le schéma GraphQL est fonctionnel (test avec GraphQL Playground).
- [ ] Les routes REST pour les branches et commits fonctionnent.
- [ ] La documentation (`rustdoc`) est à jour.
- [ ] Les commits suivent les conventions (ex: `feat(git): add gRPC support`).

---

## ⚠️ Notes importantes
1. **Dépendances** : Cette US **dépend de l'US-001** ! Assurez-vous que les modèles, la connexion PostgreSQL, et les opérations CRUD de base sont implémentés avant de commencer.
2. **gRPC vs REST** : Le gRPC est utilisé pour la **communication interne** entre services (ex: entre le module Git et le module CI). Le REST/GraphQL est utilisé pour l'**API publique** (frontend).
3. **GraphQL** : Le schéma GraphQL doit **réutiliser les modèles** définis dans `models.rs` (ex: `Repository`, `Branch`, `Commit`).
4. **Permissions** : Pour l'instant, on suppose que seul le **propriétaire** du repository peut effectuer des actions. Une gestion plus fine des permissions sera ajoutée plus tard.

---

**🚀 Prêt à coder !**
Commencez par la **T-009** (gRPC) une fois que l'US-001 est terminée, et suivez le TODO dans l'ordre.
