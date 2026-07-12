// =============================================================================
// Tardigrade-CI Git Module - Repository Card Component
// =============================================================================

import React from 'react';
import { useNavigate } from 'react-router-dom';
import { GitBranch, Clock, Trash2, MoreVertical } from 'lucide-react';
import { Repository } from '../../types/git';
import { Badge } from '../common/Badge';
import { formatDate } from '../../utils/formatters';

// ==----------------------------------------------------------------------------
// Types
// ==----------------------------------------------------------------------------

export interface RepositoryCardProps {
  repository: Repository;
  onDelete?: (id: string) => void;
  showActions?: boolean;
  className?: string;
}

// ==----------------------------------------------------------------------------
// Composant RepositoryCard
// ==----------------------------------------------------------------------------

export const RepositoryCard: React.FC<RepositoryCardProps> = ({
  repository,
  onDelete,
  showActions = true,
  className,
}) => {
  const navigate = useNavigate();

  const handleViewClick = () => {
    navigate(`/repositories/${repository.id}`);
  };

  const handleDeleteClick = (e: React.MouseEvent) => {
    e.stopPropagation();
    onDelete?.(repository.id);
  };

  const handleCardClick = (e: React.MouseEvent) => {
    // Ne pas déclencher si on clique sur un bouton d'action
    if ((e.target as HTMLElement).closest('button')) {
      return;
    }
    handleViewClick();
  };

  return (
    <div
      className={`bg-white rounded-lg shadow border border-gray-200 transition-all duration-200 hover:shadow-md hover:border-blue-300 cursor-pointer ${className || ''}`}
      onClick={handleCardClick}
    >
      <div className="p-4">
        {/* Header */}
        <div className="flex justify-between items-start">
          <div className="flex-1 min-w-0">
            <div className="flex items-center space-x-2 mb-2">
              <div className="w-8 h-8 bg-blue-100 rounded-lg flex items-center justify-center flex-shrink-0">
                <GitBranch className="w-4 h-4 text-blue-600" />
              </div>
              <div className="min-w-0">
                <h3 className="text-lg font-semibold text-gray-900 truncate">
                  {repository.name}
                </h3>
              </div>
            </div>

            {/* Description */}
            {repository.description && (
              <p className="text-sm text-gray-500 line-clamp-2 mb-3">
                {repository.description}
              </p>
            )}
          </div>

          {/* Badge de visibilité */}
          <div className="flex-shrink-0 ml-2">
            <Badge variant={repository.isPrivate ? 'default' : 'success'}>
              {repository.isPrivate ? 'Privé' : 'Public'}
            </Badge>
          </div>
        </div>

        {/* Métadonnées */}
        <div className="flex items-center justify-between mt-4">
          <div className="flex items-center space-x-4 text-sm text-gray-500">
            <span className="flex items-center space-x-1">
              <GitBranch className="w-4 h-4" />
              <span>{repository.defaultBranch}</span>
            </span>
            <span className="flex items-center space-x-1">
              <Clock className="w-4 h-4" />
              <span>{formatDate(repository.createdAt)}</span>
            </span>
          </div>

          {/* Actions */}
          {showActions && (
            <div className="flex items-center space-x-2">
              <button
                onClick={handleDeleteClick}
                className="p-2 text-gray-400 hover:text-red-500 hover:bg-red-50 rounded-lg transition-colors"
                title="Supprimer"
              >
                <Trash2 className="w-4 h-4" />
              </button>
              <button
                className="p-2 text-gray-400 hover:text-gray-600 hover:bg-gray-100 rounded-lg transition-colors"
                title="Plus d'options"
              >
                <MoreVertical className="w-4 h-4" />
              </button>
            </div>
          )}
        </div>
      </div>

      {/* Footer */}
      <div className="px-4 pb-4 border-t border-gray-100">
        <button
          onClick={handleViewClick}
          className="w-full text-sm font-medium text-blue-600 hover:text-blue-700 transition-colors py-1"
        >
          Voir le repository
        </button>
      </div>
    </div>
  );
};

// ==----------------------------------------------------------------------------
// Composant RepositoryCardCompact
// ==----------------------------------------------------------------------------

export interface RepositoryCardCompactProps {
  repository: Repository;
  onClick?: () => void;
  className?: string;
}

export const RepositoryCardCompact: React.FC<RepositoryCardCompactProps> = ({
  repository,
  onClick,
  className,
}) => {
  return (
    <div
      className={`flex items-center space-x-3 p-3 rounded-lg hover:bg-gray-50 transition-colors cursor-pointer ${className || ''}`}
      onClick={onClick}
    >
      <div className="w-8 h-8 bg-blue-100 rounded-lg flex items-center justify-center flex-shrink-0">
        <GitBranch className="w-4 h-4 text-blue-600" />
      </div>
      <div className="flex-1 min-w-0">
        <h3 className="font-medium text-gray-900 truncate">{repository.name}</h3>
        <p className="text-sm text-gray-500 truncate">{repository.description || 'Aucune description'}</p>
      </div>
      <div className="flex-shrink-0">
        <Badge variant={repository.isPrivate ? 'default' : 'success'} size="xs">
          {repository.isPrivate ? 'Privé' : 'Public'}
        </Badge>
      </div>
    </div>
  );
};
