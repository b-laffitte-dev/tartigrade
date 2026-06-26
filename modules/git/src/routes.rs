//! Routes configuration for Tardigrade Git module
//!
//! This module defines all the API routes for the Git module.

use axum::extract::FromRef;
use axum::http::Method;
use axum::routing::{delete, get, post, put};
use axum::Router;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use crate::handler::*;
use crate::AppState;

/// Create the main router for the Git module
pub fn create_router(pool: sqlx::postgres::PgPool) -> Router {
    let state = Arc::new(AppState::new(pool));

    // Build the router with all routes
    Router::new()
        // Repository routes
        .route("/repositories", post(create_repository_handler))
        .route("/repositories", get(list_repositories_handler))
        .route("/repositories/:id", get(get_repository_handler))
        .route("/repositories/:id", put(update_repository_handler))
        .route("/repositories/:id", delete(delete_repository_handler))
        .route(
            "/repositories/owner/:id",
            get(get_repository_by_owner_handler),
        )
        // Branch routes
        .route("/repositories/:repository_id/branches", post(create_branch_handler))
        .route("/repositories/:repository_id/branches", get(list_branches_handler))
        .route("/branches/:id", get(get_branch_handler))
        .route("/repositories/:repository_id/branches/:name", get(get_branch_by_name_handler))
        .route("/branches/:id", put(update_branch_handler))
        .route("/branches/:id", delete(delete_branch_handler))
        .route("/repositories/:repository_id/branches/default", get(get_default_branch_handler))
        // Commit routes
        .route("/repositories/:repository_id/commits", post(create_commit_handler))
        .route("/repositories/:repository_id/commits", get(list_commits_handler))
        .route("/commits/:id", get(get_commit_handler))
        .route("/repositories/:repository_id/commits/:hash", get(get_commit_by_hash_handler))
        .route("/repositories/:repository_id/branches/:branch_name/commits", get(list_commits_by_branch_handler))
        .route("/repositories/:repository_id/branches/:branch_name/latest", get(get_latest_commit_handler))
        // Health and info routes
        .route("/health", get(health_check_handler))
        .route("/api/info", get(get_api_info_handler))
        // Add CORS middleware
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::DELETE,
                    Method::OPTIONS,
                ])
                .allow_headers(Any),
        )
        // Add tracing
        .layer(TraceLayer::new_for_http())
        // Add state
        .with_state(state)
}

/// Create a router without CORS (for testing)
pub fn create_router_no_cors(pool: sqlx::postgres::PgPool) -> Router {
    let state = Arc::new(AppState::new(pool));

    Router::new()
        .route("/repositories", post(create_repository_handler))
        .route("/repositories", get(list_repositories_handler))
        .route("/repositories/:id", get(get_repository_handler))
        .route("/repositories/:id", put(update_repository_handler))
        .route("/repositories/:id", delete(delete_repository_handler))
        .route(
            "/repositories/owner/:id",
            get(get_repository_by_owner_handler),
        )
        // Branch routes
        .route("/repositories/:repository_id/branches", post(create_branch_handler))
        .route("/repositories/:repository_id/branches", get(list_branches_handler))
        .route("/branches/:id", get(get_branch_handler))
        .route("/repositories/:repository_id/branches/:name", get(get_branch_by_name_handler))
        .route("/branches/:id", put(update_branch_handler))
        .route("/branches/:id", delete(delete_branch_handler))
        .route("/repositories/:repository_id/branches/default", get(get_default_branch_handler))
        // Commit routes
        .route("/repositories/:repository_id/commits", post(create_commit_handler))
        .route("/repositories/:repository_id/commits", get(list_commits_handler))
        .route("/commits/:id", get(get_commit_handler))
        .route("/repositories/:repository_id/commits/:hash", get(get_commit_by_hash_handler))
        .route("/repositories/:repository_id/branches/:branch_name/commits", get(list_commits_by_branch_handler))
        .route("/repositories/:repository_id/branches/:branch_name/latest", get(get_latest_commit_handler))
        .route("/health", get(health_check_handler))
        .route("/api/info", get(get_api_info_handler))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

/// Create repository-specific router
pub fn create_repositories_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", post(create_repository_handler))
        .route("/", get(list_repositories_handler))
        .route("/:id", get(get_repository_handler))
        .route("/:id", put(update_repository_handler))
        .route("/:id", delete(delete_repository_handler))
}

/// Create branch-specific router
pub fn create_branches_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/:repository_id/branches", post(create_branch_handler))
        .route("/:repository_id/branches", get(list_branches_handler))
        .route("/:id", get(get_branch_handler))
        .route("/:repository_id/branches/:name", get(get_branch_by_name_handler))
        .route("/:id", put(update_branch_handler))
        .route("/:id", delete(delete_branch_handler))
        .route("/:repository_id/branches/default", get(get_default_branch_handler))
}

/// Create commit-specific router
pub fn create_commits_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/:repository_id/commits", post(create_commit_handler))
        .route("/:repository_id/commits", get(list_commits_handler))
        .route("/:id", get(get_commit_handler))
        .route("/:repository_id/commits/:hash", get(get_commit_by_hash_handler))
        .route("/:repository_id/branches/:branch_name/commits", get(list_commits_by_branch_handler))
        .route("/:repository_id/branches/:branch_name/latest", get(get_latest_commit_handler))
}

/// Create health router
pub fn create_health_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/health", get(health_check_handler))
        .route("/api/info", get(get_api_info_handler))
}

/// Combine all routers
pub fn create_all_routers(pool: sqlx::postgres::PgPool) -> Router {
    let state = Arc::new(AppState::new(pool));

    Router::new()
        .nest("/repositories", create_repositories_router())
        .nest("/branches", create_branches_router())
        .nest("/commits", create_commits_router())
        .nest("/health", create_health_router())
        .with_state(state)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::DELETE,
                    Method::OPTIONS,
                ])
                .allow_headers(Any),
        )
        .layer(TraceLayer::new_for_http())
}
