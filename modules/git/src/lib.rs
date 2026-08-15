//! # Tardigrade Git Module
//!
//! Module de gestion des repositories Git pour Tardigrade-CI.
//! Fournit une API REST pour créer, lire, mettre à jour et supprimer des repositories.

pub mod config;
pub mod db;
pub mod error;
pub mod handler;
pub mod models;
pub mod routes;
pub mod service;

// Re-export pour faciliter l'utilisation
pub use config::*;
pub use db::*;
pub use error::*;
pub use handler::*;
pub use models::*;
pub use routes::*;
pub use service::*;

/// Version du module
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Nom du module
pub const NAME: &str = env!("CARGO_PKG_NAME");
