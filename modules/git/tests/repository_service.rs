//! Tests d'intégration du `RepositoryService` (CRUD repositories) sur PostgreSQL.

mod common;

use serial_test::serial;

use common::{clear_tables, setup_pool};
use tardigrade_git::models::{CreateRepositoryInput, PaginationQuery};
use tardigrade_git::service::RepositoryService;

fn input(name: &str) -> CreateRepositoryInput {
    CreateRepositoryInput {
        name: name.to_string(),
        description: Some(format!("Description de {}", name)),
        is_private: false,
        default_branch: "main".to_string(),
    }
}

#[tokio::test]
#[serial]
async fn create_repository_persists_and_returns_record() {
    let pool = setup_pool().await;
    clear_tables(&pool).await;

    let svc = RepositoryService::new(pool.clone());
    let repo = svc
        .create_repository(input("repo-create"))
        .await
        .expect("create_repository");

    assert!(!repo.id.is_nil());
    assert_eq!(repo.name, "repo-create");
    assert_eq!(repo.default_branch, "main");
    assert!(!repo.is_private);
}

#[tokio::test]
#[serial]
async fn create_duplicate_repository_returns_conflict() {
    let pool = setup_pool().await;
    clear_tables(&pool).await;

    let svc = RepositoryService::new(pool.clone());
    svc.create_repository(input("repo-dup"))
        .await
        .expect("premier insert");

    let second = svc.create_repository(input("repo-dup")).await;
    assert!(
        second.is_err(),
        "un doublon doit échouer (contrainte unique sur name)"
    );
}

#[tokio::test]
#[serial]
async fn get_repository_returns_created_record() {
    let pool = setup_pool().await;
    clear_tables(&pool).await;

    let svc = RepositoryService::new(pool.clone());
    let created = svc
        .create_repository(input("repo-get"))
        .await
        .expect("create_repository");
    let fetched = svc
        .get_repository(created.id)
        .await
        .expect("get_repository")
        .expect("repository should exist");
    assert_eq!(fetched.id, created.id);
    assert_eq!(fetched.name, "repo-get");
}

#[tokio::test]
#[serial]
async fn get_repository_unknown_returns_none() {
    let pool = setup_pool().await;
    clear_tables(&pool).await;
    let svc = RepositoryService::new(pool.clone());
    let fetched = svc
        .get_repository(uuid::Uuid::new_v4())
        .await
        .expect("get_repository");
    assert!(fetched.is_none(), "un id inconnu doit renvoyer None");
}

#[tokio::test]
#[serial]
async fn list_repositories_paginates_and_counts() {
    let pool = setup_pool().await;
    clear_tables(&pool).await;

    let svc = RepositoryService::new(pool.clone());
    for i in 0..5 {
        svc.create_repository(input(&format!("repo-list-{i}")))
            .await
            .expect("create_repository");
    }

    let page = svc
        .list_repositories(PaginationQuery {
            page: 1,
            page_size: 3,
        })
        .await
        .expect("list_repositories");
    assert_eq!(page.data.len(), 3, "taille de page demandée = 3");
    assert_eq!(page.total, 5, "total de repositories créés = 5");
    assert_eq!(page.page, 1);
    assert_eq!(page.page_size, 3);
    assert_eq!(page.total_pages, 2);
}

#[tokio::test]
#[serial]
async fn update_repository_modifies_fields_and_updated_at() {
    let pool = setup_pool().await;
    clear_tables(&pool).await;

    let svc = RepositoryService::new(pool.clone());
    let created = svc
        .create_repository(input("repo-update"))
        .await
        .expect("create_repository");

    let updated = svc
        .update_repository(
            created.id,
            Some("new-name".to_string()),
            Some("nouvelle description".to_string()),
            Some(true),
            Some("develop".to_string()),
        )
        .await
        .expect("update_repository");

    assert_eq!(updated.name, "new-name");
    assert_eq!(updated.description.as_deref(), Some("nouvelle description"));
    assert!(updated.is_private);
    assert_eq!(updated.default_branch, "develop");
    // Le trigger `trg_repositories_updated_at` met à jour updated_at côté DB.
    assert!(
        updated.updated_at >= created.updated_at,
        "updated_at doit être postérieure ou égale à created_at"
    );
}

#[tokio::test]
#[serial]
async fn delete_repository_removes_record() {
    let pool = setup_pool().await;
    clear_tables(&pool).await;

    let svc = RepositoryService::new(pool.clone());
    let created = svc
        .create_repository(input("repo-delete"))
        .await
        .expect("create_repository");

    svc.delete_repository(created.id)
        .await
        .expect("delete_repository");

    let fetched = svc
        .get_repository(created.id)
        .await
        .expect("get_repository");
    assert!(fetched.is_none(), "le repository doit être supprimé");
}
