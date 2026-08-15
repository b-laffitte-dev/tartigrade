// =============================================================================
// Tardigrade-CI Git Module - Loading Component
// =============================================================================

import React from 'react';
import { clsx } from 'clsx';

// ==----------------------------------------------------------------------------
// Types
// ==----------------------------------------------------------------------------

export interface LoadingSpinnerProps {
  size?: 'sm' | 'md' | 'lg' | 'xl';
  color?: 'primary' | 'white' | 'gray';
  fullPage?: boolean;
  className?: string;
  text?: string;
}

// ==----------------------------------------------------------------------------
// Composant LoadingSpinner
// ==----------------------------------------------------------------------------

export const LoadingSpinner: React.FC<LoadingSpinnerProps> = ({
  size = 'md',
  color = 'primary',
  fullPage = false,
  className,
  text,
}) => {
  // Styles selon la taille
  const sizeClasses = {
    sm: 'w-4 h-4 border-2',
    md: 'w-6 h-6 border-2',
    lg: 'w-8 h-8 border-3',
    xl: 'w-12 h-12 border-4',
  };

  // Styles selon la couleur
  const colorClasses = {
    primary: 'border-blue-200 border-t-blue-600',
    white: 'border-white/30 border-t-white',
    gray: 'border-gray-200 border-t-gray-600',
  };

  // Conteneur
  const containerClasses = clsx(
    fullPage && 'fixed inset-0 flex items-center justify-center bg-white/80 backdrop-blur-sm z-50',
    !fullPage && 'flex items-center justify-center',
    className
  );

  return (
    <div className={containerClasses}>
      <div className="flex items-center space-x-3">
        <div
          className={`spinner animate-spin rounded-full ${sizeClasses[size]} ${colorClasses[color]}`}
          role="status"
          aria-label="Chargement"
        />
        {text && (
          <span
            className={`text-${size} ${
              color === 'white' ? 'text-white' : 'text-gray-600'
            }`}
          >
            {text}
          </span>
        )}
      </div>
    </div>
  );
};

// ==----------------------------------------------------------------------------
// Composant LoadingDots
// ==----------------------------------------------------------------------------

export interface LoadingDotsProps {
  size?: 'sm' | 'md' | 'lg';
  color?: 'primary' | 'white' | 'gray';
  className?: string;
}

export const LoadingDots: React.FC<LoadingDotsProps> = ({
  size = 'md',
  color = 'primary',
  className,
}) => {
  // Styles selon la taille
  const dotSizeClasses = {
    sm: 'w-1.5 h-1.5',
    md: 'w-2 h-2',
    lg: 'w-2.5 h-2.5',
  };

  // Styles selon la couleur
  const colorClasses = {
    primary: 'bg-blue-500',
    white: 'bg-white',
    gray: 'bg-gray-500',
  };

  return (
    <div className={clsx('flex space-x-1', className)}>
      <div
        className={`rounded-full ${dotSizeClasses[size]} ${colorClasses[color]} animate-bounce`}
        style={{ animationDelay: '0ms' }}
      />
      <div
        className={`rounded-full ${dotSizeClasses[size]} ${colorClasses[color]} animate-bounce`}
        style={{ animationDelay: '150ms' }}
      />
      <div
        className={`rounded-full ${dotSizeClasses[size]} ${colorClasses[color]} animate-bounce`}
        style={{ animationDelay: '300ms' }}
      />
    </div>
  );
};

// ==----------------------------------------------------------------------------
// Composant Skeleton
// ==----------------------------------------------------------------------------

export interface SkeletonProps {
  className?: string;
  lines?: number;
  width?: string | number;
  height?: string | number;
  variant?: 'text' | 'circular' | 'rectangular';
}

export const Skeleton: React.FC<SkeletonProps> = ({
  className,
  lines = 1,
  width,
  height,
  variant = 'rectangular',
}) => {
  // Styles selon la variante
  const variantClasses = {
    text: 'h-4 rounded',
    circular: 'rounded-full',
    rectangular: 'rounded',
  };

  // Générer les lignes
  if (lines > 1 && variant === 'text') {
    return (
      <div className={className}>
        {Array.from({ length: lines }).map((_, index) => (
          <div
            key={index}
            className={`bg-gray-200 animate-pulse ${variantClasses[variant]} mb-2`}
            style={{ width: width || (index === 0 ? '100%' : '80%') }}
          />
        ))}
      </div>
    );
  }

  return (
    <div
      className={clsx(
        'bg-gray-200 animate-pulse',
        variantClasses[variant],
        className
      )}
      style={{ width, height }}
    />
  );
};
