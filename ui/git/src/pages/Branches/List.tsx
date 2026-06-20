import React from 'react';
import { useParams } from 'react-router-dom';
import { useListBranches } from '../../hooks';
import { Loading, ErrorMessage } from '../../components/common';

// Temporary hook for branches (to be implemented)
const useListBranches = (repositoryId: string) => {
  // This is a placeholder - implement actual GraphQL query later
  return {
    branches: null,
    loading: false,
    error: null,
    refetch: () => {},
  };
};

const BranchList: React.FC = () => {
  const { repositoryId } = useParams<{ repositoryId: string }>();
  const { branches, loading, error, refetch } = useListBranches(repositoryId || '');

  if (loading && !branches) {
    return <Loading fullPage />;
  }

  if (error) {
    return <ErrorMessage error={error} onRetry={refetch} />;
  }

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold text-secondary-900">Branches</h1>
        <p className="text-secondary-500 mt-1">
          Liste des branches pour ce repository
        </p>
      </div>

      <div className="bg-white rounded-lg border border-secondary-200 p-6">
        <p className="text-secondary-500">
          Fonctionnalité à implémenter (US-003)
        </p>
      </div>
    </div>
  );
};

export { BranchList };
