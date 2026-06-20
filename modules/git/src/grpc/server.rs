//! gRPC server for Tardigrade Git module
//!
//! This module provides the gRPC server configuration and startup.
//!
//! To use:
//! 1. Enable the grpc feature: cargo build --features grpc
//! 2. Run: cargo run --bin tardigrade-git-grpc --features grpc

use std::net::SocketAddr;
use std::sync::Arc;
use tracing::{info, error};

use crate::config::DatabaseConfig;
use crate::grpc::git::GitGrpcService;
use crate::service::GitService;

/// Start the gRPC server
pub async fn start_grpc_server(
    addr: SocketAddr,
    database_config: &DatabaseConfig,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("Starting gRPC server on {}", addr);

    // Create database connection pool
    let pool = crate::config::create_pool(database_config).await?;

    // Create GitService
    let git_service = Arc::new(GitService::new(pool));

    // Create gRPC service
    let _grpc_service = GitGrpcService::new(git_service.clone());

    info!("gRPC service created, ready to accept connections");
    info!("Note: For full gRPC functionality, generate protobuf code from proto/git.proto");
    info!("      Install protoc and run: cargo build --features grpc");

    // For now, the server is created but actual gRPC endpoints need protobuf generation
    // This is a placeholder that will be enhanced when protobuf code is available

    Ok(())
}

/// Configuration for the gRPC server
#[derive(Debug, Clone)]
pub struct GrpcConfig {
    /// Address to bind the gRPC server to
    pub addr: SocketAddr,
    /// Database configuration
    pub database: DatabaseConfig,
}

impl Default for GrpcConfig {
    fn default() -> Self {
        Self {
            addr: "0.0.0.0:50051".parse().unwrap(),
            database: DatabaseConfig::default(),
        }
    }
}

/// Create a gRPC server configuration from environment variables
pub fn grpc_config_from_env() -> GrpcConfig {
    let addr = std::env::var("GRPC_ADDR")
        .unwrap_or_else(|_| "0.0.0.0:50051".to_string())
        .parse()
        .unwrap_or_else(|_| "0.0.0.0:50051".parse().unwrap());

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://user:password@localhost:5432/tardigrade".to_string());

    let database = DatabaseConfig {
        url: database_url,
        ..Default::default()
    };

    GrpcConfig {
        addr,
        database,
    }
}

/// Create a gRPC service from a GitService
pub fn create_grpc_service(git_service: Arc<GitService>) -> GitGrpcService {
    GitGrpcService::new(git_service)
}
