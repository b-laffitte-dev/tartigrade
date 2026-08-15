// =============================================================================
// Tardigrade-CI Git Module - Dashboard Page
// =============================================================================

import { Link } from 'react-router-dom';
import { GitBranch, Clock, Users, Activity, Plus } from 'lucide-react';
import { useRepositories } from '../hooks/useRepositories';
import { LoadingSpinner } from '../components/common/Loading';
import { ErrorMessage } from '../components/common/ErrorMessage';
import { RepositoryCard } from '../components/git/RepositoryCard';

// ==----------------------------------------------------------------------------
// Composant Dashboard
// ==----------------------------------------------------------------------------

export function DashboardPage() {
  const {
    repositories,
    loading,
    error,
    total,
    refresh,
  } = useRepositories(1, 10);

  return (
    <div className="space-y-8">
      {/* Header */}
      <div className="flex flex-col sm:flex-row sm:items-center sm:justify-between">
        <div>
          <h1 className="text-3xl font-bold text-gray-900">Tableau de bord</h1>
          <p className="text-gray-500 mt-1">
            Bienvenue sur Tardigrade-CI - Module Git
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

      {/* Stats */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <StatCard
          icon={<GitBranch className="w-6 h-6" />}
          label="Repositories"
          value={total}
          color="blue"
        />
        <StatCard
          icon={<Activity className="w-6 h-6" />}
          label="Activité"
          value="0"
          color="green"
        />
        <StatCard
          icon={<Clock className="w-6 h-6" />}
          label="Temps moyen"
          value="0ms"
          color="orange"
        />
        <StatCard
          icon={<Users className="w-6 h-6" />}
          label="Utilisateurs"
          value="1"
          color="purple"
        />
      </div>

      {/* Derniers repositories */}
      <div>
        <div className="flex items-center justify-between mb-4">
          <h2 className="text-xl font-semibold text-gray-900">
            Derniers repositories
          </h2>
          <button
            onClick={refresh}
            className="text-sm text-gray-500 hover:text-gray-700"
          >
            Rafraîchir
          </button>
        </div>

        {loading && !repositories.length ? (
          <LoadingSpinner fullPage={false} />
        ) : error ? (
          <ErrorMessage error={error} onRetry={refresh} />
        ) : repositories.length > 0 ? (
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {repositories.slice(0, 6).map((repo) => (
              <RepositoryCard key={repo.id} repository={repo} />
            ))}
          </div>
        ) : (
          <EmptyState
            title="Aucun repository"
            description="Créez votre premier repository pour commencer"
            actionText="Créer un repository"
            actionUrl="/repositories/create"
          />
        )}
      </div>

      {/* Quick Actions */}
      <div className="bg-white rounded-lg shadow p-6">
        <h2 className="text-xl font-semibold text-gray-900 mb-4">
          Actions rapides
        </h2>
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
          <QuickActionCard
            icon={<GitBranch className="w-5 h-5" />}
            title="Nouveau Repository"
            description="Créez un nouveau repository Git"
            to="/repositories/create"
          />
          <QuickActionCard
            icon={<Activity className="w-5 h-5" />}
            title="Voir l'activité"
            description="Consultez l'historique des actions"
            to="/settings"
          />
          <QuickActionCard
            icon={<Users className="w-5 h-5" />}
            title="Gérer les utilisateurs"
            description="Gérez les accès et permissions"
            to="/settings"
          />
        </div>
      </div>
    </div>
  );
}

// ==----------------------------------------------------------------------------
// Composants enfants
// ==----------------------------------------------------------------------------

interface StatCardProps {
  icon: React.ReactNode;
  label: string;
  value: string | number;
  color: 'blue' | 'green' | 'orange' | 'purple';
}

function StatCard({ icon, label, value, color }: StatCardProps) {
  const colorClasses = {
    blue: 'bg-blue-50 text-blue-600',
    green: 'bg-green-50 text-green-600',
    orange: 'bg-orange-50 text-orange-600',
    purple: 'bg-purple-50 text-purple-600',
  };

  return (
    <div className="bg-white rounded-lg shadow p-6">
      <div className={`w-10 h-10 rounded-lg ${colorClasses[color]} flex items-center justify-center mb-4`}>
        {icon}
      </div>
      <p className="text-2xl font-bold text-gray-900">{value}</p>
      <p className="text-sm text-gray-500">{label}</p>
    </div>
  );
}

interface QuickActionCardProps {
  icon: React.ReactNode;
  title: string;
  description: string;
  to: string;
}

function QuickActionCard({ icon, title, description, to }: QuickActionCardProps) {
  return (
    <Link
      to={to}
      className="bg-gray-50 rounded-lg p-4 hover:bg-gray-100 transition-colors group"
    >
      <div className="w-8 h-8 bg-white rounded-lg flex items-center justify-center mb-3 group-hover:bg-gray-50 transition-colors">
        {icon}
      </div>
      <h3 className="font-semibold text-gray-900 group-hover:text-blue-600">{title}</h3>
      <p className="text-sm text-gray-500">{description}</p>
    </Link>
  );
}

interface EmptyStateProps {
  title: string;
  description: string;
  actionText: string;
  actionUrl: string;
}

function EmptyState({ title, description, actionText, actionUrl }: EmptyStateProps) {
  return (
    <div className="text-center py-12 bg-white rounded-lg shadow">
      <div className="w-16 h-16 bg-gray-100 rounded-full flex items-center justify-center mx-auto mb-4">
        <GitBranch className="w-8 h-8 text-gray-400" />
      </div>
      <h3 className="text-lg font-semibold text-gray-900">{title}</h3>
      <p className="text-gray-500 mt-1">{description}</p>
      <div className="mt-6">
        <Link
          to={actionUrl}
          className="btn bg-blue-600 text-white px-4 py-2 text-sm font-medium hover:bg-blue-700 inline-flex items-center space-x-2"
        >
          <Plus className="w-4 h-4" />
          <span>{actionText}</span>
        </Link>
      </div>
    </div>
  );
}
