// =============================================================================
// Tardigrade-CI Git Module - Badge Component
// =============================================================================

import React from 'react';
import { clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';

// ==----------------------------------------------------------------------------
// Types
// ==----------------------------------------------------------------------------

export interface BadgeProps extends React.HTMLAttributes<HTMLSpanElement> {
  variant?: 'default' | 'primary' | 'secondary' | 'success' | 'warning' | 'error' | 'info';
  size?: 'xs' | 'sm' | 'md' | 'lg';
  dot?: boolean;
}

// ==----------------------------------------------------------------------------
// Composant Badge
// ==----------------------------------------------------------------------------

export const Badge = React.forwardRef<HTMLSpanElement, BadgeProps>(
  ({ className, variant = 'default', size = 'sm', dot = false, children, ...props }, ref) => {
    // Styles selon la variante
    const variantClasses = {
      default: 'bg-gray-100 text-gray-700',
      primary: 'bg-blue-100 text-blue-700',
      secondary: 'bg-gray-50 text-gray-600',
      success: 'bg-green-100 text-green-700',
      warning: 'bg-yellow-100 text-yellow-700',
      error: 'bg-red-100 text-red-700',
      info: 'bg-blue-100 text-blue-700',
    };

    // Styles selon la taille
    const sizeClasses = {
      xs: 'px-2 py-0.5 text-xs',
      sm: 'px-2.5 py-0.5 text-xs',
      md: 'px-3 py-1 text-sm',
      lg: 'px-4 py-1.5 text-sm',
    };

    // Dot color selon la variante
    const dotClasses = {
      default: 'bg-gray-500',
      primary: 'bg-blue-500',
      secondary: 'bg-gray-400',
      success: 'bg-green-500',
      warning: 'bg-yellow-500',
      error: 'bg-red-500',
      info: 'bg-blue-500',
    };

    return (
      <span
        ref={ref}
        className={twMerge(
          clsx(
            'inline-flex items-center font-medium rounded-full',
            variantClasses[variant],
            sizeClasses[size],
            'capitalize'
          ),
          className
        )}
        {...props}
      >
        {dot && (
          <span
            className={`w-1.5 h-1.5 rounded-full ${dotClasses[variant]} mr-1.5`}
            aria-hidden="true"
          />
        )}
        {children}
      </span>
    );
  }
);

Badge.displayName = 'Badge';
