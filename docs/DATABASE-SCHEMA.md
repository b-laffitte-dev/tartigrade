# 🗃️ Schéma de Base de Données - Tardigrade-CI

**Version :** 1.0  
**Dernière mise à jour :** 2026-06-17  
**Statut :** À valider (Atelier technique 20/06/2026)  

---

## 📋 Sommaire

1. [Principes de Design](#1-principes-de-design)
2. [Schéma Core (Utilisateurs & Permissions)](#2-schéma-core)
3. [Schéma Git Module](#3-schéma-git-module)
4. [Schéma CI Module](#4-schéma-ci-module)
5. [Schéma Artifact Registry](#5-schéma-artifact-registry)
6. [Index & Optimisations](#6-index--optimisations)
7. [Migrations](#7-migrations)

---

## 1️⃣ Principes de Design

### ✅ Stratégie de Séparation

**Pourquoi des instances PostgreSQL dédiées ?**
- **Isolation des workloads** : Chaque module a des patterns d'accès différents
- **Scalabilité indépendante** : Chaque module peut scaler séparément
- **Simplification du backup** : Sauvegardes ciblées par module
- **Réduction des risques** : Un problème sur un module n'affecte pas les autres

**Architecture :**
```
┌─────────────────────────────────────────────────────────────┐
│                        PostgreSQL Cluster                      │
├─────────────────┬─────────────────┬─────────────────┬─────────┤
│  Core DB         │  Git DB          │  CI DB           │  Registry│
│  (5432)          │  (5433)          │  (5434)          │  DB     │
│                 │                 │                  │  (5435) │
│  - Users        │  - Repositories  │  - Pipelines     │  - Artefacts│
│  - Permissions  │  - Branches      │  - Builds        │  - Versions│
│  - Plugins      │  - Commits       │  - Logs          │          │
│  - Audit Logs   │  - PRs           │  - Steps         │          │
│                 │  - Issues        │  - Artefacts     │          │
└─────────────────┴─────────────────┴─────────────────┴─────────┘
```

### 🔧 Outils Utilisés

| Outil | Version | Rôle |
|-------|---------|------|
| PostgreSQL | 15.x | SGBD relationnel |
| TimescaleDB | Latest | Extension timeseries |
| SQLx | 0.7.x | ORM/Query Builder Rust |
| pgAdmin | Latest | Administration (dev) |
| pg_dump | 15.x | Backup |

---

## 2️⃣ Schéma Core (Utilisateurs & Permissions)

### 📜 Tables

#### Users
```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(255) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL, -- bcrypt
    full_name VARCHAR(255),
    avatar_url VARCHAR(1024),
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    is_admin BOOLEAN NOT NULL DEFAULT FALSE,
    email_verified BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_login_at TIMESTAMPTZ,
    
    CONSTRAINT valid_email CHECK (email ~* '^[A-Za-z0-9._%-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,4}$')
);
```

#### Permissions
```sql
CREATE TABLE permissions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    category VARCHAR(50) NOT NULL, -- 'global', 'repository', 'plugin', etc.
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Permissions par défaut
INSERT INTO permissions (name, description, category) VALUES
    ('admin:all', 'Accès complet à tout', 'global'),
    ('user:create', 'Créer des utilisateurs', 'global'),
    ('user:read', 'Lire les utilisateurs', 'global'),
    ('user:update', 'Mettre à jour les utilisateurs', 'global'),
    ('user:delete', 'Supprimer des utilisateurs', 'global'),
    ('repo:create', 'Créer des repositories', 'repository'),
    ('repo:read', 'Lire les repositories', 'repository'),
    ('repo:write', 'Écrire dans les repositories', 'repository'),
    ('repo:delete', 'Supprimer des repositories', 'repository'),
    ('repo:admin', 'Administrer les repositories', 'repository'),
    ('ci:read', 'Lire les pipelines CI', 'ci'),
    ('ci:write', 'Lancer des pipelines CI', 'ci'),
    ('ci:admin', 'Administrer CI', 'ci'),
    ('registry:read', 'Lire le registry', 'registry'),
    ('registry:write', 'Écrire dans le registry', 'registry'),
    ('registry:delete', 'Supprimer du registry', 'registry'),
    ('plugin:install', 'Installer des plugins', 'plugin'),
    ('plugin:manage', 'Gérer les plugins', 'plugin');
```

#### User Permissions (RBAC)
```sql
CREATE TABLE user_permissions (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    permission_id UUID NOT NULL REFERENCES permissions(id) ON DELETE CASCADE,
    granted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    granted_by UUID NOT NULL REFERENCES users(id),
    
    PRIMARY KEY (user_id, permission_id)
);
```

#### Roles
```sql
CREATE TABLE roles (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    is_system_role BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Rôles par défaut
INSERT INTO roles (name, description, is_system_role) VALUES
    ('admin', 'Administrateur du système', TRUE),
    ('maintainer', 'Mainteneur de repositories', TRUE),
    ('developer', 'Développeur', TRUE),
    ('viewer', 'Lecteur (read-only)', TRUE);
```

#### Role Permissions
```sql
CREATE TABLE role_permissions (
    role_id UUID NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    permission_id UUID NOT NULL REFERENCES permissions(id) ON DELETE CASCADE,
    
    PRIMARY KEY (role_id, permission_id)
);

-- Assignations par défaut
-- Admin a tous les droits
INSERT INTO role_permissions 
SELECT id, (SELECT id FROM permissions WHERE name = 'admin:all') 
FROM roles WHERE name = 'admin';

-- Maintainer: droits repo + ci:write
INSERT INTO role_permissions (role_id, permission_id) VALUES
    ((SELECT id FROM roles WHERE name = 'maintainer'), (SELECT id FROM permissions WHERE name = 'repo:create')),
    ((SELECT id FROM roles WHERE name = 'maintainer'), (SELECT id FROM permissions WHERE name = 'repo:read')),
    ((SELECT id FROM roles WHERE name = 'maintainer'), (SELECT id FROM permissions WHERE name = 'repo:write')),
    ((SELECT id FROM roles WHERE name = 'maintainer'), (SELECT id FROM permissions WHERE name = 'repo:delete')),
    ((SELECT id FROM roles WHERE name = 'maintainer'), (SELECT id FROM permissions WHERE name = 'ci:read')),
    ((SELECT id FROM roles WHERE name = 'maintainer'), (SELECT id FROM permissions WHERE name = 'ci:write'));
```

#### User Roles
```sql
CREATE TABLE user_roles (
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role_id UUID NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    assigned_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    assigned_by UUID NOT NULL REFERENCES users(id),
    
    PRIMARY KEY (user_id, role_id)
);
```

#### Repository Permissions (ACL)
```sql
CREATE TABLE repository_permissions (
    repository_id UUID NOT NULL,
    user_id UUID NOT NULL,
    permission VARCHAR(50) NOT NULL, -- 'read', 'write', 'admin'
    granted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    granted_by UUID NOT NULL REFERENCES users(id),
    
    PRIMARY KEY (repository_id, user_id),
    FOREIGN KEY (repository_id) REFERENCES repositories(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
```

#### Plugin Configurations
```sql
CREATE TABLE plugin_configurations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    plugin_id UUID NOT NULL REFERENCES plugins(id) ON DELETE CASCADE,
    repository_id UUID REFERENCES repositories(id) ON DELETE CASCADE, -- NULL = global
    config JSONB NOT NULL DEFAULT '{}',
    is_enabled BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

#### Audit Logs
```sql
CREATE TABLE audit_logs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    action VARCHAR(255) NOT NULL,
    entity_type VARCHAR(255) NOT NULL,
    entity_id UUID NOT NULL,
    old_value JSONB,
    new_value JSONB,
    ip_address VARCHAR(45),
    user_agent TEXT,
    metadata JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

### 📊 Index (Core)

```sql
-- Users
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_created_at ON users(created_at);

-- User Permissions
CREATE INDEX idx_user_permissions_user_id ON user_permissions(user_id);
CREATE INDEX idx_user_permissions_permission_id ON user_permissions(permission_id);

-- User Roles
CREATE INDEX idx_user_roles_user_id ON user_roles(user_id);
CREATE INDEX idx_user_roles_role_id ON user_roles(role_id);

-- Role Permissions
CREATE INDEX idx_role_permissions_role_id ON role_permissions(role_id);
CREATE INDEX idx_role_permissions_permission_id ON role_permissions(permission_id);

-- Repository Permissions
CREATE INDEX idx_repository_permissions_repository_id ON repository_permissions(repository_id);
CREATE INDEX idx_repository_permissions_user_id ON repository_permissions(user_id);

-- Plugin Configurations
CREATE INDEX idx_plugin_configurations_plugin_id ON plugin_configurations(plugin_id);
CREATE INDEX idx_plugin_configurations_repository_id ON plugin_configurations(repository_id);

-- Audit Logs
CREATE INDEX idx_audit_logs_user_id ON audit_logs(user_id);
CREATE INDEX idx_audit_logs_created_at ON audit_logs(created_at);
CREATE INDEX idx_audit_logs_entity_type ON audit_logs(entity_type);
```

---

## 3️⃣ Schéma Git Module

### 📜 Tables

#### Repositories
```sql
CREATE TABLE repositories (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    is_private BOOLEAN NOT NULL DEFAULT FALSE,
    owner_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    default_branch VARCHAR(255) NOT NULL DEFAULT 'main',
    size BIGINT NOT NULL DEFAULT 0, -- Taille totale en octets
    stars_count INTEGER NOT NULL DEFAULT 0,
    forks_count INTEGER NOT NULL DEFAULT 0,
    open_issues_count INTEGER NOT NULL DEFAULT 0,
    open_pr_count INTEGER NOT NULL DEFAULT 0,
    is_fork BOOLEAN NOT NULL DEFAULT FALSE,
    forked_from UUID REFERENCES repositories(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT unique_repo_name_per_owner UNIQUE (owner_id, LOWER(name))
);
```

#### Branches
```sql
CREATE TABLE branches (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    repository_id UUID NOT NULL REFERENCES repositories(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    commit_hash VARCHAR(64) NOT NULL, -- SHA-1 du dernier commit
    is_protected BOOLEAN NOT NULL DEFAULT FALSE,
    protection_rule JSONB, -- Règles de protection (ex: {required_approvals: 2})
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT unique_branch_name_per_repo UNIQUE (repository_id, LOWER(name))
);
```

#### Commits
```sql
CREATE TABLE commits (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    repository_id UUID NOT NULL REFERENCES repositories(id) ON DELETE CASCADE,
    hash VARCHAR(64) NOT NULL UNIQUE, -- SHA-1 du commit
    message TEXT NOT NULL,
    author_name VARCHAR(255) NOT NULL,
    author_email VARCHAR(255) NOT NULL,
    author_date TIMESTAMPTZ NOT NULL,
    committer_name VARCHAR(255) NOT NULL,
    committer_email VARCHAR(255) NOT NULL,
    committer_date TIMESTAMPTZ NOT NULL,
    tree_hash VARCHAR(64) NOT NULL,
    parent_hashes TEXT[] NOT NULL, -- Liste des hashes parents
    stats JSONB, -- {additions: 10, deletions: 5, files_changed: 3}
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT unique_commit_hash_per_repo UNIQUE (repository_id, hash)
);
```

#### Pull Requests
```sql
CREATE TABLE pull_requests (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    repository_id UUID NOT NULL REFERENCES repositories(id) ON DELETE CASCADE,
    number INTEGER NOT NULL, -- Numéro PR (unique par repo)
    title VARCHAR(255) NOT NULL,
    description TEXT,
    source_branch VARCHAR(255) NOT NULL,
    target_branch VARCHAR(255) NOT NULL,
    author_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    assignee_id UUID REFERENCES users(id) ON DELETE SET NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'open', -- open, closed, merged
    state VARCHAR(50) NOT NULL DEFAULT 'draft', -- draft, ready_for_review, approved
    is_mergeable BOOLEAN NOT NULL DEFAULT TRUE,
    merge_commit_hash VARCHAR(64), -- SHA-1 du commit de merge
    merged_by UUID REFERENCES users(id) ON DELETE SET NULL,
    merged_at TIMESTAMPTZ,
    closed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT unique_pr_number_per_repo UNIQUE (repository_id, number)
);
```

#### Pull Request Reviews
```sql
CREATE TABLE pull_request_reviews (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    pull_request_id UUID NOT NULL REFERENCES pull_requests(id) ON DELETE CASCADE,
    reviewer_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    state VARCHAR(50) NOT NULL, -- pending, approved, rejected, commented
    body TEXT,
    commit_id UUID REFERENCES commits(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

#### Pull Request Comments
```sql
CREATE TABLE pull_request_comments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    pull_request_id UUID NOT NULL REFERENCES pull_requests(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    body TEXT NOT NULL,
    commit_id UUID REFERENCES commits(id) ON DELETE SET NULL,
    file_path VARCHAR(1024),
    line_number INTEGER,
    is_resolved BOOLEAN NOT NULL DEFAULT FALSE,
    parent_comment_id UUID REFERENCES pull_request_comments(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

#### Issues
```sql
CREATE TABLE issues (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    repository_id UUID NOT NULL REFERENCES repositories(id) ON DELETE CASCADE,
    number INTEGER NOT NULL, -- Numéro issue (unique par repo)
    title VARCHAR(255) NOT NULL,
    description TEXT,
    author_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    assignee_id UUID REFERENCES users(id) ON DELETE SET NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'open', -- open, closed
    state VARCHAR(50) NOT NULL DEFAULT 'todo', -- todo, in_progress, done
    priority VARCHAR(50) NOT NULL DEFAULT 'medium', -- low, medium, high, critical
    issue_type VARCHAR(50) NOT NULL DEFAULT 'bug', -- bug, feature, enhancement, documentation
    labels TEXT[] NOT NULL DEFAULT '{}',
    milestone_id UUID REFERENCES milestones(id) ON DELETE SET NULL,
    linked_pr_id UUID REFERENCES pull_requests(id) ON DELETE SET NULL,
    closed_by UUID REFERENCES users(id) ON DELETE SET NULL,
    closed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT unique_issue_number_per_repo UNIQUE (repository_id, number)
);
```

#### Issue Comments
```sql
CREATE TABLE issue_comments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    issue_id UUID NOT NULL REFERENCES issues(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    body TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

#### Milestones
```sql
CREATE TABLE milestones (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    repository_id UUID NOT NULL REFERENCES repositories(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    state VARCHAR(50) NOT NULL DEFAULT 'open', -- open, closed
    due_date TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    closed_at TIMESTAMPTZ
);
```

#### Webhooks
```sql
CREATE TABLE webhooks (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    repository_id UUID NOT NULL REFERENCES repositories(id) ON DELETE CASCADE,
    url VARCHAR(1024) NOT NULL,
    events TEXT[] NOT NULL, -- ['push', 'pull_request', 'issue']
    secret VARCHAR(255),
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

#### Webhook Deliveries
```sql
CREATE TABLE webhook_deliveries (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    webhook_id UUID NOT NULL REFERENCES webhooks(id) ON DELETE CASCADE,
    event_type VARCHAR(50) NOT NULL,
    payload JSONB NOT NULL,
    response_status INTEGER,
    response_body TEXT,
    attempts INTEGER NOT NULL DEFAULT 0,
    is_delivered BOOLEAN NOT NULL DEFAULT FALSE,
    delivered_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

### 📊 Index (Git Module)

```sql
-- Repositories
CREATE INDEX idx_repositories_owner_id ON repositories(owner_id);
CREATE INDEX idx_repositories_name ON repositories(LOWER(name));
CREATE INDEX idx_repositories_is_private ON repositories(is_private);
CREATE INDEX idx_repositories_created_at ON repositories(created_at);
CREATE INDEX idx_repositories_forked_from ON repositories(forked_from);

-- Branches
CREATE INDEX idx_branches_repository_id ON branches(repository_id);
CREATE INDEX idx_branches_name ON branches(LOWER(name));
CREATE INDEX idx_branches_commit_hash ON branches(commit_hash);
CREATE INDEX idx_branches_is_protected ON branches(is_protected);

-- Commits
CREATE INDEX idx_commits_repository_id ON commits(repository_id);
CREATE INDEX idx_commits_hash ON commits(hash);
CREATE INDEX idx_commits_author_email ON commits(author_email);
CREATE INDEX idx_commits_committer_email ON commits(committer_email);
CREATE INDEX idx_commits_author_date ON commits(author_date);
CREATE INDEX idx_commits_created_at ON commits(created_at);
CREATE UNIQUE INDEX idx_commits_repo_hash ON commits(repository_id, hash);

-- Pull Requests
CREATE INDEX idx_pull_requests_repository_id ON pull_requests(repository_id);
CREATE INDEX idx_pull_requests_number ON pull_requests(number);
CREATE INDEX idx_pull_requests_author_id ON pull_requests(author_id);
CREATE INDEX idx_pull_requests_status ON pull_requests(status);
CREATE INDEX idx_pull_requests_state ON pull_requests(state);
CREATE INDEX idx_pull_requests_created_at ON pull_requests(created_at);
CREATE INDEX idx_pull_requests_merged_at ON pull_requests(merged_at);

-- Pull Request Reviews
CREATE INDEX idx_pull_request_reviews_pr_id ON pull_request_reviews(pull_request_id);
CREATE INDEX idx_pull_request_reviews_reviewer_id ON pull_request_reviews(reviewer_id);
CREATE INDEX idx_pull_request_reviews_state ON pull_request_reviews(state);
CREATE INDEX idx_pull_request_reviews_created_at ON pull_request_reviews(created_at);

-- Issues
CREATE INDEX idx_issues_repository_id ON issues(repository_id);
CREATE INDEX idx_issues_number ON issues(number);
CREATE INDEX idx_issues_author_id ON issues(author_id);
CREATE INDEX idx_issues_status ON issues(status);
CREATE INDEX idx_issues_priority ON issues(priority);
CREATE INDEX idx_issues_created_at ON issues(created_at);

-- Webhooks
CREATE INDEX idx_webhooks_repository_id ON webhooks(repository_id);
CREATE INDEX idx_webhooks_is_active ON webhooks(is_active);
CREATE INDEX idx_webhook_deliveries_webhook_id ON webhook_deliveries(webhook_id);
CREATE INDEX idx_webhook_deliveries_created_at ON webhook_deliveries(created_at);
CREATE INDEX idx_webhook_deliveries_is_delivered ON webhook_deliveries(is_delivered);
```

---

## 4️⃣ Schéma CI Module

### 📜 Tables (avec TimescaleDB pour les timeseries)

```sql
-- Activer TimescaleDB
CREATE EXTENSION IF NOT EXISTS timescaledb;

-- Hypertables pour les données temporelles
SELECT create_hypertable('pipeline_logs', 'created_at');
SELECT create_hypertable('build_metrics', 'created_at');
```

#### Pipelines
```sql
CREATE TABLE pipelines (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    repository_id UUID NOT NULL REFERENCES repositories(id) ON DELETE CASCADE,
    commit_id UUID REFERENCES commits(id) ON DELETE SET NULL,
    branch_name VARCHAR(255) NOT NULL,
    trigger_type VARCHAR(50) NOT NULL, -- push, pull_request, manual, schedule, webhook
    trigger_user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    
    -- Configuration
    yml_content TEXT NOT NULL,
    yml_path VARCHAR(1024) NOT NULL DEFAULT '.tardigrade-ci.yml',
    
    -- Status
    status VARCHAR(50) NOT NULL DEFAULT 'pending', -- pending, preparing, running, success, failed, cancelled, skipped
    conclusion VARCHAR(50), -- success, failure, neutral, cancelled, skipped, timed_out
    
    -- Timing
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    duration_ms INTEGER, -- Durée totale en millisecondes
    queued_duration_ms INTEGER, -- Temps passé en file d'attente
    
    -- Metadata
    run_number INTEGER NOT NULL, -- Numéro d'exécution (incrémental par repo)
    workspace_path VARCHAR(1024), -- Chemin du workspace sur le worker
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

#### Pipeline Steps
```sql
CREATE TABLE pipeline_steps (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    pipeline_id UUID NOT NULL REFERENCES pipelines(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    step_type VARCHAR(50) NOT NULL, -- run, use, if, with, etc.
    step_number INTEGER NOT NULL, -- Numéro dans le pipeline (1, 2, 3...)
    
    -- Configuration
    run_command TEXT, -- Pour les steps 'run'
    uses VARCHAR(1024), -- Pour les steps 'uses' (ex: docker://node:18)
    with_config JSONB, -- Configuration 'with'
    env_vars JSONB NOT NULL DEFAULT '{}',
    working_directory VARCHAR(1024),
    timeout_minutes INTEGER DEFAULT 60,
    continue_on_error BOOLEAN NOT NULL DEFAULT FALSE,
    
    -- Status
    status VARCHAR(50) NOT NULL DEFAULT 'pending', -- pending, waiting, running, success, failed, cancelled, skipped
    conclusion VARCHAR(50), -- success, failure, cancelled, skipped
    exit_code INTEGER,
    
    -- Timing
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    duration_ms INTEGER,
    
    -- Outputs
    output TEXT,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

#### Pipeline Artefacts
```sql
CREATE TABLE pipeline_artefacts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    pipeline_id UUID NOT NULL REFERENCES pipelines(id) ON DELETE CASCADE,
    step_id UUID REFERENCES pipeline_steps(id) ON DELETE SET NULL,
    
    -- Identifiant
    name VARCHAR(255) NOT NULL,
    path VARCHAR(1024) NOT NULL, -- Chemin logique (ex: build/output/myapp)
    
    -- Stockage
    storage_path VARCHAR(1024) NOT NULL, -- Chemin dans MinIO
    size_bytes BIGINT NOT NULL,
    content_type VARCHAR(255),
    checksum VARCHAR(64) NOT NULL, -- SHA-256
    
    -- Metadata
    is_keep_forever BOOLEAN NOT NULL DEFAULT FALSE,
    expiration_date TIMESTAMPTZ,
    metadata JSONB NOT NULL DEFAULT '{}',
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    accessed_at TIMESTAMPTZ
);
```

#### Pipeline Logs (Hypertable TimescaleDB)
```sql
CREATE TABLE pipeline_logs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    pipeline_id UUID NOT NULL REFERENCES pipelines(id) ON DELETE CASCADE,
    step_id UUID REFERENCES pipeline_steps(id) ON DELETE SET NULL,
    
    -- Contenu
    level VARCHAR(50) NOT NULL, -- debug, info, warning, error, fatal
    message TEXT NOT NULL,
    metadata JSONB NOT NULL DEFAULT '{}',
    
    -- Timing
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

#### Build Metrics (Hypertable TimescaleDB)
```sql
CREATE TABLE build_metrics (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    pipeline_id UUID NOT NULL REFERENCES pipelines(id) ON DELETE CASCADE,
    step_id UUID REFERENCES pipeline_steps(id) ON DELETE SET NULL,
    
    -- Métriques
    metric_name VARCHAR(255) NOT NULL,
    metric_value DOUBLE PRECISION NOT NULL,
    metric_unit VARCHAR(50), -- ms, s, bytes, %, etc.
    
    -- Tags
    tags JSONB NOT NULL DEFAULT '{}', -- {os: 'ubuntu', language: 'rust', etc.}
    
    -- Timing
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

#### Runners (CI Workers)
```sql
CREATE TABLE runners (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL UNIQUE,
    status VARCHAR(50) NOT NULL DEFAULT 'offline', -- online, offline, busy, error
    os VARCHAR(50) NOT NULL, -- linux, windows, macos
    architecture VARCHAR(50) NOT NULL, -- amd64, arm64
    labels TEXT[] NOT NULL DEFAULT '{}', -- {self-hosted: true, gpu: false}
    
    -- Ressources
    cpu_cores INTEGER NOT NULL DEFAULT 2,
    memory_mb INTEGER NOT NULL DEFAULT 4096,
    disk_gb INTEGER NOT NULL DEFAULT 100,
    
    -- Connexion
    last_heartbeat_at TIMESTAMPTZ,
    last_job_id UUID REFERENCES pipeline_steps(id) ON DELETE SET NULL,
    
    -- Metadata
    version VARCHAR(50) NOT NULL, -- Version du runner
    description TEXT,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

#### Runner Jobs
```sql
CREATE TABLE runner_jobs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    runner_id UUID NOT NULL REFERENCES runners(id) ON DELETE CASCADE,
    step_id UUID NOT NULL REFERENCES pipeline_steps(id) ON DELETE CASCADE,
    
    -- Status
    status VARCHAR(50) NOT NULL DEFAULT 'queued', -- queued, in_progress, completed, failed
    
    -- Timing
    queued_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    
    -- Workers
    container_id VARCHAR(255), -- ID du container Docker
    exit_code INTEGER
);
```

#### Environments
```sql
CREATE TABLE environments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    repository_id UUID NOT NULL REFERENCES repositories(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    
    -- Configuration
    deployment_url VARCHAR(1024),
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT unique_environment_name_per_repo UNIQUE (repository_id, LOWER(name))
);
```

#### Deployments
```sql
CREATE TABLE deployments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    pipeline_id UUID NOT NULL REFERENCES pipelines(id) ON DELETE CASCADE,
    environment_id UUID NOT NULL REFERENCES environments(id) ON DELETE CASCADE,
    
    -- Status
    status VARCHAR(50) NOT NULL DEFAULT 'pending', -- pending, in_progress, success, failed, cancelled
    
    -- Metadata
    sha VARCHAR(64), -- Commit SHA déployé
    ref VARCHAR(255), -- Branche ou tag
    
    -- Timing
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

### 📊 Index (CI Module)

```sql
-- Pipelines
CREATE INDEX idx_pipelines_repository_id ON pipelines(repository_id);
CREATE INDEX idx_pipelines_commit_id ON pipelines(commit_id);
CREATE INDEX idx_pipelines_status ON pipelines(status);
CREATE INDEX idx_pipelines_conclusion ON pipelines(conclusion);
CREATE INDEX idx_pipelines_trigger_type ON pipelines(trigger_type);
CREATE INDEX idx_pipelines_started_at ON pipelines(started_at);
CREATE INDEX idx_pipelines_completed_at ON pipelines(completed_at);
CREATE INDEX idx_pipelines_created_at ON pipelines(created_at);

-- Pipeline Steps
CREATE INDEX idx_pipeline_steps_pipeline_id ON pipeline_steps(pipeline_id);
CREATE INDEX idx_pipeline_steps_status ON pipeline_steps(status);
CREATE INDEX idx_pipeline_steps_conclusion ON pipeline_steps(conclusion);
CREATE INDEX idx_pipeline_steps_step_number ON pipeline_steps(step_number);
CREATE INDEX idx_pipeline_steps_started_at ON pipeline_steps(started_at);

-- Pipeline Artefacts
CREATE INDEX idx_pipeline_artefacts_pipeline_id ON pipeline_artefacts(pipeline_id);
CREATE INDEX idx_pipeline_artefacts_step_id ON pipeline_artefacts(step_id);
CREATE INDEX idx_pipeline_artefacts_storage_path ON pipeline_artefacts(storage_path);
CREATE INDEX idx_pipeline_artefacts_created_at ON pipeline_artefacts(created_at);

-- Pipeline Logs (TimescaleDB gère ses propres index)
CREATE INDEX idx_pipeline_logs_pipeline_id ON pipeline_logs(pipeline_id);
CREATE INDEX idx_pipeline_logs_step_id ON pipeline_logs(step_id);
CREATE INDEX idx_pipeline_logs_level ON pipeline_logs(level);

-- Build Metrics (TimescaleDB)
CREATE INDEX idx_build_metrics_pipeline_id ON build_metrics(pipeline_id);
CREATE INDEX idx_build_metrics_metric_name ON build_metrics(metric_name);

-- Runners
CREATE INDEX idx_runners_status ON runners(status);
CREATE INDEX idx_runners_last_heartbeat_at ON runners(last_heartbeat_at);
CREATE INDEX idx_runners_labels ON runners USING GIN (labels);

-- Runner Jobs
CREATE INDEX idx_runner_jobs_runner_id ON runner_jobs(runner_id);
CREATE INDEX idx_runner_jobs_step_id ON runner_jobs(step_id);
CREATE INDEX idx_runner_jobs_status ON runner_jobs(status);

-- Environments
CREATE INDEX idx_environments_repository_id ON environments(repository_id);
CREATE INDEX idx_environments_name ON environments(LOWER(name));

-- Deployments
CREATE INDEX idx_deployments_pipeline_id ON deployments(pipeline_id);
CREATE INDEX idx_deployments_environment_id ON deployments(environment_id);
CREATE INDEX idx_deployments_status ON deployments(status);
```

---

## 5️⃣ Schéma Artifact Registry

### 📜 Tables

#### Artefacts
```sql
CREATE TABLE artefacts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    
    -- Identifiant
    name VARCHAR(255) NOT NULL,
    version VARCHAR(255) NOT NULL,
    
    -- Classification
    artefact_type VARCHAR(50) NOT NULL, -- docker, npm, cargo, maven, generic, etc.
    format VARCHAR(50) NOT NULL, -- container, package, binary, source, etc.
    
    -- Propriétaire
    repository_id UUID REFERENCES repositories(id) ON DELETE CASCADE,
    pipeline_id UUID REFERENCES pipelines(id) ON DELETE SET NULL,
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    
    -- Stockage
    path VARCHAR(1024) NOT NULL, -- Chemin logique (ex: myapp/v1.0.0/myapp-linux-amd64)
    storage_path VARCHAR(1024) NOT NULL, -- Chemin dans MinIO
    storage_bucket VARCHAR(255) NOT NULL DEFAULT 'artefacts',
    
    -- Metadata
    size_bytes BIGINT NOT NULL,
    content_type VARCHAR(255),
    checksum VARCHAR(64) NOT NULL, -- SHA-256
    compression VARCHAR(50), -- gzip, xz, none
    
    -- Versioning
    is_latest BOOLEAN NOT NULL DEFAULT FALSE,
    is_prerelease BOOLEAN NOT NULL DEFAULT FALSE,
    semver_metadata JSONB, -- {prerelease: 'alpha', build: '123'}
    
    -- Security
    is_public BOOLEAN NOT NULL DEFAULT FALSE,
    vulnerability_scan_status VARCHAR(50), -- pending, passed, failed
    vulnerability_scan_report JSONB,
    
    -- Metadata custom
    metadata JSONB NOT NULL DEFAULT '{}',
    
    -- Timing
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    accessed_at TIMESTAMPTZ,
    
    CONSTRAINT unique_artefact_path UNIQUE (storage_bucket, storage_path)
);
```

#### Artefact Tags
```sql
CREATE TABLE artefact_tags (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    artefact_id UUID NOT NULL REFERENCES artefacts(id) ON DELETE CASCADE,
    tag VARCHAR(255) NOT NULL,
    
    -- Metadata
    message TEXT,
    created_by UUID REFERENCES users(id) ON DELETE SET NULL,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT unique_artefact_tag UNIQUE (artefact_id, LOWER(tag))
);
```

#### Artefact Versions
```sql
CREATE TABLE artefact_versions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    artefact_name VARCHAR(255) NOT NULL, -- Nom du package (ex: myapp)
    version VARCHAR(255) NOT NULL, -- Version complète (ex: 1.0.0-beta.1+build.123)
    artefact_id UUID NOT NULL REFERENCES artefacts(id) ON DELETE CASCADE,
    
    -- Metadata
    changelog TEXT,
    published_by UUID REFERENCES users(id) ON DELETE SET NULL,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT unique_artefact_version UNIQUE (artefact_name, version)
);
```

#### Artefact Dependencies
```sql
CREATE TABLE artefact_dependencies (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    artefact_id UUID NOT NULL REFERENCES artefacts(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    version_requirement VARCHAR(255) NOT NULL, -- ^1.0.0, ~2.3.4, *
    dependency_type VARCHAR(50) NOT NULL, -- runtime, build, dev, optional
    
    CONSTRAINT unique_artefact_dependency UNIQUE (artefact_id, LOWER(name))
);
```

#### Artefact Statistics
```sql
CREATE TABLE artefact_statistics (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    artefact_id UUID NOT NULL REFERENCES artefacts(id) ON DELETE CASCADE,
    
    -- Métriques
    download_count BIGINT NOT NULL DEFAULT 0,
    star_count INTEGER NOT NULL DEFAULT 0,
    
    -- Timing (TimescaleDB)
    date TIMESTAMPTZ NOT NULL DEFAULT date_trunc('day', NOW()),
    
    UNIQUE (artefact_id, date)
);

-- Hypertable pour les stats
SELECT create_hypertable('artefact_statistics', 'date');
```

#### Storage Providers
```sql
CREATE TABLE storage_providers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL UNIQUE,
    provider_type VARCHAR(50) NOT NULL, -- minio, s3, filesystem, azure
    
    -- Configuration
    config JSONB NOT NULL, -- {endpoint: '...', access_key: '...', secret_key: '...'}
    
    -- Status
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    is_default BOOLEAN NOT NULL DEFAULT FALSE,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

#### Retention Policies
```sql
CREATE TABLE retention_policies (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    
    -- Règles
    rules JSONB NOT NULL, -- [{artefact_type: 'docker', keep_days: 30}, ...]
    
    -- Applicabilité
    applies_to_all BOOLEAN NOT NULL DEFAULT FALSE,
    repository_id UUID REFERENCES repositories(id) ON DELETE CASCADE,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

### 📊 Index (Artifact Registry)

```sql
-- Artefacts
CREATE INDEX idx_artefacts_name ON artefacts(LOWER(name));
CREATE INDEX idx_artefacts_version ON artefacts(version);
CREATE INDEX idx_artefacts_type ON artefacts(artefact_type);
CREATE INDEX idx_artefacts_format ON artefacts(format);
CREATE INDEX idx_artefacts_repository_id ON artefacts(repository_id);
CREATE INDEX idx_artefacts_pipeline_id ON artefacts(pipeline_id);
CREATE INDEX idx_artefacts_user_id ON artefacts(user_id);
CREATE INDEX idx_artefacts_storage_bucket_path ON artefacts(storage_bucket, storage_path);
CREATE INDEX idx_artefacts_is_public ON artefacts(is_public);
CREATE INDEX idx_artefacts_is_latest ON artefacts(is_latest);
CREATE INDEX idx_artefacts_created_at ON artefacts(created_at);

-- Artefact Tags
CREATE INDEX idx_artefact_tags_artefact_id ON artefact_tags(artefact_id);
CREATE INDEX idx_artefact_tags_tag ON artefact_tags(LOWER(tag));

-- Artefact Versions
CREATE INDEX idx_artefact_versions_name_version ON artefact_versions(artefact_name, version);

-- Artefact Dependencies
CREATE INDEX idx_artefact_dependencies_artefact_id ON artefact_dependencies(artefact_id);
CREATE INDEX idx_artefact_dependencies_name ON artefact_dependencies(LOWER(name));

-- Artefact Statistics
CREATE INDEX idx_artefact_statistics_artefact_id ON artefact_statistics(artefact_id);
CREATE INDEX idx_artefact_statistics_date ON artefact_statistics(date);

-- Storage Providers
CREATE INDEX idx_storage_providers_is_active ON storage_providers(is_active);
CREATE INDEX idx_storage_providers_is_default ON storage_providers(is_default);
```

---

## 6️⃣ Index & Optimisations

### ✅ Stratégie d'Indexation

1. **Index sur les clés primaires et étrangères** : Toujours créés automatiquement
2. **Index sur les colonnes utilisées dans WHERE** : Pour accélérer les requêtes
3. **Index sur les colonnes utilisées dans ORDER BY** : Pour éviter les sorts coûteux
4. **Index sur les colonnes utilisées dans JOIN** : Pour accélérer les jointures
5. **Index GIN pour les JSONB** : Pour les recherches dans les champs JSON
6. **Index partiels** : Pour les colonnes avec des valeurs spécifiques (ex: `is_active = true`)

### 🎯 Optimisations Spécifiques

#### PostgreSQL
```sql
-- Optimisation des requêtes
ALTER SYSTEM SET work_mem = '256MB';
ALTER SYSTEM SET maintenance_work_mem = '1GB';
ALTER SYSTEM SET shared_buffers = '4GB';
ALTER SYSTEM SET effective_cache_size = '8GB';

-- Autovacuum plus agressif
ALTER SYSTEM SET autovacuum_vacuum_scale_factor = 0.05;
ALTER SYSTEM SET autovacuum_analyze_scale_factor = 0.02;
```

#### TimescaleDB
```sql
-- Compression des hypertables
ALTER TABLE pipeline_logs SET (
    timescaledb.compress,
    timescaledb.compress_segmentby = 'hash'
);

-- Compression des anciennes données
SELECT add_compression_policy('pipeline_logs', INTERVAL '7 days');
SELECT add_retention_policy('pipeline_logs', INTERVAL '30 days');
```

#### Partitionnement
```sql
-- Partitionnement des grandes tables par date
CREATE TABLE pipeline_logs_2026_06 PARTITION OF pipeline_logs
    FOR VALUES FROM ('2026-06-01') TO ('2026-07-01');

CREATE TABLE pipeline_logs_2026_07 PARTITION OF pipeline_logs
    FOR VALUES FROM ('2026-07-01') TO ('2026-08-01');
```

---

## 7️⃣ Migrations

### 📁 Structure des Fichiers

```
migrations/
├── git/
│   ├── 20260715000000_create_repositories_table.sql
│   ├── 20260715000001_create_branches_table.sql
│   ├── 20260715000002_create_commits_table.sql
│   ├── 20260715000003_create_pull_requests_table.sql
│   ├── 20260715000004_create_issues_table.sql
│   └── ...
├── ci/
│   ├── 20260722000000_create_pipelines_table.sql
│   ├── 20260722000001_create_pipeline_steps_table.sql
│   └── ...
├── registry/
│   ├── 20260729000000_create_artefacts_table.sql
│   └── ...
└── core/
    ├── 20260620000000_create_users_table.sql
    ├── 20260620000001_create_permissions_table.sql
    └── ...
```

### 🔧 Outils de Migration

**Pour Rust + SQLx :**
```bash
# Installer sqlx-cli
cargo install sqlx-cli

# Créer une migration
sqlx migrate add create_repositories_table

# Appliquer les migrations
sqlx migrate run

# Revenir en arrière
sqlx migrate revert

# Vérifier le statut
sqlx migrate info
```

### 📝 Exemple de Migration

**Fichier :** `migrations/git/20260715000000_create_repositories_table.sql`

```sql
-- ↑ SQLx exige que les migrations soient réversibles

-- Migration: Créer la table repositories
CREATE TABLE repositories (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    is_private BOOLEAN NOT NULL DEFAULT FALSE,
    owner_id UUID NOT NULL,
    default_branch VARCHAR(255) NOT NULL DEFAULT 'main',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT unique_repo_name_per_owner UNIQUE (owner_id, LOWER(name))
);

-- Créer l'index
CREATE INDEX idx_repositories_owner_id ON repositories(owner_id);
CREATE INDEX idx_repositories_name ON repositories(LOWER(name));

-- Fonction pour mettre à jour updated_at
CREATE OR REPLACE FUNCTION update_repositories_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Trigger
CREATE TRIGGER update_repositories_updated_at
    BEFORE UPDATE ON repositories
    FOR EACH ROW
    EXECUTE FUNCTION update_repositories_updated_at();

-- ⬇ Réversible : Supprimer tout
DROP TRIGGER IF EXISTS update_repositories_updated_at ON repositories;
DROP FUNCTION IF EXISTS update_repositories_updated_at();
DROP INDEX IF EXISTS idx_repositories_name;
DROP INDEX IF EXISTS idx_repositories_owner_id;
DROP TABLE IF EXISTS repositories;
```

---

## 🎯 Bonnes Pratiques

### ✅ Do's
- [ ] **Toujours utiliser des UUID** pour les clés primaires
- [ ] **Ajouter des index** sur les colonnes fréquemment interrogées
- [ ] **Utiliser des constraints** pour garantir l'intégrité des données
- [ ] **Normaliser les données** (éviter la duplication)
- [ ] **Utiliser JSONB** pour les données semi-structurées
- [ ] **Créer des triggers** pour les champs mis à jour automatiquement
- [ ] **Documenter les tables** avec des commentaires
- [ ] **Utiliser des transactions** pour les opérations multi-étapes

### ❌ Don'ts
- [ ] **Ne pas utiliser de chaîne de caractères** comme clés primaires
- [ ] **Ne pas créer d'index inutiles** (surveiller la performance)
- [ ] **Ne pas stocker de données sensibles** en clair (toujours chiffrer)
- [ ] **Ne pas utiliser TEXT** pour les colonnes avec une longueur limitée
- [ ] **Ne pas oublier les foreign keys** (sauf raison valable)
- [ ] **Ne pas créer de tables trop larges** (>100 colonnes)

---

## 🔗 Références

- [PostgreSQL Documentation](https://www.postgresql.org/docs/15/index.html)
- [TimescaleDB Documentation](https://docs.timescale.com/)
- [SQLx Documentation](https://github.com/launchbadge/sqlx)
- [Indexing Strategies](https://use-the-index-luke.com/)
- [Database Design Patterns](https://www.martinfowler.com/eaaCatalog/index.html)

---

**© 2026 Tardigrade-CI**  
*Une plateforme DevOps modulaire, open-source.*
