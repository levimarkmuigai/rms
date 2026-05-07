import { closeModal, openModal } from "./utils/modal.js";

export const initBuildingModal = () => {
  const addBtn = document.getElementById('open-add-modal');
  const buildingId = document.getElementById("add-building-modal");

  if (!addBtn) return;

  addBtn.addEventListener('click', () => {
    openModal(buildingId);
  });

  buildingId.addEventListener('click', (event) => {
    if (event.target === buildingId) {
      closeModal(buildingId);
    }
  });

  window.addEventListener('keydown', (event) => {
    if (event.key === 'Escape') {
      closeModal(buildingId);
    }
  });
};

export const assignBuildingModal = () => {
  const modalBtn = document.getElementById('open-assign-caretaker');
  const modal = document.getElementById('assign-building-modal');

  if (!modalBtn || !modal) return;

  modalBtn.addEventListener('click', () => {
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
