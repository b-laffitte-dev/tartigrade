// =============================================================================
// Tardigrade-CI Git Module - API Service
// =============================================================================

import axios, { AxiosInstance, AxiosError, AxiosRequestConfig } from 'axios';
import {
  Repository,
  CreateRepositoryInput,
  UpdateRepositoryInput,
  Branch,
  CreateBranchInput,
  PaginatedResponse,
  ApiError,
  HealthCheckResponse,
  ApiInfoResponse,
} from '../types/git';

// ==----------------------------------------------------------------------------
// Configuration de l'API
// ==----------------------------------------------------------------------------

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || '/api';

// Créer une instance Axios avec configuration par défaut
const api: AxiosInstance = axios.create({
  baseURL: API_BASE_URL,
  timeout: 10000,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Intercepteur pour ajouter les en-têtes d'authentification (pour plus tard)
api.interceptors.request.use(
  (config) => {
    // const token = localStorage.getItem('token');
    // if (token) {
    //   config.headers.Authorization = `Bearer ${token}`;
    // }
    return config;
  },
  (error) => {
    return Promise.reject(error);
  }
);

// Intercepteur pour gérer les erreurs
api.interceptors.response.use(
  (response) => response,
  (error: AxiosError<ApiError>) => {
    if (error.response) {
      // Le serveur a répondu avec une erreur
      const apiError = error.response.data;
      console.error('API Error:', apiError);
      return Promise.reject(apiError);
    } else if (error.request) {
      // La requête a été faite mais pas de réponse
      console.error('Network Error:', error.message);
      return Promise.reject({
        error: 'Erreur réseau: Impossible de contacter le serveur',
        status: 0,
      });
    } else {
      // Erreur dans la configuration de la requête
      console.error('Request Error:', error.message);
      return Promise.reject({
        error: `Erreur: ${error.message}`,
        status: 0,
      });
    }
  }
);

// ==----------------------------------------------------------------------------
// Service pour les Repositories
// ==----------------------------------------------------------------------------

export const GitRepositoryService = {
  /**
   * Crée un nouveau repository
   */
  async create(input: CreateRepositoryInput): Promise<Repository> {
    const response = await api.post<Repository>('/repositories', input);
    return response.data;
  },

  /**
   * Récupère un repository par ID
   */
  async getById(id: string): Promise<Repository> {
    const response = await api.get<Repository>(`/repositories/${id}`);
    return response.data;
  },

  /**
   * Liste tous les repositories avec pagination
   */
  async listAll(params?: {
    page?: number;
    pageSize?: number;
  }): Promise<PaginatedResponse<Repository>> {
    const response = await api.get<PaginatedResponse<Repository>>('/repositories', { params });
    return response.data;
  },

  /**
   * Met à jour un repository
   */
  async update(id: string, input: UpdateRepositoryInput): Promise<Repository> {
    const response = await api.put<Repository>(`/repositories/${id}`, input);
    return response.data;
  },

  /**
   * Supprime un repository
   */
  async delete(id: string): Promise<void> {
    await api.delete(`/repositories/${id}`);
  },

  /**
   * Récupère un repository par nom
   */
  async getByName(name: string): Promise<Repository> {
    // Note: Pour l'instant, on liste tous et on filtre côté client
    // En production, on devrait avoir un endpoint /repositories/by-name/{name}
    const response = await api.get<PaginatedResponse<Repository>>('/repositories');
    const repository = response.data.data.find((repo) => repo.name === name);
    if (!repository) {
      throw { error: `Repository '${name}' not found`, status: 404 };
    }
    return repository;
  },
};

// ==----------------------------------------------------------------------------
// Service pour les Branches
// ==----------------------------------------------------------------------------

export const GitBranchService = {
  /**
   * Crée une nouvelle branche
   */
  async create(repositoryId: string, input: CreateBranchInput): Promise<Branch> {
    const response = await api.post<Branch>(`/repositories/${repositoryId}/branches`, input);
    return response.data;
  },

  /**
   * Récupère une branche par ID
   */
  async getById(id: string): Promise<Branch> {
    const response = await api.get<Branch>(`/branches/${id}`);
    return response.data;
  },

  /**
   * Liste toutes les branches d'un repository avec pagination
   */
  async listByRepository(repositoryId: string, params?: {
    page?: number;
    pageSize?: number;
  }): Promise<PaginatedResponse<Branch>> {
    const response = await api.get<PaginatedResponse<Branch>>(
      `/repositories/${repositoryId}/branches`,
      { params }
    );
    return response.data;
  },

  /**
   * Supprime une branche
   */
  async delete(repositoryId: string, branchId: string): Promise<void> {
    await api.delete(`/repositories/${repositoryId}/branches/${branchId}`);
  },
};

// ==----------------------------------------------------------------------------
// Service pour la Santé et les Infos
// ==----------------------------------------------------------------------------

export const GitHealthService = {
  /**
   * Vérifie la santé de l'API
   */
  async healthCheck(): Promise<HealthCheckResponse> {
    const response = await api.get<HealthCheckResponse>('/health');
    return response.data;
  },

  /**
   * Récupère les informations de l'API
   */
  async getApiInfo(): Promise<ApiInfoResponse> {
    const response = await api.get<ApiInfoResponse>('/');
    return response.data;
  },
};

// ==----------------------------------------------------------------------------
// Service unifié
// ==----------------------------------------------------------------------------

export const GitService = {
  repositories: GitRepositoryService,
  branches: GitBranchService,
  health: GitHealthService,
};

// ==----------------------------------------------------------------------------
// Utilitaires
// ==----------------------------------------------------------------------------

/**
 * Effectue une requête GET générique
 */
export async function apiGet<T>(url: string, config?: AxiosRequestConfig): Promise<T> {
  const response = await api.get<T>(url, config);
  return response.data;
}

/**
 * Effectue une requête POST générique
 */
export async function apiPost<T, D = unknown>(url: string, data?: D, config?: AxiosRequestConfig): Promise<T> {
  const response = await api.post<T>(url, data, config);
  return response.data;
}

/**
 * Effectue une requête PUT générique
 */
export async function apiPut<T, D = unknown>(url: string, data?: D, config?: AxiosRequestConfig): Promise<T> {
  const response = await api.put<T>(url, data, config);
  return response.data;
}

/**
 * Effectue une requête DELETE générique
 */
export async function apiDelete<T = void>(url: string, config?: AxiosRequestConfig): Promise<T> {
  const response = await api.delete<T>(url, config);
  return response.data;
}

/**
 * Configure l'URL de base de l'API
 */
export function configureApi(baseUrl: string): void {
  api.defaults.baseURL = baseUrl;
}

/**
 * Ajoute un token d'authentification (pour plus tard)
 */
export function setAuthToken(token: string): void {
  if (token) {
    api.defaults.headers.common['Authorization'] = `Bearer ${token}`;
  } else {
    delete api.defaults.headers.common['Authorization'];
  }
}

/**
 * Supprime le token d'authentification
 */
export function clearAuthToken(): void {
  delete api.defaults.headers.common['Authorization'];
}

export default GitService;
