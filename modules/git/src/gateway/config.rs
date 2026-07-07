//! API Gateway configuration for Tardigrade Git
//!
//! This module handles configuration for the API Gateway.

use serde::Deserialize;
use std::collections::HashMap;

/// API Gateway configuration
#[derive(Debug, Clone, Deserialize, Default)]
pub struct GatewayConfig {
    /// Port to listen on
    #[serde(default = "default_port")]
    pub port: u16,
    /// Host to bind to
    #[serde(default = "default_host")]
    pub host: String,
    /// Enable CORS
    #[serde(default = "default_enable_cors")]
    pub enable_cors: bool,
    /// Rate limiting configuration
    #[serde(default)]
    pub rate_limit: RateLimitConfig,
    /// Authentication configuration
    #[serde(default)]
    pub auth: AuthConfig,
    /// Service endpoints
    #[serde(default)]
    pub services: HashMap<String, ServiceConfig>,
    /// Logging configuration
    #[serde(default)]
    pub logging: LoggingConfig,
}

fn default_port() -> u16 {
    8080
}

fn default_host() -> String {
    "0.0.0.0".to_string()
}

fn default_enable_cors() -> bool {
    true
}

/// Rate limiting configuration
#[derive(Debug, Clone, Deserialize, Default)]
pub struct RateLimitConfig {
    /// Enable rate limiting
    #[serde(default = "default_rate_limit_enabled")]
    pub enabled: bool,
    /// Requests per second limit
    #[serde(default = "default_rate_limit_rps")]
    pub requests_per_second: u32,
    /// Burst size
    #[serde(default = "default_rate_limit_burst")]
    pub burst_size: u32,
}

fn default_rate_limit_enabled() -> bool {
    false
}

fn default_rate_limit_rps() -> u32 {
    100
}

fn default_rate_limit_burst() -> u32 {
    50
}

/// Authentication configuration
#[derive(Debug, Clone, Deserialize, Default)]
pub struct AuthConfig {
    /// Enable authentication
    #[serde(default = "default_auth_enabled")]
    pub enabled: bool,
    /// JWT secret key
    #[serde(default)]
    pub jwt_secret: Option<String>,
    /// JWT issuer
    #[serde(default = "default_jwt_issuer")]
    pub jwt_issuer: String,
    /// JWT expiration in hours
    #[serde(default = "default_jwt_expiration")]
    pub jwt_expiration_hours: i64,
}

fn default_auth_enabled() -> bool {
    false
}

fn default_jwt_issuer() -> String {
    "tardigrade-ci".to_string()
}

fn default_jwt_expiration() -> i64 {
    24
}

/// Service configuration
#[derive(Debug, Clone, Deserialize, Default)]
pub struct ServiceConfig {
    /// Service URL
    pub url: String,
    /// Service name
    pub name: String,
    /// Service version
    #[serde(default = "default_service_version")]
    pub version: String,
    /// Service timeout in seconds
    #[serde(default = "default_service_timeout")]
    pub timeout_seconds: u64,
    /// Service health check endpoint
    #[serde(default = "default_health_check")]
    pub health_check: String,
}

fn default_service_version() -> String {
    "1.0.0".to_string()
}

fn default_service_timeout() -> u64 {
    30
}

fn default_health_check() -> String {
    "/health".to_string()
}

/// Logging configuration
#[derive(Debug, Clone, Deserialize, Default)]
pub struct LoggingConfig {
    /// Log level
    #[serde(default = "default_log_level")]
    pub level: String,
    /// Log format (json, text)
    #[serde(default = "default_log_format")]
    pub format: String,
    /// Enable request logging
    #[serde(default = "default_log_requests")]
    pub log_requests: bool,
}

fn default_log_level() -> String {
    "info".to_string()
}

fn default_log_format() -> String {
    "json".to_string()
}

fn default_log_requests() -> bool {
    true
}

/// Load gateway configuration from environment variables
pub fn load_gateway_config_from_env() -> GatewayConfig {
    let port = std::env::var("GATEWAY_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap_or(8080);

    let host = std::env::var("GATEWAY_HOST")
        .unwrap_or_else(|_| "0.0.0.0".to_string());

    let enable_cors = std::env::var("GATEWAY_ENABLE_CORS")
        .unwrap_or_else(|_| "true".to_string())
        .parse::<bool>()
        .unwrap_or(true);

    let jwt_secret = std::env::var("JWT_SECRET").ok();
    let jwt_issuer = std::env::var("JWT_ISSUER")
        .unwrap_or_else(|_| "tardigrade-ci".to_string());
    let jwt_expiration_hours = std::env::var("JWT_EXPIRATION_HOURS")
        .unwrap_or_else(|_| "24".to_string())
        .parse::<i64>()
        .unwrap_or(24);

    GatewayConfig {
        port,
        host,
        enable_cors,
        rate_limit: RateLimitConfig {
            enabled: false, // Disabled by default
            requests_per_second: 100,
            burst_size: 50,
        },
        auth: AuthConfig {
            enabled: jwt_secret.is_some(),
            jwt_secret,
            jwt_issuer,
            jwt_expiration_hours,
        },
        services: HashMap::new(),
        logging: LoggingConfig {
            level: "info".to_string(),
            format: "json".to_string(),
            log_requests: true,
        },
    }
}

/// Create a default gateway configuration
pub fn default_gateway_config() -> GatewayConfig {
    GatewayConfig {
        port: 8080,
        host: "0.0.0.0".to_string(),
        enable_cors: true,
        rate_limit: RateLimitConfig::default(),
        auth: AuthConfig::default(),
        services: HashMap::new(),
        logging: LoggingConfig::default(),
    }
}
