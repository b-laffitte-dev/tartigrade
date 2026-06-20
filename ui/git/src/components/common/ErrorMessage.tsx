import React from 'react';
import { Button } from './Button';

export interface ErrorMessageProps {
  error: Error | null | undefined;
  onRetry?: () => void;
  message?: string;
}

const ErrorMessage: React.FC<ErrorMessageProps> = ({
  error,
  onRetry,
  message = 'Une erreur est survenue',
}) => {
  const errorMessage = error?.message || message;

  return (
    <div className="flex flex-col items-center justify-center py-12 px-4">
      <div className="text-center">
        <h3 className="text-lg font-semibold text-red-600 mb-2">Erreur</h3>
        <p className="text-secondary-600 mb-4">{errorMessage}</p>
        {onRetry && (
          <Button variant="outline" onClick={onRetry}>
            Réessayer
          </Button>
        )}
      </div>
    </div>
  );
};

export { ErrorMessage };
