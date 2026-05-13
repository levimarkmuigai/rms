import { validateText } from "./utils/validator.js";

export const requestFormValidation = () => {
  const descInput = document.getElementById('description');
  const form = document.getElementById('request-form');

  if (!descInput || !form) return;

  descInput.addEventListener('mouseout', () => validateText(descInput, 'desc-error'));

  form.addEventListener('submit', (event) => {
    const isDescInput = validateText(descInput, 'desc-error');

    if (!isDescInput) return event.preventDefault();
  });
};
