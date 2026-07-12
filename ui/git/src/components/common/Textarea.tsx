// =============================================================================
// Tardigrade-CI Git Module - Textarea Component
// =============================================================================

import React from 'react';
import { clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';

// ==----------------------------------------------------------------------------
// Types
// ==----------------------------------------------------------------------------

export interface TextareaProps extends React.TextareaHTMLAttributes<HTMLTextAreaElement> {
  label?: string;
  error?: string;
  helperText?: string;
  characterCount?: boolean;
  maxLength?: number;
  fullWidth?: boolean;
}

// ==----------------------------------------------------------------------------
// Composant Textarea
// ==----------------------------------------------------------------------------

export const Textarea = React.forwardRef<HTMLTextAreaElement, TextareaProps>(
  (
    {
      className,
      label,
      error,
      helperText,
      characterCount = false,
      maxLength,
      fullWidth = true,
      disabled,
      id,
      rows = 3,
      ...props
    },
    ref
  ) => {
    const textareaId = id || `textarea-${Math.random().toString(36).substr(2, 9)}`;
    const [value, setValue] = React.useState<string>(String(props.value || ''));

    // Synchroniser la valeur avec le prop value
    React.useEffect(() => {
      setValue(String(props.value || ''));
    }, [props.value]);

    const handleChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
      if (maxLength && e.target.value.length > maxLength) {
        return;
      }
      setValue(e.target.value);
      props.onChange?.(e);
    };

    const currentLength = value.length;
    const showCharacterCount = characterCount || maxLength;

    return (
      <div className={clsx('flex flex-col', fullWidth && 'w-full')}>
        {label && (
          <label
            htmlFor={textareaId}
            className="block text-sm font-medium text-gray-700 mb-2"
          >
            {label}
          </label>
        )}
        <textarea
          id={textareaId}
          ref={ref}
          className={twMerge(
            clsx(
              'input block w-full rounded-md border-gray-300 shadow-sm',
              'focus:border-blue-500 focus:ring-blue-500',
              'disabled:bg-gray-100 disabled:cursor-not-allowed',
              error && 'border-red-300 focus:border-red-500 focus:ring-red-500',
              'resize-none'
            ),
            className
          )}
          rows={rows}
          disabled={disabled}
          maxLength={maxLength}
          value={value}
          onChange={handleChange}
          aria-invalid={error ? 'true' : 'false'}
          aria-describedby={error ? `${textareaId}-error` : helperText ? `${textareaId}-helper` : undefined}
          {...props}
        />
        <div className="flex justify-between">
          {error && (
            <p id={`${textareaId}-error`} className="mt-1 text-sm text-red-600">
              {error}
            </p>
          )}
          {showCharacterCount && !error && (
            <p className="mt-1 text-xs text-gray-500">
              {currentLength}
              {maxLength && `/${maxLength}`}
            </p>
          )}
        </div>
        {helperText && !error && !showCharacterCount && (
          <p id={`${textareaId}-helper`} className="mt-1 text-sm text-gray-500">
            {helperText}
          </p>
        )}
      </div>
    );
  }
);

Textarea.displayName = 'Textarea';
