// =============================================================================
// Tests unitaires - mappers snake_case (API Rust) -> camelCase (TypeScript)
// =============================================================================

import { describe, it, expect } from 'vitest';
import { mapRepository, mapBranch } from './gitService';

describe('mapRepository', () => {
  it('mappe tous les champs snake_case vers camelCase', () => {
    const raw = {
      id: '11111111-1111-1111-1111-111111111111',
      name: 'test-repo',
      description: 'A test repo',
      is_private: true,
      owner_id: '22222222-2222-2222-2222-222222222222',
      default_branch: 'main',
      created_at: '2024-01-15T10:30:00Z',
      updated_at: '2024-01-16T10:30:00Z',
    };
    const repo = mapRepository(raw);
    expect(repo.id).toBe(raw.id);
    expect(repo.name).toBe('test-repo');
    expect(repo.description).toBe('A test repo');
    expect(repo.isPrivate).toBe(true);
    expect(repo.ownerId).toBe(raw.owner_id);
    expect(repo.defaultBranch).toBe('main');
    expect(repo.createdAt).toBe(raw.created_at);
    expect(repo.updatedAt).toBe(raw.updated_at);
  });

  it('gère une description nulle', () => {
    const repo = mapRepository({
      id: 'id-1',
      name: 'repo',
      description: null,
      is_private: false,
      owner_id: null,
      default_branch: 'main',
      created_at: '2024-01-15T10:30:00Z',
      updated_at: '2024-01-15T10:30:00Z',
    });
    expect(repo.description).toBeNull();
    expect(repo.ownerId).toBeNull();
    expect(repo.isPrivate).toBe(false);
  });
});

describe('mapBranch', () => {
  it('mappe tous les champs snake_case vers camelCase', () => {
    const raw = {
      id: '33333333-3333-3333-3333-333333333333',
      repository_id: '11111111-1111-1111-1111-111111111111',
      name: 'develop',
      commit_hash: 'abc123',
      created_at: '2024-01-15T11:00:00Z',
    };
    const branch = mapBranch(raw);
    expect(branch.id).toBe(raw.id);
    expect(branch.repositoryId).toBe(raw.repository_id);
    expect(branch.name).toBe('develop');
    expect(branch.commitHash).toBe('abc123');
    expect(branch.createdAt).toBe(raw.created_at);
  });

  it('gère un commit_hash nul', () => {
    const branch = mapBranch({
      id: 'id-b',
      repository_id: 'id-r',
      name: 'main',
      commit_hash: null,
      created_at: '2024-01-15T11:00:00Z',
    });
    expect(branch.commitHash).toBeNull();
  });
});
