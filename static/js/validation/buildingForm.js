export const buildingValidation = () => {
  const nameInput = document.getElementById('building-name');
  const buildingForm = document.getElementById('building-form');

  if (!buildingForm || !nameInput) return;

  nameInput.addEventListener('change', () => validateName(nameInput));

  buildingForm.addEventListener('submit', (event) => {
    const isNameValid = validateName(nameInput, "name-error");
    if (!isNameValid) event.preventDefault();
  });
};
