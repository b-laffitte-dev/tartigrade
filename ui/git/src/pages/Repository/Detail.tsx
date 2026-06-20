import React from 'react';
import { useParams, Link } from 'react-router-dom';
import { useRepository } from '../../hooks';
import { Loading, ErrorMessage, Button, Badge } from '../../components/common';
import { formatDate } from '../../utils/formatters';
import { PATHS } from '../../routes/paths';
import { ArrowLeft } from 'lucide-react';

const RepositoryDetail: React.FC = () => {
  const { id } = useParams<{ id: string }>();
  const { repository, loading, error, refetch } = useRepository(id || '');

  if (loading && !repository) {
    return <Loading fullPage />;
  }

  if (error) {
    return <ErrorMessage error={error} onRetry={refetch} />;
  }

  if (!repository) {
    return (
      <div className="text-center py-12">
        <h2 className="text-xl font-semibold text-secondary-900">
          Repository non trouvé
        </h2>
        <Link to={PATHS.repository.list} className="inline-block mt-4">
          <Button variant="outline">
            <ArrowLeft className="mr-2 h-4 w-4" />
            Retour à la liste
          </Button>
        </Link>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <div className="flex items-center gap-4">
        <Link to={PATHS.repository.list}>
          <Button variant="outline" size="sm">
            <ArrowLeft className="mr-2 h-4 w-4" />
            Retour
          </Button>
        </Link>
        <div>
          <h1 className="text-3xl font-bold text-secondary-900">{repository.name}</h1>
          <div className="flex items-center gap-2 mt-2">
            <Badge variant={repository.isPrivate ? 'default' : 'success'}>{
              repository.isPrivate ? 'Privé' : 'Public'
            }</Badge>
            <span className="text-sm text-secondary-500">
              Branche par défaut: {repository.defaultBranch}
            </span>
          </div>
        </div>
      </div>

      <div className="bg-white rounded-lg border border-secondary-200 p-6">
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div>
            <h3 className="text-lg font-semibold text-secondary-900 mb-2">
              Détails
            </h3>
            <dl className="space-y-2">
              <div>
                <dt className="text-sm text-secondary-500">ID</dt>
                <dd className="text-secondary-900">{repository.id}</dd>
              </div>
              <div>
                <dt className="text-sm text-secondary-500">Propriétaire</dt>
                <dd className="text-secondary-900">{repository.ownerId}</dd>
              </div>
              <div>
                <dt className="text-sm text-secondary-500">Créé le</dt>
                <dd className="text-secondary-900">
                  {formatDate(repository.createdAt)}
                </dd>
              </div>
              <div>
                <dt className="text-sm text-secondary-500">Mis à jour le</dt>
                <dd className="text-secondary-900">
                  {formatDate(repository.updatedAt)}
                </dd>
              </div>
            </dl>
          </div>

          <div>
            <h3 className="text-lg font-semibold text-secondary-900 mb-2">
              Description
            </h3>
            <p className="text-secondary-600">
              {repository.description || 'Aucune description'}
            </p>
          </div>
        </div>

        <div className="mt-6 flex gap-4">
          <Link to={PATHS.repository.branches(repository.id)}>
            <Button variant="outline">Voir les branches</Button>
          </Link>
        </div>
      </div>
    </div>
  );
};

export { RepositoryDetail };
