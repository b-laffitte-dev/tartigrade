import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Plus } from 'lucide-react';
import { useRepositories, useDeleteRepository } from '../hooks';
import { RepositoryCard, Button, Loading, ErrorMessage } from '../components';
import { PATHS } from '../routes/paths';

const Dashboard: React.FC = () => {
  const navigate = useNavigate();
  const [page, setPage] = useState(1);
  const pageSize = 12;

  const { repositories, loading, error, refetch } = useRepositories(undefined, page, pageSize);
  const { deleteRepository: deleteRepo } = useDeleteRepository();

  const handleCreateClick = () => {
    navigate(PATHS.repository.create);
  };

  const handleDelete = async (id: string) => {
    if (window.confirm('Êtes-vous sûr de vouloir supprimer ce repository ?')) {
      try {
        await deleteRepo({ variables: { id } });
        refetch();
      } catch (err) {
        console.error('Échec de la suppression:', err);
      }
    }
  };

  const handleNextPage = () => {
    if (repositories && page < repositories.totalPages) {
      setPage(page + 1);
    }
  };

  const handlePreviousPage = () => {
    if (page > 1) {
      setPage(page - 1);
    }
  };

  if (loading && !repositories) {
    return <Loading fullPage />;
  }

  if (error) {
    return <ErrorMessage error={error} onRetry={refetch} />;
  }

  return (
    <div className="space-y-8">
      {/* Header */}
      <div className="flex justify-between items-center">
        <div>
          <h1 className="text-3xl font-bold text-secondary-900">Mes Repositories</h1>
          <p className="text-secondary-500 mt-1">
            {repositories?.total || 0} repository{repositories?.total !== 1 ? 'ies' : ''}
          </p>
        </div>
        <Button onClick={handleCreateClick}>
          <Plus className="mr-2 h-4 w-4" />
          Nouveau Repository
        </Button>
      </div>

      {/* Content */}
      {repositories?.data.length === 0 ? (
        <div className="text-center py-16">
          <h2 className="text-xl font-semibold text-secondary-900">
            Aucun repository trouvé
          </h2>
          <p className="text-secondary-500 mt-2">
            Créez votre premier repository pour commencer.
          </p>
          <Button onClick={handleCreateClick} className="mt-4">
            <Plus className="mr-2" />
            Créer un Repository
          </Button>
        </div>
      ) : (
        <>
          {/* Repository Grid */}
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
            {repositories.data.map((repo) => (
              <RepositoryCard
                key={repo.id}
                repository={repo}
                onDelete={handleDelete}
              />
            ))}
          </div>

          {/* Pagination */}
          {repositories.totalPages > 1 && (
            <div className="flex justify-center mt-8 gap-2">
              <Button
                variant="outline"
                disabled={page === 1}
                onClick={handlePreviousPage}
              >
                Précédent
              </Button>
              <span className="px-4 py-2 text-sm text-secondary-600 bg-white rounded-lg border border-secondary-200">
                Page {page} / {repositories.totalPages}
              </span>
              <Button
                variant="outline"
                disabled={page === repositories.totalPages}
                onClick={handleNextPage}
              >
                Suivant
              </Button>
            </div>
          )}
        </>
      )}
    </div>
  );
};

export { Dashboard };
