// =============================================================================
// Tardigrade-CI Git Module - useRepositories Hook
// =============================================================================

import { useState, useEffect, useCallback, useMemo } from 'react';
import { useNavigate } from 'react-router-dom';
import toast from 'react-hot-toast';
import {
  Repository,
  CreateRepositoryInput,
} from '../types/git';
import { GitService } from '../services/gitService';

// ==----------------------------------------------------------------------------
// État du hook
// ==----------------------------------------------------------------------------

interface UseRepositoriesState {
  repositories: Repository[];
  loading: boolean;
  error: string | null;
  total: number;
  totalPages: number;
  currentPage: number;
  pageSize: number;
}

// ==----------------------------------------------------------------------------
// Hook principal
// ==----------------------------------------------------------------------------

export function useRepositories(initialPage: number = 1, initialPageSize: number = 20) {
  const [state, setState] = useState<UseRepositoriesState>({
    repositories: [],
    loading: true,
    error: null,
    total: 0,
    totalPages: 0,
    currentPage: initialPage,
    pageSize: initialPageSize,
  });

  const navigate = useNavigate();

  // Charger les repositories
  const fetchRepositories = useCallback(async () => {
    setState((prev) => ({ ...prev, loading: true, error: null }));

    try {
      const response = await GitService.repositories.listAll({
        page: state.currentPage,
        pageSize: state.pageSize,
      });

      setState({
        repositories: response.data,
        loading: false,
        error: null,
        total: response.total,
        totalPages: response.totalPages,
        currentPage: response.page,
        pageSize: response.pageSize,
      });
    } catch (error: any) {
      const errorMessage = error.error || 'Impossible de charger les repositories';
      setState((prev) => ({
        ...prev,
        loading: false,
        error: errorMessage,
      }));
      toast.error(errorMessage);
    }
  }, [state.currentPage, state.pageSize]);

  // Charger au montage
  useEffect(() => {
    fetchRepositories();
  }, [fetchRepositories]);

  // Créer un repository
  const createRepository = useCallback(
    async (input: CreateRepositoryInput) => {
      try {
        const repository = await GitService.repositories.create(input);
        toast.success(`Repository "${repository.name}" créé avec succès !`);
        
        // Recharger la liste
        await fetchRepositories();
        
        // Naviguer vers le détail du repository
        navigate(`/repositories/${repository.id}`);
        
        return repository;
      } catch (error: any) {
        const errorMessage = error.error || 'Impossible de créer le repository';
        toast.error(errorMessage);
        throw error;
      }
    },
    [fetchRepositories, navigate]
  );

  // Supprimer un repository
  const deleteRepository = useCallback(
    async (id: string) => {
      try {
        // Demander confirmation
        if (!confirm('Êtes-vous sûr de vouloir supprimer ce repository ? Cette action est irréversible.')) {
          return;
        }

        await GitService.repositories.delete(id);
        toast.success('Repository supprimé avec succès');
        
        // Recharger la liste
        await fetchRepositories();
      } catch (error: any) {
        const errorMessage = error.error || 'Impossible de supprimer le repository';
        toast.error(errorMessage);
        throw error;
      }
    },
    [fetchRepositories]
  );

  // Changer de page
  const goToPage = useCallback(
    (page: number) => {
      if (page >= 1 && page <= state.totalPages && page !== state.currentPage) {
        setState((prev) => ({ ...prev, currentPage: page }));
      }
    },
    [state.currentPage, state.totalPages]
  );

  // Changer la taille de la page
  const changePageSize = useCallback(
    (size: number) => {
      setState((prev) => ({
        ...prev,
        pageSize: size,
        currentPage: 1, // Retour à la première page
      }));
    },
    []
  );

  // Rafraîchir la liste
  const refresh = useCallback(() => {
    fetchRepositories();
  }, [fetchRepositories]);

  // Repository par ID
  const getRepositoryById = useCallback(
    (id: string) => {
      return state.repositories.find((repo) => repo.id === id) || null;
    },
    [state.repositories]
  );

  // Filtrage et tri
  const filteredRepositories = useMemo(
    () => state.repositories,
    [state.repositories]
  );

  return {
    // État
    repositories: filteredRepositories,
    loading: state.loading,
    error: state.error,
    total: state.total,
    totalPages: state.totalPages,
    currentPage: state.currentPage,
    pageSize: state.pageSize,

    // Actions
    fetchRepositories,
    createRepository,
    deleteRepository,
    goToPage,
    changePageSize,
    refresh,
    getRepositoryById,

    // Calculs
    hasRepositories: state.repositories.length > 0,
    isEmpty: state.total === 0,
    canGoToNextPage: state.currentPage < state.totalPages,
    canGoToPreviousPage: state.currentPage > 1,
  };
}

export default useRepositories;
