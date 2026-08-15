// =============================================================================
// Tardigrade-CI Git Module - TypeScript Types
// =============================================================================

// ==----------------------------------------------------------------------------
// Repository Types
// ==----------------------------------------------------------------------------

export interface Repository {
  id: string;
  name: string;
  description: string | null;
  isPrivate: boolean;
  ownerId: string | null;
  defaultBranch: string;
  createdAt: string;
  updatedAt: string;
}

export interface CreateRepositoryInput {
  name: string;
  description?: string;
  isPrivate?: boolean;
  defaultBranch?: string;
}

export interface UpdateRepositoryInput {
  name?: string;
  description?: string;
  isPrivate?: boolean;
  defaultBranch?: string;
}

// ==----------------------------------------------------------------------------
// Branch Types
// ==----------------------------------------------------------------------------

export interface Branch {
  id: string;
  repositoryId: string;
  name: string;
  commitHash: string | null;
  createdAt: string;
}

export interface CreateBranchInput {
  name: string;
  commitHash?: string;
}

// ==----------------------------------------------------------------------------
// Pagination Types
// ==----------------------------------------------------------------------------

export interface Pagination {
  page: number;
  pageSize: number;
}

export interface PaginatedResponse<T> {
  data: T[];
  page: number;
  pageSize: number;
  total: number;
  totalPages: number;
}

// ==----------------------------------------------------------------------------
// API Response Types
// ==----------------------------------------------------------------------------

export interface ApiError {
  error: string;
  status: number;
}

export interface HealthCheckResponse {
  status: string;
  module: string;
  timestamp: string;
}

export interface ApiInfoResponse {
  name: string;
  version: string;
  description: string;
  endpoints: Record<string, Record<string, string>>;
}

// ==----------------------------------------------------------------------------
// Form Types
// ==----------------------------------------------------------------------------

export interface RepositoryFormData extends CreateRepositoryInput {
  // Ajouter des champs spécifiques au formulaire si nécessaire
}

export interface BranchFormData extends CreateBranchInput {
  // Ajouter des champs spécifiques au formulaire si nécessaire
}

// ==----------------------------------------------------------------------------
// UI State Types
// ==----------------------------------------------------------------------------

export interface RepositoryListState {
  repositories: Repository[];
  loading: boolean;
  error: string | null;
  total: number;
  totalPages: number;
  currentPage: number;
  pageSize: number;
}

export interface RepositoryDetailState {
  repository: Repository | null;
  branches: Branch[];
  loading: boolean;
  error: string | null;
}

export interface FormState<T> {
  data: T;
  loading: boolean;
  error: string | null;
  success: boolean;
}

// ==----------------------------------------------------------------------------
// Utility Types
// ==----------------------------------------------------------------------------

export type Nullable<T> = T | null;
export type Optional<T> = T | undefined;
export type AsyncThunkConfig = {
  state: unknown;
  dispatch: unknown;
  extra: unknown;
  rejectValue: unknown;
  serializedErrorType: unknown;
  pendingMeta: unknown;
  fulfilledMeta: unknown;
  rejectedMeta: unknown;
};

// ==----------------------------------------------------------------------------
// Validation Schemas (pour Zod)
// ==----------------------------------------------------------------------------

import { z } from 'zod';

export const createRepositorySchema = z.object({
  name: z
    .string()
    .min(1, 'Le nom est obligatoire')
    .max(100, 'Le nom ne peut pas dépasser 100 caractères')
    .regex(
      /^[a-zA-Z0-9-_.]+$/,
      'Le nom ne peut contenir que des caractères alphanumériques, -, _ et .'
    ),
  description: z.string().max(500, 'La description ne peut pas dépasser 500 caractères').optional(),
  isPrivate: z.boolean().default(false),
  defaultBranch: z.string().min(1, 'La branche par défaut est obligatoire').default('main'),
});

export const updateRepositorySchema = z.object({
  name: z
    .string()
    .min(1, 'Le nom est obligatoire')
    .max(100, 'Le nom ne peut pas dépasser 100 caractères')
    .regex(
      /^[a-zA-Z0-9-_.]+$/,
      'Le nom ne peut contenir que des caractères alphanumériques, -, _ et .'
    )
    .optional(),
  description: z.string().max(500, 'La description ne peut pas dépasser 500 caractères').optional(),
  isPrivate: z.boolean().optional(),
  defaultBranch: z.string().min(1, 'La branche par défaut est obligatoire').optional(),
});

export const createBranchSchema = z.object({
  name: z
    .string()
    .min(1, 'Le nom de la branche est obligatoire')
    .max(255, 'Le nom de la branche ne peut pas dépasser 255 caractères'),
  commitHash: z.string().optional(),
});

export type CreateRepositorySchema = z.infer<typeof createRepositorySchema>;
export type UpdateRepositorySchema = z.infer<typeof updateRepositorySchema>;
export type CreateBranchSchema = z.infer<typeof createBranchSchema>;
