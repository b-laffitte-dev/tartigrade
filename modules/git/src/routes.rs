//! Définition des routes pour le module Git

use axum::{routing::*, Router};

use crate::{
    db::{create_pool, init_schema},
    handler::*,
};

use tardigrade_common::ModuleConfig;

/// Crée le router principal pour le module Git avec une configuration
pub async fn create_router_with_config(
    config: &ModuleConfig,
) -> Result<Router, Box<dyn std::error::Error>> {
    // Créer la pool de connexions
    // TODO: réactiver le timeout une fois que le problème de connexion est résolu
    let pool = create_pool(&config.database_url).await?;

    // Initialiser le schéma
    init_schema(&pool).await?;

    let state = AppState::new(pool);

    // Créer le router
    let api_router = Router::new()
        .route("/health", get(health_check))
        .route("/", get(api_info))
        // Routes pour les repositories
        .route("/repositories", post(create_repository))
        .route("/repositories", get(list_repositories))
        .route("/repositories/:id", get(get_repository))
        .route("/repositories/:id", put(update_repository))
        .route("/repositories/:id", delete(delete_repository))
        // Routes pour les branches
        .route("/repositories/:repository_id/branches", post(create_branch))
        .route("/repositories/:repository_id/branches", get(list_branches))
        .route("/branches/:id", get(get_branch))
        .route(
            "/repositories/:repository_id/branches/:id",
            delete(delete_branch),
        )
        .with_state(state);

    Ok(Router::new().nest("/api", api_router))
}

/// Crée un router simple pour les tests (sans base de données)
pub fn create_test_router() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/", get(api_info))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_test_router() {
        // Valide que `create_test_router` se construit sans paniquer.
        let _router = create_test_router();
    }
}
