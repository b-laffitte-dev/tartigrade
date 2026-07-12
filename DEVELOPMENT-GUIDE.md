# 🚀 Guide de Développement - Tardigrade-CI

**Version :** 1.0  
**Dernière mise à jour :** 2026-07-12  
**Statut :** ✅ Prêt pour le développement  

---

## 📖 SOMMAIRE

1. [Prérequis](#-prérequis)
2. [Structure du Projet](#-structure-du-projet)
3. [Démarrage Rapide](#-démarrage-rapide)
4. [Développement Backend (Rust)](#-développement-backend-rust)
5. [Développement Frontend (TypeScript)](#-développement-frontend-typescript)
6. [Développement avec Docker](#-développement-avec-docker)
7. [Commands Utiles](#-commands-utiles)
8. [Architecture](#-architecture)
9. [Bonnes Pratiques](#-bonnes-pratiques)
10. [Résolution des Problèmes](#-résolution-des-problèmes)

---

## 📋 PREREQUIS

### Outils Nécessaires

| Outil | Version | Installation | Vérification |
|-------|---------|--------------|--------------|
| **Rust** | 1.70+ | [rustup.rs](https://rustup.rs/) | `rustc --version` |
| **Cargo** | 1.70+ | Avec Rust | `cargo --version` |
| **Node.js** | 18.x | [nodejs.org](https://nodejs.org/) | `node --version` |
| **npm / pnpm** | Latest | Avec Node.js | `npm --version` |
| **Docker** | 20.x | [docker.com](https://docs.docker.com/get-docker/) | `docker --version` |
| **Docker Compose** | 2.x | Avec Docker | `docker-compose --version` |
| **PostgreSQL** | 15.x | [postgresql.org](https://www.postgresql.org/) | `psql --version` |

### Installation Rapide (macOS/Linux)

```bash
# Installer Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Installer Node.js (via nvm recommandé)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.5/install.sh | bash
nvm install 18
nvm use 18

# Installer Docker
# Suivre les instructions sur https://docs.docker.com/get-docker/

# Installer PostgreSQL (macOS)
brew install postgresql@15
brew services start postgresql@15
```

---

## 🏗️ STRUCTURE DU PROJET

```
tardigrade-ci/
├── modules/                    # 📦 Modules Backend (100% Rust)
│   └── git/                   # Git Module (PREMIER MODULE)
│       ├── Cargo.toml         # Dépendances du module
│       ├── config.toml       # Configuration par défaut
│       ├── Dockerfile        # Build Docker pour le module
│       └── src/              # Code source Rust
│           ├── main.rs       # Point d'entrée
│           ├── lib.rs        # Exports publics
│           ├── config.rs     # Configuration
│           ├── error.rs      # Gestion des erreurs
│           ├── models.rs     # Modèles de données
│           ├── db.rs         # Connexion DB + migrations
│           ├── service.rs    # Logique métier
│           ├── handler.rs    # Handlers Axum
│           └── routes.rs     # Définition des routes
│
├── crates/                     # 📦 Bibliothèques Rust partagées
│   └── common/               # Types et utilitaires communs
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── error.rs
│           ├── config.rs
│           └── models.rs
│
├── ui/                         # 🎨 Frontend (TypeScript + React)
│   └── git/                   # UI Git Module
│       ├── package.json      # Dépendances npm
│       ├── tsconfig.json     # Configuration TypeScript
│       ├── vite.config.ts    # Configuration Vite
│       ├── index.html
│       ├── Dockerfile        # Build Docker pour l'UI
│       ├── nginx.conf        # Configuration Nginx
│       └── src/              # Code source TypeScript
│           ├── main.tsx      # Point d'entrée
│           ├── App.tsx       # Composant principal
│           ├── types/        # Types TypeScript
│           ├── services/     # Services API
│           ├── hooks/        # Hooks React
│           ├── components/   # Composants React
│           │   ├── common/   # Composants génériques
│           │   └── git/      # Composants Git
│           ├── pages/        # Pages
│           ├── styles/       # Styles CSS
│           └── utils/        # Utilitaires
│
├── docker/                    # 🐳 Configuration Docker
│   └── docker-compose.yml    # Environnement de dev complet
│
├── Cargo.toml                 # 📦 Workspace Cargo (tous les modules)
├── rust-toolchain             # Version de Rust
├── .gitignore
├── PLAN-REVISE.md            # Plan révisé
└── README.md
```

---

## ⚡ DEMARRAGE RAPIDE

### Option 1 : Développement Local (Recommandé pour commencer)

```bash
# 1. Cloner le projet (déjà fait)
# git clone ...

# 2. Démarrer PostgreSQL
# macOS:
brew services start postgresql@15
# ou manuellement:
# pg_ctl -D /opt/homebrew/var/postgres@15 start

# Linux:
sudo service postgresql start

# 3. Créer la base de données
createdb tardigrade_git

# 4. Backend : Builder et démarrer
cd modules/git
cargo build
cargo run
# Le serveur démarre sur http://localhost:3001

# 5. Frontend : Installer et démarrer
cd ui/git
npm install  # ou pnpm install
npm run dev
# Le serveur dev démarre sur http://localhost:5173
```

### Option 2 : Développement avec Docker Compose

```bash
# 1. Démarrer tous les services (PostgreSQL + Backend + Frontend)
docker-compose -f docker/docker-compose.yml up -d

# 2. Voir les logs
docker-compose -f docker/docker-compose.yml logs -f

# 3. Accéder aux services
# - Backend : http://localhost:3001/api/health
# - Frontend : http://localhost:8080
# - PostgreSQL : postgres://postgres:postgres@localhost:5432/tardigrade_git

# 4. Arrêter les services
docker-compose -f docker/docker-compose.yml down
```

### Option 3 : Backend seul avec Docker

```bash
# 1. Démarrer PostgreSQL
docker run --name tardigrade-postgres -e POSTGRES_PASSWORD=postgres -p 5432:5432 -d postgres:15-alpine

# 2. Créer la base de données
docker exec -it tardigrade-postgres createdb -U postgres tardigrade_git

# 3. Builder et démarrer le backend
cd modules/git
cargo run
```

---

## 🦀 DEVELOPPEMENT BACKEND (RUST)

### Workspace Cargo

Le projet utilise un **workspace Cargo** pour gérer plusieurs crates :
- `tardigrade-git` : Module Git
- `tardigrade_common` : Bibliothèques partagées

```bash
# Builder tout le workspace
cargo build --workspace

# Builder un module spécifique
cd modules/git
cargo build

# Lancer un module
cargo run

# Vérifier la qualité du code
cargo check           # Vérifie la compilation
cargo clippy         # Analyse statique
cargo fmt            # Formatage du code
cargo test           # Exécute les tests
```

### Tests

```bash
# Tests unitaires
cargo test

# Tests avec coverage (nécessite cargo-tarpaulin)
cargo install cargo-tarpaulin
cargo tarpaulin --workspace

# Tests d'intégration (nécessite une DB)
# Voir modules/git/tests/integration/
```

### Ajouter une nouvelle dépendance

1. Ajouter dans `Cargo.toml` du module ou du workspace
2. Exécuter `cargo build` pour télécharger la dépendance
3. Commiter le `Cargo.lock`

```toml
# Exemple : Ajouter serde_json
[dependencies]
serde_json = "1.0"
```

### Configuration

Le module Git utilise une configuration par défaut dans `modules/git/config.toml` :

```toml
[base]
name = "tardigrade-git"
environment = "dev"
port = 3001
database_url = "postgres://postgres:postgres@localhost:5432/tardigrade_git"
```

Pour écraser avec des variables d'environnement :
```bash
# Exemple
TARDIGRADE__ENVIRONMENT=dev TARDIGRADE__PORT=4000 cargo run
```

---

## 🎨 DEVELOPPEMENT FRONTEND (TYPESCRIPT)

### Installation

```bash
cd ui/git

# Installer les dépendances
npm install  # ou pnpm install

# Démarrer le serveur de développement
npm run dev

# Builder pour la production
npm run build

# Prévisualiser le build
npm run preview
```

### Scripts Disponibles

| Script | Description |
|--------|-------------|
| `npm run dev` | Démarre Vite en mode développement |
| `npm run build` | Build pour la production |
| `npm run preview` | Prévisualise le build |
| `npm run lint` | Exécute ESLint |
| `npm run lint:fix` | Corrige les problèmes ESLint |
| `npm run format` | Formate avec Prettier |
| `npm run test` | Exécute les tests avec Vitest |
| `npm run test:ui` | Vitest en mode UI |
| `npm run test:coverage` | Tests avec coverage |

### Configuration API

Le frontend utilise `VITE_API_BASE_URL` pour la configuration :

```typescript
// Dans vite.config.ts
define: {
  __API_BASE_URL__: JSON.stringify(process.env.VITE_API_BASE_URL || '/api'),
}
```

En développement, Vite proxy les requêtes `/api` vers le backend :

```typescript
// vite.config.ts
server: {
  proxy: {
    '/api': {
      target: 'http://localhost:3001',
      changeOrigin: true,
    },
  },
}
```

### Ajouter une nouvelle dépendance

```bash
npm install <package>           # Installation
npm install --save-dev <package>  # Dépendance de développement
```

### Structure des Composants

```
ui/git/src/components/
├── common/      # Composants génériques (Button, Input, Card...)
└── git/         # Composants spécifiques au module Git
```

---

## 🐳 DEVELOPPEMENT AVEC DOCKER

### Docker Compose

Le fichier `docker/docker-compose.yml` configure :
- PostgreSQL (port 5432)
- Git Module Backend (port 3001)
- UI Frontend (port 8080)

```bash
# Démarrer tous les services
docker-compose -f docker/docker-compose.yml up -d

# Voir les logs en temps réel
docker-compose -f docker/docker-compose.yml logs -f

# Voir les logs d'un service spécifique
docker-compose -f docker/docker-compose.yml logs -f git-module

# Arrêter tous les services
docker-compose -f docker/docker-compose.yml down

# Arrêter et supprimer les volumes
docker-compose -f docker/docker-compose.yml down -v

# Rebuilder les images
docker-compose -f docker/docker-compose.yml build --no-cache

# Redémarrer un service
docker-compose -f docker/docker-compose.yml restart git-module
```

### Docker pour le Backend

```bash
# Builder l'image
docker build -t tardigrade-git -f modules/git/Dockerfile .

# Démarrer le conteneur
docker run -p 3001:3001 --name tardigrade-git tardigrade-git

# Avec variables d'environnement
docker run -p 3001:3001 \
  -e TARDIGRADE__DATABASE_URL=postgres://postgres:postgres@host.docker.internal:5432/tardigrade_git \
  --name tardigrade-git tardigrade-git
```

### Docker pour le Frontend

```bash
# Builder l'image
docker build -t tardigrade-ui -f ui/git/Dockerfile .

# Démarrer le conteneur
docker run -p 8080:80 --name tardigrade-ui tardigrade-ui
```

---

## 💻 COMMANDS UTILES

### Backend (Rust)

```bash
# Compilation
cargo build                    # Build en debug
cargo build --release          # Build en release
cargo build --workspace        # Build tout le workspace

# Qualité du code
cargo check                    # Vérification rapide
cargo clippy                  # Analyse statique
cargo clippy -- -D warnings    # Traite les warnings comme des erreurs
cargo fmt                     # Formater le code
cargo fmt --check             # Vérifier le formatage

# Tests
cargo test                    # Tous les tests
cargo test -- --nocapture     # Affiche les logs des tests
cargo test --doc              # Tests de documentation

# Documentation
cargo doc                     # Génère la documentation
cargo doc --open              # Génère et ouvre la documentation

# Audit de sécurité
cargo audit                   # Vérifie les vulnérabilités

# Info
cargo tree                    # Affiche l'arbre des dépendances
cargo outdated               # Affiche les dépendances obsolètes
```

### Frontend (TypeScript)

```bash
# Développement
npm run dev                   # Démarre le serveur de dev
npm run dev -- --host         # Accessible depuis le réseau

# Build
npm run build                 # Build pour la production
npm run build -- --mode development  # Build en mode dev

# Qualité du code
npm run lint                  # ESLint
npm run lint:fix             # Corrige ESLint
npm run format                # Prettier

# Tests
npm run test                  # Vitest
npm run test -- --watch       # Mode watch
npm run test:ui              # Interface Vitest
npm run test:coverage         # Avec coverage

# Info
npm ls                       # Affiche les dépendances
npm outdated                 # Dependances obsolètes
npm audit                    # Audit de sécurité
```

### PostgreSQL

```bash
# Connexion
psql -U postgres -d tardigrade_git

# Lister les bases de données
\l

# Lister les tables
\dt

# Exécuter une requête
SELECT * FROM repositories LIMIT 10;

# Quitter
\q
```

### Docker

```bash
# Lister les conteneurs
docker ps

# Lister tous les conteneurs (y compris arrêtés)
docker ps -a

# Lister les images
docker images

# Voir les logs d'un conteneur
docker logs <container_name>

# Entrer dans un conteneur
docker exec -it <container_name> sh

# Arrêter un conteneur
docker stop <container_name>

# Supprimer un conteneur
docker rm <container_name>

# Supprimer une image
docker rmi <image_name>

# Nettoyer
# Supprimer les conteneurs arrêtés
docker container prune

# Supprimer les images non utilisées
docker image prune

# Supprimer les volumes non utilisés
docker volume prune

# Tout nettoyer (ATTENTION!)
docker system prune -a --volumes
```

---

## 🏛️ ARCHITECTURE

### Backend (Git Module)

```
modules/git/
├── src/
│   ├── main.rs          # Point d'entrée (configure logging, charge config)
│   ├── lib.rs           # Exports publics
│   ├── config.rs        # Configuration (ModuleConfig, GitConfig)
│   ├── error.rs         # Gestion des erreurs (GitError, IntoResponse)
│   ├── models.rs        # Modèles (Repository, Branch, CreateRepositoryInput...)
│   ├── db.rs            # Base de données (create_pool, init_schema, migrations)
│   ├── service.rs       # Logique métier (RepositoryService, BranchService)
│   ├── handler.rs       # Handlers Axum (create_repository, get_repository...)
│   └── routes.rs        # Définition des routes (create_router_with_config)
├── Cargo.toml
└── config.toml
```

**Flux des requêtes :**
```
HTTP Request
    ↓
routes.rs (Router Axum)
    ↓
handler.rs (Handler)
    ↓
service.rs (Logique métier)
    ↓
db.rs (Requêtes SQL)
    ↓
PostgreSQL
```

### Frontend (Git Module UI)

```
ui/git/src/
├── main.tsx            # Point d'entrée (ReactDOM.render)
├── App.tsx             # Composant principal (routing, layout)
├── types/
│   └── git.ts          # Types TypeScript
├── services/
│   └── gitService.ts   # Appels API (Axios)
├── hooks/
│   ├── useRepositories.ts  # Hook pour la liste des repositories
│   └── useRepository.ts     # Hook pour un repository
├── components/
│   ├── common/         # Composants génériques
│   │   ├── Button.tsx
│   │   ├── Input.tsx
│   │   ├── Card.tsx
│   │   ├── Loading.tsx
│   │   ├── ErrorMessage.tsx
│   │   ├── Pagination.tsx
│   │   ├── Badge.tsx
│   │   └── Modal.tsx
│   └── git/            # Composants Git
│       ├── RepositoryCard.tsx
│       ├── BranchList.tsx
│       └── BranchCard.tsx
├── pages/
│   ├── Dashboard.tsx
│   └── Repository/
│       ├── List.tsx
│       ├── Create.tsx
│       └── Detail.tsx
├── styles/
│   └── global.css      # Styles globaux + Tailwind
└── utils/
    └── formatters.ts   # Fonctions utilitaires
```

**Flux des données :**
```
Page Component
    ↓
Custom Hook (useRepositories, useRepository)
    ↓
GitService (Axios)
    ↓
API REST (http://localhost:3001/api)
    ↓
Backend (Axum)
    ↓
Service Layer
    ↓
Database (PostgreSQL)
```

---

## ✅ BONNES PRATIQUES

### Backend (Rust)

1. **Gestion des erreurs**
   - Utiliser `thiserror` ou `anyhow` pour les erreurs
   - Toujours implémenter `IntoResponse` pour les erreurs HTTP
   - Ne jamais utiliser `unwrap()` en production
   - Utiliser `?` pour propager les erreurs

2. **Documentation**
   - Documenter toutes les fonctions publiques avec `///`
   - Utiliser des exemples dans la documentation
   - Documenter les parameters et les retours

3. **Tests**
   - Écrire des tests unitaires pour chaque fonction
   - Écrire des tests d'intégration pour les endpoints API
   - Viser >80% de couverture
   - Utiliser `cargo tarpaulin` pour mesurer la couverture

4. **Performance**
   - Utiliser `async/await` pour les opérations I/O
   - Éviter de bloquer le thread
   - Utiliser `Arc` pour partager les données entre threads
   - Limiter les allocations inutiles

5. **Sécurité** (pour plus tard)
   - Valider toutes les entrées utilisateur
   - Utiliser des types forts pour éviter les erreurs
   - Ne jamais faire confiance aux données externes

### Frontend (TypeScript)

1. **Typage**
   - Toujours utiliser le typage strict
   - Créer des interfaces pour toutes les données API
   - Utiliser des types génériques quand approprié

2. **Composants**
   - Créer des composants petits et réutilisables
   - Utiliser `React.forwardRef` pour les composants qui acceptent des refs
   - Utiliser `clsx` et `tailwind-merge` pour gérer les classes
   - Documenter les props avec JSDoc

3. **Hooks**
   - Créer des hooks personnalisés pour la logique réutilisable
   - Séparer la logique des composants
   - Utiliser `useCallback` pour mémoriser les fonctions
   - Utiliser `useMemo` pour mémoriser les calculs coûteux

4. **Appels API**
   - Centraliser les appels API dans des services
   - Utiliser Axios pour les requêtes HTTP
   - Toujours gérer les erreurs
   - Utiliser des interceptors pour la gestion globale des erreurs

5. **Tests**
   - Tester les composants avec `@testing-library/react`
   - Tester les hooks personnalisés
   - Tester les utilitaires
   - Viser >70% de couverture

### Docker

1. **Images**
   - Utiliser des images minimales (alpine, slim)
   - Utiliser multi-stage builds pour réduire la taille
   - Ne pas inclure de fichiers inutiles dans l'image

2. **Sécurité**
   - Ne pas exécuter en tant que root dans les conteneurs
   - Utiliser des utilisateurs dédiés
   - Limiter les permissions

3. **Configuration**
   - Utiliser des variables d'environnement pour la configuration
   - Ne pas commiter de secrets dans les images
   - Utiliser des healthchecks

---

## 🚨 RESOLUTION DES PROBLEMES

### Problèmes Courants

#### Backend ne démarre pas

**Symptômes :**
```
Error: Impossible de se connecter à la base de données
```

**Solutions :**
1. Vérifier que PostgreSQL est démarré : `psql -U postgres -l`
2. Vérifier la base de données existe : `createdb tardigrade_git`
3. Vérifier la configuration : `modules/git/config.toml`
4. Tester la connexion manuellement :
   ```bash
   psql -U postgres -d tardigrade_git -c "SELECT 1;"
   ```

#### Frontend ne démarre pas

**Symptômes :**
```
Error: Cannot find module '...'
```

**Solutions :**
1. Exécuter `npm install` dans `ui/git/`
2. Vérifier que Node.js 18+ est installé
3. Supprimer `node_modules/` et `package-lock.json` puis réinstaller

#### Docker Compose échoue

**Symptômes :**
```
Error: no such image: postgres:15-alpine
```

**Solutions :**
1. Exécuter `docker pull postgres:15-alpine`
2. Vérifier que Docker est démarré
3. Vérifier l'espace disque disponible : `docker system df`

#### La connexion entre frontend et backend échoue

**Symptômes :**
```
CORS error / Network error
```

**Solutions :**
1. Vérifier que le backend est démarré : `curl http://localhost:3001/api/health`
2. Vérifier la configuration du proxy dans `vite.config.ts`
3. En développement, utiliser `npm run dev` (Vite configure le proxy)
4. En production, configurer CORS dans le backend

### Debugging

#### Backend (Rust)

```bash
# Logs détaillés
RUST_LOG=debug cargo run

# Backtrace en cas de panic
RUST_BACKTRACE=1 cargo run

# Logs avec timestamp
RUST_LOG=debug,info cargo run
```

#### Frontend (TypeScript)

```bash
# Logs dans le navigateur
# Ouvrir les DevTools (F12) puis l'onglet Console

# Logs détaillés
npm run dev -- --debug
```

#### PostgreSQL

```bash
# Voir les connexions actives
psql -U postgres -c "SELECT * FROM pg_stat_activity;"

# Voir les logs PostgreSQL
# macOS:
tail -f /opt/homebrew/var/log/postgres.log
# Linux:
tail -f /var/log/postgresql/postgresql-15-main.log
```

#### Docker

```bash
# Voir les logs
docker logs <container_name>

# Entrer dans un conteneur pour debugger
docker exec -it <container_name> sh
```

---

## 📚 RESSOURCES

### Documentation Officielle

- [Rust](https://www.rust-lang.org/)
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Axum](https://docs.rs/axum/latest/axum/)
- [SQLx](https://docs.rs/sqlx/latest/sqlx/)
- [Tokio](https://tokio.rs/)
- [React](https://react.dev/)
- [TypeScript](https://www.typescriptlang.org/)
- [Vite](https://vitejs.dev/)
- [Tailwind CSS](https://tailwindcss.com/)
- [Docker](https://docs.docker.com/)
- [PostgreSQL](https://www.postgresql.org/docs/)

### Outils Utiles

- [Rust Playground](https://play.rust-lang.org/) - Tester du code Rust en ligne
- [Regex101](https://regex101.com/) - Tester des expressions régulières
- [JSON Formatter](https://jsonformatter.curiousconcept.com/) - Formater du JSON
- [JWT Decoder](https://jwt.io/) - Décoder des tokens JWT

---

## 🎯 PROCHAINES ETAPES

Maintenant que la structure est en place, voici ce que tu peux faire :

### 1. **Tester le Backend**
```bash
cd modules/git
cargo build
cargo run
# Puis tester :
curl http://localhost:3001/api/health
```

### 2. **Tester le Frontend**
```bash
cd ui/git
npm install
npm run dev
# Puis ouvrir : http://localhost:5173
```

### 3. **Tester Docker Compose**
```bash
docker-compose -f docker/docker-compose.yml up -d
# Puis tester :
# - Backend : http://localhost:3001/api/health
# - Frontend : http://localhost:8080
```

### 4. **Ajouter des fonctionnalités**
- Implémenter les tests unitaires pour le backend
- Ajouter des validations plus strictes
- Implémenter la gestion des commits
- Ajouter des filtres et recherche dans l'UI

---

**✅ Tu es prêt à commencer le développement !**

Pour des questions ou des problèmes, consulte ce guide ou demande-moi directement.

*Happy coding!* 🚀
