// =============================================================================
// Tardigrade-CI Git Module - Repository Detail Page
// =============================================================================

import { Link, useNavigate } from 'react-router-dom';
import { GitBranch, Clock, Pencil, Trash2, Plus, Code, ArrowLeft } from 'lucide-react';
import { useRepository } from '../../hooks/useRepository';
import { LoadingSpinner } from '../../components/common/Loading';
import { ErrorMessage } from '../../components/common/ErrorMessage';
import { BranchList } from '../../components/git/BranchList';
import { formatDate } from '../../utils/formatters';

// ==----------------------------------------------------------------------------
// Composant Repository Detail Page
// ==----------------------------------------------------------------------------

export function RepositoryDetailPage() {
  const { repository, branches, loading, error, deleteRepository } = useRepository();
  const navigate = useNavigate();

  const handleDelete = async () => {
    if (repository) {
      await deleteRepository();
    }
  };

  if (loading) {
    return <LoadingSpinner fullPage />;
  }

  if (error) {
    return <ErrorMessage error={error} onRetry={() => window.location.reload()} />;
  }

  if (!repository) {
    return (
      <div className="text-center py-16">
        <h2 className="text-xl font-semibold text-gray-900">Repository non trouvé</h2>
        <p className="text-gray-500 mt-2">Le repository que vous cherchez n'existe pas.</p>
        <div className="mt-6">
          <Link
            to="/repositories"
            className="btn bg-blue-600 text-white px-4 py-2 hover:bg-blue-700 inline-flex items-center space-x-2"
          >
            <ArrowLeft className="w-4 h-4" />
            <span>Retour à la liste</span>
          </Link>
        </div>
      </div>
    );
  }

  return (
    <div className="space-y-8">
      {/* Header */}
      <div className="flex items-center space-x-4">
        <button
          onClick={() => navigate('/repositories')}
          className="p-2 text-gray-500 hover:text-gray-700 hover:bg-gray-100 rounded-lg"
        >
          <ArrowLeft className="w-5 h-5" />
        </button>
        <div className="flex-1">
          <h1 className="text-3xl font-bold text-gray-900 flex items-center space-x-3">
            <span className="w-10 h-10 bg-blue-100 rounded-lg flex items-center justify-center">
              <GitBranch className="w-5 h-5 text-blue-600" />
            </span>
            <span>{repository.name}</span>
            <span
              className={`px-2 py-1 text-xs font-medium rounded-full ${
                repository.isPrivate
                  ? 'bg-gray-100 text-gray-700'
                  : 'bg-green-100 text-green-700'
              }`}
            >
              {repository.isPrivate ? 'Privé' : 'Public'}
            </span>
          </h1>
          <p className="text-gray-500 mt-2">{repository.description || 'Aucune description'}</p>
        </div>
      </div>

      {/* Métadonnées */}
      <div className="bg-white rounded-lg shadow p-6">
        <h2 className="text-lg font-semibold text-gray-900 mb-4">Informations</h2>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          <InfoItem
            icon={<GitBranch className="w-4 h-4" />}
            label="Branche par défaut"
            value={repository.defaultBranch}
          />
          <InfoItem
            icon={<Clock className="w-4 h-4" />}
            label="Créé le"
            value={formatDate(repository.createdAt)}
          />
          <InfoItem
            icon={<Clock className="w-4 h-4" />}
            label="Mis à jour le"
            value={formatDate(repository.updatedAt)}
          />
        </div>
      </div>

      {/* Actions */}
      <div className="flex items-center justify-end space-x-3">
        <Link
          to={`/repositories/${repository.id}/edit`}
          className="btn bg-gray-100 text-gray-700 px-4 py-2 text-sm font-medium hover:bg-gray-200 flex items-center space-x-2"
        >
          <Pencil className="w-4 h-4" />
          <span>Modifier</span>
        </Link>
        <button
          onClick={handleDelete}
          className="btn bg-red-100 text-red-700 px-4 py-2 text-sm font-medium hover:bg-red-200 flex items-center space-x-2"
        >
          <Trash2 className="w-4 h-4" />
          <span>Supprimer</span>
        </button>
      </div>

      {/* Branches */}
      <div className="bg-white rounded-lg shadow p-6">
        <div className="flex items-center justify-between mb-4">
          <h2 className="text-lg font-semibold text-gray-900 flex items-center space-x-2">
            <Code className="w-5 h-5" />
            <span>Branches</span>
            <span className="px-2 py-1 bg-gray-100 text-gray-700 text-xs font-medium rounded-full">
              {branches.length}
            </span>
          </h2>
          <Link
            to={`/repositories/${repository.id}/branches/create`}
            className="btn bg-blue-600 text-white px-3 py-1.5 text-sm font-medium hover:bg-blue-700 flex items-center space-x-1"
          >
            <Plus className="w-4 h-4" />
            <span>Nouvelle branche</span>
          </Link>
        </div>

        {branches.length > 0 ? (
          <BranchList branches={branches} />
        ) : (
          <EmptyBranches repositoryId={repository.id} />
        )}
      </div>

      {/* Quick Actions */}
      <div className="bg-white rounded-lg shadow p-6">
        <h2 className="text-lg font-semibold text-gray-900 mb-4">Actions rapides</h2>
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
          <ActionCard
            title="Cloner"
            description="Obtenez l'URL pour cloner ce repository"
            icon={<Code className="w-5 h-5" />}
          />
          <ActionCard
            title="Configurer"
            description="Configurer les webhooks et intégrations"
            icon={<Settings className="w-5 h-5" />}
          />
          <ActionCard
            title="Partager"
            description="Inviter des collaborateurs"
            icon={<Users className="w-5 h-5" />}
          />
        </div>
      </div>
    </div>
  );
}

// ==----------------------------------------------------------------------------
// Composants enfants
// ==----------------------------------------------------------------------------

interface InfoItemProps {
  icon: React.ReactNode;
  label: string;
  value: string;
}

function InfoItem({ icon, label, value }: InfoItemProps) {
  return (
    <div className="flex items-center space-x-3">
      <div className="w-8 h-8 bg-gray-100 rounded-lg flex items-center justify-center text-gray-500">
        {icon}
      </div>
      <div>
        <p className="text-xs text-gray-500">{label}</p>
        <p className="font-medium text-gray-900">{value}</p>
      </div>
    </div>
  );
}

function EmptyBranches({ repositoryId }: { repositoryId: string }) {
  return (
    <div className="text-center py-8">
      <div className="w-16 h-16 bg-gray-100 rounded-full flex items-center justify-center mx-auto mb-4">
        <GitBranch className="w-8 h-8 text-gray-400" />
      </div>
      <h3 className="font-semibold text-gray-900">Aucune branche</h3>
      <p className="text-gray-500 text-sm mt-1">
        Ce repository n'a pas encore de branche
      </p>
      <div className="mt-4">
        <Link
          to={`/repositories/${repositoryId}/branches/create`}
          className="btn bg-blue-600 text-white px-4 py-2 text-sm font-medium hover:bg-blue-700 inline-flex items-center space-x-2"
        >
          <Plus className="w-4 h-4" />
          <span>Créer la première branche</span>
        </Link>
      </div>
    </div>
  );
}

interface ActionCardProps {
  title: string;
  description: string;
  icon: React.ReactNode;
}

function ActionCard({ title, description, icon }: ActionCardProps) {
  return (
    <button
      // onClick={onClick}
      className="bg-gray-50 rounded-lg p-4 hover:bg-gray-100 transition-colors group text-left"
    >
      <div className="w-8 h-8 bg-white rounded-lg flex items-center justify-center mb-3 group-hover:bg-gray-50 transition-colors">
        {icon}
      </div>
      <h3 className="font-semibold text-gray-900 group-hover:text-blue-600">{title}</h3>
      <p className="text-sm text-gray-500">{description}</p>
    </button>
  );
}

// Icones temporaires (seront importées depuis lucide-react)
function Settings({ className }: { className?: string }) {
  return <span className={className}>⚙️</span>;
}

function Users({ className }: { className?: string }) {
  return <span className={className}>👥</span>;
}
