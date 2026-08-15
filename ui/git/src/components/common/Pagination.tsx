// =============================================================================
// Tardigrade-CI Git Module - Pagination Component
// =============================================================================

import React from 'react';
import { ChevronLeft, ChevronRight, ChevronsLeft, ChevronsRight } from 'lucide-react';
import { clsx } from 'clsx';

// ==----------------------------------------------------------------------------
// Types
// ==----------------------------------------------------------------------------

export interface PaginationProps {
  currentPage: number;
  totalPages: number;
  onPageChange: (page: number) => void;
  onPageSizeChange?: (size: number) => void;
  pageSize?: number;
  totalItems?: number;
  showPageSizeSelector?: boolean;
  pageSizes?: number[];
  className?: string;
}

// ==----------------------------------------------------------------------------
// Composant Pagination
// ==----------------------------------------------------------------------------

export const Pagination: React.FC<PaginationProps> = ({
  currentPage,
  totalPages,
  onPageChange,
  onPageSizeChange,
  pageSize = 20,
  totalItems = 0,
  showPageSizeSelector = false,
  pageSizes = [10, 20, 50, 100],
  className,
}) => {
  // Générer les numéros de page à afficher
  const getVisiblePages = (): (number | string)[] => {
    const visiblePages: (number | string)[] = [];

    if (totalPages <= 7) {
      // Afficher toutes les pages
      for (let i = 1; i <= totalPages; i++) {
        visiblePages.push(i);
      }
    } else {
      // Toujours afficher la première page
      visiblePages.push(1);

      // Calculer les pages autour de la page actuelle
      const start = Math.max(2, currentPage - 2);
      const end = Math.min(totalPages - 1, currentPage + 2);

      // Ajouter des points si nécessaire
      if (start > 2) {
        visiblePages.push('...');
      }

      // Ajouter les pages autour de la page actuelle
      for (let i = start; i <= end; i++) {
        visiblePages.push(i);
      }

      // Ajouter des points si nécessaire
      if (end < totalPages - 1) {
        visiblePages.push('...');
      }

      // Toujours afficher la dernière page
      visiblePages.push(totalPages);
    }

    return visiblePages;
  };

  const visiblePages = getVisiblePages();

  // Aller à la page précédente
  const goToPrevious = () => {
    if (currentPage > 1) {
      onPageChange(currentPage - 1);
    }
  };

  // Aller à la page suivante
  const goToNext = () => {
    if (currentPage < totalPages) {
      onPageChange(currentPage + 1);
    }
  };

  // Aller à la première page
  const goToFirst = () => {
    if (currentPage > 1) {
      onPageChange(1);
    }
  };

  // Aller à la dernière page
  const goToLast = () => {
    if (currentPage < totalPages) {
      onPageChange(totalPages);
    }
  };

  // Changer la taille de la page
  const handlePageSizeChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    onPageSizeChange?.(Number(e.target.value));
  };

  return (
    <div className={`flex items-center justify-between ${className || ''}`}>
      {/* Affichage du nombre total d'éléments */}
      <div className="flex items-center space-x-2 text-sm text-gray-500">
        <span>Total:</span>
        <span className="font-medium text-gray-900">{totalItems}</span>
        <span>élément{totalItems !== 1 ? 's' : ''}</span>
      </div>

      {/* Navigation */}
      <div className="flex items-center space-x-2">
        {/* Bouton première page */}
        <button
          onClick={goToFirst}
          disabled={currentPage <= 1}
          className="p-2 rounded-md text-gray-400 hover:bg-gray-100 disabled:opacity-50 disabled:cursor-not-allowed"
          aria-label="Première page"
        >
          <ChevronsLeft className="w-4 h-4" />
        </button>

        {/* Bouton page précédente */}
        <button
          onClick={goToPrevious}
          disabled={currentPage <= 1}
          className="p-2 rounded-md text-gray-400 hover:bg-gray-100 disabled:opacity-50 disabled:cursor-not-allowed"
          aria-label="Page précédente"
        >
          <ChevronLeft className="w-4 h-4" />
        </button>

        {/* Numéros de page */}
        <div className="flex items-center space-x-1">
          {visiblePages.map((page, index) => (
            <React.Fragment key={index}>
              {typeof page === 'number' ? (
                <button
                  onClick={() => onPageChange(page)}
                  className={clsx(
                    'px-3 py-2 text-sm rounded-md transition-colors',
                    currentPage === page
                      ? 'bg-blue-600 text-white font-medium'
                      : 'text-gray-700 hover:bg-gray-100'
                  )}
                  aria-current={currentPage === page ? 'page' : undefined}
                  aria-label={`Page ${page}`}
                >
                  {page}
                </button>
              ) : (
                <span className="px-3 py-2 text-sm text-gray-500">...</span>
              )}
            </React.Fragment>
          ))}
        </div>

        {/* Bouton page suivante */}
        <button
          onClick={goToNext}
          disabled={currentPage >= totalPages}
          className="p-2 rounded-md text-gray-400 hover:bg-gray-100 disabled:opacity-50 disabled:cursor-not-allowed"
          aria-label="Page suivante"
        >
          <ChevronRight className="w-4 h-4" />
        </button>

        {/* Bouton dernière page */}
        <button
          onClick={goToLast}
          disabled={currentPage >= totalPages}
          className="p-2 rounded-md text-gray-400 hover:bg-gray-100 disabled:opacity-50 disabled:cursor-not-allowed"
          aria-label="Dernière page"
        >
          <ChevronsRight className="w-4 h-4" />
        </button>
      </div>

      {/* Sélecteur de taille de page */}
      {showPageSizeSelector && onPageSizeChange && (
        <div className="flex items-center space-x-2">
          <span className="text-sm text-gray-500">Par page:</span>
          <select
            value={pageSize}
            onChange={handlePageSizeChange}
            className="text-sm border border-gray-300 rounded-md px-2 py-1"
          >
            {pageSizes.map((size) => (
              <option key={size} value={size}>
                {size}
              </option>
            ))}
          </select>
        </div>
      )}
    </div>
  );
};

// ==----------------------------------------------------------------------------
// Composant PaginationSimple
// ==----------------------------------------------------------------------------

export interface PaginationSimpleProps {
  currentPage: number;
  totalPages: number;
  onPageChange: (page: number) => void;
  className?: string;
}

export const PaginationSimple: React.FC<PaginationSimpleProps> = ({
  currentPage,
  totalPages,
  onPageChange,
  className,
}) => {
  return (
    <div className={`flex items-center justify-center space-x-2 ${className || ''}`}>
      <button
        onClick={() => onPageChange(currentPage - 1)}
        disabled={currentPage <= 1}
        className="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
      >
        Précédent
      </button>
      <span className="px-4 py-2 text-sm">
        Page {currentPage} / {totalPages}
      </span>
      <button
        onClick={() => onPageChange(currentPage + 1)}
        disabled={currentPage >= totalPages}
        className="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
      >
        Suivant
      </button>
    </div>
  );
};
