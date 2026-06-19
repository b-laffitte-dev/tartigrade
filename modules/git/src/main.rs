//! Tardigrade Git Module - Main Entry Point
//!
//! This is the main entry point for the Git module server.

use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use tracing::info;
use tracing_subscriber::{fmt, EnvFilter};

use tardigrade_git::{
    config::{create_pool_from_env, load_config_from_env, DatabaseConfig},
    create_app_state,
    routes::create_router,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    init_tracing();

    info!("Starting Tardigrade Git Module...");

    // Load configuration
    let config = load_config_from_env();
    info!(
        "Configuration loaded: host={}, port={}",
        config.server.host, config.server.port
    );

    // Create database connection pool
    let pool = create_pool_from_env().await?;
    info!("Database connection pool created");

    // Create application state
    let app_state = create_app_state(pool);

    // Create router
    let app = create_router(app_state.pool.clone());

    // Parse host and port
    let host: IpAddr = config
        .server
        .host
        .parse()
        .unwrap_or_else(|_| IpAddr::from_str("0.0.0.0").unwrap());
    let port = config.server.port;
    let addr = SocketAddr::from((host, port));

    info!("Server listening on {}", addr);
    info!("Available endpoints:");
    info!("  POST /repositories - Create a new repository");
    info!("  GET /repositories - List repositories");
    info!("  GET /repositories/:id - Get repository by ID");
    info!("  PUT /repositories/:id - Update repository");
    info!("  DELETE /repositories/:id - Delete repository");
    info!("  GET /health - Health check");
    info!("  GET /api/info - API information");

    // Start server
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

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

/// Check database connection
pub async fn check_db_connection(_config: &DatabaseConfig) -> Result<(), sqlx::Error> {
    let pool = create_pool_from_env().await?;

    // Simple query to test connection
    let _: i64 = sqlx::query_scalar("SELECT 1").fetch_one(&pool).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_server_config() {
        // For now, just test that the configuration loading works
        let config = load_config_from_env();
        assert_eq!(config.server.port, 3001);
        assert_eq!(config.server.host, "0.0.0.0");
    }
}
