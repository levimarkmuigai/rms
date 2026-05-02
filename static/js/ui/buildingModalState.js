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
