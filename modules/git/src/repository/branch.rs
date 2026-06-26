//! Branch repository operations for Tardigrade Git module
//!
//! This module implements CRUD operations for Git branches.

use crate::error::GitError;
use crate::models::branch::{
    Branch, CreateBranchInput, DbBranch, ListBranchesQuery, PaginatedResponse, UpdateBranchInput,
};
use chrono::{DateTime, Utc};
use sqlx::postgres::PgPool;
use uuid::Uuid;

/// Create a new branch
pub async fn create_branch(
    pool: &PgPool,
    repository_id: Uuid,
    input: CreateBranchInput,
) -> Result<Branch, GitError> {
    // Validate input
    input.validate().map_err(GitError::ValidationError)?;

    // Check if repository exists
    let repo_exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM repositories WHERE id = $1)",
    )
    .bind(repository_id)
    .fetch_one(pool)
    .await
    .unwrap_or(false);

    if !repo_exists {
        return Err(GitError::RepositoryNotFound);
    }

    // Check if branch with same name already exists for this repository
    let existing: Option<Uuid> = sqlx::query_scalar(
        "SELECT id FROM branches WHERE repository_id = $1 AND name = $2",
    )
    .bind(repository_id)
    .bind(&input.name)
    .fetch_optional(pool)
    .await?;

    if existing.is_some() {
        return Err(GitError::BranchAlreadyExists(input.name.clone()));
    }

    let now = Utc::now();

    // If this is set as default, unset the current default branch
    if input.is_default {
        sqlx::query("UPDATE branches SET is_default = false WHERE repository_id = $1")
            .bind(repository_id)
            .execute(pool)
            .await
            .ok(); // Ignore errors here, we'll handle them later
    }

    let row = sqlx::query(
        r#"
        INSERT INTO branches (repository_id, name, commit_hash, is_default, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, repository_id, name, commit_hash, is_default, created_at, updated_at
        "#
    )
    .bind(repository_id)
    .bind(&input.name)
    .bind(&input.commit_hash)
    .bind(input.is_default)
    .bind(now)
    .bind(now)
    .map(|row: sqlx::postgres::PgRow| DbBranch {
        id: row.get("id"),
        repository_id: row.get("repository_id"),
        name: row.get("name"),
        commit_hash: row.get("commit_hash"),
        is_default: row.get("is_default"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
    .fetch_one(pool)
    .await?;

    Ok(Branch::from(row))
}

/// Get a branch by ID
pub async fn get_branch(pool: &PgPool, id: Uuid) -> Result<Option<Branch>, GitError> {
    let row = sqlx::query(
        r#"
        SELECT id, repository_id, name, commit_hash, is_default, created_at, updated_at
        FROM branches WHERE id = $1
        "#,
    )
    .bind(id)
    .map(|row: sqlx::postgres::PgRow| DbBranch {
        id: row.get("id"),
        repository_id: row.get("repository_id"),
        name: row.get("name"),
        commit_hash: row.get("commit_hash"),
        is_default: row.get("is_default"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
    .fetch_optional(pool)
    .await?;

    Ok(row.map(Branch::from))
}

/// Get a branch by name and repository ID
pub async fn get_branch_by_name(
    pool: &PgPool,
    repository_id: Uuid,
    name: &str,
) -> Result<Option<Branch>, GitError> {
    let row = sqlx::query(
        r#"
        SELECT id, repository_id, name, commit_hash, is_default, created_at, updated_at
        FROM branches WHERE repository_id = $1 AND name = $2
        "#,
    )
    .bind(repository_id)
    .bind(name)
    .map(|row: sqlx::postgres::PgRow| DbBranch {
        id: row.get("id"),
        repository_id: row.get("repository_id"),
        name: row.get("name"),
        commit_hash: row.get("commit_hash"),
        is_default: row.get("is_default"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
    .fetch_optional(pool)
    .await?;

    Ok(row.map(Branch::from))
}

/// List branches for a repository with pagination
pub async fn list_branches(
    pool: &PgPool,
    query: ListBranchesQuery,
) -> Result<PaginatedResponse<Branch>, GitError> {
    let offset = query.offset();
    let limit = query.limit();

    let rows = sqlx::query(
        r#"
        SELECT id, repository_id, name, commit_hash, is_default, created_at, updated_at
        FROM branches
        WHERE repository_id = $1
        ORDER BY is_default DESC, name ASC
        LIMIT $2 OFFSET $3
        "#
    )
    .bind(query.repository_id)
    .bind(limit)
    .bind(offset)
    .map(|row: sqlx::postgres::PgRow| DbBranch {
        id: row.get("id"),
        repository_id: row.get("repository_id"),
        name: row.get("name"),
        commit_hash: row.get("commit_hash"),
        is_default: row.get("is_default"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
    .fetch_all(pool)
    .await?;

    // Count total
    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)::BIGINT FROM branches WHERE repository_id = $1",
    )
    .bind(query.repository_id)
    .fetch_one(pool)
    .await
    .unwrap_or(0);

    let branches: Vec<Branch> = rows.into_iter().map(Branch::from).collect();

    Ok(PaginatedResponse::new(
        branches,
        query.page,
        query.page_size,
        count,
    ))
}

/// Update a branch
pub async fn update_branch(
    pool: &PgPool,
    id: Uuid,
    input: UpdateBranchInput,
) -> Result<Branch, GitError> {
    // First check if branch exists
    let existing_branch = get_branch(pool, id).await?.ok_or(GitError::BranchNotFound)?;

    // Validate new name if provided
    if let Some(ref name) = input.name {
        if name.is_empty() || name.len() > 255 {
            return Err(GitError::ValidationError(
                "Branch name must be between 1 and 255 characters".to_string(),
            ));
        }

        // Check if new name already exists for this repository
        if let Some(existing_with_name) =
            get_branch_by_name(pool, existing_branch.repository_id, name).await?
        {
            if existing_with_name.id != id {
                return Err(GitError::BranchAlreadyExists(name.clone()));
            }
        }
    }

    // Validate commit hash if provided
    if let Some(ref commit_hash) = input.commit_hash {
        if commit_hash.is_empty() || commit_hash.len() > 64 {
            return Err(GitError::ValidationError(
                "Commit hash must be between 1 and 64 characters".to_string(),
            ));
        }
    }

    let now = Utc::now();
    let mut branch = existing_branch;

    // Apply updates to the existing branch
    branch.update(&input);

    // If this is set as default, unset the current default branch
    if let Some(true) = input.is_default {
        sqlx::query("UPDATE branches SET is_default = false WHERE repository_id = $1 AND id != $2")
            .bind(branch.repository_id)
            .bind(id)
            .execute(pool)
            .await
            .ok();
    }

    // Now update in database
    let row = sqlx::query(
        r#"
        UPDATE branches
        SET name = $1, commit_hash = $2, is_default = $3, updated_at = $4
        WHERE id = $5
        RETURNING id, repository_id, name, commit_hash, is_default, created_at, updated_at
        "#
    )
    .bind(&branch.name)
    .bind(&branch.commit_hash)
    .bind(branch.is_default)
    .bind(now)
    .bind(id)
    .map(|row: sqlx::postgres::PgRow| DbBranch {
        id: row.get("id"),
        repository_id: row.get("repository_id"),
        name: row.get("name"),
        commit_hash: row.get("commit_hash"),
        is_default: row.get("is_default"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
    .fetch_optional(pool)
    .await?;

    match row {
        Some(db_branch) => Ok(Branch::from(db_branch)),
        None => Err(GitError::BranchNotFound),
    }
}

/// Delete a branch
pub async fn delete_branch(pool: &PgPool, id: Uuid) -> Result<(), GitError> {
    // First check if branch exists and is not the default branch
    let branch = get_branch(pool, id).await?.ok_or(GitError::BranchNotFound)?;

    if branch.is_default {
        return Err(GitError::CannotDeleteDefaultBranch);
    }

    let result = sqlx::query("DELETE FROM branches WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(GitError::BranchNotFound);
    }

    Ok(())
}

/// Get the default branch for a repository
pub async fn get_default_branch(
    pool: &PgPool,
    repository_id: Uuid,
) -> Result<Option<Branch>, GitError> {
    let row = sqlx::query(
        r#"
        SELECT id, repository_id, name, commit_hash, is_default, created_at, updated_at
        FROM branches WHERE repository_id = $1 AND is_default = true
        LIMIT 1
        "#,
    )
    .bind(repository_id)
    .map(|row: sqlx::postgres::PgRow| DbBranch {
        id: row.get("id"),
        repository_id: row.get("repository_id"),
        name: row.get("name"),
        commit_hash: row.get("commit_hash"),
        is_default: row.get("is_default"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
    .fetch_optional(pool)
    .await?;

    Ok(row.map(Branch::from))
}

/// Check if a branch exists
pub async fn branch_exists(pool: &PgPool, id: Uuid) -> Result<bool, GitError> {
    let result: Option<bool> =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM branches WHERE id = $1)")
            .bind(id)
            .fetch_one(pool)
            .await?;

    Ok(result.unwrap_or(false))
}

/// Check if a branch name exists for a repository
pub async fn branch_name_exists(
    pool: &PgPool,
    repository_id: Uuid,
    name: &str,
) -> Result<bool, GitError> {
    let result: Option<bool> = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM branches WHERE repository_id = $1 AND name = $2)",
    )
    .bind(repository_id)
    .bind(name)
    .fetch_one(pool)
    .await?;

    Ok(result.unwrap_or(false))
}

/// Get branch count for a repository
pub async fn count_branches(pool: &PgPool, repository_id: Uuid) -> Result<i64, GitError> {
    let count: i64 =
        sqlx::query_scalar("SELECT COUNT(*)::BIGINT FROM branches WHERE repository_id = $1")
            .bind(repository_id)
            .fetch_one(pool)
            .await
            .unwrap_or(0);

    Ok(count)
}
