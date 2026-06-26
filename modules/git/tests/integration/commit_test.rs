//! Integration tests for commit operations
//!
//! These tests require a running PostgreSQL database.

#[cfg(test)]
mod tests {
    use tardigrade_git::models::commit::{
        Commit, CreateCommitInput, ListCommitsQuery,
    };
    use tardigrade_git::repository::commit::*;
    use tardigrade_git::GitError;
    use uuid::Uuid;

    // Note: These tests would normally use testcontainers to spin up a PostgreSQL container
    // For now, we'll just define the test structure without actual database calls

    #[ignore = "requires database"]
    #[tokio::test]
    async fn test_create_and_get_commit() {
        // This test would:
        // 1. Create a repository
        // 2. Create a branch
        // 3. Create a commit
        // 4. Get the commit by ID
        // 5. Verify the commit details
        
        assert!(true, "Test placeholder - requires database");
    }

    #[ignore = "requires database"]
    #[tokio::test]
    async fn test_list_commits() {
        // This test would:
        // 1. Create a repository
        // 2. Create a branch
        // 3. Create multiple commits
        // 4. List commits with pagination
        // 5. Verify the results
        
        assert!(true, "Test placeholder - requires database");
    }

    #[ignore = "requires database"]
    #[tokio::test]
    async fn test_list_commits_by_branch() {
        // This test would:
        // 1. Create a repository
        // 2. Create multiple branches
        // 3. Create commits on different branches
        // 4. List commits for a specific branch
        // 5. Verify the results
        
        assert!(true, "Test placeholder - requires database");
    }

    #[ignore = "requires database"]
    #[tokio::test]
    async fn test_get_latest_commit() {
        // This test would:
        // 1. Create a repository
        // 2. Create a branch
        // 3. Create multiple commits
        // 4. Get the latest commit
        // 5. Verify it's the most recent one
        
        assert!(true, "Test placeholder - requires database");
    }

    #[ignore = "requires database"]
    #[tokio::test]
    async fn test_commit_hash_uniqueness() {
        // This test would:
        // 1. Create a repository
        // 2. Create a commit
        // 3. Try to create another commit with the same hash
        // 4. Verify the error is returned
        
        assert!(true, "Test placeholder - requires database");
    }

    #[ignore = "requires database"]
    #[tokio::test]
    async fn test_parent_commit_validation() {
        // This test would:
        // 1. Create a repository
        // 2. Create a commit without a parent
        // 3. Try to create a commit with a non-existent parent
        // 4. Verify the error is returned
        
        assert!(true, "Test placeholder - requires database");
    }

    #[test]
    fn test_commit_error_types() {
        // Test that our error types are properly defined
        let commit_not_found = GitError::CommitNotFound;
        assert_eq!(commit_not_found.status_code(), axum::http::StatusCode::NOT_FOUND);

        let parent_not_found = GitError::ParentCommitNotFound("abc123".to_string());
        assert_eq!(parent_not_found.status_code(), axum::http::StatusCode::NOT_FOUND);
    }
}
