-- Migration initiale : schéma du module Git
-- Tables : repositories, branches, commits + index associés

-- =============================================================================
-- Table : repositories
-- =============================================================================
CREATE TABLE IF NOT EXISTS repositories (
    id UUID PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    is_private BOOLEAN NOT NULL DEFAULT FALSE,
    owner_id UUID,
    default_branch VARCHAR(255) NOT NULL DEFAULT 'main',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_repositories_name ON repositories(name);
CREATE INDEX IF NOT EXISTS idx_repositories_owner_id ON repositories(owner_id);
CREATE INDEX IF NOT EXISTS idx_repositories_created_at ON repositories(created_at);

-- =============================================================================
-- Table : branches
-- =============================================================================
CREATE TABLE IF NOT EXISTS branches (
    id UUID PRIMARY KEY,
    repository_id UUID NOT NULL REFERENCES repositories(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    commit_hash VARCHAR(64),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_branches_repo_name ON branches(repository_id, name);
CREATE INDEX IF NOT EXISTS idx_branches_repository_id ON branches(repository_id);

-- =============================================================================
-- Table : commits (simplifiée pour le MVP)
-- =============================================================================
CREATE TABLE IF NOT EXISTS commits (
    hash VARCHAR(64) PRIMARY KEY,
    repository_id UUID NOT NULL REFERENCES repositories(id) ON DELETE CASCADE,
    message TEXT NOT NULL,
    author_name VARCHAR(255),
    author_email VARCHAR(255),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_commits_repository_id ON commits(repository_id);
CREATE INDEX IF NOT EXISTS idx_commits_created_at ON commits(created_at);
