import { closeModal, openModal } from "./utils/modal.js";

export const initBuildingModal = () => {
  const addBtn = document.getElementById('open-add-modal');
  const modal = document.getElementById('add-building-modal');
  if (!addBtn || !modal) return;

  addBtn.addEventListener('click', () => openModal(modal));
  modal.addEventListener('click', (e) => { if (e.target === modal) closeModal(modal); });
  window.addEventListener('keydown', (e) => { if (e.key === 'Escape') closeModal(modal); });
};

export const assignBuildingModal = () => {
  const modal = document.getElementById('assign-building-modal');
  if (!modal) return;

  document.addEventListener('click', (e) => {
    const btn = e.target.closest('.open-assign-caretaker');
    if (!btn) return;
    openModal(modal);
  });

  modal.addEventListener('click', (e) => { if (e.target === modal) closeModal(modal); });
  window.addEventListener('keydown', (e) => { if (e.key === 'Escape') closeModal(modal); });
};

export const addUnitModal = () => {
  const modal = document.getElementById('add-unit-modal');
  if (!modal) return;

  document.addEventListener('click', (e) => {
    const btn = e.target.closest('.open-add-unit');
    if (!btn) return;
    modal.querySelector('input[name="building-id"]').value = btn.dataset.id;
    openModal(modal);
  });

  modal.addEventListener('click', (e) => {
    if (e.target === modal) closeModal(modal);
  });

  window.addEventListener('keydown', (e) => {
    if (e.key === 'Escape') closeModal(modal);
  });
};
