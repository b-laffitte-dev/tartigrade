// =============================================================================
// Tardigrade-CI Git Module - useRepository Hook
// =============================================================================

import { useState, useEffect, useCallback } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import toast from 'react-hot-toast';
import { Repository, Branch, UpdateRepositoryInput, CreateBranchInput } from '../types/git';
import { GitService } from '../services/gitService';

// ==----------------------------------------------------------------------------
// État du hook
// ==----------------------------------------------------------------------------

interface UseRepositoryState {
  repository: Repository | null;
  branches: Branch[];
  loading: boolean;
  error: string | null;
}

// ==----------------------------------------------------------------------------
// Hook principal
// ==----------------------------------------------------------------------------

export function useRepository() {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();

  const [state, setState] = useState<UseRepositoryState>({
    repository: null,
    branches: [],
    loading: true,
    error: null,
  });

  // Charger le repository et ses branches
  const fetchRepository = useCallback(async () => {
    if (!id) return;

    setState((prev) => ({ ...prev, loading: true, error: null }));

    try {
      // Charger le repository
      const repository = await GitService.repositories.getById(id);

      // Charger les branches
      const branchesResponse = await GitService.branches.listByRepository(id);

      setState({
        repository,
        branches: branchesResponse.data,
        loading: false,
        error: null,
      });
    } catch (error: any) {
      const errorMessage = error.error || 'Impossible de charger le repository';
      setState((prev) => ({
        ...prev,
        loading: false,
        error: errorMessage,
      }));
      toast.error(errorMessage);
    }
  }, [id]);

  // Charger au montage ou quand l'ID change
  useEffect(() => {
    fetchRepository();
  }, [fetchRepository]);

  // Mettre à jour le repository
  const updateRepository = useCallback(
    async (input: UpdateRepositoryInput) => {
      if (!id) return;

      try {
        const repository = await GitService.repositories.update(id, input);
        toast.success('Repository mis à jour avec succès');
        
        // Rafraîchir
        await fetchRepository();
        
        return repository;
      } catch (error: any) {
        const errorMessage = error.error || 'Impossible de mettre à jour le repository';
        toast.error(errorMessage);
        throw error;
      }
    },
    [id, fetchRepository]
  );

  // Supprimer le repository
  const deleteRepository = useCallback(async () => {
    if (!id) return;

    try {
      if (!confirm('Êtes-vous sûr de vouloir supprimer ce repository ? Cette action est irréversible.')) {
        return;
      }

      await GitService.repositories.delete(id);
      toast.success('Repository supprimé avec succès');
      navigate('/repositories');
    } catch (error: any) {
      const errorMessage = error.error || 'Impossible de supprimer le repository';
      toast.error(errorMessage);
      throw error;
    }
  }, [id, navigate]);

  // Créer une branche
  const createBranch = useCallback(
    async (input: CreateBranchInput) => {
      if (!id) return;

      try {
        const branch = await GitService.branches.create(id, input);
        toast.success(`Branche "${branch.name}" créée avec succès`);
        
        // Rafraîchir
        await fetchRepository();
        
        return branch;
      } catch (error: any) {
        const errorMessage = error.error || 'Impossible de créer la branche';
        toast.error(errorMessage);
        throw error;
      }
    },
    [id, fetchRepository]
  );

  // Supprimer une branche
  const deleteBranch = useCallback(
    async (branchId: string) => {
      if (!id) return;

      try {
        if (!confirm('Êtes-vous sûr de vouloir supprimer cette branche ?')) {
          return;
        }

        await GitService.branches.delete(id, branchId);
        toast.success('Branche supprimée avec succès');
        
        // Rafraîchir
        await fetchRepository();
      } catch (error: any) {
        const errorMessage = error.error || 'Impossible de supprimer la branche';
        toast.error(errorMessage);
        throw error;
      }
    },
    [id, fetchRepository]
  );

  // Rafraîchir
  const refresh = useCallback(() => {
    fetchRepository();
  }, [fetchRepository]);

  // Obtenir une branche par ID
  const getBranchById = useCallback(
    (branchId: string) => {
      return state.branches.find((branch) => branch.id === branchId) || null;
    },
    [state.branches]
  );

  // Obtenir la branche par défaut
  const getDefaultBranch = useCallback(() => {
    if (!state.repository) return null;
    return state.branches.find(
      (branch) => branch.name === state.repository?.defaultBranch
    );
  }, [state.repository, state.branches]);

  return {
    // État
    repository: state.repository,
    branches: state.branches,
    loading: state.loading,
    error: state.error,
    
    // Actions
    fetchRepository,
    updateRepository,
    deleteRepository,
    createBranch,
    deleteBranch,
    refresh,
    getBranchById,
    getDefaultBranch,
    
    // Calculs
    hasBranches: state.branches.length > 0,
    repositoryId: id,
  };
}

export default useRepository;
