//! Repository operations for Tardigrade Git module
//!
//! This module implements CRUD operations for Git repositories.

use crate::error::GitError;
use crate::models::{
    Branch, Commit, CreateBranchInput, CreateCommitInput, CreateRepositoryInput, DbBranch, DbCommit, 
    DbRepository, ListRepositoriesQuery, PaginatedResponse, Repository, UpdateRepositoryInput,
};
use chrono::{DateTime, Utc};
use sqlx::postgres::PgPool;
use sqlx::{FromRow, Row};
use uuid::Uuid;

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

// ============================================================================
// Branch Repository Operations
// ============================================================================

/// Create a new branch in a repository
pub async fn create_branch(
    pool: &PgPool,
    repository_id: Uuid,
    input: CreateBranchInput,
    owner_id: Uuid,
) -> Result<Branch, GitError> {
    // Validate input
    input.validate().map_err(GitError::ValidationError)?;

    // Check if repository exists and belongs to the owner
    let repository = get_repository_by_id_and_owner(pool, repository_id, owner_id).await?;
    if repository.is_none() {
        return Err(GitError::RepositoryNotFound);
    }

    // Check if branch with same name already exists for this repository
    let existing: Option<Uuid> = sqlx::query_scalar(
        "SELECT id FROM branches WHERE repository_id = $1 AND name = $2"
    )
    .bind(repository_id)
    .bind(&input.name)
    .fetch_optional(pool)
    .await?;

    if existing.is_some() {
        return Err(GitError::BranchAlreadyExists(input.name.clone()));
    }

    // Determine initial commit hash
    let commit_hash = input.commit_hash.unwrap_or_else(|| {
        // For now, use an empty hash - in a real implementation, this would be the hash
        // of the initial commit or the default branch's current commit
        "0000000000000000000000000000000000000000".to_string()
    });

    let now = Utc::now();

    let row = sqlx::query(
        r#"
        INSERT INTO branches (repository_id, name, commit_hash, created_at)
        VALUES ($1, $2, $3, $4)
        RETURNING id, repository_id, name, commit_hash, created_at
        "#
    )
    .bind(repository_id)
    .bind(&input.name)
    .bind(&commit_hash)
    .bind(now)
    .map(|row: sqlx::postgres::PgRow| DbBranch {
        id: row.get("id"),
        repository_id: row.get("repository_id"),
        name: row.get("name"),
        commit_hash: row.get("commit_hash"),
        created_at: row.get("created_at"),
    })
    .fetch_one(pool)
    .await?;

    Ok(Branch::from(row))
}

/// Get a branch by ID
pub async fn get_branch(pool: &PgPool, id: Uuid) -> Result<Option<Branch>, GitError> {
    let row = sqlx::query(
        r#"
        SELECT id, repository_id, name, commit_hash, created_at
        FROM branches WHERE id = $1
        "#,
    )
    .bind(id)
    .map(|row: sqlx::postgres::PgRow| DbBranch {
        id: row.get("id"),
        repository_id: row.get("repository_id"),
        name: row.get("name"),
        commit_hash: row.get("commit_hash"),
        created_at: row.get("created_at"),
    })
    .fetch_optional(pool)
    .await?;

    Ok(row.map(Branch::from))
}

/// Get a branch by repository ID and branch name
pub async fn get_branch_by_repository_and_name(
    pool: &PgPool,
    repository_id: Uuid,
    name: &str,
) -> Result<Option<Branch>, GitError> {
    let row = sqlx::query(
        r#"
        SELECT id, repository_id, name, commit_hash, created_at
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
        created_at: row.get("created_at"),
    })
    .fetch_optional(pool)
    .await?;

    Ok(row.map(Branch::from))
}

/// List branches for a repository with pagination
pub async fn list_branches(
    pool: &PgPool,
    repository_id: Uuid,
    page: i32,
    page_size: i32,
) -> Result<PaginatedResponse<Branch>, GitError> {
    let offset = ((page - 1) * page_size) as i64;
    let limit = page_size as i64;

    let rows = sqlx::query(
        r#"
        SELECT id, repository_id, name, commit_hash, created_at
        FROM branches
        WHERE repository_id = $1
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(repository_id)
    .bind(limit)
    .bind(offset)
    .map(|row: sqlx::postgres::PgRow| DbBranch {
        id: row.get("id"),
        repository_id: row.get("repository_id"),
        name: row.get("name"),
        commit_hash: row.get("commit_hash"),
        created_at: row.get("created_at"),
    })
    .fetch_all(pool)
    .await?;

    let total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)::BIGINT FROM branches WHERE repository_id = $1"
    )
    .bind(repository_id)
    .fetch_one(pool)
    .await
    .unwrap_or(0);

    let branches: Vec<Branch> = rows.into_iter().map(Branch::from).collect();

    Ok(PaginatedResponse::new(branches, page, page_size, total))
}

/// Delete a branch from a repository
pub async fn delete_branch(
    pool: &PgPool,
    repository_id: Uuid,
    branch_name: &str,
    owner_id: Uuid,
) -> Result<(), GitError> {
    // First, get the repository to check permissions and get default branch
    let repository = get_repository_by_id_and_owner(pool, repository_id, owner_id).await?;
    if repository.is_none() {
        return Err(GitError::RepositoryNotFound);
    }
    let repository = repository.unwrap();

    // Check if branch exists
    let branch = get_branch_by_repository_and_name(pool, repository_id, branch_name).await?;
    if branch.is_none() {
        return Err(GitError::BranchNotFound);
    }

    // Prevent deletion of default branch
    if branch_name == repository.default_branch {
        return Err(GitError::CannotDeleteDefaultBranch);
    }

    let result = sqlx::query("DELETE FROM branches WHERE repository_id = $1 AND name = $2")
        .bind(repository_id)
        .bind(branch_name)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(GitError::BranchNotFound);
    }

    Ok(())
}

// ============================================================================
// Commit Repository Operations
// ============================================================================

/// Create a new commit in a branch
pub async fn create_commit(
    pool: &PgPool,
    repository_id: Uuid,
    branch_name: &str,
    input: CreateCommitInput,
    committer_id: Uuid,
    committer_name: &str,
    committer_email: &str,
) -> Result<Commit, GitError> {
    // Validate input
    input.validate().map_err(GitError::ValidationError)?;

    // Check if repository exists
    let repository = get_repository(pool, repository_id).await?;
    if repository.is_none() {
        return Err(GitError::RepositoryNotFound);
    }

    // Check if branch exists
    let branch = get_branch_by_repository_and_name(pool, repository_id, branch_name).await?;
    if branch.is_none() {
        return Err(GitError::BranchNotFound);
    }

    // Check permissions (simplified: only owner can commit for now)
    let repository = repository.unwrap();
    if repository.owner_id != committer_id {
        return Err(GitError::PermissionDenied);
    }

    // Generate a unique commit hash (simplified: use UUID for now)
    // In a real implementation, this would be a SHA-1 hash of the commit content
    let commit_hash = Uuid::new_v4().to_string();

    let now = Utc::now();

    let row = sqlx::query(
        r#"
        INSERT INTO commits (
            repository_id, hash, message, author_name, author_email, 
            committer_name, committer_email, created_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id, repository_id, hash, message, author_name, author_email, 
                  committer_name, committer_email, created_at
        "#
    )
    .bind(repository_id)
    .bind(&commit_hash)
    .bind(&input.message)
    .bind(&input.author_name)
    .bind(&input.author_email)
    .bind(committer_name)
    .bind(committer_email)
    .bind(now)
    .map(|row: sqlx::postgres::PgRow| DbCommit {
        id: row.get("id"),
        repository_id: row.get("repository_id"),
        hash: row.get("hash"),
        message: row.get("message"),
        author_name: row.get("author_name"),
        author_email: row.get("author_email"),
        committer_name: row.get("committer_name"),
        committer_email: row.get("committer_email"),
        created_at: row.get("created_at"),
    })
    .fetch_one(pool)
    .await?;

    // Update the branch's commit hash
    sqlx::query(
        "UPDATE branches SET commit_hash = $1 WHERE repository_id = $2 AND name = $3"
    )
    .bind(&commit_hash)
    .bind(repository_id)
    .bind(branch_name)
    .execute(pool)
    .await?;

    Ok(Commit::from(row))
}

/// Get a commit by ID
pub async fn get_commit(pool: &PgPool, id: Uuid) -> Result<Option<Commit>, GitError> {
    let row = sqlx::query(
        r#"
        SELECT id, repository_id, hash, message, author_name, author_email, 
               committer_name, committer_email, created_at
        FROM commits WHERE id = $1
        "#,
    )
    .bind(id)
    .map(|row: sqlx::postgres::PgRow| DbCommit {
        id: row.get("id"),
        repository_id: row.get("repository_id"),
        hash: row.get("hash"),
        message: row.get("message"),
        author_name: row.get("author_name"),
        author_email: row.get("author_email"),
        committer_name: row.get("committer_name"),
        committer_email: row.get("committer_email"),
        created_at: row.get("created_at"),
    })
    .fetch_optional(pool)
    .await?;

    Ok(row.map(Commit::from))
}

/// Get a commit by hash and repository
pub async fn get_commit_by_hash(
    pool: &PgPool,
    repository_id: Uuid,
    hash: &str,
) -> Result<Option<Commit>, GitError> {
    let row = sqlx::query(
        r#"
        SELECT id, repository_id, hash, message, author_name, author_email, 
               committer_name, committer_email, created_at
        FROM commits WHERE repository_id = $1 AND hash = $2
        "#,
    )
    .bind(repository_id)
    .bind(hash)
    .map(|row: sqlx::postgres::PgRow| DbCommit {
        id: row.get("id"),
        repository_id: row.get("repository_id"),
        hash: row.get("hash"),
        message: row.get("message"),
        author_name: row.get("author_name"),
        author_email: row.get("author_email"),
        committer_name: row.get("committer_name"),
        committer_email: row.get("committer_email"),
        created_at: row.get("created_at"),
    })
    .fetch_optional(pool)
    .await?;

    Ok(row.map(Commit::from))
}

/// List commits for a branch with pagination
pub async fn list_commits(
    pool: &PgPool,
    repository_id: Uuid,
    branch_name: &str,
    page: i32,
    page_size: i32,
) -> Result<PaginatedResponse<Commit>, GitError> {
    // First, verify the branch exists
    let branch = get_branch_by_repository_and_name(pool, repository_id, branch_name).await?;
    if branch.is_none() {
        return Err(GitError::BranchNotFound);
    }

    let offset = ((page - 1) * page_size) as i64;
    let limit = page_size as i64;

    let rows = sqlx::query(
        r#"
        SELECT id, repository_id, hash, message, author_name, author_email, 
               committer_name, committer_email, created_at
        FROM commits
        WHERE repository_id = $1
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
        "#,
    )
    .bind(repository_id)
    .bind(limit)
    .bind(offset)
    .map(|row: sqlx::postgres::PgRow| DbCommit {
        id: row.get("id"),
        repository_id: row.get("repository_id"),
        hash: row.get("hash"),
        message: row.get("message"),
        author_name: row.get("author_name"),
        author_email: row.get("author_email"),
        committer_name: row.get("committer_name"),
        committer_email: row.get("committer_email"),
        created_at: row.get("created_at"),
    })
    .fetch_all(pool)
    .await?;

    let total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)::BIGINT FROM commits WHERE repository_id = $1"
    )
    .bind(repository_id)
    .fetch_one(pool)
    .await
    .unwrap_or(0);

    let commits: Vec<Commit> = rows.into_iter().map(Commit::from).collect();

    Ok(PaginatedResponse::new(commits, page, page_size, total))
}

/// Clone repository - generate clone URL
pub async fn clone_repository(
    pool: &PgPool,
    repository_id: Uuid,
    user_id: Uuid,
    method: &str,
) -> Result<String, GitError> {
    // Check if repository exists and user has access
    let repository = get_repository(pool, repository_id).await?;
    if repository.is_none() {
        return Err(GitError::RepositoryNotFound);
    }

    let repository = repository.unwrap();

    // For now, allow clone if repository is public or user is owner
    // In a real implementation, we'd check collaborative permissions
    if repository.is_private && repository.owner_id != user_id {
        return Err(GitError::PermissionDenied);
    }

    let repo_name = repository.name;
    let owner_id = repository.owner_id;

    // Generate clone URL based on method
    match method.to_lowercase().as_str() {
        "ssh" => Ok(format!("git@tardigrade-ci.com:{}/{}.git", owner_id, repo_name)),
        "https" => Ok(format!("https://tardigrade-ci.com/{}/{}.git", owner_id, repo_name)),
        _ => Err(GitError::CloneError(format!("Unsupported clone method: {}", method))),
    }
}

/// Push to repository - create a new commit
pub async fn push_to_repository(
    pool: &PgPool,
    repository_id: Uuid,
    branch_name: &str,
    input: CreateCommitInput,
    user_id: Uuid,
    committer_name: &str,
    committer_email: &str,
) -> Result<String, GitError> {
    // Create commit (which will also update branch's commit hash)
    let commit = create_commit(
        pool,
        repository_id,
        branch_name,
        input,
        user_id,
        committer_name,
        committer_email,
    ).await?;

    Ok(commit.hash)
}
