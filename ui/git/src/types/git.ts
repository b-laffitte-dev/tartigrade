// Types for Git module

export interface Repository {
  id: string;
  name: string;
  description: string | null;
  isPrivate: boolean;
  ownerId: string;
  defaultBranch: string;
  createdAt: string;
  updatedAt: string;
}

export interface Branch {
  id: string;
  repositoryId: string;
  name: string;
  commitHash: string;
  createdAt: string;
}

export interface CreateRepositoryInput {
  name: string;
  description?: string;
  isPrivate: boolean;
  defaultBranch?: string;
}

export interface CreateBranchInput {
  name: string;
  commitHash?: string;
}

export interface PaginatedResponse<T> {
  data: T[];
  page: number;
  pageSize: number;
  total: number;
  totalPages: number;
}

export interface ListRepositoriesQuery {
  ownerId?: string;
  page?: number;
  pageSize?: number;
  search?: string;
  isPrivate?: boolean;
}
