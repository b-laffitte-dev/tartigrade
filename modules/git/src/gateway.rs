//! API Gateway for Tardigrade Git module
//!
//! This module provides a centralized API Gateway that:
//! - Routes requests to the appropriate handlers
//! - Centralizes error handling
//! - Adds logging for all requests/responses

use axum::body::Body;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Router;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::{debug, error, info};
use uuid::Uuid;

use crate::error::{ErrorResponse, GitError};
use crate::handler::get_commit;
use crate::routes::*;
use crate::AppState;

/// API Gateway configuration
#[derive(Debug, Clone)]
pub struct GatewayConfig {
    /// Enable request/response logging
    pub enable_logging: bool,
    /// Enable metrics collection
    pub enable_metrics: bool,
    /// Service name for tracing
    pub service_name: String,
}

impl Default for GatewayConfig {
    fn default() -> Self {
        Self {
            enable_logging: true,
            enable_metrics: false,
            service_name: "tardigrade-git-gateway".to_string(),
        }
    }
}

/// Create the main API Gateway router
/// This router wraps all existing routes under /api/v1 and adds gateway functionality
pub fn create_gateway_router(pool: sqlx::postgres::PgPool) -> Router {
    let state = Arc::new(AppState::new(pool));

    // Build the gateway router with all routes nested under /api/v1
    Router::new()
        .nest("/api/v1/repositories", create_repositories_router())
        .nest(
            "/api/v1/repositories/:repository_id/branches",
            create_branches_router(),
        )
        .nest(
            "/api/v1/repositories/:repository_id/branches/:branch_name/commits",
            create_commits_router(),
        )
        .nest(
            "/api/v1/repositories/:repository_id",
            create_clone_push_router(),
        )
        .nest(
            "/api/v1/commits",
            Router::new().route("/:commit_id", axum::routing::get(get_commit)),
        )
        .nest("/api/v1/health", create_health_router())
        .route("/health", axum::routing::get(gateway_health_check))
        .route("/api/info", axum::routing::get(gateway_api_info))
        // Add tracing layer
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
        )
        // Add state to all routes
        .with_state(state)
}

/// Health check endpoint for the gateway
pub async fn gateway_health_check() -> impl IntoResponse {
    info!("Gateway health check");
    axum::Json(serde_json::json!({
        "status": "healthy",
        "service": "tardigrade-git-gateway",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

/// API info endpoint for the gateway
pub async fn gateway_api_info() -> impl IntoResponse {
    info!("Gateway API info requested");
    axum::Json(serde_json::json!({
        "service": "tardigrade-git-gateway",
        "version": env!("CARGO_PKG_VERSION"),
        "endpoints": {
            "repositories": "/api/v1/repositories",
            "branches": "/api/v1/repositories/:id/branches",
            "commits": "/api/v1/repositories/:id/branches/:name/commits",
            "clone": "/api/v1/repositories/:id/clone",
            "push": "/api/v1/repositories/:id/push",
            "health": "/health"
        }
    }))
}

/// Centralized error handler for the API Gateway
/// This can be used in handlers to standardize error responses
pub fn handle_git_error(error: GitError) -> impl IntoResponse {
    let status = error.status_code();
    let error_response = error.to_error_response();

    // Log the error appropriately
    match status {
        StatusCode::NOT_FOUND => debug!("Not found: {}", error_response.message),
        StatusCode::BAD_REQUEST => debug!("Bad request: {}", error_response.message),
        StatusCode::FORBIDDEN => debug!("Forbidden: {}", error_response.message),
        StatusCode::CONFLICT => debug!("Conflict: {}", error_response.message),
        _ => error!("Internal server error: {}", error_response.message),
    }

    (status, axum::Json(error_response))
}

/// Generate a unique request ID for tracing
pub fn generate_request_id() -> String {
    Uuid::new_v4().to_string()
}

/// Metrics collector (placeholder for future implementation)
#[derive(Debug, Clone, Default)]
pub struct GatewayMetrics {
    // Placeholder for metrics
}

impl GatewayMetrics {
    /// Record a request for metrics
    pub fn record_request(&self, method: &str, path: &str, status: StatusCode) {
        debug!("Metrics: {} {} -> {}", method, path, status);
    }

    /// Record latency for metrics
    pub fn record_latency(&self, path: &str, latency: std::time::Duration) {
        debug!("Metrics: {} latency: {:?}", path, latency);
    }
}

/// Create gateway router with custom configuration
/// Note: In this simplified version, config is not fully used
/// For advanced configuration, extend this function
pub fn create_gateway_with_config(
    pool: sqlx::postgres::PgPool,
    _config: GatewayConfig,
) -> Router {
    create_gateway_router(pool)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_request_id() {
        let id1 = generate_request_id();
        let id2 = generate_request_id();
        assert!(!id1.is_empty());
        assert!(id1 != id2);
    }

    #[test]
    fn test_gateway_config_default() {
        let config = GatewayConfig::default();
        assert!(config.enable_logging);
        assert!(!config.enable_metrics);
        assert_eq!(config.service_name, "tardigrade-git-gateway");
    }

    #[tokio::test]
    async fn test_gateway_health_check() {
        let response = gateway_health_check().await.into_response();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_gateway_api_info() {
        let response = gateway_api_info().await.into_response();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[test]
    fn test_error_handler() {
        let error = GitError::RepositoryNotFound;
        let response = handle_git_error(error);
        let (status, json): (StatusCode, axum::Json<ErrorResponse>) = response.into_response();
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert_eq!(json.error, "not_found");
    }
}
