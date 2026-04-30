import { closeModal, openModal } from "./utils/modal.js";

export const initProfileModal = () => {
  const profileBtn = document.getElementById('profile-btn');
  const profileId = document.getElementById('profile-modal');

  if (!profileBtn || !profileId) return;

  profileBtn.addEventListener('click', () => {
    openModal(profileId);
  });

  profileId.addEventListener('click', (event) => {
    if (event.target === profileId) {
      closeModal(profileId);
    }
  });


  window.addEventListener('keydown', (event) => {
    if (event.key === 'Escape') {
      closeModal(profileId);
    }
  });
};
