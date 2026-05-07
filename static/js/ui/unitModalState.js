import { closeModal, openModal } from "./utils/modal.js";

export const assignUnitModal = () => {
  const modalBtn = document.getElementById('open-assign-tenant');
  const unitModal = document.getElementById('assign-unit-modal');

  if (!modalBtn || !unitModal) return;

  modalBtn.addEventListener('click', () => {
    openModal(unitModal);
  });

  unitModal.addEventListener('click', (event) => {
    if (event.target === unitModal) {
      closeModal(unitModal);
    }
  });

  window.addEventListener('keydown', (event) => {
    if (event.key === 'Escape') {
      closeModal(unitModal);
    }
  });
};

export const addUnitModal = () => {
  const modalBtn = document.getElementById('open-add-unit');
  const unitModal = document.getElementById('add-unit-modal');

  if (!modalBtn || !unitModal) return;

  modalBtn.addEventListener('click', () => {
    openModal(unitModal);
  });

  unitModal.addEventListener('click', (event) => {
    if (event.target === unitModal) {
      closeModal(unitModal)
    }
  });

  window.addEventListener('keydown', (event) => {
    if (event.key === 'Escape') {
      closeModal(unitModal)
    }
  });
};
