//! Commit repository operations for Tardigrade Git module
//!
//! This module implements CRUD operations for Git commits.

use crate::error::GitError;
use crate::models::commit::{
    Commit, CreateCommitInput, DbCommit, ListCommitsQuery, PaginatedResponse,
};
use chrono::{DateTime, Utc};
use sqlx::postgres::PgPool;
use uuid::Uuid;

/// Create a new commit
pub async fn create_commit(
    pool: &PgPool,
    repository_id: Uuid,
    input: CreateCommitInput,
) -> Result<Commit, GitError> {
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

    // Check if branch exists for this repository
    let branch_exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM branches WHERE repository_id = $1 AND name = $2)",
    )
    .bind(repository_id)
    .bind(&input.branch_name)
    .fetch_one(pool)
    .await
    .unwrap_or(false);

    if !branch_exists {
        return Err(GitError::BranchNotFound);
    }

    // Check if parent commit exists (if provided)
    if let Some(ref parent_hash) = input.parent_hash {
        let parent_exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM commits WHERE repository_id = $1 AND hash = $2)",
        )
        .bind(repository_id)
        .bind(parent_hash)
        .fetch_one(pool)
        .await
        .unwrap_or(false);

        if !parent_exists {
            return Err(GitError::ParentCommitNotFound(parent_hash.clone()));
        }
    }

    let now = Utc::now();

    let row = sqlx::query(
        r#"
        INSERT INTO commits (repository_id, hash, parent_hash, message, author_name, author_email, committer_name, committer_email, branch_name, created_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING id, repository_id, hash, parent_hash, message, author_name, author_email, committer_name, committer_email, branch_name, created_at
        "#
    )
    .bind(repository_id)
    .bind(&input.hash)
    .bind(input.parent_hash.as_deref())
    .bind(&input.message)
    .bind(&input.author_name)
    .bind(&input.author_email)
    .bind(input.committer_name.as_deref())
    .bind(input.committer_email.as_deref())
    .bind(&input.branch_name)
    .bind(now)
    .map(|row: sqlx::postgres::PgRow| DbCommit {
        id: row.get("id"),
        repository_id: row.get("repository_id"),
        hash: row.get("hash"),
        parent_hash: row.get("parent_hash"),
        message: row.get("message"),
        author_name: row.get("author_name"),
        author_email: row.get("author_email"),
        committer_name: row.get("committer_name"),
        committer_email: row.get("committer_email"),
        branch_name: row.get("branch_name"),
        created_at: row.get("created_at"),
    })
    .fetch_one(pool)
    .await?;

    Ok(Commit::from(row))
}

/// Get a commit by ID
pub async fn get_commit(pool: &PgPool, id: Uuid) -> Result<Option<Commit>, GitError> {
    let row = sqlx::query(
        r#"
        SELECT id, repository_id, hash, parent_hash, message, author_name, author_email, committer_name, committer_email, branch_name, created_at
        FROM commits WHERE id = $1
        "#,
    )
    .bind(id)
    .map(|row: sqlx::postgres::PgRow| DbCommit {
        id: row.get("id"),
        repository_id: row.get("repository_id"),
        hash: row.get("hash"),
        parent_hash: row.get("parent_hash"),
        message: row.get("message"),
        author_name: row.get("author_name"),
        author_email: row.get("author_email"),
        committer_name: row.get("committer_name"),
        committer_email: row.get("committer_email"),
        branch_name: row.get("branch_name"),
        created_at: row.get("created_at"),
    })
    .fetch_optional(pool)
    .await?;

    Ok(row.map(Commit::from))
}

/// Get a commit by hash and repository ID
pub async fn get_commit_by_hash(
    pool: &PgPool,
    repository_id: Uuid,
    hash: &str,
) -> Result<Option<Commit>, GitError> {
    let row = sqlx::query(
        r#"
        SELECT id, repository_id, hash, parent_hash, message, author_name, author_email, committer_name, committer_email, branch_name, created_at
        FROM commits WHERE repository_id = $1 AND hash = $2
        LIMIT 1
        "#,
    )
    .bind(repository_id)
    .bind(hash)
    .map(|row: sqlx::postgres::PgRow| DbCommit {
        id: row.get("id"),
        repository_id: row.get("repository_id"),
        hash: row.get("hash"),
        parent_hash: row.get("parent_hash"),
        message: row.get("message"),
        author_name: row.get("author_name"),
        author_email: row.get("author_email"),
        committer_name: row.get("committer_name"),
        committer_email: row.get("committer_email"),
        branch_name: row.get("branch_name"),
        created_at: row.get("created_at"),
    })
    .fetch_optional(pool)
    .await?;

    Ok(row.map(Commit::from))
}

/// List commits for a repository with pagination
pub async fn list_commits(
    pool: &PgPool,
    query: ListCommitsQuery,
) -> Result<PaginatedResponse<Commit>, GitError> {
    let offset = query.offset();
    let limit = query.limit();

    let mut sql = "SELECT id, repository_id, hash, parent_hash, message, author_name, author_email, committer_name, committer_email, branch_name, created_at FROM commits WHERE repository_id = $1".to_string();
    let mut params: Vec<&(dyn sqlx::types::ToSql + Sync)> = vec![&query.repository_id];
    let mut param_count = 1;

    // Add branch filter if provided
    if let Some(ref branch_name) = query.branch_name {
        sql.push_str(" AND branch_name = $");
        sql.push_str(&(param_count + 1).to_string());
        params.push(branch_name.as_str());
        param_count += 1;
    }

    // Add search filter if provided
    if let Some(ref search) = query.search {
        sql.push_str(" AND (message ILIKE $");
        sql.push_str(&(param_count + 1).to_string());
        sql.push_str(" OR author_name ILIKE $");
        sql.push_str(&(param_count + 1).to_string());
        sql.push_str(")");
        let search_pattern = format!("%{}%", search);
        params.push(&search_pattern);
        param_count += 1;
    }

    sql.push_str(" ORDER BY created_at DESC LIMIT $");
    sql.push_str(&(param_count + 1).to_string());
    sql.push_str(" OFFSET $");
    sql.push_str(&(param_count + 2).to_string());

    params.push(&limit);
    params.push(&offset);

    // Execute the query
    let mut query_builder = sqlx::query(&sql);
    for param in params {
        query_builder = query_builder.bind(param);
    }

    let rows: Vec<DbCommit> = query_builder
        .map(|row: sqlx::postgres::PgRow| DbCommit {
            id: row.get("id"),
            repository_id: row.get("repository_id"),
            hash: row.get("hash"),
            parent_hash: row.get("parent_hash"),
            message: row.get("message"),
            author_name: row.get("author_name"),
            author_email: row.get("author_email"),
            committer_name: row.get("committer_name"),
            committer_email: row.get("committer_email"),
            branch_name: row.get("branch_name"),
            created_at: row.get("created_at"),
        })
        .fetch_all(pool)
        .await?;

    // Count total
    let mut count_sql = "SELECT COUNT(*)::BIGINT FROM commits WHERE repository_id = $1".to_string();
    let mut count_params: Vec<&(dyn sqlx::types::ToSql + Sync)> = vec![&query.repository_id];
    let mut count_param_count = 1;

    if let Some(ref branch_name) = query.branch_name {
        count_sql.push_str(" AND branch_name = $");
        count_sql.push_str(&(count_param_count + 1).to_string());
        count_params.push(branch_name.as_str());
        count_param_count += 1;
    }

    if query.search.is_some() {
        count_sql.push_str(" AND (message ILIKE $");
        count_sql.push_str(&(count_param_count + 1).to_string());
        count_sql.push_str(" OR author_name ILIKE $");
        count_sql.push_str(&(count_param_count + 1).to_string());
        count_sql.push_str(")");
        let search_pattern = format!("%{}%", query.search.as_ref().unwrap());
        count_params.push(&search_pattern);
    }

    let mut count_query = sqlx::query_scalar(&count_sql);
    for param in count_params {
        count_query = count_query.bind(param);
    }

    let count: i64 = count_query.fetch_one(pool).await.unwrap_or(0);

    let commits: Vec<Commit> = rows.into_iter().map(Commit::from).collect();

    Ok(PaginatedResponse::new(
        commits,
        query.page,
        query.page_size,
        count,
    ))
}

/// Get commits for a specific branch
pub async fn list_commits_by_branch(
    pool: &PgPool,
    repository_id: Uuid,
    branch_name: &str,
    page: i32,
    page_size: i32,
) -> Result<PaginatedResponse<Commit>, GitError> {
    let query = ListCommitsQuery {
        repository_id,
        branch_name: Some(branch_name.to_string()),
        page,
        page_size,
        search: None,
    };

    list_commits(pool, query).await
}

/// Get the latest commit for a branch
pub async fn get_latest_commit(
    pool: &PgPool,
    repository_id: Uuid,
    branch_name: &str,
) -> Result<Option<Commit>, GitError> {
    let row = sqlx::query(
        r#"
        SELECT id, repository_id, hash, parent_hash, message, author_name, author_email, committer_name, committer_email, branch_name, created_at
        FROM commits WHERE repository_id = $1 AND branch_name = $2
        ORDER BY created_at DESC
        LIMIT 1
        "#,
    )
    .bind(repository_id)
    .bind(branch_name)
    .map(|row: sqlx::postgres::PgRow| DbCommit {
        id: row.get("id"),
        repository_id: row.get("repository_id"),
        hash: row.get("hash"),
        parent_hash: row.get("parent_hash"),
        message: row.get("message"),
        author_name: row.get("author_name"),
        author_email: row.get("author_email"),
        committer_name: row.get("committer_name"),
        committer_email: row.get("committer_email"),
        branch_name: row.get("branch_name"),
        created_at: row.get("created_at"),
    })
    .fetch_optional(pool)
    .await?;

    Ok(row.map(Commit::from))
}

/// Check if a commit exists
pub async fn commit_exists(pool: &PgPool, id: Uuid) -> Result<bool, GitError> {
    let result: Option<bool> =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM commits WHERE id = $1)")
            .bind(id)
            .fetch_one(pool)
            .await?;

    Ok(result.unwrap_or(false))
}

/// Check if a commit hash exists for a repository
pub async fn commit_hash_exists(
    pool: &PgPool,
    repository_id: Uuid,
    hash: &str,
) -> Result<bool, GitError> {
    let result: Option<bool> = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM commits WHERE repository_id = $1 AND hash = $2)",
    )
    .bind(repository_id)
    .bind(hash)
    .fetch_one(pool)
    .await?;

    Ok(result.unwrap_or(false))
}

/// Get commit count for a repository
pub async fn count_commits(pool: &PgPool, repository_id: Uuid) -> Result<i64, GitError> {
    let count: i64 =
        sqlx::query_scalar("SELECT COUNT(*)::BIGINT FROM commits WHERE repository_id = $1")
            .bind(repository_id)
            .fetch_one(pool)
            .await
            .unwrap_or(0);

    Ok(count)
}

/// Get commit count for a branch
pub async fn count_commits_by_branch(
    pool: &PgPool,
    repository_id: Uuid,
    branch_name: &str,
) -> Result<i64, GitError> {
    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)::BIGINT FROM commits WHERE repository_id = $1 AND branch_name = $2",
    )
    .bind(repository_id)
    .bind(branch_name)
    .fetch_one(pool)
    .await
    .unwrap_or(0);

    Ok(count)
}
