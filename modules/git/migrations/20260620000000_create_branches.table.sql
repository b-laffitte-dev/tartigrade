-- Migration: Create branches table
-- Timestamp: 2026-06-20 00:00:00 UTC
-- Up: Create the branches table with indexes

CREATE TABLE IF NOT EXISTS branches (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    repository_id UUID NOT NULL REFERENCES repositories(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    commit_hash VARCHAR(64) NOT NULL,
    is_default BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_branches_repository_id ON branches(repository_id);
CREATE INDEX IF NOT EXISTS idx_branches_name ON branches(name);
CREATE INDEX IF NOT EXISTS idx_branches_commit_hash ON branches(commit_hash);
CREATE INDEX IF NOT EXISTS idx_branches_is_default ON branches(is_default);
CREATE INDEX IF NOT EXISTS idx_branches_created_at ON branches(created_at);

-- Create unique constraint for branch names per repository
CREATE UNIQUE INDEX IF NOT EXISTS idx_branches_name_repository_unique 
ON branches(name, repository_id);

-- Create trigger to update updated_at on row update
CREATE OR REPLACE FUNCTION update_branches_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

DROP TRIGGER IF EXISTS update_branches_updated_at ON branches;
CREATE TRIGGER update_branches_updated_at
    BEFORE UPDATE ON branches
    FOR EACH ROW
    EXECUTE FUNCTION update_branches_updated_at_column();

-- Comments for documentation
COMMENT ON TABLE branches IS 'Git branches managed by Tardigrade-CI';
COMMENT ON COLUMN branches.id IS 'Unique identifier for the branch';
COMMENT ON COLUMN branches.repository_id IS 'Repository ID this branch belongs to';
COMMENT ON COLUMN branches.name IS 'Branch name (must be unique per repository)';
COMMENT ON COLUMN branches.commit_hash IS 'Commit hash this branch points to';
COMMENT ON COLUMN branches.is_default IS 'Whether this is the default branch';
COMMENT ON COLUMN branches.created_at IS 'Creation timestamp';
COMMENT ON COLUMN branches.updated_at IS 'Last update timestamp';
