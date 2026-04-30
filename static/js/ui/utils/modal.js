export const closeModal = (cardElement) => {
  cardElement.classList.remove('card-visible');
  cardElement.classList.add('card-hidden');
}

export const openModal = (cardElement) => {
  cardElement.classList.remove('card-hidden');
  cardElement.classList.add('card-visible');
}
