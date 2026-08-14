// =============================================================================
// Tardigrade-CI Git Module - Modal Provider
// =============================================================================
//
// Composant Provider isolé afin que les fichiers ne mélangent pas composants
// et contextes (règle `react-refresh/only-export-components`).
// Le contexte et les types vivent dans `modalContext.ts`.
// =============================================================================

import React, { useState } from 'react';
import { Modal } from './Modal';
import { ModalContext, type ModalState } from './modalContext';

interface ModalProviderProps {
  children: React.ReactNode;
}

export const ModalProvider: React.FC<ModalProviderProps> = ({ children }) => {
  const [modalState, setModalState] = useState<ModalState>({
    isOpen: false,
  });

  const openModal = (config: ModalState) => {
    setModalState({
      ...config,
      isOpen: true,
    });
  };

  const closeModal = () => {
    setModalState({
      isOpen: false,
    });
  };

  return (
    <ModalContext.Provider value={{ ...modalState, openModal, closeModal }}>
      {children}
      <Modal
        isOpen={modalState.isOpen}
        onClose={closeModal}
        title={modalState.title}
        size={modalState.size}
        footer={modalState.footer}
      >
        {modalState.content}
      </Modal>
    </ModalContext.Provider>
  );
};
