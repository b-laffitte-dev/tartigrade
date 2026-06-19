# 🚀 Sprint 1 - Git Module MVP

**Projet :** Tardigrade-CI  
**Durée :** 2 semaines (2026-07-15 → 2026-07-28)  
**Équipe :** Benzo (Dev Java Senior + Supervision Rust) + Agent IA (Génération Code)  
**Objectif :** Livrer un **Git Module fonctionnel** en Rust avec les opérations CRUD de base  
**Statut :** ⬜ À démarrer  

---

## 🎯 Objectifs du Sprint

### ✅ Livrables Principaux
- [ ] **Module Git en Rust** (Axum + SQLx + PostgreSQL) avec :
  - CRUD des repositories
  - Gestion des branches
  - Gestion des commits (basique)
  - API gRPC interne
  - API GraphQL publique
- [ ] **Intégration avec l'API Gateway**
- [ ] **Tests unitaires et d'intégration** (>80% couverture)
- [ ] **Documentation technique**
- [ ] **Dockerfile** pour déploiement local

### 📊 Métriques de Succès
| Métrique | Cible | Outil de Mesure |
|----------|-------|-----------------|
| Fonctionnalités implémentées | 100% (US-001 à US-006) | Backlog |
| Couverture de tests | >80% | `cargo tarpaulin` |
| Clippy Warnings | 0 | `cargo clippy -- -D warnings` |
| Build Time | <2min | `cargo build --release` |
| Docker Image Size | <100Mo | `docker images` |
| Temps de réponse API | <100ms (P95) | Benchmark local |

---

## 📋 Backlog du Sprint

### 🎯 Épic : **Git Module MVP**
**Objectif :** Fournir un gestionnaire de code source Git fonctionnel et auto-hébergé

| ID | User Story | Priorité | Points | Statut | Affecté | Dépendances |
|----|------------|----------|--------|--------|---------|--------------|
| **US-001** | En tant que développeur, je veux **créer un repository Git** afin de stocker mon code source | **High** | 5 | ⬜ To Do | Benzo + IA | Aucune |
| **US-002** | En tant que développeur, je veux **cloner/pusher du code** afin de collaborer avec mon équipe | **High** | 8 | ⬜ To Do | Benzo + IA | US-001 |
| **US-003** | En tant que mainteneur, je veux **gérer les branches** (create/delete/protect) afin de contrôler les contributions | **High** | 3 | ⬜ To Do | Benzo + IA | US-001 |
| **US-004** | En tant que développeur, je veux **créer des Pull Requests** afin de proposer des changements | **High** | 8 | ⬜ To Do | Benzo + IA | US-001, US-002 |
| **US-005** | En tant que reviewer, je veux **commenter et approver des PR** afin de maintenir la qualité | **Medium** | 5 | ⬜ To Do | Benzo + IA | US-004 |
| **US-006** | En tant que mainteneur, je veux **gérer les issues** afin de suivre les bugs et features | **Medium** | 5 | ⬜ To Do | Benzo + IA | US-001 |

### 📌 Tâches Techniques

#### 🔹 Semaine 1 (2026-07-15 → 2026-07-21) - Fondations

| ID | Tâche | Priorité | Complexité | Statut | Affecté | Estimation |
|----|-------|----------|------------|--------|---------|------------|
| **T-001** | Configurer l'environnement de développement Rust | **High** | ⭐⭐ | ⬜ To Do | Benzo | 2h |
| **T-002** | Créer la structure du workspace Cargo | **High** | ⭐⭐ | ⬜ To Do | Benzo | 2h |
| **T-003** | Définir les modèles de données (Repository, Branch, Commit, etc.) | **High** | ⭐⭐⭐ | ⬜ To Do | Benzo + IA | 4h |
| **T-004** | Configurer la connexion PostgreSQL avec SQLx | **High** | ⭐⭐⭐ | ⬜ To Do | Benzo + IA | 4h |
| **T-005** | Implémenter le CRUD des repositories (GitService) | **High** | ⭐⭐⭐⭐ | ⬜ To Do | Benzo + IA | 8h |
| **T-006** | Écrire les tests unitaires pour GitService | **High** | ⭐⭐⭐ | ⬜ To Do | Benzo | 4h |
| **T-007** | Configurer Axum et créer les handlers HTTP | **High** | ⭐⭐⭐ | ⬜ To Do | Benzo + IA | 4h |
| **T-008** | Créer les routes Axum pour le Git Module | **High** | ⭐⭐ | ⬜ To Do | Benzo | 2h |

#### 🔹 Semaine 2 (2026-07-22 → 2026-07-28) - Intégration & Tests

| ID | Tâche | Priorité | Complexité | Statut | Affecté | Estimation |
|----|-------|----------|------------|--------|---------|------------|
| **T-009** | Implémenter le support gRPC pour le Git Module | **High** | ⭐⭐⭐⭐ | ⬜ To Do | Benzo + IA | 6h |
| **T-010** | Intégrer le Git Module avec l'API Gateway | **High** | ⭐⭐⭐ | ⬜ To Do | Benzo | 4h |
| **T-011** | Implémenter le schéma GraphQL pour Git | **High** | ⭐⭐⭐ | ⬜ To Do | Benzo + IA | 4h |
| **T-012** | Gérer les branches (create/delete) | **High** | ⭐⭐⭐ | ⬜ To Do | Benzo + IA | 6h |
| **T-013** | Implémenter les commits basiques | **Medium** | ⭐⭐⭐ | ⬜ To Do | Benzo + IA | 6h |
| **T-014** | Écrire les tests d'intégration | **High** | ⭐⭐⭐ | ⬜ To Do | Benzo | 4h |
| **T-015** | Optimiser les performances | **Medium** | ⭐⭐ | ⬜ To Do | Benzo | 4h |
| **T-016** | Créer le Dockerfile pour le Git Module | **High** | ⭐⭐ | ⬜ To Do | Benzo | 2h |
| **T-017** | Documenter le module (README, rustdoc) | **High** | ⭐⭐ | ⬜ To Do | Benzo | 4h |
| **T-018** | Sprint Review & Rétrospective | **High** | ⭐ | ⬜ To Do | Équipe | 2h |

---

## 📅 Calendrier Détaillé

### 🗓️ Semaine 1 (15-21 Juillet 2026)

```mermaid
gantt
    title Sprint 1 - Semaine 1 (Fondations)
    dateFormat  YYYY-MM-DD
    todayMarker stroke-width:2px,stroke:#FF5733,stroke-dasharray: 5 5
    
    section 🛠️ Setup & Préparation
    T-001 - Env Rust          :a1, 2026-07-15, 2h
    T-002 - Workspace Cargo   :a2, after a1, 2h
    
    section 📦 Modèles & DB
    T-003 - Modèles données    :b1, after a2, 4h
    T-004 - Connexion PostgreSQL :b2, after b1, 4h
    
    section 🗃️ Service Git
    T-005 - CRUD Repositories  :c1, after b2, 8h
    T-006 - Tests unitaires    :c2, after c1, 4h
    
    section 🌐 API HTTP
    T-007 - Handlers Axum      :d1, after c1, 4h
    T-008 - Routes Axum        :d2, after d1, 2h
```

### 🗓️ Semaine 2 (22-28 Juillet 2026)

```mermaid
gantt
    title Sprint 1 - Semaine 2 (Intégration & Tests)
    dateFormat  YYYY-MM-DD
    todayMarker stroke-width:2px,stroke:#FF5733,stroke-dasharray: 5 5
    
    section 🔌 Intégration
    T-009 - gRPC                 :e1, 2026-07-22, 6h
    T-010 - Intégration API GW  :e2, after e1, 4h
    T-011 - GraphQL             :e3, after e2, 4h
    
    section 📝 Fonctionnalités Étendues
    T-012 - Gestion branches    :f1, after e1, 6h
    T-013 - Commits basiques    :f2, after f1, 6h
    
    section 🧪 Tests & Qualité
    T-014 - Tests intégration   :g1, after f2, 4h
    T-015 - Optimisation       :g2, after g1, 4h
    T-016 - Dockerfile          :g3, after g2, 2h
    T-017 - Documentation       :g4, after g3, 4h
    
    section 📊 Clôture
    T-018 - Review & Rétro      :h1, 2026-07-28, 2h
```

---

## 👥 Rôles & Responsabilités

| Rôle | Personnes | Responsabilités |
|------|-----------|-------------------|
| **Product Owner** | Benzo | Priorisation, validation des fonctionnalités, gestion du backlog |
| **Tech Lead / Dev** | Benzo | Architecture, revue de code, intégration |
| **IA Developer** | Mistral Vibe / Copilot | Génération de code Rust, suggestions d'implémentation |
| **QA** | Benzo | Tests, validation qualité, benchmarking |

---

## 🛠️ Workflow de Développement

### 1️⃣ **Préparation (Benzo)**
- [ ] Définir la **user story** clairement
- [ ] Créer une **branche** : `git checkout -b sprint-1/git-[id]`
- [ ] Rédiger un **fichier `TODO.md`** dans le dossier du module
- [ ] Préparer les **tests** (TDD)
- [ ] Créer les **templates** de code (structs, traits)

### 2️⃣ **Génération de Code (IA)**
- [ ] Utiliser le **prompt standard** (voir [TECHNOLOGY-STACK.md#-prompts-ia-recommandés](TECHNOLOGY-STACK.md#-prompts-ia-recommandés))
- [ ] Générer le **code Rust idiomatique**
- [ ] Inclure **documentation rustdoc**
- [ ] Ajouter les **tests unitaires**

### 3️⃣ **Validation (Benzo)**
- [ ] **Compiler** : `cargo build`
- [ ] **Linter** : `cargo clippy -- -D warnings` (doit passer sans warnings)
- [ ] **Formatter** : `cargo fmt`
- [ ] **Tests** : `cargo test` (100% de succès)
- [ ] **Sécurité** : `cargo audit` + `cargo deny check`
- [ ] **Revue manuelle** : Lire chaque ligne, comprendre la logique
- [ ] **Benchmark** : Si performance critique

### 4️⃣ **Intégration**
- [ ] **Commit** : `git commit -m "[Sprint-1] [T-XXX] Description"`
- [ ] **Push** : `git push origin sprint-1/git-[id]`
- [ ] **PR** : Créer une Pull Request avec :
  - Description détaillée
  - Screenshot/logs si applicable
  - Checklist des vérifications
- [ ] **Revue** : Auto-revue (relire à froid après 24h)
- [ ] **Merge** : `git merge --no-ff`

---

## 📁 Structure des Fichiers pour le Git Module

```
modules/
└── git/
    ├── Cargo.toml                 # Dépendances spécifiques au module
    ├── src/
    │   ├── main.rs                # Point d'entrée du module
    │   ├── lib.rs                 # Exposes public API
    │   ├── config.rs              # Configuration du module
    │   ├── error.rs               # Types d'erreurs (GitError)
    │   ├── models.rs              # Structs (Repository, Branch, Commit, etc.)
    │   ├── repository.rs          # Logique repository
    │   │   ├── mod.rs
    │   │   ├── entity.rs          # Entités DB
    │   │   └── queries.rs         # Requêtes SQLx
    │   ├── service.rs             # GitService (CRUD)
    │   │   ├── mod.rs
    │   │   └── tests.rs           # Tests du service
    │   ├── handler.rs             # Handlers Axum
    │   │   ├── mod.rs
    │   │   ├── repository.rs      # Handlers repositories
    │   │   └── tests.rs           # Tests des handlers
    │   ├── routes.rs               # Routes Axum
    │   ├── grpc/                   # gRPC
    │   │   ├── mod.rs
    │   │   ├── git.proto           # Définition Protobuf
    │   │   └── server.rs          # Implémentation serveurs
    │   └── graphql/                # GraphQL
    │       ├── mod.rs
    │       ├── schema.rs          # Schéma GraphQL
    │       └── resolvers.rs        # Résolveurs
    ├── tests/
    │   ├── unit/                  # Tests unitaires
    │   │   ├── repository.rs
    │   │   └── service.rs
    │   └── integration/           # Tests d'intégration
    │       ├── api.rs
    │       └── grpc.rs
    ├── migrations/                # Migrations DB
    │   ├── 20260715000000_create_repositories_table.sql
    │   ├── 20260715000001_create_branches_table.sql
    │   └── 20260715000002_create_commits_table.sql
    ├── Dockerfile                 # Container Docker
    └── README.md                  # Documentation du module
```

---

## 🎯 Définition des Types (à implémenter)

### 📦 Modèles de Données

```rust
// modules/git/src/models.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Représente un repository Git
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Repository {
    /// Identifiant unique du repository
    pub id: Uuid,
    /// Nom du repository
    pub name: String,
    /// Description du repository
    pub description: Option<String>,
    /// Visibilité du repository (privé ou public)
    pub is_private: bool,
    /// ID du propriétaire
    pub owner_id: Uuid,
    /// Branche par défaut
    pub default_branch: String,
    /// Date de création
    pub created_at: DateTime<Utc>,
    /// Date de dernière mise à jour
    pub updated_at: DateTime<Utc>,
}

/// Représente une branche Git
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Branch {
    /// Identifiant unique
    pub id: Uuid,
    /// ID du repository parent
    pub repository_id: Uuid,
    /// Nom de la branche
    pub name: String,
    /// Hash du dernier commit
    pub commit_hash: String,
    /// Date de création
    pub created_at: DateTime<Utc>,
    /// Repository parent (chargé lazy)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<Repository>,
}

/// Représente un commit Git
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Commit {
    /// Identifiant unique
    pub id: Uuid,
    /// ID du repository parent
    pub repository_id: Uuid,
    /// Hash du commit (SHA-1)
    pub hash: String,
    /// Message du commit
    pub message: String,
    /// Nom de l'auteur
    pub author_name: String,
    /// Email de l'auteur
    pub author_email: String,
    /// Date de commit (par l'auteur)
    pub author_date: DateTime<Utc>,
    /// Nom du committer
    pub committer_name: String,
    /// Email du committer
    pub committer_email: String,
    /// Date de commit (par le committer)
    pub committer_date: DateTime<Utc>,
    /// Hash de l'arbre (tree)
    pub tree_hash: String,
    /// Hashes des parents
    pub parent_hashes: Vec<String>,
    /// Date de création dans la DB
    pub created_at: DateTime<Utc>,
    /// Repository parent (chargé lazy)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<Repository>,
}

/// Input pour créer un repository
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRepositoryInput {
    pub name: String,
    pub description: Option<String>,
    pub is_private: bool,
}

/// Input pour créer une branche
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBranchInput {
    pub name: String,
    pub source_branch: Option<String>, // Branche source (pour créer à partir de), default = default_branch
}

/// Réponse paginée
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub page: i32,
    pub page_size: i32,
    pub total: i64,
    pub total_pages: i64,
}
```

### ❌ Gestion des Erreurs

```rust
// modules/git/src/error.rs
use thiserror::Error;

/// Erreurs spécifiques au module Git
#[derive(Error, Debug)]
pub enum GitError {
    /// Erreur de base de données
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    /// Repository non trouvé
    #[error("Repository not found")]
    RepositoryNotFound,
    
    /// Repository déjà existant
    #[error("Repository '{0}' already exists")]
    RepositoryAlreadyExists(String),
    
    /// Branche non trouvée
    #[error("Branch not found")]
    BranchNotFound,
    
    /// Branche déjà existante
    #[error("Branch '{0}' already exists")]
    BranchAlreadyExists(String),
    
    /// Commit non trouvé
    #[error("Commit not found")]
    CommitNotFound,
    
    /// Permission refusée
    #[error("Permission denied")]
    PermissionDenied,
    
    /// Erreur de validation
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    /// Erreur interne
    #[error("Internal server error")]
    InternalError,
}

impl From<GitError> for axum::http::StatusCode {
    fn from(err: GitError) -> Self {
        match err {
            GitError::RepositoryNotFound => StatusCode::NOT_FOUND,
            GitError::RepositoryAlreadyExists(_) => StatusCode::CONFLICT,
            GitError::BranchNotFound => StatusCode::NOT_FOUND,
            GitError::BranchAlreadyExists(_) => StatusCode::CONFLICT,
            GitError::CommitNotFound => StatusCode::NOT_FOUND,
            GitError::PermissionDenied => StatusCode::FORBIDDEN,
            GitError::ValidationError(_) => StatusCode::BAD_REQUEST,
            GitError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            GitError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for GitError {
    fn into_response(self) -> Response {
        let status = StatusCode::from(self);
        let body = Json(json!({
            "error": self.to_string(),
            "status": status.as_u16(),
        }));
        (status, body).into_response()
    }
}
```

### 🗃️ Service Git

```rust
// modules/git/src/service.rs
use crate::error::GitError;
use crate::models::{Branch, CreateBranchInput, CreateRepositoryInput, Repository};
use sqlx::PgPool;
use uuid::Uuid;

/// Service pour gérer les repositories Git
#[derive(Debug, Clone)]
pub struct GitService {
    pool: PgPool,
}

impl GitService {
    /// Crée un nouveau GitService
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    
    /// Crée un nouveau repository
    #[instrument(skip(self, input))]
    pub async fn create_repository(
        &self,
        owner_id: Uuid,
        input: CreateRepositoryInput,
    ) -> Result<Repository, GitError> {
        // Vérifier que le repository n'existe pas déjà
        let existing = sqlx::query_scalar!(
            r#"SELECT 1 FROM repositories WHERE owner_id = $1 AND name = $2"#,
            owner_id,
            input.name.to_lowercase()
        )
        .fetch_optional(&self.pool)
        .await?;
        
        if existing.is_some() {
            return Err(GitError::RepositoryAlreadyExists(input.name));
        }
        
        // Créer le repository
        let repository = sqlx::query_as!(
            Repository,
            r#"
            INSERT INTO repositories (name, description, is_private, owner_id, default_branch)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, name, description, is_private, owner_id, default_branch, created_at, updated_at
            "#,
            input.name.to_lowercase(),
            input.description,
            input.is_private,
            owner_id,
            input.default_branch.unwrap_or_else(|| "main".to_string())
        )
        .fetch_one(&self.pool)
        .await?;
        
        // Créer la branche par défaut
        self.create_branch(
            repository.id,
            CreateBranchInput {
                name: repository.default_branch.clone(),
                source_branch: None,
            }
        ).await?;
        
        Ok(repository)
    }
    
    /// Récupère un repository par ID
    #[instrument(skip(self))]
    pub async fn get_repository(&self, id: Uuid) -> Result<Option<Repository>, GitError> {
        let repository = sqlx::query_as!(
            Repository,
            r#"SELECT * FROM repositories WHERE id = $1"#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(repository)
    }
    
    /// Liste les repositories (avec pagination)
    #[instrument(skip(self))]
    pub async fn list_repositories(
        &self,
        owner_id: Option<Uuid>,
        page: i32,
        page_size: i32,
    ) -> Result<PaginatedResponse<Repository>, GitError> {
        let offset = (page - 1) * page_size;
        
        // Compter le total
        let total: i64 = if let Some(owner) = owner_id {
            sqlx::query_scalar!(
                "SELECT COUNT(*) FROM repositories WHERE owner_id = $1",
                owner
            )
            .fetch_one(&self.pool)
            .await?
        } else {
            sqlx::query_scalar!("SELECT COUNT(*) FROM repositories")
                .fetch_one(&self.pool)
                .await?
        };
        
        // Récupérer les données
        let repositories = if let Some(owner) = owner_id {
            sqlx::query_as!(
                Repository,
                r#"
                SELECT * FROM repositories 
                WHERE owner_id = $1 
                ORDER BY created_at DESC 
                LIMIT $2 OFFSET $3
                "#,
                owner,
                page_size,
                offset
            )
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query_as!(
                Repository,
                r#"
                SELECT * FROM repositories 
                ORDER BY created_at DESC 
                LIMIT $1 OFFSET $2
                "#,
                page_size,
                offset
            )
            .fetch_all(&self.pool)
            .await?
        };
        
        let total_pages = (total as f64 / page_size as f64).ceil() as i64;
        
        Ok(PaginatedResponse {
            data: repositories,
            page,
            page_size,
            total,
            total_pages,
        })
    }
    
    /// Supprime un repository
    #[instrument(skip(self))]
    pub async fn delete_repository(&self, id: Uuid, owner_id: Uuid) -> Result<(), GitError> {
        // Vérifier que le repository existe et appartient à l'utilisateur
        let exists = sqlx::query_scalar!(
            r#"SELECT 1 FROM repositories WHERE id = $1 AND owner_id = $2"#,
            id,
            owner_id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        if exists.is_none() {
            return Err(GitError::RepositoryNotFound);
        }
        
        // Supprimer le repository (cascade supprimera les branches et commits)
        sqlx::query!("DELETE FROM repositories WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }
    
    /// Crée une nouvelle branche
    #[instrument(skip(self, input))]
    pub async fn create_branch(
        &self,
        repository_id: Uuid,
        input: CreateBranchInput,
    ) -> Result<Branch, GitError> {
        // Vérifier que la branche n'existe pas déjà
        let existing = sqlx::query_scalar!(
            r#"SELECT 1 FROM branches WHERE repository_id = $1 AND name = $2"#,
            repository_id,
            input.name.to_lowercase()
        )
        .fetch_optional(&self.pool)
        .await?;
        
        if existing.is_some() {
            return Err(GitError::BranchAlreadyExists(input.name));
        }
        
        // Récupérer le repository pour vérifier qu'il existe
        let repository = self.get_repository(repository_id).await?;
        if repository.is_none() {
            return Err(GitError::RepositoryNotFound);
        }
        
        // Si source_branch n'est pas spécifié, utiliser la branche par défaut
        let source_branch = input.source_branch
            .unwrap_or_else(|| repository.unwrap().default_branch);
        
        // Récupérer le hash du commit de la source_branch
        let commit_hash = sqlx::query_scalar!(
            r#"SELECT commit_hash FROM branches WHERE repository_id = $1 AND name = $2"#,
            repository_id,
            source_branch
        )
        .fetch_optional(&self.pool)
        .await?;
        
        let commit_hash = commit_hash.ok_or(GitError::BranchNotFound)?;
        
        // Créer la branche
        let branch = sqlx::query_as!(
            Branch,
            r#"
            INSERT INTO branches (repository_id, name, commit_hash)
            VALUES ($1, $2, $3)
            RETURNING id, repository_id, name, commit_hash, created_at
            "#,
            repository_id,
            input.name.to_lowercase(),
            commit_hash
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(branch)
    }
    
    /// Liste les branches d'un repository
    #[instrument(skip(self))]
    pub async fn list_branches(
        &self,
        repository_id: Uuid,
        page: i32,
        page_size: i32,
    ) -> Result<PaginatedResponse<Branch>, GitError> {
        let offset = (page - 1) * page_size;
        
        // Vérifier que le repository existe
        let repository = self.get_repository(repository_id).await?;
        if repository.is_none() {
            return Err(GitError::RepositoryNotFound);
        }
        
        // Compter le total
        let total: i64 = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM branches WHERE repository_id = $1",
            repository_id
        )
        .fetch_one(&self.pool)
        .await?;
        
        // Récupérer les données
        let branches = sqlx::query_as!(
            Branch,
            r#"
            SELECT * FROM branches 
            WHERE repository_id = $1 
            ORDER BY created_at DESC 
            LIMIT $2 OFFSET $3
            "#,
            repository_id,
            page_size,
            offset
        )
        .fetch_all(&self.pool)
        .await?;
        
        let total_pages = (total as f64 / page_size as f64).ceil() as i64;
        
        Ok(PaginatedResponse {
            data: branches,
            page,
            page_size,
            total,
            total_pages,
        })
    }
}
```

### 🌐 Handlers Axum

```rust
// modules/git/src/handler/repository.rs
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json, Router,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::error::GitError;
use crate::models::{CreateRepositoryInput, PaginatedResponse, Repository};
use crate::service::GitService;

/// Query parameters pour la pagination
#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

impl Default for PaginationQuery {
    fn default() -> Self {
        Self {
            page: Some(1),
            page_size: Some(20),
        }
    }
}

/// Crée un nouveau repository
#[instrument(skip(state, input))]
pub async fn create_repository(
    State(service): State<GitService>,
    owner_id: Uuid, // Extrait du JWT (via middleware d'auth)
    Json(input): Json<CreateRepositoryInput>,
) -> Result<Json<Repository>, GitError> {
    let repository = service
        .create_repository(owner_id, input.into_inner())
        .await?;
    Ok(Json(repository))
}

/// Récupère un repository par ID
#[instrument(skip(state))]
pub async fn get_repository(
    State(service): State<GitService>,
    Path(id): Path<Uuid>,
) -> Result<Json<Repository>, GitError> {
    let repository = service
        .get_repository(id)
        .await?
        .ok_or(GitError::RepositoryNotFound)?;
    Ok(Json(repository))
}

/// Liste les repositories
#[instrument(skip(state))]
pub async fn list_repositories(
    State(service): State<GitService>,
    owner_id: Option<Uuid>, // Extrait du JWT (optionnel)
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<PaginatedResponse<Repository>>, GitError> {
    let page = pagination.page.unwrap_or(1);
    let page_size = pagination.page_size.unwrap_or(20);
    
    let response = service
        .list_repositories(owner_id, page, page_size)
        .await?;
    Ok(Json(response))
}

/// Supprime un repository
#[instrument(skip(state))]
pub async fn delete_repository(
    State(service): State<GitService>,
    Path(id): Path<Uuid>,
    owner_id: Uuid, // Extrait du JWT
) -> Result<Json<()>, GitError> {
    service.delete_repository(id, owner_id).await?;
    Ok(Json(()))
}

/// Crée une nouvelle branche
#[instrument(skip(state, input))]
pub async fn create_branch(
    State(service): State<GitService>,
    Path(repository_id): Path<Uuid>,
    Json(input): Json<CreateBranchInput>,
) -> Result<Json<Branch>, GitError> {
    let branch = service
        .create_branch(repository_id.into_inner(), input.into_inner())
        .await?;
    Ok(Json(branch))
}

/// Liste les branches d'un repository
#[instrument(skip(state))]
pub async fn list_branches(
    State(service): State<GitService>,
    Path(repository_id): Path<Uuid>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<PaginatedResponse<Branch>>, GitError> {
    let page = pagination.page.unwrap_or(1);
    let page_size = pagination.page_size.unwrap_or(20);
    
    let response = service
        .list_branches(repository_id.into_inner(), page, page_size)
        .await?;
    Ok(Json(response))
}

/// Crée un router pour les repositories
pub fn repository_router(service: GitService) -> Router {
    Router::new()
        .route("/", axum::routing::post(create_repository))
        .route("/", axum::routing::get(list_repositories))
        .route("/:id", axum::routing::get(get_repository))
        .route("/:id", axum::routing::delete(delete_repository))
        .route("/:id/branches", axum::routing::post(create_branch))
        .route("/:id/branches", axum::routing::get(list_branches))
        .with_state(service)
}
```

### 🔌 Routes Axum

```rust
// modules/git/src/routes.rs
use axum::Router;

use crate::handler::repository::{repository_router, RepositoryService};
use crate::service::GitService;

/// Crée le router principal du module Git
pub fn create_router(pool: sqlx::PgPool) -> Router {
    let service = GitService::new(pool);
    
    Router::new()
        .nest("/repositories", repository_router(service))
        // Ajouter d'autres routes ici (commits, pull requests, etc.)
}
```

### 📡 gRPC (Protobuf + Tonic)

**Fichier `modules/git/proto/git.proto` :**
```protobuf
syntax = "proto3";

package tardigrade.git.v1;

option rust_package = "tardigrade.git.v1";

// Repository represents a Git repository
message Repository {
  string id = 1;
  string name = 2;
  string description = 3;
  bool is_private = 4;
  string owner_id = 5;
  string default_branch = 6;
  int64 created_at = 7;
  int64 updated_at = 8;
}

// Branch represents a Git branch
message Branch {
  string id = 1;
  string repository_id = 2;
  string name = 3;
  string commit_hash = 4;
  int64 created_at = 5;
}

// CreateRepositoryRequest represents a request to create a repository
message CreateRepositoryRequest {
  string name = 1;
  string description = 2;
  bool is_private = 3;
  string default_branch = 4;
}

// GetRepositoryRequest represents a request to get a repository
message GetRepositoryRequest {
  string id = 1;
}

// ListRepositoriesRequest represents a request to list repositories
message ListRepositoriesRequest {
  string owner_id = 1;
  int32 page = 2;
  int32 page_size = 3;
}

// ListRepositoriesResponse represents a response to list repositories
message ListRepositoriesResponse {
  repeated Repository repositories = 1;
  int32 page = 2;
  int32 page_size = 3;
  int64 total = 4;
  int64 total_pages = 5;
}

// CreateBranchRequest represents a request to create a branch
message CreateBranchRequest {
  string repository_id = 1;
  string name = 2;
  string source_branch = 3;
}

// ListBranchesRequest represents a request to list branches
message ListBranchesRequest {
  string repository_id = 1;
  int32 page = 2;
  int32 page_size = 3;
}

// ListBranchesResponse represents a response to list branches
message ListBranchesResponse {
  repeated Branch branches = 1;
  int32 page = 2;
  int32 page_size = 3;
  int64 total = 4;
  int64 total_pages = 5;
}

// GitService defines the gRPC service for Git operations
service GitService {
  // Repository operations
  rpc CreateRepository (CreateRepositoryRequest) returns (Repository);
  rpc GetRepository (GetRepositoryRequest) returns (Repository);
  rpc ListRepositories (ListRepositoriesRequest) returns (ListRepositoriesResponse);
  rpc DeleteRepository (GetRepositoryRequest) returns (google.protobuf.Empty);
  
  // Branch operations
  rpc CreateBranch (CreateBranchRequest) returns (Branch);
  rpc ListBranches (ListBranchesRequest) returns (ListBranchesResponse);
}
```

**Implémentation du serveur gRPC :**
```rust
// modules/git/src/grpc/server.rs
use tonic::{Request, Response, Status};
use uuid::Uuid;

use crate::error::GitError;
use crate::models::{Branch, CreateBranchInput, CreateRepositoryInput, Repository};
use crate::service::GitService;

// Inclure le code généré par tonic-build
pub mod git {
    tonic::include_proto!("git");
}

use git::{GitServiceServer, *};

/// Implémentation du service gRPC Git
#[derive(Debug, Default)]
pub struct GitGrpcService {
    pub service: GitService,
}

#[tonic::async_trait]
impl GitService for GitGrpcService {
    async fn create_repository(
        &self,
        request: Request<CreateRepositoryRequest>,
    ) -> Result<Response<Repository>, Status> {
        let input = CreateRepositoryInput {
            name: request.into_inner().name,
            description: request.description,
            is_private: request.is_private,
            default_branch: request.default_branch,
        };
        
        // TODO: Extraire owner_id du contexte (via auth)
        let owner_id = Uuid::nil(); // À remplacer
        
        let repository = self
            .service
            .create_repository(owner_id, input)
            .await
            .map_err(|e| Status::from_error(Box::new(e)))?;
        
        Ok(Response::new(repository.into()))
    }
    
    async fn get_repository(
        &self,
        request: Request<GetRepositoryRequest>,
    ) -> Result<Response<Repository>, Status> {
        let id = Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| Status::invalid_argument("Invalid repository ID"))?;
        
        let repository = self
            .service
            .get_repository(id)
            .await
            .map_err(|e| Status::from_error(Box::new(e)))?
            .ok_or_else(|| Status::not_found("Repository not found"))?;
        
        Ok(Response::new(repository.into()))
    }
    
    async fn list_repositories(
        &self,
        request: Request<ListRepositoriesRequest>,
    ) -> Result<Response<ListRepositoriesResponse>, Status> {
        let req = request.into_inner();
        let owner_id = req.owner_id.map(|s| Uuid::parse_str(&s).unwrap_or(Uuid::nil()));
        let page = req.page;
        let page_size = req.page_size;
        
        let response = self
            .service
            .list_repositories(owner_id, page, page_size)
            .await
            .map_err(|e| Status::from_error(Box::new(e)))?;
        
        Ok(Response::new(ListRepositoriesResponse {
            repositories: response.data.into_iter().map(|r| r.into()).collect(),
            page: response.page,
            page_size: response.page_size,
            total: response.total,
            total_pages: response.total_pages,
        }))
    }
    
    async fn delete_repository(
        &self,
        request: Request<GetRepositoryRequest>,
    ) -> Result<Response<()>, Status> {
        let id = Uuid::parse_str(&request.into_inner().id)
            .map_err(|_| Status::invalid_argument("Invalid repository ID"))?;
        
        // TODO: Extraire owner_id du contexte
        let owner_id = Uuid::nil(); // À remplacer
        
        self.service
            .delete_repository(id, owner_id)
            .await
            .map_err(|e| Status::from_error(Box::new(e)))?;
        
        Ok(Response::new(()))
    }
    
    async fn create_branch(
        &self,
        request: Request<CreateBranchRequest>,
    ) -> Result<Response<Branch>, Status> {
        let req = request.into_inner();
        let repository_id = Uuid::parse_str(&req.repository_id)
            .map_err(|_| Status::invalid_argument("Invalid repository ID"))?;
        
        let input = CreateBranchInput {
            name: req.name,
            source_branch: req.source_branch,
        };
        
        let branch = self
            .service
            .create_branch(repository_id, input)
            .await
            .map_err(|e| Status::from_error(Box::new(e)))?;
        
        Ok(Response::new(branch.into()))
    }
    
    async fn list_branches(
        &self,
        request: Request<ListBranchesRequest>,
    ) -> Result<Response<ListBranchesResponse>, Status> {
        let req = request.into_inner();
        let repository_id = Uuid::parse_str(&req.repository_id)
            .map_err(|_| Status::invalid_argument("Invalid repository ID"))?;
        
        let response = self
            .service
            .list_branches(repository_id, req.page, req.page_size)
            .await
            .map_err(|e| Status::from_error(Box::new(e)))?;
        
        Ok(Response::new(ListBranchesResponse {
            branches: response.data.into_iter().map(|b| b.into()).collect(),
            page: response.page,
            page_size: response.page_size,
            total: response.total,
            total_pages: response.total_pages,
        }))
    }
}

// Conversion entre nos modèles et ceux de Protobuf
impl From<Repository> for git::Repository {
    fn from(repo: Repository) -> Self {
        Self {
            id: repo.id.to_string(),
            name: repo.name,
            description: repo.description.unwrap_or_default(),
            is_private: repo.is_private,
            owner_id: repo.owner_id.to_string(),
            default_branch: repo.default_branch,
            created_at: repo.created_at.timestamp(),
            updated_at: repo.updated_at.timestamp(),
        }
    }
}

impl From<Branch> for git::Branch {
    fn from(branch: Branch) -> Self {
        Self {
            id: branch.id.to_string(),
            repository_id: branch.repository_id.to_string(),
            name: branch.name,
            commit_hash: branch.commit_hash,
            created_at: branch.created_at.timestamp(),
        }
    }
}
```

---

## 🧪 Tests

### Tests Unitaires (GitService)

```rust
// modules/git/src/service/tests.rs
#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;
    use uuid::Uuid;
    
    // Setup de la DB de test
    async fn setup_test_db() -> PgPool {
        let pool = PgPool::connect("postgres://tardigrade:tardigrade@localhost:5432/tardigrade_test")
            .await
            .expect("Failed to connect to test database");
        
        // Créer les tables
        sqlx::migrate!("../migrations")
            .run(&pool)
            .await
            .expect("Failed to migrate test database");
        
        pool
    }
    
    // Cleanup de la DB de test
    async fn cleanup_test_db(pool: &PgPool) {
        sqlx::query!("DELETE FROM branches")
            .execute(pool)
            .await
            .expect("Failed to clean branches");
        sqlx::query!("DELETE FROM repositories")
            .execute(pool)
            .await
            .expect("Failed to clean repositories");
    }
    
    #[tokio::test]
    async fn test_create_repository() {
        let pool = setup_test_db().await;
        let service = GitService::new(pool.clone());
        
        let owner_id = Uuid::new_v4();
        let input = CreateRepositoryInput {
            name: "test-repo".to_string(),
            description: Some("A test repository".to_string()),
            is_private: false,
        };
        
        let repository = service
            .create_repository(owner_id, input)
            .await
            .expect("Failed to create repository");
        
        assert_eq!(repository.name, "test-repo");
        assert_eq!(repository.description, Some("A test repository".to_string()));
        assert_eq!(repository.is_private, false);
        assert_eq!(repository.owner_id, owner_id);
        assert_eq!(repository.default_branch, "main");
        
        cleanup_test_db(&pool).await;
    }
    
    #[tokio::test]
    async fn test_create_repository_duplicate() {
        let pool = setup_test_db().await;
        let service = GitService::new(pool.clone());
        
        let owner_id = Uuid::new_v4();
        let input = CreateRepositoryInput {
            name: "duplicate-repo".to_string(),
            description: None,
            is_private: false,
        };
        
        // Premier repository
        service
            .create_repository(owner_id, input.clone())
            .await
            .expect("Failed to create first repository");
        
        // Deuxième repository avec le même nom
        let result = service.create_repository(owner_id, input).await;
        
        assert!(matches!(result, Err(GitError::RepositoryAlreadyExists(_))));
        
        cleanup_test_db(&pool).await;
    }
    
    #[tokio::test]
    async fn test_get_repository() {
        let pool = setup_test_db().await;
        let service = GitService::new(pool.clone());
        
        let owner_id = Uuid::new_v4();
        let input = CreateRepositoryInput {
            name: "get-test-repo".to_string(),
            description: None,
            is_private: true,
        };
        
        let created = service
            .create_repository(owner_id, input)
            .await
            .expect("Failed to create repository");
        
        let retrieved = service
            .get_repository(created.id)
            .await
            .expect("Failed to get repository")
            .expect("Repository not found");
        
        assert_eq!(retrieved.id, created.id);
        assert_eq!(retrieved.name, created.name);
        assert_eq!(retrieved.is_private, true);
        
        cleanup_test_db(&pool).await;
    }
    
    #[tokio::test]
    async fn test_list_repositories() {
        let pool = setup_test_db().await;
        let service = GitService::new(pool.clone());
        
        let owner_id = Uuid::new_v4();
        
        // Créer 3 repositories
        for i in 0..3 {
            let input = CreateRepositoryInput {
                name: format!("repo-{}", i),
                description: None,
                is_private: false,
            };
            service
                .create_repository(owner_id, input)
                .await
                .expect("Failed to create repository");
        }
        
        // Lister les repositories
        let response = service
            .list_repositories(Some(owner_id), 1, 10)
            .await
            .expect("Failed to list repositories");
        
        assert_eq!(response.data.len(), 3);
        assert_eq!(response.total, 3);
        assert_eq!(response.page, 1);
        assert_eq!(response.page_size, 10);
        assert_eq!(response.total_pages, 1);
        
        cleanup_test_db(&pool).await;
    }
    
    #[tokio::test]
    async fn test_delete_repository() {
        let pool = setup_test_db().await;
        let service = GitService::new(pool.clone());
        
        let owner_id = Uuid::new_v4();
        let input = CreateRepositoryInput {
            name: "delete-test-repo".to_string(),
            description: None,
            is_private: false,
        };
        
        let created = service
            .create_repository(owner_id, input)
            .await
            .expect("Failed to create repository");
        
        // Supprimer le repository
        service
            .delete_repository(created.id, owner_id)
            .await
            .expect("Failed to delete repository");
        
        // Vérifier qu'il n'existe plus
        let result = service.get_repository(created.id).await;
        assert!(matches!(result, Ok(None)));
        
        cleanup_test_db(&pool).await;
    }
    
    #[tokio::test]
    async fn test_create_branch() {
        let pool = setup_test_db().await;
        let service = GitService::new(pool.clone());
        
        let owner_id = Uuid::new_v4();
        let repo_input = CreateRepositoryInput {
            name: "branch-test-repo".to_string(),
            description: None,
            is_private: false,
        };
        
        let repository = service
            .create_repository(owner_id, repo_input)
            .await
            .expect("Failed to create repository");
        
        // La branche par défaut "main" doit exister
        let branches = service
            .list_branches(repository.id, 1, 10)
            .await
            .expect("Failed to list branches");
        
        assert_eq!(branches.data.len(), 1);
        assert_eq!(branches.data[0].name, "main");
        
        // Créer une nouvelle branche
        let input = CreateBranchInput {
            name: "develop".to_string(),
            source_branch: None, // Doit utiliser la branche par défaut
        };
        
        let branch = service
            .create_branch(repository.id, input)
            .await
            .expect("Failed to create branch");
        
        assert_eq!(branch.name, "develop");
        assert_eq!(branch.repository_id, repository.id);
        
        // Vérifier que les 2 branches existent
        let branches = service
            .list_branches(repository.id, 1, 10)
            .await
            .expect("Failed to list branches");
        
        assert_eq!(branches.data.len(), 2);
        
        cleanup_test_db(&pool).await;
    }
}
```

### Tests d'Intégration (Handlers)

```rust
// modules/git/src/handler/tests.rs
#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;
    
    async fn create_test_app() -> Router {
        let pool = PgPool::connect("postgres://tardigrade:tardigrade@localhost:5432/tardigrade_test")
            .await
            .expect("Failed to connect to test database");
        
        // Appliquer les migrations
        sqlx::migrate!("../migrations")
            .run(&pool)
            .await
            .expect("Failed to migrate test database");
        
        // Créer le router
        super::super::routes::create_router(pool)
    }
    
    #[tokio::test]
    async fn test_create_repository_handler() {
        let app = create_test_app().await;
        
        let owner_id = Uuid::new_v4();
        let input = CreateRepositoryInput {
            name: "handler-test-repo".to_string(),
            description: Some("Test via handler".to_string()),
            is_private: false,
        };
        
        let request = Request::builder()
            .method("POST")
            .uri("/repositories")
            .header("Content-Type", "application/json")
            .header("X-User-ID", owner_id.to_string()) // Simule l'auth middleware
            .body(Body::from(serde_json::to_vec(&input).unwrap()))
            .unwrap();
        
        let response = app.oneshot(request).await.unwrap();
        
        assert_eq!(response.status(), StatusCode::CREATED);
        
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let repository: Repository = serde_json::from_slice(&body).unwrap();
        
        assert_eq!(repository.name, "handler-test-repo");
        assert_eq!(repository.description, Some("Test via handler".to_string()));
        assert_eq!(repository.owner_id, owner_id);
    }
    
    #[tokio::test]
    async fn test_get_repository_handler() {
        let app = create_test_app().await;
        let pool = PgPool::connect("postgres://tardigrade:tardigrade@localhost:5432/tardigrade_test")
            .await
            .unwrap();
        
        let service = GitService::new(pool);
        let owner_id = Uuid::new_v4();
        let input = CreateRepositoryInput {
            name: "get-handler-test-repo".to_string(),
            description: None,
            is_private: false,
        };
        
        let repository = service
            .create_repository(owner_id, input)
            .await
            .unwrap();
        
        let request = Request::builder()
            .method("GET")
            .uri(&format!("/repositories/{}", repository.id))
            .body(Body::empty())
            .unwrap();
        
        let response = app.oneshot(request).await.unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
        
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let retrieved: Repository = serde_json::from_slice(&body).unwrap();
        
        assert_eq!(retrieved.id, repository.id);
        assert_eq!(retrieved.name, repository.name);
    }
    
    #[tokio::test]
    async fn test_list_repositories_handler() {
        let app = create_test_app().await;
        
        let request = Request::builder()
            .method("GET")
            .uri("/repositories")
            .body(Body::empty())
            .unwrap();
        
        let response = app.oneshot(request).await.unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
        
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let response: PaginatedResponse<Repository> = serde_json::from_slice(&body).unwrap();
        
        assert!(response.data.is_empty()); // Pas de repositories au début
    }
}
```

---

## 📦 Dockerfile

```dockerfile
# modules/git/Dockerfile

# Build stage
FROM rust:1.70-slim AS builder

# Installer les dépendances de build
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copier le code source
WORKDIR /usr/src/tardigrade-git
COPY . .

# Build en release
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

# Installer les dépendances runtime
RUN apt-get update && apt-get install -y \
    libssl1.1 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copier le binaire
COPY --from=builder /usr/src/tardigrade-git/target/release/tardigrade-git /usr/local/bin/

# Configurer l'utilisateur
RUN useradd -m tardigrade && \
    chown -R tardigrade:tardigrade /usr/local/bin/tardigrade-git
USER tardigrade

# Exposer le port
EXPOSE 3001

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3001/health || exit 1

# Point d'entrée
ENTRYPOINT ["/usr/local/bin/tardigrade-git"]
```

---

## 📝 Documentation du Module

### `modules/git/README.md`

```markdown
# 📦 Git Module

> **Module de gestion des repositories Git pour Tardigrade-CI**

Ce module offre une API pour gérer les repositories Git, branches, commits, pull requests et issues.

## 🚀 Fonctionnalités

### MVP (v0.1.0)
- ✅ CRUD des repositories
- ✅ Gestion des branches (création, liste)
- ✅ Commits basiques
- ✅ API REST (Axum)
- ✅ API gRPC (Tonic)
- ✅ Intégration GraphQL (via API Gateway)

### Futur
- [ ] Pull Requests completes
- [ ] Issues avancées
- [ ] Webhooks Git
- [ ] Search dans le code
- [ ] Git LFS

## 📡 API

### REST (Axum)

| Méthode | Endpoint | Description |
|---------|----------|-------------|
| `POST` | `/repositories` | Crée un nouveau repository |
| `GET` | `/repositories` | Liste les repositories |
| `GET` | `/repositories/:id` | Récupère un repository |
| `DELETE` | `/repositories/:id` | Supprime un repository |
| `POST` | `/repositories/:id/branches` | Crée une nouvelle branche |
| `GET` | `/repositories/:id/branches` | Liste les branches |

### gRPC

Voir [git.proto](proto/git.proto) pour la définition complète.

### GraphQL

Voir l'[API Gateway](../../api/) pour le schéma GraphQL.

## 🛠️ Configuration

### Variables d'Environnement

| Variable | Description | Défaut | Requise |
|----------|-------------|--------|---------|
| `DATABASE_URL` | URL de connexion PostgreSQL | - | ✅ |
| `REDIS_URL` | URL de connexion Redis | - | ✅ |
| `NATS_URL` | URL de connexion NATS | - | ✅ |
| `LOG_LEVEL` | Niveau de logging (debug, info, warn, error) | `info` | ❌ |
| `PORT` | Port du serveur | `3001` | ❌ |

### Exemple de Configuration

```bash
# .env
export DATABASE_URL="postgres://tardigrade:tardigrade@localhost:5432/tardigrade"
export REDIS_URL="redis://localhost:6379"
export NATS_URL="nats://localhost:4222"
export LOG_LEVEL="debug"
export PORT="3001"
```

## 🏗️ Architecture

```
┌─────────────────────────────────────────┐
│              Git Module                  │
├─────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐      │
│  │  Handlers   │  │   Routes    │      │
│  │  (Axum)    │  │  (Axum)     │      │
│  └──────┬──────┘  └──────┬──────┘      │
│         │                │              │
│         └────────────────┘              │
│                  ▼                       │
│  ┌─────────────────────────────────────┐│
│  │           GitService                ││
│  │  (Logique métier)                   ││
│  └─────────────┬─────────────────────┘│
│                │                         │
│         ┌──────▼──────┐                 │
│         ▼             ▼                 │
│  ┌───────────┐   ┌───────────┐          │
│  │ PostgreSQL│   │   Redis   │          │
│  └───────────┘   └───────────┘          │
└─────────────────────────────────────────┘
```

## 🚀 Déploiement

### Local (Docker Compose)

```bash
# Dans la racine du projet
docker-compose up -d tardigrade-git
```

### Kubernetes

```bash
# Appliquer les manifests Kubernetes
kubectl apply -f k8s/overlays/dev/
```

## 🧪 Tests

### Tests Unitaires

```bash
cargo test --package tardigrade-git --lib
```

### Tests d'Intégration

```bash
cargo test --package tardigrade-git --test integration
```

### Tous les Tests

```bash
cargo test --package tardigrade-git
```

### Avec Coverage

```bash
cargo tarpaulin --package tardigrade-git
```

## 📦 Dépendances

Voir [Cargo.toml](Cargo.toml) pour la liste complète.

## 🤝 Contribution

1. Fork le projet
2. Crée une branche (`git checkout -b feature/git-[description]`)
3. Commit tes changements (`git commit -m 'Add [feature]'`)
4. Push vers la branche (`git push origin feature/git-[description]`)
5. Ouvre une Pull Request

## 📄 License

AGPL-3.0 - Voir [LICENSE](../../LICENSE) pour plus de détails.
```

---

## 📊 Suivi du Sprint

### 📈 Burndown Chart (À mettre à jour quotidiennement)

```mermaid
gantt
    title Sprint 1 - Burndown Chart
    dateFormat  YYYY-MM-DD
    todayMarker stroke-width:2px,stroke:#FF5733,stroke-dasharray: 5 5
    
    section Points Restants
    Points initiaux          :a1, 2026-07-15, 38d
    J1                      :after a1, 38d
    J2                      :after a1, 38d
    J3                      :after a1, 38d
    J4                      :after a1, 38d
    J5                      :after a1, 38d
    J6                      :after a1, 38d
    J7                      :after a1, 38d
    J8                      :after a1, 38d
    J9                      :after a1, 38d
    J10                     :after a1, 38d
```

**Total Points :** 38 (US-001:5 + US-002:8 + US-003:3 + US-004:8 + US-005:5 + US-006:5 + Tâches: 4)

### 📋 Journal Quotidien

| Date | Points Restants | Tâches Terminées | Blocages | Notes |
|------|------------------|------------------|----------|-------|
| 2026-07-15 | 38 | Aucune | Aucun | Début du sprint |
| 2026-07-16 | | | | |
| 2026-07-17 | | | | |
| 2026-07-18 | | | | |
| 2026-07-19 | | | | |
| 2026-07-20 | | | | |
| 2026-07-21 | | | | |
| 2026-07-22 | | | | |
| 2026-07-23 | | | | |
| 2026-07-24 | | | | |
| 2026-07-25 | | | | |
| 2026-07-26 | | | | |
| 2026-07-27 | | | | |
| 2026-07-28 | | | | Clôture du sprint |

---

## ✅ Critères de Réussite du Sprint

- [ ] Toutes les **user stories** du sprint sont implémentées
- [ ] **100% des tests** passent
- [ ] **0 warnings** avec `cargo clippy -- -D warnings`
- [ ] **Documentation** complète (rustdoc + README)
- [ ] **Dockerfile** fonctionnel
- [ ] **Intégration** avec l'API Gateway
- [ ] **Performance** : Temps de réponse < 100ms (P95)
- [ ] **Review de code** : Chaque PR a été revu
- [ ] **Sprint Review** : Démo fonctionnelle
- [ ] **Rétrospective** : Leçons apprises documentées

---

## 🎯 Prochaines Étapes (Sprint 2)

- [ ] **CI Module MVP** (Orchestrateur + Workers)
- [ ] **Intégration Git ↔ CI** (Triggers sur push)
- [ ] **Artifact Registry** (Stockage basique)
- [ ] **Plugin System** (Squelette)

---

**📌 Sprint 1 - Git Module MVP**  
*Prêt à démarrer le 15 Juillet 2026*
