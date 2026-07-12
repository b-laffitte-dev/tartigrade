//! Configuration du module Git

use serde::Deserialize;
use std::path::Path;
use tardigrade_common::GitModuleConfig;

/// Configuration spécifique au module Git
#[derive(Debug, Clone, Deserialize)]
pub struct GitConfig {
    /// Configuration de base
    #[serde(flatten)]
    pub base: GitModuleConfig,
    /// Port spécifique pour le module Git (écrase base.port si présent)
    pub git_port: Option<u16>,
    /// Activer les hooks Git (pour plus tard)
    #[serde(default = "default_enable_hooks")]
    pub enable_hooks: bool,
}

fn default_enable_hooks() -> bool {
    false
}

impl Default for GitConfig {
    fn default() -> Self {
        Self {
            base: GitModuleConfig::default(),
            git_port: None,
            enable_hooks: false,
        }
    }
}

impl GitConfig {
    /// Charge la configuration depuis un fichier
    pub fn load<P: AsRef<Path>>(config_path: P) -> Result<Self, config::ConfigError> {
        let config_path = config_path.as_ref();
        let mut builder = config::Config::builder()
            .add_source(config::File::from(config_path).format(config::FileFormat::Toml));

        // Ajouter les variables d'environnement avec préfixe
        builder = builder.add_source(
            config::Environment::with_prefix("TARDIGRADE_GIT")
                .prefix_separator("_")
                .separator("__"),
        );

        // En mode dev, permettre de charger depuis le répertoire courant
        if std::env::var("RUST_ENV").as_deref() != Ok("prod") {
            builder = builder.add_source(
                config::File::with_name("modules/git/local").required(false),
            );
        }

        let config: Self = builder.build()?.try_deserialize()?;
        Ok(config)
    }

    /// Retourne le port à utiliser
    pub fn port(&self) -> u16 {
        self.git_port.unwrap_or(self.base.base.port)
    }
}

/// Configuration par défaut pour le développement local
pub fn default_dev_config() -> GitConfig {
    GitConfig {
        base: GitModuleConfig {
            base: tardigrade_common::ModuleConfig {
                name: "tardigrade-git".to_string(),
                environment: "dev".to_string(),
                port: 3001,
                database_url: "postgres://postgres:postgres@localhost:5432/tardigrade_git".to_string(),
                database_timeout: 30,
                log_level: "debug".to_string(),
                debug: true,
            },
            storage_path: Some("./data/git".to_string()),
        },
        git_port: None,
        enable_hooks: false,
    }
}
