import { validateText } from "./utils/validator.js";

export const buildingValidation = () => {
  const nameInput = document.getElementById('add-building-name');
  const buildingForm = document.getElementById('add-building-form');

  if (!buildingForm || !nameInput) return;

  nameInput.addEventListener('mouseout', () => validateText(nameInput, "name-error"));

  buildingForm.addEventListener('submit', (event) => {
    const isNameValid = validateText(nameInput, "name-error");
    if (!isNameValid) event.preventDefault();
  });
};
