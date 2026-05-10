import { validateText } from "./utils/validator.js";


export const unitFormValidation = () => {
  const numberInput = document.getElementById('unit-number');
  const rentInput = document.getElementById("rent-amount");
  const form = document.getElementById("add-unit-form");
  if (!numberInput || !rentInput || !form) return;

  numberInput.addEventListener('change', () => validateText(nameInput, "unit-number-error"));
  rentInput.addEventListener(('change'), () => validateText(rentInput, "rent-amount-error"));

  form.addEventListener('submit', (event) => {
    const isNumberInput = validateText(numberInput, "unit-number-error");
    const isRentInput = validateText(rentInput, "rent-amount-error");
    if (!isNumberInput || !isRentInput) return event.preventDefault();
  })
};
