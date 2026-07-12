//! # Tardigrade Common
//!
//! Bibliothèque commune pour tous les modules Tardigrade-CI.
//! Contient les types, erreurs et utilitaires partagés.

pub mod config;
pub mod error;
pub mod models;

// Re-export pour faciliter l'utilisation
pub use config::*;
pub use error::*;
pub use models::*;
