//! Unit tests for commit repository operations

use tardigrade_git::models::commit::{
    Commit, CreateCommitInput, ListCommitsQuery,
};
use tardigrade_git::repository::commit::*;
use uuid::Uuid;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_commit_input_validation() {
        let mut input = CreateCommitInput::default();
        input.hash = "a".repeat(40);
        input.message = "Initial commit".to_string();
        input.author_name = "John Doe".to_string();
        input.author_email = "john@example.com".to_string();
        input.branch_name = "main".to_string();
        assert!(input.validate().is_ok());

        let mut input = CreateCommitInput::default();
        input.hash = "".to_string();
        assert!(input.validate().is_err());

        let mut input = CreateCommitInput::default();
        input.hash = "abc".to_string(); // Too short
        assert!(input.validate().is_err());

        let mut input = CreateCommitInput::default();
        input.hash = "abc123".to_string(); // Invalid length
        assert!(input.validate().is_err());

        let mut input = CreateCommitInput::default();
        input.hash = "a".repeat(40);
        input.message = "".to_string();
        assert!(input.validate().is_err());

        let mut input = CreateCommitInput::default();
        input.hash = "a".repeat(40);
        input.message = "Valid message".to_string();
        input.author_email = "invalid-email".to_string();
        assert!(input.validate().is_err());
    }

    #[test]
    fn test_list_commits_query_defaults() {
        let query = ListCommitsQuery::default();
        assert_eq!(query.page, 1);
        assert_eq!(query.page_size, 20);
        assert_eq!(query.offset(), 0);
        assert_eq!(query.limit(), 20);
    }

    #[test]
    fn test_list_commits_query_pagination() {
        let query = ListCommitsQuery {
            repository_id: Uuid::new_v4(),
            branch_name: None,
            page: 2,
            page_size: 10,
            search: None,
        };
        assert_eq!(query.offset(), 10);
        assert_eq!(query.limit(), 10);
    }

    #[test]
    fn test_commit_model_creation() {
        let commit = Commit::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            "a".repeat(40),
            Some("b".repeat(40)),
            "Initial commit".to_string(),
            "John Doe".to_string(),
            "john@example.com".to_string(),
            Some("John Doe".to_string()),
            Some("john@example.com".to_string()),
            "main".to_string(),
            chrono::Utc::now(),
        );

        assert_eq!(commit.hash.len(), 40);
        assert_eq!(commit.message, "Initial commit");
        assert_eq!(commit.author_name, "John Doe");
        assert_eq!(commit.branch_name, "main");
    }
}
