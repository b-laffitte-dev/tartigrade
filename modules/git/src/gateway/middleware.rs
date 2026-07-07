//! Middleware for API Gateway
//!
//! This module provides middleware components for the API Gateway.

use axum::extract::Request;
use axum::http::{HeaderValue, Method, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use axum::RequestExt;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing::{info, warn};

use super::config::GatewayConfig;

/// Create CORS middleware based on configuration
pub fn create_cors_middleware(config: &GatewayConfig) -> CorsLayer {
    if config.enable_cors {
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods([
                Method::GET,
                Method::POST,
                Method::PUT,
                Method::DELETE,
                Method::OPTIONS,
                Method::PATCH,
                Method::HEAD,
            ])
            .allow_headers(Any)
            .allow_credentials(true)
    } else {
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods([Method::GET])
    }
}

/// Request logging middleware
pub async fn request_logger_middleware<B>(
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let headers = request.headers().clone();

    // Extract client IP if available
    let client_ip = headers
        .get("X-Forwarded-For")
        .or_else(|| headers.get("X-Real-IP"))
        .map(|v| v.to_str().unwrap_or("unknown").to_string())
        .unwrap_or_else(|| "unknown".to_string());

    info!(
        "Request: {} {} from {}",
        method,
        uri,
        client_ip
    );

    let response = next.run(request).await;

    let status = response.status();
    info!(
        "Response: {} {} -> {}",
        method,
        uri,
        status
    );

    Ok(response)
}

/// Authentication middleware
pub async fn auth_middleware<B>(
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    // Check for Authorization header
    let auth_header = request.headers().get("Authorization");
    
    if let Some(header) = auth_header {
        let token = header.to_str().map_err(|_| StatusCode::BAD_REQUEST)?;
        
        // In a real implementation, validate the JWT token here
        // For now, just log and pass through
        info!("Authorization token present");
        
        // Add user context to request extensions
        // This would be used by handlers to get the authenticated user
    } else {
        warn!("No Authorization header - request may be unauthenticated");
    }

    Ok(next.run(request).await)
}

/// Rate limiting middleware
pub async fn rate_limit_middleware<B>(
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    // In a real implementation, this would track request rates
    // and return 429 Too Many Requests if the limit is exceeded
    
    // For now, just pass through
    Ok(next.run(request).await)
}

/// Error handling middleware
pub async fn error_handler_middleware<B>(
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let response = next.run(request).await;
    
    // Log errors
    if response.status().is_server_error() {
        warn!(
            "Server error: {}",
            response.status()
        );
    } else if response.status().is_client_error() {
        info!(
            "Client error: {}",
            response.status()
        );
    }

    Ok(response)
}

/// Create a combined middleware stack
pub fn create_middleware_stack(config: &GatewayConfig) -> Vec<axum::middleware::Middleware<fn(_, _) -> _>> {
    let mut middlewares = Vec::new();

    // Add CORS middleware
    middlewares.push(axum::middleware::from_fn(request_logger_middleware));

    // Add authentication middleware if enabled
    if config.auth.enabled {
        middlewares.push(axum::middleware::from_fn(auth_middleware));
    }

    // Add rate limiting middleware if enabled
    if config.rate_limit.enabled {
        middlewares.push(axum::middleware::from_fn(rate_limit_middleware));
    }

    // Add error handling middleware
    middlewares.push(axum::middleware::from_fn(error_handler_middleware));

    middlewares
}

/// Health check middleware
pub async fn health_check_middleware<B>(
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    // Check if this is a health check request
    if request.uri().path() == "/health" {
        return Ok(Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(axum::body::Body::from(
                serde_json::json!({
                    "status": "healthy",
                    "module": "git-gateway",
                    "version": "0.1.0",
                    "timestamp": chrono::Utc::now().to_rfc3339()
                }).to_string()
            ))
            .unwrap());
    }

    Ok(next.run(request).await)
}
