//! Repository operations for Tardigrade Git module
//!
//! This module implements CRUD operations for Git repositories.

use crate::error::GitError;
use crate::models::{
    CreateRepositoryInput, DbRepository, ListRepositoriesQuery, PaginatedResponse, Repository,
    UpdateRepositoryInput,
};
use chrono::{DateTime, Utc};
use sqlx::postgres::PgPool;
use sqlx::{FromRow, Row};
use uuid::Uuid;

// Re-export branch and commit repository modules
pub mod branch;
pub mod commit;

pub use branch::*;
pub use commit::*;

/// Create a new repository
pub async fn create_repository(
    pool: &PgPool,
    input: CreateRepositoryInput,
    owner_id: Uuid,
) -> Result<Repository, GitError> {
    // Validate input
    input.validate().map_err(GitError::ValidationError)?;

    // Check if repository with same name already exists for this owner
    let existing: Option<Uuid> =
        sqlx::query_scalar("SELECT id FROM repositories WHERE name = $1 AND owner_id = $2")
            .bind(&input.name)
            .bind(owner_id)
            .fetch_optional(pool)
            .await?;

    if existing.is_some() {
        return Err(GitError::RepositoryAlreadyExists(input.name.clone()));
    }

    let now = Utc::now();

    let row = sqlx::query(
        r#"
        INSERT INTO repositories (name, description, is_private, owner_id, default_branch, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, name, description, is_private, owner_id, default_branch, created_at, updated_at
        "#
    )
    .bind(&input.name)
    .bind(input.description.as_deref())
    .bind(input.is_private)
    .bind(owner_id)
    .bind(&input.default_branch)
    .bind(now)
    .bind(now)
    .map(|row: sqlx::postgres::PgRow| DbRepository {
        id: row.get("id"),
        name: row.get("name"),
        description: row.get("description"),
        is_private: row.get("is_private"),
        owner_id: row.get("owner_id"),
        default_branch: row.get("default_branch"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
    .fetch_one(pool)
    .await?;

    Ok(Repository::from(row))
}

/// Get a repository by ID
pub async fn get_repository(pool: &PgPool, id: Uuid) -> Result<Option<Repository>, GitError> {
    let row = sqlx::query(
        r#"
        SELECT id, name, description, is_private, owner_id, default_branch, created_at, updated_at
        FROM repositories WHERE id = $1
        "#,
    )
    .bind(id)
    .map(|row: sqlx::postgres::PgRow| DbRepository {
        id: row.get("id"),
        name: row.get("name"),
        description: row.get("description"),
        is_private: row.get("is_private"),
        owner_id: row.get("owner_id"),
        default_branch: row.get("default_branch"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
    .fetch_optional(pool)
    .await?;

    Ok(row.map(Repository::from))
}

/// Get a repository by ID with owner check
pub async fn get_repository_by_id_and_owner(
    pool: &PgPool,
    id: Uuid,
    owner_id: Uuid,
) -> Result<Option<Repository>, GitError> {
    let row = sqlx::query(
        r#"
        SELECT id, name, description, is_private, owner_id, default_branch, created_at, updated_at
        FROM repositories WHERE id = $1 AND owner_id = $2
        "#,
    )
    .bind(id)
    .bind(owner_id)
    .map(|row: sqlx::postgres::PgRow| DbRepository {
        id: row.get("id"),
        name: row.get("name"),
        description: row.get("description"),
        is_private: row.get("is_private"),
        owner_id: row.get("owner_id"),
        default_branch: row.get("default_branch"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
    .fetch_optional(pool)
    .await?;

    Ok(row.map(Repository::from))
}

/// Get a repository by name and owner
pub async fn get_repository_by_name_and_owner(
    pool: &PgPool,
    name: &str,
    owner_id: Uuid,
) -> Result<Option<Repository>, GitError> {
    let row = sqlx::query(
        r#"
        SELECT id, name, description, is_private, owner_id, default_branch, created_at, updated_at
        FROM repositories WHERE name = $1 AND owner_id = $2
        "#,
    )
    .bind(name)
    .bind(owner_id)
    .map(|row: sqlx::postgres::PgRow| DbRepository {
        id: row.get("id"),
        name: row.get("name"),
        description: row.get("description"),
        is_private: row.get("is_private"),
        owner_id: row.get("owner_id"),
        default_branch: row.get("default_branch"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
    .fetch_optional(pool)
    .await?;

    Ok(row.map(Repository::from))
}

/// List repositories with pagination
pub async fn list_repositories(
    pool: &PgPool,
    query: ListRepositoriesQuery,
) -> Result<PaginatedResponse<Repository>, GitError> {
    let offset = query.offset();
    let limit = query.limit();

    // For now, use a simple query - filter by owner only
    let rows = if let Some(owner_id) = query.owner_id {
        sqlx::query(
            r#"
            SELECT id, name, description, is_private, owner_id, default_branch, created_at, updated_at
            FROM repositories
            WHERE owner_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#
        )
        .bind(owner_id)
        .bind(limit)
        .bind(offset)
        .map(|row: sqlx::postgres::PgRow| DbRepository {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            is_private: row.get("is_private"),
            owner_id: row.get("owner_id"),
            default_branch: row.get("default_branch"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query(
            r#"
            SELECT id, name, description, is_private, owner_id, default_branch, created_at, updated_at
            FROM repositories
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#
        )
        .bind(limit)
        .bind(offset)
        .map(|row: sqlx::postgres::PgRow| DbRepository {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            is_private: row.get("is_private"),
            owner_id: row.get("owner_id"),
            default_branch: row.get("default_branch"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
        .fetch_all(pool)
        .await?
    };

    // Count total
    let count: i64 = if let Some(owner_id) = query.owner_id {
        sqlx::query_scalar("SELECT COUNT(*)::BIGINT FROM repositories WHERE owner_id = $1")
            .bind(owner_id)
            .fetch_one(pool)
            .await
            .unwrap_or(0)
    } else {
        sqlx::query_scalar("SELECT COUNT(*)::BIGINT FROM repositories")
            .fetch_one(pool)
            .await
            .unwrap_or(0)
    };

    let repositories: Vec<Repository> = rows.into_iter().map(Repository::from).collect();

    Ok(PaginatedResponse::new(
        repositories,
        query.page,
        query.page_size,
        count,
    ))
}

/// List repositories by owner with pagination
pub async fn list_repositories_by_owner(
    pool: &PgPool,
    owner_id: Uuid,
    page: i32,
    page_size: i32,
) -> Result<PaginatedResponse<Repository>, GitError> {
    let query = ListRepositoriesQuery {
        owner_id: Some(owner_id),
        page,
        page_size,
        search: None,
        is_private: None,
    };

    list_repositories(pool, query).await
}

/// Update a repository
pub async fn update_repository(
    pool: &PgPool,
    id: Uuid,
    owner_id: Uuid,
    input: UpdateRepositoryInput,
) -> Result<Repository, GitError> {
    // First check if repository exists and belongs to the owner
    let existing_repo = get_repository_by_id_and_owner(pool, id, owner_id).await?;
    if existing_repo.is_none() {
        return Err(GitError::RepositoryNotFound);
    }

    let existing = existing_repo.unwrap();

    // Validate new name if provided
    if let Some(ref name) = input.name {
        if name.is_empty() || name.len() > 255 {
            return Err(GitError::ValidationError(
                "Repository name must be between 1 and 255 characters".to_string(),
            ));
        }

        // Check if new name already exists for this owner
        if let Some(existing_with_name) =
            get_repository_by_name_and_owner(pool, name, owner_id).await?
        {
            if existing_with_name.id != id {
                return Err(GitError::RepositoryAlreadyExists(name.clone()));
            }
        }
    }

    // Validate branch name if provided
    if let Some(ref branch) = input.default_branch {
        if branch.is_empty() || branch.len() > 255 {
            return Err(GitError::ValidationError(
                "Default branch name must be between 1 and 255 characters".to_string(),
            ));
        }
    }

    let now = Utc::now();
    let mut repo = existing;

    // Apply updates to the existing repository
    repo.update(&input);

    // Now update in database - update all fields for simplicity
    let row = sqlx::query(
        r#"
        UPDATE repositories
        SET name = $1, description = $2, is_private = $3, default_branch = $4, updated_at = $5
        WHERE id = $6 AND owner_id = $7
        RETURNING id, name, description, is_private, owner_id, default_branch, created_at, updated_at
        "#
    )
    .bind(&repo.name)
    .bind(repo.description.as_deref())
    .bind(repo.is_private)
    .bind(&repo.default_branch)
    .bind(now)
    .bind(id)
    .bind(owner_id)
    .map(|row: sqlx::postgres::PgRow| DbRepository {
        id: row.get("id"),
        name: row.get("name"),
        description: row.get("description"),
        is_private: row.get("is_private"),
        owner_id: row.get("owner_id"),
        default_branch: row.get("default_branch"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
    .fetch_optional(pool)
    .await?;

    match row {
        Some(db_repo) => Ok(Repository::from(db_repo)),
        None => Err(GitError::RepositoryNotFound),
    }
}

/// Delete a repository
pub async fn delete_repository(pool: &PgPool, id: Uuid, owner_id: Uuid) -> Result<(), GitError> {
    let result = sqlx::query("DELETE FROM repositories WHERE id = $1 AND owner_id = $2")
        .bind(id)
        .bind(owner_id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(GitError::RepositoryNotFound);
    }

    Ok(())
}

/// Check if a repository exists
pub async fn repository_exists(pool: &PgPool, id: Uuid) -> Result<bool, GitError> {
    let result: Option<bool> =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM repositories WHERE id = $1)")
            .bind(id)
            .fetch_one(pool)
            .await?;

    Ok(result.unwrap_or(false))
}

/// Check if a repository name exists for a given owner
pub async fn repository_name_exists(
    pool: &PgPool,
    name: &str,
    owner_id: Uuid,
) -> Result<bool, GitError> {
    let result: Option<bool> = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM repositories WHERE name = $1 AND owner_id = $2)",
    )
    .bind(name)
    .bind(owner_id)
    .fetch_one(pool)
    .await?;

    Ok(result.unwrap_or(false))
}

/// Get repository count by owner
pub async fn count_repositories_by_owner(pool: &PgPool, owner_id: Uuid) -> Result<i64, GitError> {
    let count: i64 =
        sqlx::query_scalar("SELECT COUNT(*)::BIGINT FROM repositories WHERE owner_id = $1")
            .bind(owner_id)
            .fetch_one(pool)
            .await
            .unwrap_or(0);

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_repository_input_validation() {
        let input = CreateRepositoryInput {
            name: "test-repo".to_string(),
            description: Some("Test description".to_string()),
            is_private: true,
            default_branch: "main".to_string(),
        };

        assert!(input.validate().is_ok());
    }

    #[tokio::test]
    async fn test_invalid_repository_name() {
        let input = CreateRepositoryInput {
            name: "".to_string(),
            description: None,
            is_private: false,
            default_branch: "main".to_string(),
        };

        assert!(input.validate().is_err());
    }
}
