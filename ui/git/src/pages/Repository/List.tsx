import React from 'react';
import { useRepositories } from '../../hooks';
import { RepositoryCard, Loading, ErrorMessage } from '../../components';

const RepositoryList: React.FC = () => {
  const { repositories, loading, error, refetch } = useRepositories();

  if (loading && !repositories) {
    return <Loading fullPage />;
  }

  if (error) {
    return <ErrorMessage error={error} onRetry={refetch} />;
  }

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold text-secondary-900">Liste des Repositories</h1>
        <p className="text-secondary-500 mt-1">
          {repositories?.total || 0} repository{repositories?.total !== 1 ? 'ies' : ''}
        </p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        {repositories?.data.map((repo) => (
          <RepositoryCard key={repo.id} repository={repo} />
        ))}
      </div>
    </div>
  );
};

export { RepositoryList };
