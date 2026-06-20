//! gRPC server binary for Tardigrade Git module
//!
//! This binary starts a gRPC server for the Git service.
//! Run with: cargo run --bin tardigrade-git-grpc --features grpc

use tracing::info;

use tardigrade_git::grpc::server::{start_grpc_server, grpc_config_from_env};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    info!("Starting Tardigrade Git gRPC Server");

    // Load configuration from environment
    let grpc_config = grpc_config_from_env();

    info!("Configuration: {:?}", grpc_config);

    // Start the gRPC server
    start_grpc_server(grpc_config.addr, &grpc_config.database).await?;

    Ok(())
}
