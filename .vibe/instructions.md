# Instructions pour Mistral Vibe - Projet Tardigrade-CI

## Contexte
Ce projet est une **plateforme DevOps modulaire open-source** conçue pour apprendre Rust tout en construisant un produit réel.

## Objectifs du Projet
1. **Apprendre Rust** en construisant un vrai projet
2. **Créer une architecture modulaire** extensible
3. **Prioriser la qualité du code** > vitesse de développement
4. **Comprendre les patterns** Rust (ownership, async, error handling)

## Stack Technique
- **Backend**: Rust 100% (Axum, Tokio, SQLx, Tonic, Async-GraphQL)
- **Frontend**: React + TypeScript + Vite + TailwindCSS
- **Base de données**: PostgreSQL
- **Cache**: Redis (optionnel pour le MVP)
- **Conteneurisation**: Docker + Docker Compose

## Structure du Projet
```
tardigrade-ci/
├── modules/                    # Modules Backend (Rust)
│   └── git/                   # Git Module (PREMIER MODULE)
│
├── crates/                     # Bibliothèques Rust partagées
│   └── common/
│
├── ui/                         # Frontend (TypeScript + React)
│   └── git/                   # UI Git Module
│
├── docker/
│   └── docker-compose.yml
│
├── Cargo.toml                 # Workspace Rust
├── rust-toolchain
└── README.md
```

## Règles de Développement

### Backend (Rust)
- **Pas de `unwrap()`** en production (gérer les erreurs proprement)
- **Documentation** avec `///` (rustdoc)
- **Tests** > 80% couverture (cargo tarpaulin)
- **0 Clippy warnings** (`cargo clippy -- -D warnings`)
- **Code formaté** (`cargo fmt`)
- **Gestion des erreurs** avec `thiserror` ou `anyhow`

### Frontend (TypeScript)
- **Typage strict** (`strict: true` dans tsconfig)
- **0 ESLint warnings** (`npm run lint`)
- **Tests** > 70% couverture (vitest)
- **Code formaté** (Prettier)

### Docker
- **Multi-stage builds** pour optimiser les images
- **Healthchecks** pour tous les services
- **Images minimales** (alpine, slim)

## Workflow de Développement

### Backend (Rust)
```
1. Branche : git checkout -b feature/git-[nom]
2. Code : Implémentation avec tests (TDD si possible)
3. Validation :
   - cargo build
   - cargo clippy -- -D warnings
   - cargo fmt
   - cargo test
4. Commit : git commit -m "feat: [description]"
5. Push : git push origin feature/git-[nom]
```

### Frontend (TypeScript)
```
1. Branche : git checkout -b feature/git-ui-[nom]
2. Code : Implémentation avec typage strict
3. Validation :
   - npm run build
   - npm run lint
   - npm run test
4. Commit : git commit -m "feat(ui): [description]"
5. Push : git push origin feature/git-ui-[nom]
```

## Roadmap

### Étape 0 : Fondations (En Cours)
✅ Structure de fichiers modulaire
✅ Workspace Cargo
✅ Configuration Docker Compose
✅ Setup Vite + TypeScript + Tailwind
✅ Dockerfiles backend + frontend

### Étape 1 : Backend Git Module (Fondations)
- Modèles de données (Repository, Branch)
- Connexion PostgreSQL (SQLx)
- Migrations DB
- Service Repository (CRUD)
- Service Branch (CRUD)
- Handlers Axum
- Routes REST
- Tests unitaires
- Tests d'intégration

### Étape 2 : Frontend Git Module (Fondations)
- Types TypeScript
- Service API (Axios)
- Composants UI communs
- Pages (Dashboard, List, Create, Detail)
- Hooks personnalisés
- Tests

### Étape 3 : Intégration & Conteneurisation
- Connecter frontend/backend
- Docker Compose complet
- Tests end-to-end
- Documentation

### Étape 4 : Fonctionnalités Avancées (Plus tard)
- Authentification (JWT)
- gRPC (communication interne)
- GraphQL (API publique)
- CI Module
- Artifact Registry
- Plugin System

## Conseils pour Mistral Vibe

1. **Ne pas faire d'hypothèses** : Toujours demander confirmation si quelque chose n'est pas clair
2. **Proposer des alternatives** : Quand un choix technique est ambigu, proposer plusieurs options
3. **Challenge les décisions** : Questionner les choix pour s'assurer qu'ils sont optimaux
4. **Générer du code idiomatique** : Rust et TypeScript doivent suivre les meilleures pratiques
5. **Inclure des tests** : Toujours inclure des tests unitaires et d'intégration
6. **Documenter** : Ajouter des commentaires et de la documentation
7. **Optimiser progressivement** : Commencer simple, optimiser après

## Questions Fréquentes

### Pourquoi Rust 100% ?
C'est un projet d'apprentissage pour maîtriser Rust. L'objectif est de devenir compétent avec ce langage.

### Pourquoi pas de deadline ?
C'est un projet de loisir. La qualité et l'apprentissage sont plus importants que la vitesse.

### Pourquoi la sécurité est reportée ?
Pour se concentrer sur les fondations d'abord. La sécurité sera ajoutée quand le produit sera plus mature.

### Pourquoi PostgreSQL et pas une autre base de données ?
PostgreSQL est polyvalent, ouvert, bien supporté par SQLx, et suffit pour le MVP.

### Pourquoi Axum et pas Actix-Web ou Rocket ?
Axum est moderne, bien intégré avec Tokio, et recommandé pour les nouvelles applications Rust.
