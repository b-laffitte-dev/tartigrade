# 🛠️ Stack Technique - Tardigrade-CI

**Version :** 1.0  
**Dernière mise à jour :** 2026-06-17  
**Statut :** Validé (Atelier technique 20/06/2026)  
**Auteur :** Benzo + Mistral Vibe

---

## 🎯 Résumé des Choix Techniques

> **Philosophie :** *"Moins de technologies, plus de cohérence, maximum de performance et de sécurité."*

| **Catégorie** | **Choix** | **Alternatives Écartées** | **Justification** |
|--------------|-----------|-------------------------|------------------|
| **Backend** | Rust (100%) | Rust + Go, Go seul | Simplicité, sécurité mémoire, performance |
| **Frontend** | TypeScript + React | Svelte, Vue, Vanilla TS | Écosystème DevOps, productivité |
| **Base de données** | PostgreSQL (instances dédiées) | MySQL, MongoDB, multi-DB | Polyvalence, ACID, écosystème |
| **Cache** | Redis | Memcached | Performance, structures riches |
| **Messagerie** | NATS + JetStream | Kafka, RabbitMQ | Latence ultra-faible, persistance |
| **Stockage Artefacts** | MinIO | Harbor, Nexus, AWS S3 | Auto-hébergement, compatible S3 |
| **Communication Interne** | gRPC | REST, GraphQL | Typage fort, performance |
| **API Publique** | GraphQL | REST, gRPC | Flexibilité, introspection |
| **Containerisation** | Docker | Podman | Standard industrie |
| **Orchestration** | Kubernetes | Docker Compose, Nomad | Scalabilité, résilience |
| **Infrastructure** | Terraform | Ansible, Pulumi | Déclaratif, multi-cloud |
| **Développement** | IA + Supervision Humaine | Équipe Rust dédiée | Productivité, coût réduit |

---

## 📚 Stack Technique Complète

### 🏗️ **Backend (100% Rust)**

#### Framework & Runtime
| Technologie | Version | Rôle | Lien |
|-------------|---------|------|------|
| **Rust** | 1.70+ (stable) | Langage principal | [rust-lang.org](https://www.rust-lang.org/) |
| **Cargo** | 1.70+ | Gestion des dépendances | Intégré |
| **Axum** | 0.7.x | Framework web | [github.com/tokio-rs/axum](https://github.com/tokio-rs/axum) |
| **Tokio** | 1.0.x | Runtime async | [tokio.rs](https://tokio.rs/) |
| **Tonic** | 0.10.x | gRPC | [github.com/hyperium/tonic](https://github.com/hyperium/tonic) |
| **Prost** | 0.12.x | Protobuf | [github.com/tokio-rs/prost](https://github.com/tokio-rs/prost) |
| **Async-GraphQL** | 6.0.x | GraphQL | [github.com/async-graphql/async-graphql](https://github.com/async-graphql/async-graphql) |

#### Base de Données
| Technologie | Version | Rôle | Lien |
|-------------|---------|------|------|
| **PostgreSQL** | 15.x | DB principale | [postgresql.org](https://www.postgresql.org/) |
| **SQLx** | 0.7.x | ORM/Query Builder | [github.com/launchbadge/sqlx](https://github.com/launchbadge/sqlx) |
| **Redis** | 7.x | Cache | [redis.io](https://redis.io/) |
| **TimescaleDB** | Latest | Extension timeseries | [timescale.com](https://www.timescale.com/) |

#### Messagerie
| Technologie | Version | Rôle | Lien |
|-------------|---------|------|------|
| **NATS** | 2.10.x | Message Broker | [nats.io](https://nats.io/) |
| **async-nats** | 0.30.x | Client NATS Rust | [github.com/nats-io/nats.rs](https://github.com/nats-io/nats.rs) |
| **JetStream** | Intégré | Persistance | [docs.nats.io/jetstream](https://docs.nats.io/jetstream) |

#### Stockage
| Technologie | Version | Rôle | Lien |
|-------------|---------|------|------|
| **MinIO** | Latest | Stockage objets | [min.io](https://min.io/) |
| **minio-rs** | Latest | Client MinIO Rust | [github.com/minio/minio-rs](https://github.com/minio/minio-rs) |

#### Sécurité
| Technologie | Version | Rôle | Lien |
|-------------|---------|------|------|
| **jsonwebtoken** | 9.x | JWT | [github.com/Keats/jsonwebtoken](https://github.com/Keats/jsonwebtoken) |
| **bcrypt** | 0.15.x | Hashage mots de passe | [github.com/Keats/bcrypt-rs](https://github.com/Keats/bcrypt-rs) |
| **casbin-rs** | Latest | RBAC | [github.com/casbin/casbin-rs](https://github.com/casbin/casbin-rs) |
| **rustls** | Latest | TLS | [github.com/rustls/rustls](https://github.com/rustls/rustls) |
| **cargo-audit** | Latest | Audit sécurité | [github.com/RustSec/cargo-audit](https://github.com/RustSec/cargo-audit) |
| **cargo-deny** | Latest | Vérification licences | [github.com/EmbarkStudios/cargo-deny](https://github.com/EmbarkStudios/cargo-deny) |

#### Logging & Observabilité
| Technologie | Version | Rôle | Lien |
|-------------|---------|------|------|
| **tracing** | 0.1.x | Logging | [github.com/tokio-rs/tracing](https://github.com/tokio-rs/tracing) |
| **tracing-subscriber** | 0.3.x | Subscriber | [github.com/tokio-rs/tracing-subscriber](https://github.com/tokio-rs/tracing-subscriber) |
| **metrics** | 0.21.x | Métriques | [github.com/metrics-rs/metrics](https://github.com/metrics-rs/metrics) |
| **metrics-exporter-prometheus** | 0.14.x | Export Prometheus | [github.com/metrics-rs/metrics-exporter-prometheus](https://github.com/metrics-rs/metrics-exporter-prometheus) |
| **opentelemetry** | Latest | Tracing distribué | [opentelemetry.io](https://opentelemetry.io/) |

#### Tests & Qualité
| Technologie | Version | Rôle | Lien |
|-------------|---------|------|------|
| **tokio-test** | 0.4.x | Tests async | [github.com/tokio-rs/tokio-test](https://github.com/tokio-rs/tokio-test) |
| **criterion** | 0.5.x | Benchmark | [github.com/bheisler/criterion.rs](https://github.com/bheisler/criterion.rs) |
| **clippy** | 0.1.x | Linter | Intégré |
| **rustfmt** | 1.0.x | Formatter | Intégré |
| **tarpaulin** | Latest | Coverage | [github.com/xd009642/tarpaulin](https://github.com/xd009642/tarpaulin) |

#### Utilitaires
| Technologie | Version | Rôle | Lien |
|-------------|---------|------|------|
| **config** | 0.13.x | Configuration | [github.com/mehcode/config-rs](https://github.com/mehcode/config-rs) |
| **serde** | 1.0.x | Sérialisation | [github.com/serde-rs/serde](https://github.com/serde-rs/serde) |
| **serde_json** | 1.0.x | JSON | [github.com/serde-rs/json](https://github.com/serde-rs/json) |
| **uuid** | 1.4.x | UUID | [github.com/uuid-rs/uuid](https://github.com/uuid-rs/uuid) |
| **chrono** | 0.4.x | Dates/Heures | [github.com/chronotope/chrono](https://github.com/chronotope/chrono) |
| **thiserror** | 1.0.x | Gestion erreurs | [github.com/dtolnay/thiserror](https://github.com/dtolnay/thiserror) |
| **anyhow** | 1.0.x | Erreurs dynamiques | [github.com/dtolnay/anyhow](https://github.com/dtolnay/anyhow) |
| **libloading** | 0.8.x | Chargement dynamique | [github.com/nagisa/rust_libloading](https://github.com/nagisa/rust_libloading) |

---

### 🎨 **Frontend (TypeScript)**

#### Framework & Build
| Technologie | Version | Rôle | Lien |
|-------------|---------|------|------|
| **TypeScript** | 5.x | Langage | [typescriptlang.org](https://www.typescriptlang.org/) |
| **React** | 18.x | Framework | [react.dev](https://react.dev/) |
| **Vite** | 5.x | Build Tool | [vitejs.dev](https://vitejs.dev/) |
| **ESLint** | 8.x | Linter | [eslint.org](https://eslint.org/) |
| **Prettier** | 3.x | Formatter | [prettier.io](https://prettier.io/) |

#### UI & Styling
| Technologie | Version | Rôle | Lien |
|-------------|---------|------|------|
| **TailwindCSS** | 3.x | Utility-first CSS | [tailwindcss.com](https://tailwindcss.com/) |
| **shadcn/ui** | Latest | Composants | [ui.shadcn.com](https://ui.shadcn.com/) |
| **radix-ui** | Latest | Primitives accessibles | [radix-ui.com](https://www.radix-ui.com/) |
| **lucide-react** | Latest | Icônes | [lucide.dev](https://lucide.dev/) |

#### State Management
| Technologie | Version | Rôle | Lien |
|-------------|---------|------|------|
| **Zustand** | 4.x | State Management | [github.com/pmndrs/zustand](https://github.com/pmndrs/zustand) |
| **React Query** | 5.x | Server State | [tanstack.com/query](https://tanstack.com/query) |

#### HTTP & API
| Technologie | Version | Rôle | Lien |
|-------------|---------|------|------|
| **GraphQL Request** | 6.x | Client GraphQL | [github.com/prisma-labs/graphql-request](https://github.com/prisma-labs/graphql-request) |
| **Axios** | 1.x | HTTP Client | [axios-http.com](https://axios-http.com/) |
| **Socket.IO Client** | 4.x | WebSockets | [socket.io](https://socket.io/) |

#### Tests
| Technologie | Version | Rôle | Lien |
|-------------|---------|------|------|
| **Vitest** | 1.x | Tests unitaires | [vitest.dev](https://vitest.dev/) |
| **Testing Library** | 14.x | Tests composants | [testing-library.com](https://testing-library.com/) |
| **MSW** | 2.x | Mock HTTP | [mswjs.io](https://mswjs.io/) |

---

### 🗃️ **Base de Données**

| Instance | Technologie | Port | Volume | Utilisation |
|----------|-------------|------|--------|-------------|
| `tardigrade-postgres` | PostgreSQL 15 | 5432 | `postgres_data` | Core (users, permissions) |
| `tardigrade-postgres-git` | PostgreSQL 15 + TimescaleDB | 5433 | `postgres_git_data` | Git Module |
| `tardigrade-postgres-ci` | PostgreSQL 15 + TimescaleDB | 5434 | `postgres_ci_data` | CI Module |
| `tardigrade-postgres-registry` | PostgreSQL 15 | 5435 | `postgres_registry_data` | Registry Module |
| `tardigrade-redis` | Redis 7 | 6379 | `redis_data` | Cache (tous modules) |

---

### 📡 **Messagerie & Événements**

| Service | Technologie | Ports | Volume | Rôle |
|---------|-------------|-------|--------|------|
| `tardigrade-nats` | NATS 2.10 + JetStream | 4222 (client), 8222 (HTTP) | `nats_data` | Pub/Sub + Persistance |

**Topics/Streams NATS :**
| Stream | Subjects | Description |
|--------|----------|-------------|
| `GIT` | `git.>` | Tous les événements Git |
| `CI` | `ci.>` | Tous les événements CI |
| `REGISTRY` | `registry.>` | Tous les événements Registry |
| `PLUGIN` | `plugin.>` | Événements du Plugin System |

---

### 💾 **Stockage**

| Service | Technologie | Ports | Volume | Rôle |
|---------|-------------|-------|--------|------|
| `tardigrade-minio` | MinIO | 9000 (API), 9001 (Console) | `minio_data` | Stockage artefacts |

**Buckets MinIO :**
| Bucket | Description | Lifecycle Policy |
|--------|-------------|------------------|
| `artefacts` | Artefacts CI | 30 jours |
| `logs` | Logs CI | 7 jours |
| `plugins` | Plugins uploadés | Aucun |
| `backups` | Backups | Aucun |

---

### 🚢 **Déploiement**

#### Conteneurs Docker
| Service | Image | Port Exposé | Dépendances |
|---------|-------|-------------|--------------|
| `api` | `ghcr.io/tardigrade-ci/api:latest` | 3000 | postgres, redis, nats |
| `git` | `ghcr.io/tardigrade-ci/git:latest` | 3001 | postgres, redis, nats |
| `ci` | `ghcr.io/tardigrade-ci/ci:latest` | 3002 | postgres, redis, nats, minio |
| `registry` | `ghcr.io/tardigrade-ci/registry:latest` | 3003 | postgres, redis, minio |
| `plugin-manager` | `ghcr.io/tardigrade-ci/plugin-manager:latest` | 3004 | postgres, redis, nats, minio |
| `web` | `ghcr.io/tardigrade-ci/web:latest` | 8080 | api |

#### Kubernetes (Production)
| Ressource | Type | Réplicas (Dev/Prod) | CPU (req/lim) | Memory (req/lim) |
|-----------|------|--------------------|----------------|------------------|
| `api` | Deployment | 2 / 3-10 | 250m / 500m | 256Mi / 512Mi |
| `git` | Deployment | 2 / 2-5 | 250m / 500m | 256Mi / 512Mi |
| `ci` | Deployment | 2 / 3-15 | 500m / 1 | 512Mi / 1Gi |
| `registry` | Deployment | 2 / 2-5 | 250m / 500m | 256Mi / 512Mi |
| `plugin-manager` | Deployment | 1 / 2-3 | 250m / 500m | 256Mi / 512Mi |
| `web` | Deployment | 2 / 2-5 | 100m / 200m | 128Mi / 256Mi |
| `postgres` | StatefulSet | 1 / 3 | 500m / 1 | 1Gi / 2Gi |
| `postgres-git` | StatefulSet | 1 / 3 | 500m / 1 | 1Gi / 2Gi |
| `postgres-ci` | StatefulSet | 1 / 3 | 500m / 1 | 2Gi / 4Gi |
| `postgres-registry` | StatefulSet | 1 / 3 | 500m / 1 | 1Gi / 2Gi |
| `redis` | StatefulSet | 1 / 3 | 100m / 200m | 256Mi / 512Mi |
| `nats` | StatefulSet | 1 / 3 | 200m / 400m | 256Mi / 512Mi |
| `minio` | StatefulSet | 1 / 4 | 500m / 1 | 2Gi / 4Gi |

---

### 🔍 **Monitoring & Observabilité**

| Service | Technologie | Port | Rôle |
|---------|-------------|------|------|
| `prometheus` | Prometheus | 9090 | Collecte des métriques |
| `grafana` | Grafana | 3000 | Visualisation |
| `loki` | Loki | 3100 | Stockage des logs |
| `promtail` | Promtail | - | Collection des logs |
| `jaeger` | Jaeger | 16686 | Tracing distribué |
| `alertmanager` | Alertmanager | 9093 | Alertes |

---

## 📊 **Comparaison avec la Concurrence**

| **Critère** | **Tardigrade-CI** | **GitLab** | **Gitea** | **Harbor** | **Woodpecker** |
|-------------|------------------|------------|----------|------------|---------------|
| **Langage Backend** | Rust (100%) | Ruby + Go | Go | Go | Go |
| **Langage Frontend** | TypeScript + React | JavaScript + Vue | JavaScript | JavaScript + Vue | JavaScript + Vue |
| **Base de données** | PostgreSQL | PostgreSQL | SQLite/MySQL/PostgreSQL | PostgreSQL/MySQL | PostgreSQL |
| **Messagerie** | NATS + JetStream | Sidekiq (Redis) | - | - | NATS |
| **Stockage Artefacts** | MinIO | Object Storage | - | MinIO/S3 | - |
| **Performance CI** | ⭐⭐⭐⭐⭐ (Rust) | ⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ |
| **Modularité** | ⭐⭐⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐⭐ |
| **Auto-hébergement** | ✅ Oui | ✅ Oui | ✅ Oui | ✅ Oui | ✅ Oui |
| **Plugin System** | ✅ (Rust + TS) | ✅ (Ruby/Go) | ❌ Non | ❌ Non | ✅ (Go) |
| **Multi-format Artefacts** | ✅ (Docker, npm, cargo) | ✅ (avec Premium) | ❌ Non | ✅ (Docker/OCI) | ❌ Non |
| **Complexité Déploiement** | ⭐⭐⭐ | ⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ |
| **Courbe Apprentissage** | ⭐⭐⭐ (Rust) | ⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ |

---

## 🎯 **Pourquoi Ces Choix ?**

### ✅ **Backend 100% Rust**
**Avantages :**
- **Une seule stack** à maintenir (pas de complexité Rust/Go)
- **Sécurité mémoire** sur toute la codebase
- **Performance cohérente** pour tous les modules
- **Écosystème DevOps** en croissance (Kubernetes explore Rust, etc.)

**Mitigation des risques :**
- Utilisation d'**agent IA** pour générer du code
- **Supervision humaine** par dev Java senior
- **Formation continue** et montée en compétence
- **Templates de code** pour standardiser les patterns

### ✅ **PostgreSQL Unifié**
**Avantages :**
- **Simplicité** : Une seule technologie DB à maintenir
- **Polyvalence** : Gère JSON, full-text search, timeseries (avec extension)
- **Isolation** : Instances dédiées par module pour éviter les interférences
- **Coût réduit** : Moins de licences, moins de formation

### ✅ **NATS + JetStream**
**Avantages :**
- **Latence ultra-faible** (<1ms) pour les triggers CI
- **Débit élevé** (millions de messages/sec)
- **JetStream** ajoute persistance et replay
- **Déploiement simple** (un seul binaire)
- **Compatible Kafka** si migration future nécessaire

### ✅ **MinIO**
**Avantages :**
- **Auto-hébergeable** sans dépendance cloud
- **Compatible S3** : Tous les outils existants fonctionnent
- **Léger et simple** à déployer
- **Abstraction `StorageProvider`** pour migration future

### ✅ **Plugins TypeScript + Rust**
**Avantages :**
- **Ouverture à la communauté** JS/TS (npm est le plus grand registry)
- **Performance** pour les plugins critiques (Rust)
- **Flexibilité** : Chaque plugin peut choisir son langage
- **Sandboxing** unifié via containers Docker

### ✅ **gRPC + GraphQL**
**Avantages :**
- **gRPC** : Communication interne typée et performante
- **GraphQL** : API publique flexible et introspectable
- **Partage des types** : Protobuf peut être réutilisé dans GraphQL
- **Meilleur des deux mondes** pour chaque cas d'usage

---

## 🚀 **Roadmap Technologique**

### **MVP (v0.1.0 - Décembre 2026)**
| **Module** | **Technologies** | **Statut** |
|------------|-----------------|------------|
| Git Module | Rust + Axum + PostgreSQL | ⬜ À faire |
| CI Module | Rust + Axum + Tokio + NATS | ⬜ À faire |
| Artifact Registry | Rust + Axum + MinIO | ⬜ À faire |
| Plugin System | Rust + Docker + gRPC | ⬜ À faire |
| API Gateway | Rust + Axum + GraphQL + gRPC | ⬜ À faire |
| Frontend | TypeScript + React + Vite | ⬜ À faire |

### **V1.0 (2027)**
| **Fonctionnalité** | **Technologies** | **Statut** |
|------------------|-----------------|------------|
| RBAC Avancé | Casbin + PostgreSQL | ⬜ Planifié |
| UI Améliorée | React + TypeScript + D3.js | ⬜ Planifié |
| Multi-region | Kubernetes + NATS Leafnodes | ⬜ Planifié |
| Marketplace Plugins | Rust + TypeScript + React | ⬜ Planifié |
| Intégrations Natives | Rust + gRPC | ⬜ Planifié |

### **V2.0 (2028)**
| **Fonctionnalité** | **Technologies** | **Statut** |
|------------------|-----------------|------------|
| Migration Kafka | Kafka + NATS (hybride) | ⬜ Futur |
| Support WASM Plugins | Rust + WASM | ⬜ Futur |
| AI/ML Integration | Python + Rust | ⬜ Futur |
| Edge Computing | Rust + WASM | ⬜ Futur |

---

## 📖 **Ressources pour Apprendre**

### **Rust (Pour Devs Java)**
- [Rust for Java Developers](https://www.youtube.com/watch?v=5gMjx0_2m70) (Vidéo)
- [The Rust Book](https://doc.rust-lang.org/book/) (Livre officiel)
- [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/) (Exemples)
- [Rust Cookbook](https://rust-cookbook.rs/) (Recettes)
- [Exercism Rust Track](https://exercism.org/tracks/rust) (Exercices)
- [Rustlings](https://github.com/rust-lang/rustlings) (Petits exercices)

### **TypeScript / React**
- [TypeScript Handbook](https://www.typescriptlang.org/docs/handbook/) (Documentation)
- [React Documentation](https://react.dev/learn) (Docs officielles)
- [Vite Documentation](https://vitejs.dev/guide/) (Build Tool)
- [TailwindCSS Docs](https://tailwindcss.com/docs) (Styling)

### **DevOps & Infrastructure**
- [Kubernetes Documentation](https://kubernetes.io/docs/home/) (Orchestration)
- [Terraform Documentation](https://developer.hashicorp.com/terraform/docs) (IaC)
- [Docker Documentation](https://docs.docker.com/) (Conteneurs)
- [NATS Documentation](https://docs.nats.io/) (Messagerie)
- [MinIO Documentation](https://min.io/docs/minio/linux/index.html) (Stockage)

### **Monitoring**
- [Prometheus Documentation](https://prometheus.io/docs/introduction/overview/) (Métriques)
- [Grafana Documentation](https://grafana.com/docs/) (Visualisation)
- [OpenTelemetry](https://opentelemetry.io/docs/) (Tracing)

---

## 💬 **Support & Communauté**

| **Type** | **Ressource** | **Lien** | **Statut** |
|----------|---------------|----------|------------|
| **Documentation** | Site officiel | [docs.tardigrade-ci.dev](https://docs.tardigrade-ci.dev) | À créer |
| **Code** | Repository GitHub | [github.com/tardigrade-ci/tardigrade](https://github.com/tardigrade-ci/tardigrade) | À créer |
| **Discussions** | Discord | [discord.gg/tardigrade-ci](https://discord.gg/tardigrade-ci) | À créer |
| **Issues** | GitHub Issues | [github.com/tardigrade-ci/tardigrade/issues](https://github.com/tardigrade-ci/tardigrade/issues) | À créer |
| **Email** | Support | contact@tardigrade-ci.dev | À créer |
| **Blog** | Annonces | [blog.tardigrade-ci.dev](https://blog.tardigrade-ci.dev) | À créer |

---

## 📝 **Historique des Versions**

| **Version** | **Date** | **Auteur** | **Changements** |
|-------------|----------|------------|----------------|
| 1.0 | 2026-06-17 | Benzo + Mistral Vibe | Création initiale, validation des choix techniques |

---

**© 2026 Tardigrade-CI**  
*Une plateforme DevOps modulaire, open-source, conçue pour survivre à tous les environnements.*
