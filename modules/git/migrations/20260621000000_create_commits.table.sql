-- Migration: Create commits table
-- Timestamp: 2026-06-21 00:00:00 UTC
-- Up: Create the commits table with indexes

CREATE TABLE IF NOT EXISTS commits (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    repository_id UUID NOT NULL REFERENCES repositories(id) ON DELETE CASCADE,
    hash VARCHAR(40) NOT NULL,
    parent_hash VARCHAR(40),
    message TEXT NOT NULL,
    author_name VARCHAR(255) NOT NULL,
    author_email VARCHAR(255) NOT NULL,
    committer_name VARCHAR(255),
    committer_email VARCHAR(255),
    branch_name VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_commits_repository_id ON commits(repository_id);
CREATE INDEX IF NOT EXISTS idx_commits_hash ON commits(hash);
CREATE INDEX IF NOT EXISTS idx_commits_parent_hash ON commits(parent_hash);
CREATE INDEX IF NOT EXISTS idx_commits_branch_name ON commits(branch_name);
CREATE INDEX IF NOT EXISTS idx_commits_author_email ON commits(author_email);
CREATE INDEX IF NOT EXISTS idx_commits_created_at ON commits(created_at);

-- Create unique constraint for commit hashes per repository
CREATE UNIQUE INDEX IF NOT EXISTS idx_commits_hash_repository_unique 
ON commits(hash, repository_id);

-- Comments for documentation
COMMENT ON TABLE commits IS 'Git commits managed by Tardigrade-CI';
COMMENT ON COLUMN commits.id IS 'Unique identifier for the commit';
COMMENT ON COLUMN commits.repository_id IS 'Repository ID this commit belongs to';
COMMENT ON COLUMN commits.hash IS 'Commit hash (SHA-1, 40 characters)';
COMMENT ON COLUMN commits.parent_hash IS 'Parent commit hash (optional, for first commit)';
COMMENT ON COLUMN commits.message IS 'Commit message';
COMMENT ON COLUMN commits.author_name IS 'Author name';
COMMENT ON COLUMN commits.author_email IS 'Author email';
COMMENT ON COLUMN commits.committer_name IS 'Committer name (optional, defaults to author)';
COMMENT ON COLUMN commits.committer_email IS 'Committer email (optional, defaults to author)';
COMMENT ON COLUMN commits.branch_name IS 'Branch name this commit belongs to';
COMMENT ON COLUMN commits.created_at IS 'Creation timestamp';
