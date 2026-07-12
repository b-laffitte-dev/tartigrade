//! Point d'entrée du module Git Tardigrade-CI
//!
//! Lance le serveur HTTP Axum et configure le logging.

use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tardigrade_git::{config::GitConfig, routes::create_router_with_config};
use tardigrade_common::ModuleConfig;
use tracing_subscriber::{fmt, EnvFilter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Charger la configuration par défaut pour le développement
    let git_config = GitConfig::default();
    let module_config = &git_config.base.base;

    tracing::info!(
        name = %tardigrade_git::NAME,
        version = %tardigrade_git::VERSION,
        env = %module_config.environment,
        port = %module_config.port,
        database_url = %module_config.database_url,
        "Démarrage du module Git avec configuration par défaut"
    );

    // Configurer le logging
    setup_logging(&module_config.log_level);

    // Créer le router avec la configuration
    let app = create_router_with_config(module_config).await?;

    // Créer l'adresse du serveur
    let addr = SocketAddr::from(([0, 0, 0, 0], module_config.port));

    tracing::info!(
        address = %addr,
        "Serveur en écoute"
    );

    // Lancer le serveur
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Configure le logging avec le niveau spécifié
fn setup_logging(log_level: &str) {
    let filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(log_level))
        .unwrap_or_else(|_| EnvFilter::new("info"));

    fmt()
        .with_env_filter(filter)
        .with_target(false)
        .with_line_number(true)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_file(false)
        .init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_constant() {
        assert!(!tardigrade_git::VERSION.is_empty());
    }

    #[test]
    fn test_name_constant() {
        assert_eq!(tardigrade_git::NAME, "tardigrade-git");
    }
}
