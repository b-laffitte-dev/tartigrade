# 📌 TODO - US-001 : Créer un repository Git

**User Story** : [US-001](https://github.com/b-laffitte-dev/tartigrade/issues/14) - Créer un repository Git (5 pts)
**Branche** : `sprint-1/us-001-create-repository`
**Assigné à** : @b-laffitte-dev
**Statut** : ⏳ En cours

---

## 🎯 Objectif
Implémenter un endpoint pour créer un repository Git avec les champs de base :
- Nom (`name`)
- Description (`description`)
- Visibilité (`is_private`)
- Propriétaire (`owner_id`)
- Branche par défaut (`default_branch`)

---

## ✅ Tâches à compléter

### 🔹 T-001 : Configurer env Rust (2h) - [Issue #1](https://github.com/b-laffitte-dev/tartigrade/issues/1)
- [ ] Installer Rust 1.70+ (`rustup install 1.70`)
- [ ] Configurer les outils : `cargo`, `clippy`, `rustfmt`, `cargo-audit`
- [ ] Créer un fichier `.rust-toolchain` pour la version de Rust
- [ ] Vérifier : `cargo --version` et `rustc --version`

**Fichiers** :
- `.rust-toolchain`
- `rust-toolchain.toml` (optionnel)

**Commandes de vérification** :
```bash
rustc --version
cargo --version
cargo clippy --version
cargo fmt --version
```

---

### 🔹 T-002 : Structure workspace Cargo (2h) - [Issue #2](https://github.com/b-laffitte-dev/tartigrade/issues/2)
- [ ] Créer un fichier `Cargo.toml` pour le module Git
- [ ] Ajouter les dépendances :
  - `axum` (web framework)
  - `sqlx` (PostgreSQL)
  - `tokio` (async runtime)
  - `serde` (sérialisation)
  - `uuid` (identifiants)
  - `thiserror` (gestion des erreurs)
  - `async-trait` (traits async)
- [ ] Créer la structure des dossiers :
  ```
  modules/git/
  ├── Cargo.toml
  ├── src/
  │   ├── main.rs
  │   ├── lib.rs
  │   ├── config.rs
  │   ├── error.rs
  │   ├── models.rs
  │   ├── repository.rs
  │   ├── service.rs
  │   ├── handler/
  │   │   └── mod.rs
  │   └── routes.rs
  ├── tests/
  │   ├── unit/
  │   └── integration/
  └── migrations/
  ```

**Fichiers** :
- `modules/git/Cargo.toml`
- `modules/git/src/main.rs` (point d'entrée)
- `modules/git/src/lib.rs` (bibliothèque)

**Commandes de vérification** :
```bash
cd modules/git
cargo check
```

---

### 🔹 T-003 : Modèles de données (4h) - [Issue #3](https://github.com/b-laffitte-dev/tartigrade/issues/3)
- [ ] Définir le modèle `Repository` dans `models.rs` :
  ```rust
  pub struct Repository {
      pub id: Uuid,
      pub name: String,
      pub description: Option<String>,
      pub is_private: bool,
      pub owner_id: Uuid,
      pub default_branch: String,
      pub created_at: DateTime<Utc>,
      pub updated_at: DateTime<Utc>,
  }
  ```
- [ ] Définir le modèle `CreateRepositoryInput` :
  ```rust
  pub struct CreateRepositoryInput {
      pub name: String,
      pub description: Option<String>,
      pub is_private: bool,
      pub default_branch: Option<String>, // Default: "main"
  }
  ```
- [ ] Définir le modèle `PaginatedResponse<T>` pour la pagination :
  ```rust
  pub struct PaginatedResponse<T> {
      pub data: Vec<T>,
      pub page: i32,
      pub page_size: i32,
      pub total: i64,
      pub total_pages: i32,
  }
  ```

**Fichiers** :
- `modules/git/src/models.rs`

**Commandes de vérification** :
```bash
cargo check --lib
```

---

### 🔹 T-004 : Connexion PostgreSQL (4h) - [Issue #4](https://github.com/b-laffitte-dev/tartigrade/issues/4)
- [ ] Configurer la connexion à PostgreSQL avec `sqlx` :
  - Créer un fichier `config.rs` pour gérer la configuration :
    ```rust
    pub struct DatabaseConfig {
        pub url: String,
    }
    ```
  - Créer un `PgPool` (pool de connexions) :
    ```rust
    pub async fn create_pool(config: &DatabaseConfig) -> Result<PgPool, sqlx::Error> {
        PgPool::connect(&config.url).await
    }
    ```
- [ ] Créer une migration initiale pour la table `repositories` :
  ```sql
  -- migrations/20260619000000_create_repositories.table.sql
  CREATE TABLE repositories (
      id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
      name VARCHAR(255) NOT NULL,
      description TEXT,
      is_private BOOLEAN NOT NULL DEFAULT FALSE,
      owner_id UUID NOT NULL,
      default_branch VARCHAR(255) NOT NULL DEFAULT 'main',
      created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
      updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
  );
  
  CREATE INDEX idx_repositories_owner_id ON repositories(owner_id);
  CREATE INDEX idx_repositories_name ON repositories(name);
  ```
- [ ] Configurer les variables d'environnement :
  ```env
  DATABASE_URL=postgres://user:password@localhost:5432/tardigrade
  ```

**Fichiers** :
- `modules/git/src/config.rs`
- `modules/git/migrations/20260619000000_create_repositories.table.sql`

**Commandes de vérification** :
```bash
# Appliquer les migrations
sqlx migrate run

# Vérifier la connexion
cargo run --bin tardigrade-git check-db
```

---

### 🔹 T-005 : CRUD repositories (8h) - [Issue #5](https://github.com/b-laffitte-dev/tartigrade/issues/5)
- [ ] Implémenter les fonctions CRUD dans `repository.rs` :
  - `create_repository(pool: &PgPool, input: CreateRepositoryInput, owner_id: Uuid) -> Result<Repository, GitError>`
  - `get_repository(pool: &PgPool, id: Uuid) -> Result<Option<Repository>, GitError>`
  - `list_repositories(pool: &PgPool, owner_id: Option<Uuid>, page: i32, page_size: i32) -> Result<PaginatedResponse<Repository>, GitError>`
  - `delete_repository(pool: &PgPool, id: Uuid, owner_id: Uuid) -> Result<(), GitError>`
- [ ] Gérer les erreurs dans `error.rs` :
  ```rust
  #[derive(Error, Debug)]
  pub enum GitError {
      #[error("Database error: {0}")]
      Database(#[from] sqlx::Error),
      #[error("Repository not found")]
      RepositoryNotFound,
      #[error("Repository '{0}' already exists")]
      RepositoryAlreadyExists(String),
      #[error("Permission denied")]
      PermissionDenied,
      #[error("Validation error: {0}")]
      ValidationError(String),
  }
  ```

**Fichiers** :
- `modules/git/src/repository.rs`
- `modules/git/src/error.rs`

**Commandes de vérification** :
```bash
cargo test --lib repository
```

---

### 🔹 T-006 : Tests unitaires (4h) - [Issue #6](https://github.com/b-laffitte-dev/tartigrade/issues/6)
- [ ] Écrire des tests unitaires pour les fonctions CRUD :
  - Test `create_repository` (succès + doublon)
  - Test `get_repository` (trouvé + introuvable)
  - Test `list_repositories` (pagination)
  - Test `delete_repository` (succès + permission refusée)
- [ ] Utiliser `sqlx::test` pour les tests avec base de données :
  ```rust
  #[sqlx::test]
  async fn test_create_repository(pool: PgPool) {
      let repo = create_repository(&pool, input, owner_id).await.unwrap();
      assert_eq!(repo.name, input.name);
  }
  ```

**Fichiers** :
- `modules/git/tests/unit/repository.rs`

**Commandes de vérification** :
```bash
cargo test --lib
cargo tarpaulin --lib  # Pour la couverture de code
```

---

### 🔹 T-007 : Configurer Axum + handlers (4h) - [Issue #7](https://github.com/b-laffitte-dev/tartigrade/issues/7)
- [ ] Configurer Axum dans `main.rs` :
  ```rust
  use axum::{Router, routing::{get, post}, Json, extract::Path, Extension};
  use std::sync::Arc;
  
  #[tokio::main]
  async fn main() {
      let pool = create_pool(&DatabaseConfig { url: std::env::var("DATABASE_URL").unwrap() }).await.unwrap();
      let shared_state = Arc::new(AppState { pool });
      
      let app = Router::new()
          .route("/repositories", post(create_repository_handler))
          .route("/repositories/:id", get(get_repository_handler))
          .route("/repositories", get(list_repositories_handler))
          .route("/repositories/:id", delete(delete_repository_handler))
          .layer(Extension(shared_state));
      
      axum::Server::bind(&"0.0.0.0:3001".parse().unwrap())
          .serve(app.into_make_service())
          .await
          .unwrap();
  }
  ```
- [ ] Implémenter les handlers dans `handler/repository.rs` :
  ```rust
  pub async fn create_repository_handler(
      State(state): State<Arc<AppState>>,
      Json(input): Json<CreateRepositoryInput>,
  ) -> Result<Json<Repository>, GitError> {
      let repo = create_repository(&state.pool, input, owner_id).await?;
      Ok(Json(repo))
  }
  ```

**Fichiers** :
- `modules/git/src/main.rs`
- `modules/git/src/handler/repository.rs`
- `modules/git/src/routes.rs`

**Commandes de vérification** :
```bash
cargo build
cargo run
```

---

### 🔹 T-008 : Routes Axum (2h) - [Issue #8](https://github.com/b-laffitte-dev/tartigrade/issues/8)
- [ ] Définir les routes dans `routes.rs` :
  ```rust
  pub fn create_router(pool: PgPool) -> Router {
      let state = Arc::new(AppState { pool });
      Router::new()
          .route("/repositories", post(create_repository_handler))
          .route("/repositories/:id", get(get_repository_handler))
          .route("/repositories", get(list_repositories_handler))
          .route("/repositories/:id", delete(delete_repository_handler))
          .layer(Extension(state))
  }
  ```
- [ ] Intégrer les routes dans `main.rs` :
  ```rust
  let app = create_router(pool);
  ```

**Fichiers** :
- `modules/git/src/routes.rs`

**Commandes de vérification** :
```bash
cargo check
curl -X POST http://localhost:3001/repositories -d '{"name": "test-repo", "is_private": false}' -H "Content-Type: application/json"
```

---

## 📅 Planning suggéré pour l'US-001
| **Jour**  | **Tâches** | **Durée** | **Priorité** |
|-----------|------------|-----------|--------------|
| Jour 1    | T-001 + T-002 | 4h | ⭐⭐⭐ |
| Jour 2    | T-003 + T-004 | 8h | ⭐⭐⭐ |
| Jour 3    | T-005 | 8h | ⭐⭐⭐ |
| Jour 4    | T-006 + T-007 | 8h | ⭐⭐ |
| Jour 5    | T-008 | 2h | ⭐ |

---

## 🔥 Commandes utiles

### Build et test
```bash
# Build
cargo build

# Check (vérification des types)
cargo check

# Lint (Clippy)
cargo clippy -- -D warnings

# Format
cargo fmt

# Tests unitaires
cargo test --lib

# Tests avec couverture
cargo tarpaulin --lib

# Run
cargo run
```

### PostgreSQL
```bash
# Démarrer PostgreSQL (si Docker)
docker run --name tardigrade-db -e POSTGRES_PASSWORD=password -p 5432:5432 -d postgres

# Appliquer les migrations
sqlx migrate run

# Vérifier la connexion
psql -h localhost -U postgres -d tardigrade
```

---

## 📚 Ressources
- [Documentation Axum](https://docs.rs/axum/latest/axum/)
- [Documentation SQLx](https://docs.rs/sqlx/latest/sqlx/)
- [SPRINT-1-PLAN.md](https://github.com/b-laffitte-dev/tartigrade/blob/main/SPRINT-1-PLAN.md)
- [ARCHITECTURE.md](https://github.com/b-laffitte-dev/tartigrade/blob/main/ARCHITECTURE.md)

---

## ✅ Checklist avant de merger
- [ ] Toutes les tâches (T-001 à T-008) sont complétées.
- [ ] `cargo build` passe sans erreur.
- [ ] `cargo clippy -- -D warnings` passe sans warning.
- [ ] `cargo test --lib` passe avec >80% de couverture.
- [ ] La documentation (`rustdoc`) est à jour.
- [ ] Les commits suivent les conventions (ex: `feat(git): add repository creation`).

---

**🚀 Prêt à coder !**
Commencez par la **T-001** (Configurer env Rust) et suivez le TODO dans l'ordre.
