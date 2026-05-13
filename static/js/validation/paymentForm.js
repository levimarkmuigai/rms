/**import { validateNumber } from "./utils/validator.js";

export const paymentFormValidation = () => {
  const numberInput = document.getElementById('phone');
  const form = document.getElementById('payment-form');

  if (!numberInput || !form) return;

  numberInput.addEventListener('mouseout', () => validateNumber(numberInput, 'phone-error'));

  form.addEventListener('submit', (event) => {
    const isNumberInput = validateNumber(numberInput, 'phone-error');

    if (!isNumberInput) return event.preventDefault();
  });
};*/
