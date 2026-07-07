//! Tardigrade Git Module - Main Entry Point with API Gateway
//!
//! This is an alternative main entry point that starts the server with API Gateway functionality.

use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use tracing::info;
use tracing_subscriber::{fmt, EnvFilter};

use tardigrade_git::{
    config::{create_pool_from_env, load_config_from_env, load_gateway_config_from_env, DatabaseConfig, GatewayConfig},
    create_app_state,
    gateway::start_gateway_server,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    init_tracing();

    info!("Starting Tardigrade Git Module with API Gateway...");

    // Load configuration
    let config = load_config_from_env();
    let gateway_config = load_gateway_config_from_env();
    
    info!(
        "Configuration loaded: host={}, port={}",
        config.server.host, config.server.port
    );
    info!(
        "Gateway configuration: host={}, port={}",
        gateway_config.host, gateway_config.port
    );

    // Create database connection pool
    let pool = create_pool_from_env().await?;
    info!("Database connection pool created");

    // Create application state
    let _app_state = create_app_state(pool.clone());

    // Start API Gateway server
    let gateway_addr = SocketAddr::from((
        IpAddr::from_str(&gateway_config.host).unwrap_or_else(|_| IpAddr::from_str("0.0.0.0").unwrap()),
        gateway_config.port,
    ));

    info!("API Gateway server listening on {}", gateway_addr);
    info!("Available endpoints:");
    info!("  POST /api/v1/repositories - Create a new repository");
    info!("  GET /api/v1/repositories - List repositories");
    info!("  GET /api/v1/repositories/:id - Get repository by ID");
    info!("  PUT /api/v1/repositories/:id - Update repository");
    info!("  DELETE /api/v1/repositories/:id - Delete repository");
    info!("  POST /api/v1/repositories/:id/branches - Create a branch");
    info!("  GET /api/v1/repositories/:id/branches - List branches");
    info!("  GET /api/v1/branches/:id - Get branch by ID");
    info!("  PUT /api/v1/branches/:id - Update branch");
    info!("  DELETE /api/v1/branches/:id - Delete branch");
    info!("  POST /api/v1/repositories/:id/commits - Create a commit");
    info!("  GET /api/v1/repositories/:id/commits - List commits");
    info!("  GET /api/v1/commits/:id - Get commit by ID");
    info!("  GET /health - Health check");
    info!("  GET /api/info - API information");

    // Start the gateway server
    start_gateway_server(pool, gateway_config).await?;

    Ok(())
}

/// Initialize tracing subscriber
fn init_tracing() {
    let filter = EnvFilter::from_default_env()
        .add_directive("tardigrade_git=debug".parse().unwrap())
        .add_directive("sqlx=info".parse().unwrap())
        .add_directive("axum=info".parse().unwrap());

    fmt()
        .with_env_filter(filter)
        .with_target(true)
        .with_level(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();
}
