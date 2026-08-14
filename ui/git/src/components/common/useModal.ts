// =============================================================================
// Tardigrade-CI Git Module - useModal Hook
// =============================================================================

import { useContext } from 'react';
import { ModalContext, type ModalContextValue } from './modalContext';

export function useModal(): ModalContextValue {
  const context = useContext(ModalContext);
  if (!context) {
    throw new Error('useModal doit être utilisé dans un ModalProvider');
  }
  return context;
}
