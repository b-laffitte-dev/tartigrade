-- Migration: Create repositories table
-- Timestamp: 2026-06-19 00:00:00 UTC
-- Up: Create the repositories table with indexes

CREATE TABLE IF NOT EXISTS repositories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    is_private BOOLEAN NOT NULL DEFAULT FALSE,
    owner_id UUID NOT NULL,
    default_branch VARCHAR(255) NOT NULL DEFAULT 'main',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_repositories_owner_id ON repositories(owner_id);
CREATE INDEX IF NOT EXISTS idx_repositories_name ON repositories(name);
CREATE INDEX IF NOT EXISTS idx_repositories_created_at ON repositories(created_at);
CREATE INDEX IF NOT EXISTS idx_repositories_is_private ON repositories(is_private);

-- Create unique constraint for repository names per owner
CREATE UNIQUE INDEX IF NOT EXISTS idx_repositories_name_owner_unique 
ON repositories(name, owner_id);

-- Create trigger to update updated_at on row update
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

DROP TRIGGER IF EXISTS update_repositories_updated_at ON repositories;
CREATE TRIGGER update_repositories_updated_at
    BEFORE UPDATE ON repositories
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Comments for documentation
COMMENT ON TABLE repositories IS 'Git repositories managed by Tardigrade-CI';
COMMENT ON COLUMN repositories.id IS 'Unique identifier for the repository';
COMMENT ON COLUMN repositories.name IS 'Repository name (must be unique per owner)';
COMMENT ON COLUMN repositories.description IS 'Repository description';
COMMENT ON COLUMN repositories.is_private IS 'Whether the repository is private';
COMMENT ON COLUMN repositories.owner_id IS 'User ID who owns this repository';
COMMENT ON COLUMN repositories.default_branch IS 'Default branch name';
COMMENT ON COLUMN repositories.created_at IS 'Creation timestamp';
COMMENT ON COLUMN repositories.updated_at IS 'Last update timestamp';
