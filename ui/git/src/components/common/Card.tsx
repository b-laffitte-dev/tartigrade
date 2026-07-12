// =============================================================================
// Tardigrade-CI Git Module - Card Component
// =============================================================================

import React from 'react';
import { clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';

// ==----------------------------------------------------------------------------
// Types
// ==----------------------------------------------------------------------------

export interface CardProps extends React.HTMLAttributes<HTMLDivElement> {
  variant?: 'default' | 'bordered' | 'elevated';
  padding?: 'none' | 'sm' | 'md' | 'lg';
  shadow?: 'none' | 'sm' | 'md' | 'lg';
}

interface CardHeaderProps extends React.HTMLAttributes<HTMLDivElement> {}

interface CardContentProps extends React.HTMLAttributes<HTMLDivElement> {}

interface CardFooterProps extends React.HTMLAttributes<HTMLDivElement> {}

interface CardComponent extends React.ForwardRefExoticComponent<CardProps & React.RefAttributes<HTMLDivElement>> {
  Header: React.ForwardRefExoticComponent<CardHeaderProps & React.RefAttributes<HTMLDivElement>>;
  Content: React.ForwardRefExoticComponent<CardContentProps & React.RefAttributes<HTMLDivElement>>;
  Footer: React.ForwardRefExoticComponent<CardFooterProps & React.RefAttributes<HTMLDivElement>>;
}

// ==----------------------------------------------------------------------------
// Composant Card
// ==----------------------------------------------------------------------------

export const Card = React.forwardRef<HTMLDivElement, CardProps>(
  (
    {
      className,
      variant = 'default',
      padding = 'md',
      shadow = 'md',
      children,
      ...props
    },
    ref
  ) => {
    // Styles selon la variante
    const variantClasses = {
      default: 'bg-white',
      bordered: 'bg-white border border-gray-200',
      elevated: 'bg-white shadow-lg',
    };

    // Styles selon le padding
    const paddingClasses = {
      none: '',
      sm: 'p-4',
      md: 'p-6',
      lg: 'p-8',
    };

    // Styles selon le shadow
    const shadowClasses = {
      none: '',
      sm: 'shadow-sm',
      md: 'shadow',
      lg: 'shadow-lg',
    };

    return (
      <div
        ref={ref}
        className={twMerge(
          clsx(
            'rounded-lg',
            variantClasses[variant],
            paddingClasses[padding],
            shadowClasses[shadow],
            'transition-shadow duration-200'
          ),
          className
        )}
        {...props}
      >
        {children}
      </div>
    );
  }
) as CardComponent;

Card.displayName = 'Card';

// ==----------------------------------------------------------------------------
// Sous-composants Card
// ==----------------------------------------------------------------------------

Card.Header = React.forwardRef<HTMLDivElement, CardHeaderProps>(
  ({ className, ...props }, ref) => (
    <div
      ref={ref}
      className={twMerge(clsx('pb-4 border-b border-gray-200', className))}
      {...props}
    />
  )
);

Card.Header.displayName = 'Card.Header';

Card.Content = React.forwardRef<HTMLDivElement, CardContentProps>(
  ({ className, ...props }, ref) => (
    <div
      ref={ref}
      className={twMerge(clsx('py-4', className))}
      {...props}
    />
  )
);

Card.Content.displayName = 'Card.Content';

Card.Footer = React.forwardRef<HTMLDivElement, CardFooterProps>(
  ({ className, ...props }, ref) => (
    <div
      ref={ref}
      className={twMerge(clsx('pt-4 border-t border-gray-200', className))}
      {...props}
    />
  )
);

Card.Footer.displayName = 'Card.Footer';
