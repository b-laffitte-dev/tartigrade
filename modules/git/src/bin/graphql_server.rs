//! GraphQL server binary for Tardigrade Git module
//!
//! This binary starts a GraphQL server for the Git service.
//! Run with: cargo run --bin tardigrade-git-graphql --features graphql

use tracing::info;

use tardigrade_git::config::create_pool_from_env;
use tardigrade_git::graphql::schema::create_graphql_router;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    info!("Starting Tardigrade Git GraphQL Server");

    // Create database connection pool
    let pool = create_pool_from_env().await?;

    info!("Database connection established");

    // Create GraphQL router
    let app = create_graphql_router(pool);

    // Determine port from environment or use default
    let port = std::env::var("GRAPHQL_PORT")
        .unwrap_or_else(|_| "4000".to_string())
        .parse::<u16>()
        .unwrap_or(4000);

    let addr = format!("0.0.0.0:{}", port);

    info!("GraphQL server listening on {}", addr);
    info!("GraphQL Playground available at http://localhost:{}/graphql", port);

    // Start the server
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
