import { openModal, closeModal } from "./utils/modal.js";


export const requestModal = () => {
  const modalBtn = document.getElementById('report-btn');
  const requestModal = document.getElementById('request-modal');

  if (!modalBtn || !requestModal) return;

  modalBtn.addEventListener('click', () => {
    openModal(requestModal);
  });

  requestModal.addEventListener('click', (event) => {
    if (event.target === requestModal) {
      closeModal(requestModal);
    }
  });

  window.addEventListener('keydown', (event) => {
    if (event.key === 'Escape') {
      closeModal(requestModal);
    }
  });
};

export const RentModal = () => {
  const modalBtn = document.getElementById('rent-btn');
  const rentModal = document.getElementById('rent-modal');

  if (!modalBtn || !rentModal) return;

  modalBtn.addEventListener('click', () => {
    openModal(rentModal);
  });

  rentModal.addEventListener('click', (event) => {
    if (event.target === rentModal) {
      closeModal(rentModal);
    }
  });

  window.addEventListener('keydown', (event) => {
    if (event.key === 'Escape') {
      closeModal(rentModal);
    }
  });
};
