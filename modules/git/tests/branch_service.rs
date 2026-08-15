//! Tests d'intégration du `BranchService` (CRUD branches) sur PostgreSQL.

mod common;

use serial_test::serial;

use common::{clear_tables, setup_pool};
use tardigrade_git::models::{CreateBranchInput, CreateRepositoryInput, PaginationQuery};
use tardigrade_git::service::{BranchService, RepositoryService};

async fn seed_repository(svc: &RepositoryService) -> uuid::Uuid {
    // Nom unique pour éviter les collisions entre tests parallèles.
    let unique = uuid::Uuid::new_v4().simple().to_string();
    let repo = svc
        .create_repository(CreateRepositoryInput {
            name: format!("repo-for-branches-{unique}"),
            description: None,
            is_private: false,
            default_branch: "main".to_string(),
        })
        .await
        .expect("create_repository");
    repo.id
}

#[tokio::test]
#[serial]
async fn create_branch_persists_and_links_to_repository() {
    let pool = setup_pool().await;
    clear_tables(&pool).await;

    let repo_svc = RepositoryService::new(pool.clone());
    let repo_id = seed_repository(&repo_svc).await;

    let branch_svc = BranchService::new(pool.clone());
    let branch = branch_svc
        .create_branch(
            repo_id,
            CreateBranchInput {
                name: "feature/test".to_string(),
                commit_hash: Some("abc123".to_string()),
            },
        )
        .await
        .expect("create_branch");

    assert_eq!(branch.repository_id, repo_id);
    assert_eq!(branch.name, "feature/test");
    assert_eq!(branch.commit_hash.as_deref(), Some("abc123"));
}

#[tokio::test]
#[serial]
async fn create_branch_in_unknown_repository_fails() {
    let pool = setup_pool().await;
    clear_tables(&pool).await;

    let branch_svc = BranchService::new(pool.clone());
    let result = branch_svc
        .create_branch(
            uuid::Uuid::new_v4(),
            CreateBranchInput {
                name: "orphan".to_string(),
                commit_hash: None,
            },
        )
        .await;
    assert!(
        result.is_err(),
        "créer une branche sur un repo inexistant doit échouer"
    );
}

#[tokio::test]
#[serial]
async fn create_duplicate_branch_fails() {
    let pool = setup_pool().await;
    clear_tables(&pool).await;

    let repo_svc = RepositoryService::new(pool.clone());
    let repo_id = seed_repository(&repo_svc).await;

    let branch_svc = BranchService::new(pool.clone());
    branch_svc
        .create_branch(
            repo_id,
            CreateBranchInput {
                name: "develop".to_string(),
                commit_hash: None,
            },
        )
        .await
        .expect("première branche");

    let dup = branch_svc
        .create_branch(
            repo_id,
            CreateBranchInput {
                name: "develop".to_string(),
                commit_hash: None,
            },
        )
        .await;
    assert!(dup.is_err(), "un doublon de branche doit échouer");
}

#[tokio::test]
#[serial]
async fn list_branches_returns_only_those_of_repository() {
    let pool = setup_pool().await;
    clear_tables(&pool).await;

    let repo_svc = RepositoryService::new(pool.clone());
    let repo_id = seed_repository(&repo_svc).await;

    let branch_svc = BranchService::new(pool.clone());
    for name in &["dev", "staging", "prod"] {
        branch_svc
            .create_branch(
                repo_id,
                CreateBranchInput {
                    name: name.to_string(),
                    commit_hash: None,
                },
            )
            .await
            .expect("create_branch");
    }

    let branches = branch_svc
        .list_branches(
            repo_id,
            PaginationQuery {
                page: 1,
                page_size: 20,
            },
        )
        .await
        .expect("list_branches");
    assert_eq!(branches.data.len(), 3);
    assert_eq!(branches.total, 3);
}

#[tokio::test]
#[serial]
async fn delete_branch_removes_record() {
    let pool = setup_pool().await;
    clear_tables(&pool).await;

    let repo_svc = RepositoryService::new(pool.clone());
    let repo_id = seed_repository(&repo_svc).await;

    let branch_svc = BranchService::new(pool.clone());
    let branch = branch_svc
        .create_branch(
            repo_id,
            CreateBranchInput {
                name: "to-delete".to_string(),
                commit_hash: None,
            },
        )
        .await
        .expect("create_branch");

    branch_svc
        .delete_branch(branch.id, repo_id)
        .await
        .expect("delete_branch");

    let fetched = branch_svc.get_branch(branch.id).await.expect("get_branch");
    assert!(fetched.is_none(), "la branche doit être supprimée");
}

#[tokio::test]
#[serial]
async fn cascade_delete_repository_removes_branches() {
    let pool = setup_pool().await;
    clear_tables(&pool).await;

    let repo_svc = RepositoryService::new(pool.clone());
    let repo_id = seed_repository(&repo_svc).await;

    let branch_svc = BranchService::new(pool.clone());
    let branch = branch_svc
        .create_branch(
            repo_id,
            CreateBranchInput {
                name: "cascade-test".to_string(),
                commit_hash: None,
            },
        )
        .await
        .expect("create_branch");

    // Supprimer le repository doit cascader vers ses branches (ON DELETE CASCADE).
    repo_svc
        .delete_repository(repo_id)
        .await
        .expect("delete_repository");

    let fetched = branch_svc.get_branch(branch.id).await.expect("get_branch");
    assert!(
        fetched.is_none(),
        "la branche doit être supprimée en cascade"
    );
}
