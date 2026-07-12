//! Module de service pour la logique métier

use crate::{error::{GitError, GitResult}, models::*, DbPool};
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use tardigrade_common::models::PaginatedResponse;
use uuid::Uuid;

/// Service pour gérer les repositories
#[derive(Debug, Clone)]
pub struct RepositoryService {
    pool: DbPool,
}

impl RepositoryService {
    /// Crée un nouveau RepositoryService
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// Crée un nouveau repository
    pub async fn create_repository(&self, input: CreateRepositoryInput) -> GitResult<Repository> {
        // Valider l'entrée
        input.validate().map_err(GitError::validation)?;

        // Convertir en Repository
        let repo = input.into_repository();

        // Insérer dans la base de données
        let row = sqlx::query_as::<_, RepositoryRow>(
            r#"
            INSERT INTO repositories (id, name, description, is_private, default_branch, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, name, description, is_private, owner_id, default_branch, created_at, updated_at
            "#,
        )
        .bind(repo.id)
        .bind(&repo.name)
        .bind(repo.description)
        .bind(repo.is_private)
        .bind(&repo.default_branch)
        .bind(repo.created_at)
        .bind(repo.updated_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.into())
    }

    /// Récupère un repository par ID
    pub async fn get_repository(&self, id: Uuid) -> GitResult<Option<Repository>> {
        let row = sqlx::query_as::<_, RepositoryRow>(
            r#"
            SELECT id, name, description, is_private, owner_id, default_branch, created_at, updated_at
            FROM repositories
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| r.into()))
    }

    /// Récupère un repository par nom
    pub async fn get_repository_by_name(&self, name: &str) -> GitResult<Option<Repository>> {
        let row = sqlx::query_as::<_, RepositoryRow>(
            r#"
            SELECT id, name, description, is_private, owner_id, default_branch, created_at, updated_at
            FROM repositories
            WHERE name = $1
            "#,
        )
        .bind(name)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| r.into()))
    }

    /// Liste les repositories avec pagination
    pub async fn list_repositories(
        &self,
        pagination: PaginationQuery,
    ) -> GitResult<RepositoryListResponse> {
        let offset = pagination.offset();
        let limit = pagination.limit();

        // Compter le total
        let total: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM repositories",
        )
        .fetch_one(&self.pool)
        .await?;

        // Récupérer les données
        let repos = sqlx::query_as::<_, RepositoryRow>(
            r#"
            SELECT id, name, description, is_private, owner_id, default_branch, created_at, updated_at
            FROM repositories
            ORDER BY created_at DESC
            OFFSET $1 LIMIT $2
            "#,
        )
        .bind(offset)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        let data = repos.into_iter().map(|r| r.into()).collect();

        Ok(PaginatedResponse::new(
            data,
            pagination.page,
            pagination.page_size,
            total,
        ))
    }

    /// Met à jour un repository
    pub async fn update_repository(
        &self,
        id: Uuid,
        name: Option<String>,
        description: Option<String>,
        is_private: Option<bool>,
        default_branch: Option<String>,
    ) -> GitResult<Repository> {
        let repo = self.get_repository(id).await?;
        let repo = repo.ok_or_else(|| GitError::repository_not_found(id.to_string()))?;

        let mut updated_repo = repo.clone();

        if let Some(name) = name {
            updated_repo.set_name(name);
        }
        if let Some(desc) = description {
            updated_repo.set_description(Some(desc));
        }
        if let Some(is_private) = is_private {
            updated_repo.set_visibility(is_private);
        }
        if let Some(branch) = default_branch {
            updated_repo.set_default_branch(branch);
        }

        let row = sqlx::query_as::<_, RepositoryRow>(
            r#"
            UPDATE repositories
            SET name = COALESCE($2, name),
                description = COALESCE($3, description),
                is_private = COALESCE($4, is_private),
                default_branch = COALESCE($5, default_branch),
                updated_at = NOW()
            WHERE id = $1
            RETURNING id, name, description, is_private, owner_id, default_branch, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(updated_repo.name)
        .bind(updated_repo.description)
        .bind(updated_repo.is_private)
        .bind(updated_repo.default_branch)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.into())
    }

    /// Supprime un repository
    pub async fn delete_repository(&self, id: Uuid) -> GitResult<()> {
        let exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM repositories WHERE id = $1)",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        if !exists {
            return Err(GitError::repository_not_found(id.to_string()));
        }

        sqlx::query("DELETE FROM repositories WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

/// Service pour gérer les branches
#[derive(Debug, Clone)]
pub struct BranchService {
    pool: DbPool,
}

impl BranchService {
    /// Crée un nouveau BranchService
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// Crée une nouvelle branche
    pub async fn create_branch(
        &self,
        repository_id: Uuid,
        input: CreateBranchInput,
    ) -> GitResult<Branch> {
        // Valider l'entrée
        input.validate().map_err(GitError::validation)?;

        // Vérifier que le repository existe
        let exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM repositories WHERE id = $1)",
        )
        .bind(repository_id)
        .fetch_one(&self.pool)
        .await?;

        if !exists {
            return Err(GitError::repository_not_found(repository_id.to_string()));
        }

        // Vérifier que la branche n'existe pas déjà
        let branch_exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM branches WHERE repository_id = $1 AND name = $2)",
        )
        .bind(repository_id)
        .bind(&input.name)
        .fetch_one(&self.pool)
        .await?;

        if branch_exists {
            return Err(GitError::branch_exists(&input.name, repository_id.to_string()));
        }

        // Convertir en Branch
        let branch = input.into_branch(repository_id);

        // Insérer dans la base de données
        let row = sqlx::query_as::<_, BranchRow>(
            r#"
            INSERT INTO branches (id, repository_id, name, commit_hash, created_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, repository_id, name, commit_hash, created_at
            "#,
        )
        .bind(branch.id)
        .bind(branch.repository_id)
        .bind(&branch.name)
        .bind(branch.commit_hash)
        .bind(branch.created_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.into())
    }

    /// Récupère une branche par ID
    pub async fn get_branch(&self, id: Uuid) -> GitResult<Option<Branch>> {
        let row = sqlx::query_as::<_, BranchRow>(
            r#"
            SELECT id, repository_id, name, commit_hash, created_at
            FROM branches
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| r.into()))
    }

    /// Liste les branches d'un repository avec pagination
    pub async fn list_branches(
        &self,
        repository_id: Uuid,
        pagination: PaginationQuery,
    ) -> GitResult<BranchListResponse> {
        // Vérifier que le repository existe
        let exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM repositories WHERE id = $1)",
        )
        .bind(repository_id)
        .fetch_one(&self.pool)
        .await?;

        if !exists {
            return Err(GitError::repository_not_found(repository_id.to_string()));
        }

        let offset = pagination.offset();
        let limit = pagination.limit();

        // Compter le total
        let total: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM branches WHERE repository_id = $1",
        )
        .bind(repository_id)
        .fetch_one(&self.pool)
        .await?;

        // Récupérer les données
        let branches = sqlx::query_as::<_, BranchRow>(
            r#"
            SELECT id, repository_id, name, commit_hash, created_at
            FROM branches
            WHERE repository_id = $1
            ORDER BY created_at DESC
            OFFSET $2 LIMIT $3
            "#,
        )
        .bind(repository_id)
        .bind(offset)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        let data = branches.into_iter().map(|r| r.into()).collect();

        Ok(PaginatedResponse::new(
            data,
            pagination.page,
            pagination.page_size,
            total,
        ))
    }

    /// Supprime une branche
    pub async fn delete_branch(&self, id: Uuid, repository_id: Uuid) -> GitResult<()> {
        let exists: bool = sqlx::query_scalar(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM branches 
                WHERE id = $1 AND repository_id = $2
            )"#,
        )
        .bind(id)
        .bind(repository_id)
        .fetch_one(&self.pool)
        .await?;

        if !exists {
            return Err(GitError::branch_not_found(id.to_string(), repository_id.to_string()));
        }

        sqlx::query("DELETE FROM branches WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

/// Lignes SQLx pour Repository
#[derive(Debug, FromRow)]
struct RepositoryRow {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub is_private: bool,
    pub owner_id: Option<Uuid>,
    pub default_branch: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<RepositoryRow> for Repository {
    fn from(row: RepositoryRow) -> Self {
        Self {
            id: row.id,
            name: row.name,
            description: row.description,
            is_private: row.is_private,
            owner_id: row.owner_id,
            default_branch: row.default_branch,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

/// Lignes SQLx pour Branch
#[derive(Debug, FromRow)]
struct BranchRow {
    pub id: Uuid,
    pub repository_id: Uuid,
    pub name: String,
    pub commit_hash: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<BranchRow> for Branch {
    fn from(row: BranchRow) -> Self {
        Self {
            id: row.id,
            repository_id: row.repository_id,
            name: row.name,
            commit_hash: row.commit_hash,
            created_at: row.created_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;

    // Tests unitaires pour la validation
    #[test]
    fn test_create_repository_input_validation() {
        let input = CreateRepositoryInput {
            name: "".to_string(),
            description: None,
            is_private: false,
            default_branch: "main".to_string(),
        };

        assert!(input.validate().is_err());
    }

    #[test]
    fn test_create_branch_input_validation() {
        let input = CreateBranchInput {
            name: "".to_string(),
            commit_hash: None,
        };

        assert!(input.validate().is_err());
    }
}
