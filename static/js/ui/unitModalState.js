import { closeModal, openModal } from "./utils/modal.js";

export const initUnitModal = () => {
  const modalBtn = document.getElementById('open-add-unit');
  const unitModal = document.getElementById('add-unit-modal');

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
