import { closeModal, openModal } from "./utils/modal.js";

export const assignUnitModal = () => {
  const modal = document.getElementById('assign-unit-modal');

  if (!modal) return;

  document.addEventListener('click', (event) => {
    const btn = event.target.closest('.open-assign-tenant');
    if (!btn) return;
    modal.querySelector('input[name="unit_id"').value = btn.dataset.id;
    openModal(modal);
  });

  modal.addEventListener('click', (event) => {
    if (event.target === modal) {
      closeModal(modal);
    }
  });

  window.addEventListener('keydown', (event) => {
    if (event.key === 'Escape') {
      closeModal(modal);
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
