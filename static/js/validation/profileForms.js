export const profileValidation = () => {
  const firstNameInput = document.getElementById('update-fname');
  if (!firstNameInput) return;
  const lastNameInput = document.getElementById('update-lname');
  const emailInput = document.getElementById('update-email');
  const numberInput = document.getElementById('update-number');
  const passwordInput = document.getElementById('update-password');

  firstNameInput.addEventListener('change', () => validateName(firstNameInput, "update-fname-error"));
  lastNameInput.addEventListener('change', () => validateName(lastNameInput, "update-lname-error"));
  emailInput.addEventListener('change', () => validateEmail(emailInput, "update-email-error"));
  numberInput.addEventListener('change', () => validateNumber(numberInput, "update-number-error"));
  passwordInput.addEventListener('change', () => validatePassword(passwordInput, "update-password-error"));

  const profileForm = document.getElementById('profile-form');

  profileForm.addEventListener('submit', (event) => {
    const isFirstNameValid = validateName(firstNameInput, "fname-error");
    const isLastNameValid = validateName(lastNameInput, "lname-error");
    const isEmailValid = validateEmail(emailInput, "email-error");
    const isNumberValid = validateNumber(numberInput, "number-error");
    const isPasswordValid = validatePassword(passwordInput, "password-error");
    if (!isFirstNameValid || !isLastNameValid || !isEmailValid || !isNumberValid || !isPasswordValid) event.preventDefault();
  });
};
