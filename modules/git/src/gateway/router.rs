//! Router for API Gateway
//!
//! This module provides the main router for the API Gateway.

use axum::extract::FromRef;
use axum::http::Method;
use axum::routing::{delete, get, post, put};
use axum::Router;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use super::config::GatewayConfig;
use super::middleware::*;
use crate::handler::*;
use crate::AppState;

/// Create the main API Gateway router
pub fn create_gateway_router(
    pool: sqlx::postgres::PgPool,
    config: GatewayConfig,
) -> Router {
    let state = Arc::new(AppState::new(pool));

    // Create middleware stack
    let middlewares = create_middleware_stack(&config);

    // Build the router with all routes
    let mut router = Router::new()
        // Repository routes
        .route("/api/v1/repositories", post(create_repository_handler))
        .route("/api/v1/repositories", get(list_repositories_handler))
        .route("/api/v1/repositories/:id", get(get_repository_handler))
        .route("/api/v1/repositories/:id", put(update_repository_handler))
        .route("/api/v1/repositories/:id", delete(delete_repository_handler))
        
        // Branch routes
        .route("/api/v1/repositories/:repository_id/branches", post(create_branch_handler))
        .route("/api/v1/repositories/:repository_id/branches", get(list_branches_handler))
        .route("/api/v1/branches/:id", get(get_branch_handler))
        .route("/api/v1/branches/:id", put(update_branch_handler))
        .route("/api/v1/branches/:id", delete(delete_branch_handler))
        .route("/api/v1/repositories/:repository_id/branches/default", get(get_default_branch_handler))
        
        // Commit routes
        .route("/api/v1/repositories/:repository_id/commits", post(create_commit_handler))
        .route("/api/v1/repositories/:repository_id/commits", get(list_commits_handler))
        .route("/api/v1/commits/:id", get(get_commit_handler))
        .route("/api/v1/repositories/:repository_id/commits/:hash", get(get_commit_by_hash_handler))
        .route("/api/v1/repositories/:repository_id/branches/:branch_name/commits", get(list_commits_by_branch_handler))
        .route("/api/v1/repositories/:repository_id/branches/:branch_name/latest", get(get_latest_commit_handler))
        
        // Health and info routes
        .route("/health", get(health_check_handler))
        .route("/api/info", get(get_api_info_handler))
        .route("/api/v1/info", get(get_api_info_handler))
        
        // GraphQL endpoint (if feature enabled)
        #[cfg(feature = "graphql")]
        .route("/api/v1/graphql", post(crate::graphql::graphql_handler))
        .route("/api/v1/graphql", get(crate::graphql::graphql_playground_handler));

    // Apply middlewares
    for middleware in middlewares {
        router = router.layer(middleware);
    }

    // Add CORS layer
    router = router.layer(create_cors_middleware(&config));

    // Add tracing
    router = router.layer(TraceLayer::new_for_http());

    // Add state
    router = router.with_state(state);

    router
}

/// Create a router with API versioning
pub fn create_versioned_router(
    pool: sqlx::postgres::PgPool,
    config: GatewayConfig,
) -> Router {
    let state = Arc::new(AppState::new(pool));

    // Create v1 router
    let v1_router = Router::new()
        .route("/repositories", post(create_repository_handler))
        .route("/repositories", get(list_repositories_handler))
        .route("/repositories/:id", get(get_repository_handler))
        .route("/repositories/:id", put(update_repository_handler))
        .route("/repositories/:id", delete(delete_repository_handler))
        .route("/repositories/:repository_id/branches", post(create_branch_handler))
        .route("/repositories/:repository_id/branches", get(list_branches_handler))
        .route("/branches/:id", get(get_branch_handler))
        .route("/branches/:id", put(update_branch_handler))
        .route("/branches/:id", delete(delete_branch_handler))
        .route("/repositories/:repository_id/branches/default", get(get_default_branch_handler))
        .route("/repositories/:repository_id/commits", post(create_commit_handler))
        .route("/repositories/:repository_id/commits", get(list_commits_handler))
        .route("/commits/:id", get(get_commit_handler))
        .route("/repositories/:repository_id/commits/:hash", get(get_commit_by_hash_handler))
        .route("/repositories/:repository_id/branches/:branch_name/commits", get(list_commits_by_branch_handler))
        .route("/repositories/:repository_id/branches/:branch_name/latest", get(get_latest_commit_handler));

    // Create main router with versioning
    Router::new()
        .nest("/api/v1", v1_router)
        .route("/health", get(health_check_handler))
        .route("/api/info", get(get_api_info_handler))
        .layer(create_cors_middleware(&config))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

/// Create a router with service discovery
pub fn create_service_router(
    pool: sqlx::postgres::PgPool,
    config: GatewayConfig,
) -> Router {
    let state = Arc::new(AppState::new(pool));

    // Create main router
    let mut router = Router::new();

    // Add routes for each service
    for (service_name, service_config) in &config.services {
        // In a real implementation, we would proxy requests to the service
        // For now, we'll just add the routes directly
        if service_name == "git" {
            router = router
                .route("/repositories", post(create_repository_handler))
                .route("/repositories", get(list_repositories_handler))
                .route("/repositories/:id", get(get_repository_handler));
        }
    }

    // Add health check
    router = router.route("/health", get(health_check_handler));

    // Add middlewares
    router = router
        .layer(create_cors_middleware(&config))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    router
}

/// Start the API Gateway server
pub async fn start_gateway_server(
    pool: sqlx::postgres::PgPool,
    config: GatewayConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    use axum::Server;
    use std::net::SocketAddr;

    let router = create_gateway_router(pool, config.clone());

    let addr = SocketAddr::from((
        config.host.parse().unwrap_or_else(|_| "0.0.0.0".parse().unwrap()),
        config.port,
    ));

    tracing::info!("Starting API Gateway server on {}", addr);

    Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
