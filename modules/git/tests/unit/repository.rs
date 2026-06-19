//! Unit tests for repository operations
//!
//! These tests can be run without a database connection.

use tardigrade_git::models::{
    CreateRepositoryInput, ListRepositoriesQuery, PaginatedResponse, Repository, UpdateRepositoryInput,
};
use uuid::Uuid;
use chrono::{DateTime, Utc, TimeZone};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_repository_input_default() {
        let input = CreateRepositoryInput::default();
        assert_eq!(input.name, "");
        assert_eq!(input.description, None);
        assert_eq!(input.is_private, false);
        assert_eq!(input.default_branch, "main");
    }

    #[test]
    fn test_create_repository_input_validation_valid() {
        let input = CreateRepositoryInput {
            name: "my-test-repo".to_string(),
            description: Some("A test repository".to_string()),
            is_private: true,
            default_branch: "main".to_string(),
        };

        assert!(input.validate().is_ok());
    }

    #[test]
    fn test_create_repository_input_validation_empty_name() {
        let input = CreateRepositoryInput {
            name: "".to_string(),
            description: None,
            is_private: false,
            default_branch: "main".to_string(),
        };

        assert!(input.validate().is_err());
        let err = input.validate().unwrap_err();
        assert!(err.contains("cannot be empty"));
    }

    #[test]
    fn test_create_repository_input_validation_name_too_long() {
        let input = CreateRepositoryInput {
            name: "a".repeat(256),
            description: None,
            is_private: false,
            default_branch: "main".to_string(),
        };

        assert!(input.validate().is_err());
        let err = input.validate().unwrap_err();
        assert!(err.contains("cannot exceed 255"));
    }

    #[test]
    fn test_create_repository_input_validation_invalid_characters() {
        let input = CreateRepositoryInput {
            name: "invalid@repo#name".to_string(),
            description: None,
            is_private: false,
            default_branch: "main".to_string(),
        };

        assert!(input.validate().is_err());
        let err = input.validate().unwrap_err();
        assert!(err.contains("alphanumeric"));
    }

    #[test]
    fn test_create_repository_input_validation_description_too_long() {
        let input = CreateRepositoryInput {
            name: "test-repo".to_string(),
            description: Some("a".repeat(5001)),
            is_private: false,
            default_branch: "main".to_string(),
        };

        assert!(input.validate().is_err());
        let err = input.validate().unwrap_err();
        assert!(err.contains("5000 characters"));
    }

    #[test]
    fn test_create_repository_input_validation_invalid_branch() {
        let input = CreateRepositoryInput {
            name: "test-repo".to_string(),
            description: None,
            is_private: false,
            default_branch: "".to_string(),
        };

        assert!(input.validate().is_err());
    }

    #[test]
    fn test_create_repository_input_validation_valid_branch_names() {
        let valid_branches = vec![
            "main",
            "master",
            "develop",
            "feature/test",
            "release/v1.0",
            "hotfix-1.0.1",
            "my-branch",
            "my_branch",
            "my.branch",
        ];

        for branch in valid_branches {
            let input = CreateRepositoryInput {
                name: "test-repo".to_string(),
                description: None,
                is_private: false,
                default_branch: branch.to_string(),
            };
            assert!(input.validate().is_ok(), "Branch name '{}' should be valid", branch);
        }
    }

    #[test]
    fn test_paginated_response_calculation() {
        let data: Vec<i32> = (1..=10).collect();

        // Test with 10 items, page 1, page_size 5
        let response = PaginatedResponse::new(data.clone(), 1, 5, 10);
        assert_eq!(response.data.len(), 5); // Note: we pass the full data, but the response should handle it
        assert_eq!(response.page, 1);
        assert_eq!(response.page_size, 5);
        assert_eq!(response.total, 10);
        assert_eq!(response.total_pages, 2);
        assert!(response.has_more_pages());
        assert!(!response.has_previous_pages());

        // Test with 10 items, page 2, page_size 5
        let response = PaginatedResponse::new(vec![6, 7, 8, 9, 10], 2, 5, 10);
        assert_eq!(response.page, 2);
        assert_eq!(response.total_pages, 2);
        assert!(!response.has_more_pages());
        assert!(response.has_previous_pages());
    }

    #[test]
    fn test_paginated_response_edge_cases() {
        // Empty data
        let response: PaginatedResponse<i32> = PaginatedResponse::new(vec![], 1, 10, 0);
        assert_eq!(response.data.len(), 0);
        assert_eq!(response.total_pages, 0);
        assert!(!response.has_more_pages());
        assert!(!response.has_previous_pages());

        // Single page
        let response = PaginatedResponse::new(vec![1, 2, 3], 1, 10, 3);
        assert_eq!(response.total_pages, 1);
        assert!(!response.has_more_pages());
    }

    #[test]
    fn test_list_repositories_query_default() {
        let query = ListRepositoriesQuery::default();
        assert_eq!(query.page, 1);
        assert_eq!(query.page_size, 20);
        assert_eq!(query.owner_id, None);
        assert_eq!(query.search, None);
        assert_eq!(query.is_private, None);
    }

    #[test]
    fn test_list_repositories_query_offset_limit() {
        // Page 1, page_size 10 -> offset 0, limit 10
        let query = ListRepositoriesQuery {
            page: 1,
            page_size: 10,
            ..Default::default()
        };
        assert_eq!(query.offset(), 0);
        assert_eq!(query.limit(), 10);

        // Page 2, page_size 10 -> offset 10, limit 10
        let query = ListRepositoriesQuery {
            page: 2,
            page_size: 10,
            ..Default::default()
        };
        assert_eq!(query.offset(), 10);
        assert_eq!(query.limit(), 10);

        // Page 3, page_size 7 -> offset 14, limit 7
        let query = ListRepositoriesQuery {
            page: 3,
            page_size: 7,
            ..Default::default()
        };
        assert_eq!(query.offset(), 14);
        assert_eq!(query.limit(), 7);
    }

    #[test]
    fn test_list_repositories_query_max_page_size() {
        // Page size > 100 should be clamped to 100
        let query = ListRepositoriesQuery {
            page: 1,
            page_size: 200,
            ..Default::default()
        };
        assert_eq!(query.limit(), 100);

        // Page size < 1 should be clamped to 1
        let query = ListRepositoriesQuery {
            page: 1,
            page_size: 0,
            ..Default::default()
        };
        assert_eq!(query.limit(), 1);
    }

    #[test]
    fn test_update_repository_input_default() {
        let input = UpdateRepositoryInput::default();
        assert_eq!(input.name, None);
        assert_eq!(input.description, None);
        assert_eq!(input.is_private, None);
        assert_eq!(input.default_branch, None);
    }

    #[test]
    fn test_repository_new() {
        let id = Uuid::new_v4();
        let owner_id = Uuid::new_v4();
        let now = Utc::now();

        let repo = Repository::new(
            id,
            "test-repo".to_string(),
            Some("Test description".to_string()),
            true,
            owner_id,
            "main".to_string(),
            now,
            now,
        );

        assert_eq!(repo.id, id);
        assert_eq!(repo.name, "test-repo");
        assert_eq!(repo.description, Some("Test description".to_string()));
        assert_eq!(repo.is_private, true);
        assert_eq!(repo.owner_id, owner_id);
        assert_eq!(repo.default_branch, "main");
        assert_eq!(repo.created_at, now);
        assert_eq!(repo.updated_at, now);
    }

    #[test]
    fn test_repository_update() {
        let id = Uuid::new_v4();
        let owner_id = Uuid::new_v4();
        let now = Utc::now();

        let mut repo = Repository::new(
            id,
            "test-repo".to_string(),
            Some("Original description".to_string()),
            false,
            owner_id,
            "main".to_string(),
            now,
            now,
        );

        // Record the original updated_at
        let original_updated_at = repo.updated_at;

        // Create update input
        let update_input = UpdateRepositoryInput {
            name: Some("updated-repo".to_string()),
            description: Some("Updated description".to_string()),
            is_private: Some(true),
            default_branch: Some("develop".to_string()),
        };

        // Update the repository
        repo.update(&update_input);

        // Verify updates
        assert_eq!(repo.name, "updated-repo");
        assert_eq!(repo.description, Some("Updated description".to_string()));
        assert_eq!(repo.is_private, true);
        assert_eq!(repo.default_branch, "develop");
        
        // Updated_at should have changed
        assert_ne!(repo.updated_at, original_updated_at);
    }

    #[test]
    fn test_repository_partial_update() {
        let id = Uuid::new_v4();
        let owner_id = Uuid::new_v4();
        let now = Utc::now();

        let mut repo = Repository::new(
            id,
            "test-repo".to_string(),
            Some("Original description".to_string()),
            false,
            owner_id,
            "main".to_string(),
            now,
            now,
        );

        // Partial update - only change the name
        let update_input = UpdateRepositoryInput {
            name: Some("renamed-repo".to_string()),
            description: None,
            is_private: None,
            default_branch: None,
        };

        repo.update(&update_input);

        // Only name should change
        assert_eq!(repo.name, "renamed-repo");
        assert_eq!(repo.description, Some("Original description".to_string()));
        assert_eq!(repo.is_private, false);
        assert_eq!(repo.default_branch, "main");
    }

    #[test]
    fn test_valid_repository_names() {
        let valid_names = vec![
            "simple",
            "my-repo",
            "my_repo",
            "my.repo",
            "repo-1.0",
            "test_repo.name",
            "a",
            "a".repeat(255), // Max length
        ];

        for name in valid_names {
            let input = CreateRepositoryInput {
                name: name.clone(),
                description: None,
                is_private: false,
                default_branch: "main".to_string(),
            };
            assert!(input.validate().is_ok(), "Name '{}' should be valid", name);
        }
    }

    #[test]
    fn test_invalid_repository_names() {
        let invalid_names = vec![
            "", // Empty
            " ", // Space
            "my repo", // Space in middle
            "my@repo", // @ character
            "my#repo", // # character
            "my$repo", // $ character
            "my%repo", // % character
            "my&repo", // & character
            "my*repo", // * character
            "my(repo", // ( character
            "my)repo", // ) character
            "my[repo]", // Brackets
            "my{repo}", // Braces
            "my\\repo", // Backslash
            "my/repo", // Forward slash
            "my:repo", // Colon
            "my;repo", // Semicolon
            "my'repo", // Single quote
            "my\"repo", // Double quote
            "my<repo>", // Angle brackets
            "my|repo", // Pipe
            "my?repo", // Question mark
            "my!repo", // Exclamation mark
        ];

        for name in invalid_names {
            let input = CreateRepositoryInput {
                name: name.clone(),
                description: None,
                is_private: false,
                default_branch: "main".to_string(),
            };
            assert!(input.validate().is_err(), "Name '{}' should be invalid", name);
        }
    }
}
