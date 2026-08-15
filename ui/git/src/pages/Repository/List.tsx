// =============================================================================
// Tardigrade-CI Git Module - Repository List Page
// =============================================================================

import { Link } from 'react-router-dom';
import { Plus, Search, GitBranch } from 'lucide-react';
import { useRepositories } from '../../hooks/useRepositories';
import { LoadingSpinner } from '../../components/common/Loading';
import { ErrorMessage } from '../../components/common/ErrorMessage';
import { RepositoryCard } from '../../components/git/RepositoryCard';
import { Pagination } from '../../components/common/Pagination';

// ==----------------------------------------------------------------------------
// Composant Repository List Page
// ==----------------------------------------------------------------------------

export function RepositoryListPage() {
  const {
    repositories,
    loading,
    error,
    total,
    totalPages,
    currentPage,
    pageSize,
    goToPage,
    changePageSize,
    refresh,
    deleteRepository,
    hasRepositories,
    isEmpty,
  } = useRepositories();

  return (
    <div className="space-y-8">
      {/* Header */}
      <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h1 className="text-3xl font-bold text-gray-900">Repositories</h1>
          <p className="text-gray-500 mt-1">
            {total} repository{total !== 1 ? 'ies' : ''} créés
          </p>
        </div>
        <div className="mt-4 sm:mt-0">
          <Link
            to="/repositories/create"
            className="btn bg-blue-600 text-white px-4 py-2 text-sm font-medium hover:bg-blue-700 flex items-center space-x-2"
          >
            <Plus className="w-4 h-4" />
            <span>Nouveau Repository</span>
          </Link>
        </div>
      </div>

      {/* Barre de recherche */}
      <div className="bg-white rounded-lg shadow p-4">
        <div className="relative max-w-md">
          <Search className="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-400" />
          <input
            type="text"
            placeholder="Rechercher un repository..."
            className="input w-full pl-10 pr-4 py-2 text-sm"
            // onChange={handleSearch}
            disabled
          />
        </div>
        <p className="text-xs text-gray-500 mt-2">
          La recherche sera implémentée dans une version future
        </p>
      </div>

      {/* Liste des repositories */}
      <div>
        {loading && !hasRepositories ? (
          <LoadingSpinner fullPage={false} />
        ) : error ? (
          <ErrorMessage error={error} onRetry={refresh} />
        ) : isEmpty ? (
          <EmptyState
            title="Aucun repository"
            description="Créez votre premier repository pour commencer à versionner votre code"
            actionText="Créer un repository"
            actionUrl="/repositories/create"
          />
        ) : (
          <>
            {/* Grille de repositories */}
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
              {repositories.map((repo) => (
                <RepositoryCard
                  key={repo.id}
                  repository={repo}
                  onDelete={deleteRepository}
                />
              ))}
            </div>

            {/* Pagination */}
            {totalPages > 1 && (
              <Pagination
                currentPage={currentPage}
                totalPages={totalPages}
                onPageChange={goToPage}
                onPageSizeChange={changePageSize}
                pageSize={pageSize}
                totalItems={total}
              />
            )}
          </>
        )}
      </div>
    </div>
  );
}

// ==----------------------------------------------------------------------------
// Composants enfants
// ==----------------------------------------------------------------------------

interface EmptyStateProps {
  title: string;
  description: string;
  actionText: string;
  actionUrl: string;
}

function EmptyState({ title, description, actionText, actionUrl }: EmptyStateProps) {
  return (
    <div className="text-center py-16 bg-white rounded-lg shadow">
      <div className="w-20 h-20 bg-gray-100 rounded-full flex items-center justify-center mx-auto mb-6">
        <GitBranch className="w-10 h-10 text-gray-400" />
      </div>
      <h3 className="text-xl font-semibold text-gray-900">{title}</h3>
      <p className="text-gray-500 mt-2 max-w-md mx-auto">{description}</p>
      <div className="mt-8">
        <Link
          to={actionUrl}
          className="btn bg-blue-600 text-white px-6 py-3 font-medium hover:bg-blue-700 inline-flex items-center space-x-2"
        >
          <Plus className="w-5 h-5" />
          <span>{actionText}</span>
        </Link>
      </div>
    </div>
  );
}
