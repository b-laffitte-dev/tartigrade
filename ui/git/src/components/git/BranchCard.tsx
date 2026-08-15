// =============================================================================
// Tardigrade-CI Git Module - Branch Card Component
// =============================================================================

import React from 'react';
import { GitBranch, Trash2 } from 'lucide-react';
import { Branch } from '../../types/git';
import { Badge } from '../common/Badge';
import { formatDate } from '../../utils/formatters';

// ==----------------------------------------------------------------------------
// Types
// ==----------------------------------------------------------------------------

export interface BranchCardProps {
  branch: Branch;
  onDelete?: (branchId: string) => void;
  showActions?: boolean;
  className?: string;
  isDefault?: boolean;
}

// ==----------------------------------------------------------------------------
// Composant BranchCard
// ==----------------------------------------------------------------------------

export const BranchCard: React.FC<BranchCardProps> = ({
  branch,
  onDelete,
  showActions = true,
  className,
  isDefault = false,
}) => {
  return (
    <div
      className={`bg-white rounded-lg shadow border border-gray-200 p-4 flex items-center justify-between ${className || ''}`}
    >
      <div className="flex items-center space-x-3 flex-1 min-w-0">
        <div className="w-8 h-8 bg-orange-100 rounded-lg flex items-center justify-center flex-shrink-0">
          <GitBranch className="w-4 h-4 text-orange-600" />
        </div>
        <div className="min-w-0">
          <h4 className="font-medium text-gray-900 flex items-center space-x-2">
            <span>{branch.name}</span>
            {isDefault && (
              <Badge variant="info" size="xs">
                Par défaut
              </Badge>
            )}
          </h4>
          <p className="text-sm text-gray-500">{formatDate(branch.createdAt)}</p>
        </div>
      </div>

      <div className="flex items-center space-x-3">
        {branch.commitHash && (
          <code className="text-xs text-gray-600 bg-gray-100 px-2 py-1 rounded hidden sm:block">
            {branch.commitHash.substring(0, 8)}
          </code>
        )}

        {showActions && onDelete && (
          <button
            onClick={() => onDelete(branch.id)}
            className="p-2 text-gray-400 hover:text-red-500 hover:bg-red-50 rounded-lg transition-colors"
            title="Supprimer"
          >
            <Trash2 className="w-4 h-4" />
          </button>
        )}
      </div>
    </div>
  );
};
