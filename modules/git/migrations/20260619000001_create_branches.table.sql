-- Migration: Create branches table
-- Timestamp: 2026-06-19 00:00:01 UTC
-- Up: Create the branches table with indexes

CREATE TABLE IF NOT EXISTS branches (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    repository_id UUID NOT NULL REFERENCES repositories(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    commit_hash VARCHAR(40) NOT NULL DEFAULT '',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_branches_repository_id ON branches(repository_id);
CREATE INDEX IF NOT EXISTS idx_branches_name ON branches(name);
CREATE INDEX IF NOT EXISTS idx_branches_created_at ON branches(created_at);
CREATE INDEX IF NOT EXISTS idx_branches_commit_hash ON branches(commit_hash);

-- Create unique constraint for branch names per repository
CREATE UNIQUE INDEX IF NOT EXISTS idx_branches_name_repository_unique 
ON branches(name, repository_id);

-- Comments for documentation
COMMENT ON TABLE branches IS 'Git branches for repositories in Tardigrade-CI';
COMMENT ON COLUMN branches.id IS 'Unique identifier for the branch';
COMMENT ON COLUMN branches.repository_id IS 'Repository ID this branch belongs to';
COMMENT ON COLUMN branches.name IS 'Branch name (must be unique per repository)';
COMMENT ON COLUMN branches.commit_hash IS 'Current commit hash for this branch';
COMMENT ON COLUMN branches.created_at IS 'Creation timestamp';
