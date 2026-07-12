# 📋 Tardigrade-CI - Plan Révisé (Version Loisir/Apprentissage)

**Dernière mise à jour :** 2026-07-12  
**Version :** 2.0 (Adapté pour apprentissage Rust)  
**Statut :** ✅ Validé par Benzo  

---

## 🎯 NOUVELLES CONTRAINTES & CHOIX

### ✅ **Choix Validés par Benzo**
| Décision | Choix | Justification |
|----------|-------|---------------|
| **Langage Backend** | Rust 100% | Projet de loisir pour **apprendre Rust** |
| **Contraintes Temps** | Aucune | Projet personnel, pas de deadline |
| **Sécurité** | Reportée | Ajoutée une fois le produit mature |
| **Approche** | Par étapes | Fondations solides avant fonctionnalités |

### 🎯 **Nouveaux Objectifs**
1. **Apprendre Rust** en construisant un vrai projet
2. **Architecture modulaire** pour faciliter l'évolution
3. **Qualité du code** > Vitesse de développement
4. **Comprendre les patterns** Rust (ownership, async, error handling)

### 📌 **Ce Qui Change vs Plan Initial**
| Élément | Avant | Maintenant |
|---------|-------|------------|
| Deadline MVP | Décembre 2026 | **Aucune** |
| Scope Sprint 1 | 45 points | **Réduit et étalé** |
| Sécurité | Dès le début | **Reportée** (Sprint 3+) |
| NATS | Sprint 1 | **Reporté** (quand besoin de messagerie) |
| API Gateway | Sprint 1 | **Reporté** (quand plusieurs modules) |
| Auth JWT | Sprint 1 | **Reportée** (Sprint 2) |
| GraphQL | Sprint 1 | **Reporté** (Sprint 2) |

---

## 🏗️ NOUVELLE ARCHITECTURE CIBLE

```
tardigrade-ci/
├── modules/                    # Modules Backend (100% Rust)
│   └── git/                   # Git Module (PREMIER MODULE)
│       ├── Cargo.toml
│       ├── src/
│       │   ├── main.rs       # Point d'entrée
│       │   ├── lib.rs        # API publique
│       │   ├── config.rs     # Configuration
│       │   ├── error.rs      # Gestion des erreurs
│       │   ├── models.rs     # Modèles de données
│       │   ├── db.rs         # Connexion DB
│       │   ├── repository.rs # Entité Repository
│       │   ├── service.rs    # Logique métier
│       │   ├── handler.rs    # Handlers Axum
│       │   └── routes.rs     # Définition des routes
│       ├── tests/
│       │   └── integration/
│       ├── migrations/       # Migrations SQL
│       └── Dockerfile
│
├── crates/                     # Bibliothèques Rust partagées
│   └── common/
│       ├── Cargo.toml
│       ├── src/
│       │   ├── error.rs
│       │   ├── config.rs
│       │   └── models.rs
│
├── ui/                         # Frontend (TypeScript + React)
│   └── git/                   # UI Git Module
│       ├── package.json
│       ├── tsconfig.json
│       ├── vite.config.ts
│       ├── tailwind.config.js
│       ├── src/
│       │   ├── main.tsx
│       │   ├── App.tsx
│       │   ├── components/
│       │   │   ├── common/   # Button, Input, Card...
│       │   │   └── git/      # RepositoryCard, BranchList...
│       │   ├── pages/
│       │   │   ├── Dashboard.tsx
│       │   │   └── Repository/
│       │   ├── hooks/
│       │   ├── services/     # gitService.ts (appels API)
│       │   ├── types/
│       │   └── styles/
│       └── Dockerfile
│
├── docker/
│   └── docker-compose.yml      # Environnement de dev
│
├── Cargo.toml                 # Workspace Rust
├── rust-toolchain              # Version Rust
└── README.md
```

---

## 🚀 NOUVELLE ROADMAP (Sans Deadlines)

### **Étape 0 : Fondations (En Cours)**
**Objectif :** Structure modulaire + environnement de dev

| ID | Tâche | Type | Priorité | Statut |
|----|-------|------|----------|--------|
| E0-01 | Créer structure de fichiers modulaire | Backend/Frontend/Docker | **High** | ⬜ |
| E0-02 | Configurer workspace Cargo | Backend | **High** | ⬜ |
| E0-03 | Créer docker-compose.yml (PostgreSQL + Redis) | DevOps | **High** | ⬜ |
| E0-04 | Setup Vite + TypeScript + Tailwind | Frontend | **High** | ⬜ |
| E0-05 | Dockerfiles backend + frontend | DevOps | **High** | ⬜ |

### **Étape 1 : Backend Git Module (Fondations)**
**Objectif :** CRUD repositories + branches (sans auth, sans GraphQL)

| ID | Tâche | Type | Priorité | Dépendances |
|----|-------|------|----------|--------------|
| E1-01 | Modèles de données (Repository, Branch) | Backend | **High** | E0-01 |
| E1-02 | Connexion PostgreSQL (SQLx) | Backend | **High** | E1-01 |
| E1-03 | Migrations DB | Backend | **High** | E1-02 |
| E1-04 | Service Repository (CRUD) | Backend | **High** | E1-03 |
| E1-05 | Service Branch (CRUD) | Backend | **High** | E1-03 |
| E1-06 | Handlers Axum | Backend | **High** | E1-04 |
| E1-07 | Routes REST | Backend | **High** | E1-06 |
| E1-08 | Tests unitaires (cargo test) | Backend | **High** | E1-07 |
| E1-09 | Tests d'intégration | Backend | **Medium** | E1-08 |

### **Étape 2 : Frontend Git Module (Fondations)**
**Objectif :** UI basique pour interagir avec le backend

| ID | Tâche | Type | Priorité | Dépendances |
|----|-------|------|----------|--------------|
| E2-01 | Types TypeScript (Repository, Branch) | Frontend | **High** | E0-04 |
| E2-02 | Service API (Fetch/Axios) | Frontend | **High** | E2-01 |
| E2-03 | Composants UI communs | Frontend | **High** | E0-04 |
| E2-04 | Page Dashboard (liste repos) | Frontend | **High** | E2-02 |
| E2-05 | Page Création Repository | Frontend | **High** | E2-02 |
| E2-06 | Page Détail Repository + Branches | Frontend | **Medium** | E2-04 |

### **Étape 3 : Intégration & Conteneurisation**
**Objectif :** Tout fait marcher ensemble

| ID | Tâche | Type | Priorité | Dépendances |
|----|-------|------|----------|--------------|
| E3-01 | Connecter frontend/backend | Intégration | **High** | E1-07, E2-04 |
| E3-02 | Docker Compose complet | DevOps | **High** | E0-03, E0-05 |
| E3-03 | Tests end-to-end | Intégration | **Medium** | E3-01 |
| E3-04 | Documentation | Docs | **Medium** | E3-02 |

### **Étape 4 : Fonctionnalités Avancées (Plus tard)**
**Objectif :** Ajouter des fonctionnalités une fois les fondations solides

| ID | Tâche | Type | Priorité |
|----|-------|------|----------|
| E4-01 | Authentification (JWT) | Backend | Medium |
| E4-02 | gRPC (communication interne) | Backend | Medium |
| E4-03 | GraphQL (API publique) | Backend | Medium |
| E4-04 | CI Module (MV) | Backend | Low |
| E4-05 | Artifact Registry | Backend | Low |
| E4-06 | Plugin System | Backend | Low |

---

## 📊 NOUVEAUX CRITERES DE QUALITE

### **Backend (Rust)**
- [ ] **Pas de `unwrap()`** en production (gérer les erreurs proprement)
- [ ] **Documentation** avec `///` (rustdoc)
- [ ] **Tests** > 80% couverture (cargo tarpaulin)
- [ ] **0 Clippy warnings** (`cargo clippy -- -D warnings`)
- [ ] **Code formaté** (`cargo fmt`)

### **Frontend (TypeScript)**
- [ ] **Typage strict** (`strict: true` dans tsconfig)
- [ ] **0 ESLint warnings** (`npm run lint`)
- [ ] **Tests** > 70% couverture (vitest)
- [ ] **Code formaté** (Prettier)

### **Docker**
- [ ] **Multi-stage builds** pour optimiser les images
- [ ] **Healthchecks** pour tous les services
- [ ] **Images minimales** (alpine, slim)

---

## 🛠️ NOUVEL WORKFLOW DE DEVELOPPEMENT

### **Backend (Rust)**
```
1. Branche : git checkout -b feature/git-[nom]
2. Code : Implémentation avec tests (TDD si possible)
3. Validation :
   - cargo build
   - cargo clippy -- -D warnings
   - cargo fmt
   - cargo test
   - cargo tarpaulin (coverage)
4. Commit : git commit -m "feat: [description]"
5. Push : git push origin feature/git-[nom]
```

### **Frontend (TypeScript)**
```
1. Branche : git checkout -b feature/git-ui-[nom]
2. Code : Implémentation avec typage strict
3. Validation :
   - npm run build
   - npm run lint
   - npm run test
   - npm run format
4. Commit : git commit -m "feat(ui): [description]"
5. Push : git push origin feature/git-ui-[nom]
```

---

## 🎯 PROCHAINE ETAPE (A FAIRE MAINTENANT)

**Objectif :** Créer la structure de fichiers modulaire

```bash
# Structure à créer :
tardigrade-ci/
├── modules/git/
│   ├── Cargo.toml
│   └── src/
│       └── (fichiers de base)
├── crates/common/
│   ├── Cargo.toml
│   └── src/
├── ui/git/
│   ├── package.json
│   └── src/
├── docker/
│   └── docker-compose.yml
├── Cargo.toml
├── rust-toolchain
└── README.md
```

**Action :**
1. Je vais créer cette structure maintenant
2. Ensuite, on configurera le workspace Cargo
3. Ensuite, on créera le docker-compose.yml
4. Ensuite, on setup le frontend

---

## 📝 NOTES & DECISIONS A VALIDER

### ❓ **Questions Ouvertes**
1. **Versions Rust** : Quelle version utiliser ? (Recommandé : **1.70+ stable**)
2. **Gestionnaire de paquets frontend** : npm ou pnpm ? (Recommandé : **pnpm**)
3. **Ports API** : Quel port pour le Git Module ? (Recommandé : **3001**)
4. **Port Frontend** : Quel port pour le dev server ? (Recommandé : **5173**)
5. **Nom des images Docker** : Quel naming convention ? (Recommandé : `tardigrade-git`, `tardigrade-ui`)

### ✅ **Décisions Déjà Prises**
- Backend : **100% Rust** (Axum + SQLx + Tokio)
- Frontend : **React + TypeScript + Vite + TailwindCSS**
- Base de données : **PostgreSQL** (une instance pour le MVP)
- Cache : **Redis** (pour plus tard)
- Conteneurisation : **Docker + Docker Compose** (Kubernetes plus tard)
- Communication : **REST JSON** pour le MVP (GraphQL/gRPC plus tard)
- Authentification : **Reportée** (pas de sécurité pour le MVP)

---

**© 2026 Tardigrade-CI**  
*Une plateforme DevOps modulaire, open-source, conçue pour apprendre Rust.*
