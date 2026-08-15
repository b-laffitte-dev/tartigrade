// =============================================================================
// Tardigrade-CI Git Module - ErrorMessage Component
// =============================================================================

import React from 'react';
import { AlertCircle, X, RefreshCw } from 'lucide-react';
import { Button } from './Button';

// ==----------------------------------------------------------------------------
// Types
// ==----------------------------------------------------------------------------

export interface ErrorMessageProps {
  error: string;
  onRetry?: () => void;
  onDismiss?: () => void;
  title?: string;
  fullPage?: boolean;
  className?: string;
}

// ==----------------------------------------------------------------------------
// Composant ErrorMessage
// ==----------------------------------------------------------------------------

export const ErrorMessage: React.FC<ErrorMessageProps> = ({
  error,
  onRetry,
  onDismiss,
  title = 'Erreur',
  fullPage = false,
  className,
}) => {
  const containerClasses = `
    bg-red-50 border border-red-200 rounded-lg p-4
    ${fullPage ? 'min-h-[400px] flex flex-col justify-center items-center' : ''}
    ${className || ''}
  `;

  return (
    <div className={containerClasses}>
      <div className={`flex items-start ${fullPage ? 'text-center' : ''}`}>
        <div className="flex-shrink-0">
          <AlertCircle className="w-5 h-5 text-red-500" />
        </div>
        <div className={`ml-3 ${fullPage ? 'mt-2' : ''}`}>
          <h3 className="text-lg font-semibold text-red-800">{title}</h3>
          <p className="text-red-600 mt-1">{error}</p>
        </div>
        {onDismiss && (
          <button
            onClick={onDismiss}
            className="ml-auto p-1 text-red-400 hover:text-red-600"
          >
            <X className="w-5 h-5" />
          </button>
        )}
      </div>
      {onRetry && (
        <div className={`mt-4 ${fullPage ? 'text-center' : 'flex justify-end'}`}>
          <Button
            variant="outline"
            onClick={onRetry}
            leftIcon={<RefreshCw className="w-4 h-4" />}
          >
            Réessayer
          </Button>
        </div>
      )}
    </div>
  );
};

// ==----------------------------------------------------------------------------
// Composant ErrorBoundary
// ==----------------------------------------------------------------------------

interface ErrorBoundaryProps {
  children: React.ReactNode;
  fallback?: React.ReactNode;
  onError?: (error: Error, errorInfo: React.ErrorInfo) => void;
}

interface ErrorBoundaryState {
  hasError: boolean;
  error: Error | null;
}

export class ErrorBoundary extends React.Component<ErrorBoundaryProps, ErrorBoundaryState> {
  constructor(props: ErrorBoundaryProps) {
    super(props);
    this.state = { hasError: false, error: null };
  }

  static getDerivedStateFromError(error: Error): ErrorBoundaryState {
    return { hasError: true, error };
  }

  componentDidCatch(error: Error, errorInfo: React.ErrorInfo): void {
    console.error('ErrorBoundary caught an error:', error, errorInfo);
    this.props.onError?.(error, errorInfo);
  }

  render(): React.ReactNode {
    if (this.state.hasError) {
      if (this.props.fallback) {
        return this.props.fallback;
      }

      return (
        <ErrorMessage
          error={this.state.error?.message || 'Une erreur inattendue est survenue'}
          onRetry={() => this.setState({ hasError: false, error: null })}
          fullPage
        />
      );
    }

    return this.props.children;
  }
}

// ==----------------------------------------------------------------------------
// Composant EmptyState
// ==----------------------------------------------------------------------------

export interface EmptyStateProps {
  title: string;
  description?: string;
  icon?: React.ReactNode;
  action?: {
    text: string;
    onClick: () => void;
  };
  className?: string;
}

export const EmptyState: React.FC<EmptyStateProps> = ({
  title,
  description,
  icon,
  action,
  className,
}) => {
  return (
    <div className={`text-center py-12 ${className || ''}`}>
      {icon && (
        <div className="w-16 h-16 bg-gray-100 rounded-full flex items-center justify-center mx-auto mb-6">
          {icon}
        </div>
      )}
      <h3 className="text-xl font-semibold text-gray-900">{title}</h3>
      {description && (
        <p className="text-gray-500 mt-2 max-w-md mx-auto">{description}</p>
      )}
      {action && (
        <div className="mt-6">
          <Button onClick={action.onClick}>
            {action.text}
          </Button>
        </div>
      )}
    </div>
  );
};
