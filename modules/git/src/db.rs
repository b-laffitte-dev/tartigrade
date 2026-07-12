//! Module de connexion à la base de données

use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;

/// Pool de connexions PostgreSQL
pub type DbPool = PgPool;

/// Crée une nouvelle pool de connexions à PostgreSQL
pub async fn create_pool(database_url: &str) -> Result<DbPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(30))
        .connect(database_url)
        .await?;

    // Tester la connexion
    sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(&pool)
        .await?;

    Ok(pool)
}

/// Vérifie que la connexion à la base de données fonctionne
pub async fn check_connection(pool: &DbPool) -> Result<(), sqlx::Error> {
    sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(pool)
        .await
        .map(|_| ())
}

/// Exécute une migration SQL
pub async fn run_migration(pool: &DbPool, sql: &str) -> Result<(), sqlx::Error> {
    sqlx::query(sql).execute(pool).await.map(|_| ())
}

/// Initialise le schéma de la base de données
pub async fn init_schema(pool: &DbPool) -> Result<(), sqlx::Error> {
    // Créer la table repositories
    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS repositories (
            id UUID PRIMARY KEY,
            name VARCHAR(100) NOT NULL UNIQUE,
            description TEXT,
            is_private BOOLEAN NOT NULL DEFAULT FALSE,
            owner_id UUID,
            default_branch VARCHAR(255) NOT NULL DEFAULT 'main',
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )"#,
    )
    .execute(pool)
    .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_repositories_name ON repositories(name)")
        .execute(pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_repositories_owner_id ON repositories(owner_id)")
        .execute(pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_repositories_created_at ON repositories(created_at)")
        .execute(pool)
        .await?;

    // Créer la table branches
    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS branches (
            id UUID PRIMARY KEY,
            repository_id UUID NOT NULL REFERENCES repositories(id) ON DELETE CASCADE,
            name VARCHAR(255) NOT NULL,
            commit_hash VARCHAR(64),
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )"#,
    )
    .execute(pool)
    .await?;

    sqlx::query("CREATE UNIQUE INDEX IF NOT EXISTS idx_branches_repo_name ON branches(repository_id, name)")
        .execute(pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_branches_repository_id ON branches(repository_id)")
        .execute(pool)
        .await?;

    // Créer la table commits (simplifiée pour le MVP)
    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS commits (
            hash VARCHAR(64) PRIMARY KEY,
            repository_id UUID NOT NULL REFERENCES repositories(id) ON DELETE CASCADE,
            message TEXT NOT NULL,
            author_name VARCHAR(255),
            author_email VARCHAR(255),
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )"#,
    )
    .execute(pool)
    .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_commits_repository_id ON commits(repository_id)")
        .execute(pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_commits_created_at ON commits(created_at)")
        .execute(pool)
        .await?;

    Ok(())
}

/// Supprime toutes les données (UTILITAIRE DE DEV SEULEMENT)
pub async fn clear_database(pool: &DbPool) -> Result<(), sqlx::Error> {
    let tables = vec!["commits", "branches", "repositories"];
    
    for table in tables {
        sqlx::query(&format!("TRUNCATE TABLE {} CASCADE", table))
            .execute(pool)
            .await?;
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    #[ignore = "Requiert une base de données PostgreSQL en cours d'exécution"]
    async fn test_create_pool() {
        let database_url = env::var("TEST_DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/test".to_string());
        
        let pool = create_pool(&database_url).await;
        assert!(pool.is_ok());
    }

    #[tokio::test]
    #[ignore = "Requiert une base de données PostgreSQL en cours d'exécution"]
    async fn test_check_connection() {
        let database_url = env::var("TEST_DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/test".to_string());
        
        let pool = create_pool(&database_url).await.unwrap();
        let result = check_connection(&pool).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore = "Requiert une base de données PostgreSQL en cours d'exécution"]
    async fn test_init_schema() {
        let database_url = env::var("TEST_DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/test".to_string());
        
        let pool = create_pool(&database_url).await.unwrap();
        let result = init_schema(&pool).await;
        assert!(result.is_ok());
    }
}
