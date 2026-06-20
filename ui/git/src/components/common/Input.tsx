import React from 'react';
import { cn } from '../../utils/cn';

export interface InputProps extends React.InputHTMLAttributes<HTMLInputElement> {
  label?: string;
  error?: string;
  hint?: string;
  leftIcon?: React.ReactNode;
  rightIcon?: React.ReactNode;
}

const Input = React.forwardRef<HTMLInputElement, InputProps>(
  (
    { className, type = 'text', label, error, hint, leftIcon, rightIcon, disabled, ...props },
    ref
  ) => {
    return (
      <div className="w-full">
        {label && (
          <label className="block text-sm font-medium text-secondary-700 mb-1">
            {label}
          </label>
        )}
        <div className="relative">
          {leftIcon && (
            <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
              {leftIcon}
            </div>
          )}
          <input
            ref={ref}
            type={type}
            disabled={disabled}
            className={cn(
              `
                block w-full rounded-lg border bg-white px-3 py-2
                text-secondary-900 placeholder:text-secondary-400
                transition-colors duration-200
                focus:outline-none focus:ring-2 focus:ring-offset-0
                disabled:cursor-not-allowed disabled:bg-secondary-50 disabled:text-secondary-500
              `,
              error
                ? 'border-red-500 focus:border-red-500 focus:ring-red-200'
                : 'border-secondary-300 focus:border-primary-500 focus:ring-primary-200',
              leftIcon && 'pl-10',
              rightIcon && 'pr-10',
              className
            )}
            {...props}
          />
          {rightIcon && (
            <div className="absolute inset-y-0 right-0 pr-3 flex items-center pointer-events-none">
              {rightIcon}
            </div>
          )}
        </div>
        {error && (
          <p className="mt-1 text-sm text-red-600">{error}</p>
        )}
        {hint && !error && (
          <p className="mt-1 text-sm text-secondary-500">{hint}</p>
        )}
      </div>
    );
  }
);

Input.displayName = 'Input';

export { Input };
