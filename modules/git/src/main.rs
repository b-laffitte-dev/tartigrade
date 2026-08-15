//! Point d'entrée du module Git Tardigrade-CI
//!
//! Lance le serveur HTTP Axum et configure le logging.

use std::net::SocketAddr;
use tardigrade_git::{config::GitConfig, routes::create_router_with_config};
use tokio::net::TcpListener;
use tracing_subscriber::{fmt, EnvFilter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Charger la configuration depuis le fichier puis les variables d'environnement.
    // `GitConfig::load` ajoute déjà la source `Environment`; en cas d'absence du
    // fichier de configuration, on retombe sur les valeurs par défaut.
    let git_config =
        GitConfig::load("modules/git/config.toml").unwrap_or_else(|_| GitConfig::default());
    let module_config = &git_config.base.base;

    tracing::info!(
        name = %tardigrade_git::NAME,
        version = %tardigrade_git::VERSION,
        env = %module_config.environment,
        port = %module_config.port,
        database_url = %module_config.database_url,
        database_url_with_timeout = %module_config.database_url_with_timeout(),
        "Démarrage du module Git avec configuration"
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
    #[test]
    fn test_version_constant() {
        assert!(!tardigrade_git::VERSION.is_empty());
    }

    #[test]
    fn test_name_constant() {
        assert_eq!(tardigrade_git::NAME, "tardigrade-git");
    }
}
