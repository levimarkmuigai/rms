export const unitFormValidation = () => {
  const numberInput = document.getElementById('unit-number');
  const rentInput = document.getElementById("rent-amount");
  const form = document.getElementById("add-unit-form");
  if (!numberInput || !rentInput || !form) return;

  numberInput.addEventListener('change', () => validateName(nameInput, "unit-number-error"));
  rentInput.addEventListener(('change'), () => validateName(rentInput, "rent-amount-error"));

  form.addEventListener('submit', (event) => {
    const isNumberInput = validateName(numberInput, "unit-number-error");
    const isRentInput = validateName(rentInput, "rent-amount-error");
    if (!isNumberInput || !isRentInput) return event.preventDefault();
  })
};
