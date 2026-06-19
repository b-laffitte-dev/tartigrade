//! Configuration module for Tardigrade Git
//!
//! This module handles database configuration and connection pooling.

use serde::Deserialize;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;

/// Database configuration
#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    /// PostgreSQL connection URL
    pub url: String,
    /// Maximum number of connections in the pool
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
    /// Timeout for acquiring a connection from the pool
    #[serde(default = "default_acquire_timeout")]
    pub acquire_timeout_seconds: u64,
    /// Timeout for connection idle time before being closed
    #[serde(default = "default_idle_timeout")]
    pub idle_timeout_seconds: u64,
    /// Maximum lifetime of a connection
    #[serde(default = "default_max_lifetime")]
    pub max_lifetime_seconds: Option<u64>,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "postgres://user:password@localhost:5432/tardigrade".to_string(),
            max_connections: default_max_connections(),
            acquire_timeout_seconds: default_acquire_timeout(),
            idle_timeout_seconds: default_idle_timeout(),
            max_lifetime_seconds: default_max_lifetime(),
        }
    }
}

fn default_max_connections() -> u32 {
    20
}

fn default_acquire_timeout() -> u64 {
    30
}

fn default_idle_timeout() -> u64 {
    300
}

fn default_max_lifetime() -> Option<u64> {
    Some(3600) // 1 hour
}

/// Application configuration
#[derive(Debug, Clone, Deserialize, Default)]
pub struct AppConfig {
    /// Database configuration
    pub database: DatabaseConfig,
    /// HTTP server configuration
    pub server: ServerConfig,
}

/// HTTP server configuration
#[derive(Debug, Clone, Deserialize, Default)]
pub struct ServerConfig {
    /// Host to bind to
    #[serde(default = "default_host")]
    pub host: String,
    /// Port to bind to
    #[serde(default = "default_port")]
    pub port: u16,
    /// Enable CORS
    #[serde(default = "default_enable_cors")]
    pub enable_cors: bool,
    /// Log level
    #[serde(default = "default_log_level")]
    pub log_level: String,
}

fn default_host() -> String {
    "0.0.0.0".to_string()
}

fn default_port() -> u16 {
    3001
}

fn default_enable_cors() -> bool {
    true
}

fn default_log_level() -> String {
    "info".to_string()
}

/// Create a new database connection pool
pub async fn create_pool(config: &DatabaseConfig) -> Result<PgPool, sqlx::Error> {
    let pool_options = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .acquire_timeout(Duration::from_secs(config.acquire_timeout_seconds))
        .idle_timeout(Some(Duration::from_secs(config.idle_timeout_seconds)));

    if let Some(max_lifetime) = config.max_lifetime_seconds {
        pool_options.max_lifetime(Some(Duration::from_secs(max_lifetime)));
    }

    PgPool::connect(&config.url).await
}

/// Create a connection pool from environment variables
pub async fn create_pool_from_env() -> Result<PgPool, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://user:password@localhost:5432/tardigrade".to_string());

    let config = DatabaseConfig {
        url: database_url,
        ..Default::default()
    };

    create_pool(&config).await
}

/// Load configuration from environment variables
pub fn load_config_from_env() -> AppConfig {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://user:password@localhost:5432/tardigrade".to_string());

    let host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("SERVER_PORT")
        .unwrap_or_else(|_| "3001".to_string())
        .parse::<u16>()
        .unwrap_or(3001);

    AppConfig {
        database: DatabaseConfig {
            url: database_url,
            ..Default::default()
        },
        server: ServerConfig {
            host,
            port,
            ..Default::default()
        },
    }
}
