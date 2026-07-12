// =============================================================================
// Tardigrade-CI Git Module - Branch List Component
// =============================================================================

import React from 'react';
import { Branch } from '../../types/git';
import { BranchCard } from './BranchCard';
import { formatDate } from '../../utils/formatters';

// ==----------------------------------------------------------------------------
// Types
// ==----------------------------------------------------------------------------

export interface BranchListProps {
  branches: Branch[];
  onDelete?: (branchId: string) => void;
  className?: string;
}

// ==----------------------------------------------------------------------------
// Composant BranchList
// ==----------------------------------------------------------------------------

export const BranchList: React.FC<BranchListProps> = ({
  branches,
  onDelete,
  className,
}) => {
  return (
    <div className={className}>
      <div className="overflow-x-auto">
        <table className="min-w-full divide-y divide-gray-200">
          <thead className="bg-gray-50">
            <tr>
              <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Nom
              </th>
              <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Hash du commit
              </th>
              <th className="px-4 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                Créée le
              </th>
              <th className="px-4 py-3 text-right text-xs font-medium text-gray-500 uppercase tracking-wider">
                Actions
              </th>
            </tr>
          </thead>
          <tbody className="bg-white divide-y divide-gray-200">
            {branches.map((branch) => (
              <tr key={branch.id} className="hover:bg-gray-50 transition-colors">
                <td className="px-4 py-3 whitespace-nowrap">
                  <div className="flex items-center space-x-2">
                    {branch.name === branches.find(b => b.name === b.repositoryId)?.name && (
                      <span className="w-2 h-2 bg-blue-500 rounded-full" title="Branche par défaut" />
                    )}
                    <span className="font-medium text-gray-900">{branch.name}</span>
                  </div>
                </td>
                <td className="px-4 py-3 whitespace-nowrap">
                  {branch.commitHash ? (
                    <code className="text-xs text-gray-600 bg-gray-100 px-2 py-1 rounded">
                      {branch.commitHash.substring(0, 8)}
                    </code>
                  ) : (
                    <span className="text-xs text-gray-400">Aucun commit</span>
                  )}
                </td>
                <td className="px-4 py-3 whitespace-nowrap text-sm text-gray-500">
                  {formatDate(branch.createdAt)}
                </td>
                <td className="px-4 py-3 whitespace-nowrap text-right">
                  {onDelete && (
                    <button
                      onClick={() => onDelete(branch.id)}
                      className="text-red-600 hover:text-red-800 text-sm font-medium"
                    >
                      Supprimer
                    </button>
                  )}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      {/* Version mobile */}
      <div className="md:hidden">
        <div className="space-y-3">
          {branches.map((branch) => (
            <BranchCard
              key={branch.id}
              branch={branch}
              onDelete={onDelete}
            />
          ))}
        </div>
      </div>
    </div>
  );
};

// ==----------------------------------------------------------------------------
// Composant BranchListSimple
// ==----------------------------------------------------------------------------

export interface BranchListSimpleProps {
  branches: Branch[];
  onSelect?: (branch: Branch) => void;
  className?: string;
}

export const BranchListSimple: React.FC<BranchListSimpleProps> = ({
  branches,
  onSelect,
  className,
}) => {
  return (
    <div className={`space-y-2 ${className || ''}`}>
      {branches.map((branch) => (
        <button
          key={branch.id}
          onClick={() => onSelect?.(branch)}
          className="w-full flex items-center space-x-3 p-3 rounded-lg hover:bg-gray-50 transition-colors text-left"
        >
          <div className="w-6 h-6 bg-gray-100 rounded-full flex items-center justify-center flex-shrink-0">
            <span className="text-xs font-medium text-gray-600">B</span>
          </div>
          <div className="flex-1 min-w-0">
            <h4 className="font-medium text-gray-900 truncate">{branch.name}</h4>
            <p className="text-xs text-gray-500 truncate">
              {formatDate(branch.createdAt)}
            </p>
          </div>
          {branch.commitHash && (
            <code className="text-xs text-gray-400 bg-gray-100 px-2 py-0.5 rounded">
              {branch.commitHash.substring(0, 6)}
            </code>
          )}
        </button>
      ))}
    </div>
  );
};
