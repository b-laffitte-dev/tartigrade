//! Helpers partagés pour les tests d'intégration du module Git.
//!
//! Ces tests nécessitent une instance PostgreSQL accessible. L'URL de
//! connexion est lue depuis la variable d'environnement `TEST_DATABASE_URL`
//! (par défaut `postgres://postgres:postgres@localhost:5432/tardigrade_git_test`).
//! Une base de test dédiée est créée puis ses tables sont initialisées via
//! les migrations SQLx avant chaque suite de tests.

use tardigrade_git::db::{create_pool, DbPool};

/// URL de la base de test.
pub fn test_database_url() -> String {
    std::env::var("TEST_DATABASE_URL").unwrap_or_else(|_| {
        "postgres://postgres:postgres@localhost:5432/tardigrade_git_test".to_string()
    })
}

/// Crée une pool de connexions vers la base de test et exécute les migrations.
///
/// La base doit déjà exister (créée en amont, par exemple par le service
/// PostgreSQL du CI ou par le script de setup local).
pub async fn setup_pool() -> DbPool {
    let url = test_database_url();
    let pool = create_pool(&url)
        .await
        .expect("Connexion à la base de test impossible — PostgreSQL est-il démarré ?");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Échec d'exécution des migrations sur la base de test");
    pool
}

/// Vide toutes les tables de la base de test (ordre des dépendances respecté).
pub async fn clear_tables(pool: &DbPool) {
    sqlx::query("TRUNCATE TABLE commits, branches, repositories CASCADE")
        .execute(pool)
        .await
        .expect("TRUNCATE échoué sur la base de test");
}
