import { validateDate } from "./utils/validator.js";


export const noticeFormValidation = () => {
  const dateInput = document.getElementById('date');
  const form = document.getElementById('vacancy-form');

  if (!dateInput || !form) return;

  dateInput.addEventListener('mouseout', () => validateDate(dateInput, "date-error"));

  form.addEventListener('submit', (event) => {
    const isDateValid = validateDate(dateInput, 'date-error');

    if (!isDateValid) return event.preventDefault();
  });
};
