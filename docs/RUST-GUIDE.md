# 🦀 Rust Development Guide - Tardigrade-CI

**Version :** 1.0 | **Last Updated :** 2026-06-19 | **Status :** Draft | **Author :** Benzo + Mistral Vibe

---

## 📋 Table of Contents
1. [Introduction](#1-introduction) | 2. [Environment](#2-development-environment) | 3. [Project Structure](#3-project-structure) | 4. [Coding Standards](#4-coding-standards) | 5. [SQLx Database](#5-database-access) | 6. [gRPC](#6-grpc) | 7. [GraphQL](#7-graphql) | 8. [Error Handling](#8-error-handling) | 9. [Testing](#9-testing) | 10. [IA Workflow](#10-ia-workflow)

---

## 1️⃣ Introduction

### Why Rust?
- ✅ Memory Safety (no null, no buffer overflows)
- ✅ Performance (native speed)
- ✅ Concurrency (fearless async/await)
- ✅ Reliability (compile-time checks)
- ✅ Security (ideal for CI/CD operations)

### Learning Resources
- [The Rust Book](https://doc.rust-lang.org/book/) (Essential)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Async Rust Book](https://rust-lang.github.io/async-book/)

---

## 2️⃣ Development Environment

### Install Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Add Targets
```bash
rustup target add x86_64-unknown-linux-musl
```

### Install Tools
```bash
cargo install cargo-edit cargo-watch cargo-audit sqlx-cli tonic-build protobuf-compiler
```

### VS Code Setup
Extensions: `rust-analyzer`, `Better TOML`, `YAML`, `Docker`

`.vscode/settings.json`:
```json
{
  "rust-analyzer.checkOnSave.command": "clippy",
  "editor.formatOnSave": true
}
```

---

## 3️⃣ Project Structure

```
tardigrade-api-gateway/
├── Cargo.toml
├── migrations/            # SQLx migrations
│   └── 20240101000000_init.sql
├── proto/               # Protocol Buffers
│   └── api.proto
└── src/
    ├── main.rs
    ├── config.rs
    ├── error.rs
    ├── database/
    │   ├── mod.rs
    │   └── connection.rs
    ├── grpc/
    │   ├── server.rs
    │   └── client.rs
    ├── graphql/
    │   ├── schema.rs
    │   └── server.rs
    ├── handlers/
    ├── models/
    ├── services/
    └── utils/
```

### Cargo.toml Template
```toml
[package]
name = "tardigrade-api-gateway"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.7"
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-native-tls", "chrono", "json"] }
tonic = "0.10"
prost = "0.12"
async-graphql = "6.0"
async-graphql-axum = "6.0"
jsonwebtoken = "9.0"
bcrypt = "0.15"
thiserror = "1.0"
anyhow = "1.0"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }

[dev-dependencies]
mockall = "0.11"
tokio-test = "0.4"
serial_test = "2.0"

[build-dependencies]
tonic-build = "0.10"
prost-build = "0.12"

[profile.release]
opt-level = 3
lto = "fat"
codegen-units = 1
panic = "abort"
strip = true
```

---

## 4️⃣ Coding Standards

### Naming Conventions
| Item | Convention | Example |
|------|------------|---------|
| Crate/Module | snake_case | `user_service.rs` |
| Type/Enum | PascalCase | `UserService` |
| Variable/Function | snake_case | `get_user()` |
| Constant | SCREAMING_SNAKE | `MAX_CONNECTIONS` |
| Trait | PascalCase | `UserRepository` |

### Formatting & Linting
```bash
cargo fmt          # Format
cargo fmt --check  # Check formatting
cargo clippy       # Lint
```

### Documentation
```rust
/// Represents a user in the system.
///
/// # Examples
/// ```
/// let user = User::new("john_doe", "john@example.com");
/// ```
#[derive(Debug, Clone)]
pub struct User {
    /// Unique identifier
    pub id: Uuid,
    /// User's login username
    pub username: String,
}
```

---

## 5️⃣ Database Access (SQLx)

### Connection Pool
```rust
use sqlx::postgres::PgPoolOptions;

pub struct DatabaseConnection {
    pub pool: sqlx::PgPool,
}

impl DatabaseConnection {
    pub async fn new(url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(20)
            .connect_timeout(std::time::Duration::from_secs(30))
            .connect(url)
            .await?;
        sqlx::migrate!().run(&pool).await?;
        Ok(Self { pool })
    }
}
```

### Model Example
```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub is_active: bool,
    pub is_admin: bool,
    #[sqlx(rename = "created_at")]
    pub created_at: DateTime<Utc>,
    #[sqlx(rename = "updated_at")]
    pub updated_at: DateTime<Utc>,
}
```

### Query Examples
```rust
// SELECT with WHERE
let user = sqlx::query_as::<_, User>(
    "SELECT * FROM users WHERE id = $1"
)
.bind(user_id)
.fetch_one(&pool)
.await?;

// INSERT with RETURNING
let new_user = sqlx::query_as::<_, User>(
    "INSERT INTO users (username, email) VALUES ($1, $2) RETURNING *"
)
.bind(&username)
.bind(&email)
.fetch_one(&pool)
.await?;

// Transaction
let mut tx = pool.begin().await?;
sqlx::query_as::<_, User>(...)
    .fetch_one(&mut *tx)
    .await?;
tx.commit().await?;
```

### Migrations
```bash
sqlx migrate add create_users_table
sqlx migrate run
sqlx migrate revert
```

---

## 6️⃣ gRPC Implementation

### Protocol Buffer
```protobuf
syntax = "proto3";
package tardigrade.api;

service ApiGateway {
    rpc GetUser(GetUserRequest) returns (GetUserResponse);
    rpc CreateUser(CreateUserRequest) returns (CreateUserResponse);
}

message GetUserRequest {
    string id = 1;
}

message GetUserResponse {
    User user = 1;
}

message User {
    string id = 1;
    string username = 2;
    string email = 3;
}
```

### build.rs
```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .out_dir("src/grpc/proto")
        .compile(&["proto/api.proto"], &["proto/"])?;
    println!("cargo:rerun-if-changed=proto/");
    Ok(())
}
```

### Server Implementation
```rust
use tonic::{Request, Response, Status};

pub struct GrpcServer {
    db: Arc<DatabaseConnection>,
}

#[tonic::async_trait]
impl ApiGateway for GrpcServer {
    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<GetUserResponse>, Status> {
        let id = request.into_inner().id;
        let uuid = Uuid::parse_str(&id).map_err(|e| Status::invalid_argument(e.to_string()))?;
        
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(uuid)
            .fetch_optional(&self.db.pool)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;
        
        match user {
            Some(user) => Ok(Response::new(GetUserResponse { user: Some(user.into()) })),
            None => Err(Status::not_found("User not found")),
        }
    }
}
```

### Client Implementation
```rust
pub struct GrpcClient {
    client: ApiGatewayClient<Channel>,
}

impl GrpcClient {
    pub async fn new(addr: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let channel = Channel::from_shared(addr.to_string())?.connect().await?;
        Ok(Self { client: ApiGatewayClient::new(channel) })
    }

    pub async fn get_user(&mut self, id: &str) -> Result<Option<User>, Box<dyn std::error::Error>> {
        let request = Request::new(GetUserRequest { id: id.to_string() });
        let response = self.client.get_user(request).await?;
        Ok(response.into_inner().user.map(|u| User { ... }))
    }
}
```

---

## 7️⃣ GraphQL Implementation

### Schema Definition
```rust
use async_graphql::{Object, Schema, SimpleObject, ID};

#[derive(SimpleObject)]
pub struct UserType {
    pub id: ID,
    pub username: String,
    pub email: String,
}

#[derive(InputObject)]
pub struct CreateUserInput {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn user(&self, ctx: &Context<'_>, id: ID) -> Result<Option<UserType>> {
        let db = ctx.data::<DatabaseConnection>()?;
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(Uuid::parse_str(&id.to_string())?)
            .fetch_optional(&db.pool)
            .await?;
        Ok(user.map(Into::into))
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_user(&self, ctx: &Context<'_>, input: CreateUserInput) -> Result<UserType> {
        let db = ctx.data::<DatabaseConnection>()?;
        // Create user logic
        Ok(user.into())
    }
}

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema() -> AppSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription).finish()
}
```

### Server Setup
```rust
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{Router, routing::get, extract::State, Json};

pub async fn graphql_handler(
    State(schema): State<AppSchema>,
    request: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(request.into_inner()).await.into()
}

pub async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

pub fn create_graphql_router(db: Arc<DatabaseConnection>) -> Router {
    let schema = create_schema();
    Router::new()
        .route("/graphql", get(graphql_playground).post(graphql_handler))
        .with_state(schema)
        .with_state(db)
}
```

---

## 8️⃣ Error Handling

### Custom Error Type
```rust
use axum::response::{IntoResponse, Response};

pub type Result<T, E = AppError> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    
    #[error("Authentication required")]
    AuthenticationRequired,
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Internal server error: {0}")]
    InternalServerError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::AuthenticationRequired => (StatusCode::UNAUTHORIZED, "Authentication required"),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.as_str()),
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
            AppError::DatabaseError(e) => {
                tracing::error!("Database error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error")
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
        };
        (status, Json(json!({ "error": message }))).into_response()
    }
}
```

### Validation
```rust
use validator::Validate;

#[derive(Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 1, max = 255))]
    pub username: String,
    
    #[validate(email)]
    pub email: String,
    
    #[validate(length(min = 8))]
    pub password: String,
}

pub async fn create_user(
    State(db): State<DatabaseConnection>,
    Json(input): Json<CreateUserRequest>,
) -> Result<Json<User>, AppError> {
    input.validate()?;
    // ...
}
```

---

## 9️⃣ Testing

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_new() {
        let user = User::new("john".to_string(), "john@example.com".to_string());
        assert_eq!(user.username, "john");
    }
}
```

### Integration Tests
```rust
#[sqlx::test]
async fn test_get_user(pool: PgPool) {
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (username, email) VALUES ($1, $2) RETURNING *"
    )
    .bind("test")
    .bind("test@example.com")
    .fetch_one(&pool)
    .await
    .unwrap();

    assert_eq!(user.username, "test");
}
```

---

## 🔟1️⃣0️⃣ IA-Assisted Development Workflow

### For Java Developers

| Java | Rust | Notes |
|------|------|-------|
| `null` | `Option<T>` | No null references |
| Exceptions | `Result<T, E>` | Explicit error handling |
| Classes | `struct` + `impl` | Data and methods separate |
| Interfaces | `trait` | Similar but more powerful |
| Spring DI | Manual DI | Pass dependencies explicitly |

### IA Prompt Tips

1. **Be specific**: "Write a Rust Axum handler with SQLx for PostgreSQL"
2. **Provide context**: Include tech stack and requirements
3. **Review carefully**: Check for `unsafe`, `unwrap()`, panics
4. **Iterate**: Ask IA to revise based on feedback

### Code Review Checklist
- [ ] No `unsafe` blocks
- [ ] Proper error handling (no `unwrap()` in production)
- [ ] All inputs validated
- [ ] Database queries use prepared statements
- [ ] Authentication/authorization checks
- [ ] Appropriate logging
- [ ] Tests included
- [ ] Documentation added
- [ ] No hardcoded secrets

---

## 📚 Resources
- [Rust Book](https://doc.rust-lang.org/book/)
- [Axum Docs](https://docs.rs/axum/latest/axum/)
- [SQLx Docs](https://docs.rs/sqlx/latest/sqlx/)
- [Tonic Docs](https://docs.rs/tonic/latest/tonic/)
- [Rust Users Forum](https://users.rust-lang.org/)
- [Tardigrade-CI Discord](https://discord.gg/tardigrade-ci)

---
*Last updated: 2026-06-19*
