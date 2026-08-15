// =============================================================================
// Tardigrade-CI Git Module - Modal Context
// =============================================================================
//
// Contexte et types isolés afin que les fichiers de composants n'exportent
// que des composants React (règle `react-refresh/only-export-components`).
// =============================================================================

import React, { createContext } from 'react';

// ==----------------------------------------------------------------------------
// Types
// ==----------------------------------------------------------------------------

export interface ModalState {
  isOpen: boolean;
  title?: string;
  content?: React.ReactNode;
  size?: 'sm' | 'md' | 'lg' | 'xl' | 'full';
  footer?: React.ReactNode;
}

export interface ModalContextValue extends ModalState {
  openModal: (config: ModalState) => void;
  closeModal: () => void;
}

// ==----------------------------------------------------------------------------
// Contexte
// ==----------------------------------------------------------------------------

export const ModalContext = createContext<ModalContextValue | undefined>(undefined);
