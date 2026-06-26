//! Unit tests for branch repository operations

use tardigrade_git::models::branch::{
    Branch, CreateBranchInput, ListBranchesQuery, UpdateBranchInput,
};
use tardigrade_git::repository::branch::*;
use uuid::Uuid;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_branch_input_validation() {
        let mut input = CreateBranchInput::default();
        input.name = "main".to_string();
        input.commit_hash = "abc123".to_string();
        assert!(input.validate().is_ok());

        let mut input = CreateBranchInput::default();
        input.name = "".to_string();
        input.commit_hash = "abc123".to_string();
        assert!(input.validate().is_err());

        let mut input = CreateBranchInput::default();
        input.name = "invalid@name".to_string();
        input.commit_hash = "abc123".to_string();
        assert!(input.validate().is_err());

        let mut input = CreateBranchInput::default();
        input.name = "main".to_string();
        input.commit_hash = "".to_string();
        assert!(input.validate().is_err());
    }

    #[test]
    fn test_list_branches_query_defaults() {
        let query = ListBranchesQuery::default();
        assert_eq!(query.page, 1);
        assert_eq!(query.page_size, 20);
        assert_eq!(query.offset(), 0);
        assert_eq!(query.limit(), 20);
    }

    #[test]
    fn test_list_branches_query_pagination() {
        let query = ListBranchesQuery {
            repository_id: Uuid::new_v4(),
            page: 2,
            page_size: 10,
            search: None,
        };
        assert_eq!(query.offset(), 10);
        assert_eq!(query.limit(), 10);
    }

    #[test]
    fn test_branch_model_creation() {
        let branch = Branch::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            "main".to_string(),
            "abc123".to_string(),
            true,
            chrono::Utc::now(),
            chrono::Utc::now(),
        );

        assert_eq!(branch.name, "main");
        assert_eq!(branch.commit_hash, "abc123");
        assert!(branch.is_default);
    }

    #[test]
    fn test_branch_model_update() {
        let mut branch = Branch::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            "main".to_string(),
            "abc123".to_string(),
            true,
            chrono::Utc::now(),
            chrono::Utc::now(),
        );

        let input = UpdateBranchInput {
            name: Some("feature/new".to_string()),
            commit_hash: Some("def456".to_string()),
            is_default: Some(false),
        };

        branch.update(&input);

        assert_eq!(branch.name, "feature/new");
        assert_eq!(branch.commit_hash, "def456");
        assert!(!branch.is_default);
    }
}
