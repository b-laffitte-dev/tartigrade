//! Module de configuration commune

use ::config::{Config, Environment, File, FileFormat};
use serde::Deserialize;
use std::path::Path;

/// Configuration de base pour un module Tardigrade
#[derive(Debug, Clone, Deserialize)]
pub struct ModuleConfig {
    /// Nom du module
    pub name: String,
    /// Environnement (dev, test, prod)
    pub environment: String,
    /// Port du serveur HTTP
    pub port: u16,
    /// URL de la base de données PostgreSQL
    pub database_url: String,
    /// Timeout de connexion à la DB (secondes)
    #[serde(default = "default_db_timeout")]
    pub database_timeout: u64,
    /// Log level (debug, info, warn, error)
    #[serde(default = "default_log_level")]
    pub log_level: String,
    /// Activer le mode debug
    #[serde(default = "default_debug")]
    pub debug: bool,
}

fn default_db_timeout() -> u64 {
    30
}

fn default_log_level() -> String {
    "info".to_string()
}

fn default_debug() -> bool {
    false
}

impl Default for ModuleConfig {
    fn default() -> Self {
        Self {
            name: "tardigrade-module".to_string(),
            environment: "dev".to_string(),
            port: 3000,
            database_url: "postgres://postgres:postgres@localhost:5432/postgres".to_string(),
            database_timeout: 30,
            log_level: "info".to_string(),
            debug: false,
        }
    }
}

impl ModuleConfig {
    /// Charge la configuration depuis un fichier et l'environnement
    pub fn load<P: AsRef<Path>>(config_path: P) -> Result<Self, config::ConfigError> {
        let config_path = config_path.as_ref();
        let mut builder = Config::builder()
            .add_source(File::from(config_path).format(FileFormat::Toml))
            .add_source(Environment::with_prefix("TARDIGRADE"));

        // En mode dev, permettre de charger depuis le répertoire courant
        if std::env::var("RUST_ENV").as_deref() != Ok("prod") {
            builder = builder.add_source(File::with_name("local").required(false));
        }

        let config: Self = builder.build()?.try_deserialize()?;
        Ok(config)
    }

    /// Retourne l'URL de la base de données avec timeout
    pub fn database_url_with_timeout(&self) -> String {
        if self.database_url.contains('?') {
            format!(
                "{}&connect_timeout={}",
                self.database_url, self.database_timeout
            )
        } else {
            format!(
                "{}?connect_timeout={}",
                self.database_url, self.database_timeout
            )
        }
    }
}

/// Configuration spécifique pour le module Git
#[derive(Debug, Clone, Deserialize)]
pub struct GitModuleConfig {
    #[serde(flatten)]
    pub base: ModuleConfig,
    /// Chemin de stockage des repositories (pour plus tard)
    pub storage_path: Option<String>,
}

impl Default for GitModuleConfig {
    fn default() -> Self {
        Self {
            base: ModuleConfig::default(),
            storage_path: None,
        }
    }
}

impl GitModuleConfig {
    pub fn load<P: AsRef<Path>>(config_path: P) -> Result<Self, config::ConfigError> {
        let config_path = config_path.as_ref();
        let mut builder = Config::builder()
            .add_source(File::from(config_path).format(FileFormat::Toml))
            .add_source(Environment::with_prefix("TARDIGRADE_GIT"));

        let config: Self = builder.build()?.try_deserialize()?;
        Ok(config)
    }
}
