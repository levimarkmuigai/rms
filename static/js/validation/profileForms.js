import { validateEmail, validateNumber, validatePassword, validateText } from "./utils/validator.js";

export const profileValidation = () => {
  const firstNameInput = document.getElementById('update-fname');
  const lastNameInput = document.getElementById('update-lname');
  const emailInput = document.getElementById('update-email');
  const numberInput = document.getElementById('update-number');
  const passwordInput = document.getElementById('update-password');
  const profileForm = document.getElementById('profile-form');

  if (!firstNameInput || !lastNameInput || !emailInput || !numberInput || !passwordInput || !profileForm) return;

  firstNameInput.addEventListener('mouseout', () => validateText(firstNameInput, 'update-fname-error'));
  lastNameInput.addEventListener('mouseout', () => validateText(lastNameInput, 'update-lname-error'));
  emailInput.addEventListener('mouseout', () => validateEmail(emailInput, 'update-email-error'));
  numberInput.addEventListener('mouseout', () => validateNumber(numberInput, 'update-number-error'));
  passwordInput.addEventListener('mouseout', () => validatePassword(passwordInput, 'update-password-error'));

  profileForm.addEventListener('submit', (event) => {
    const isFirstNameValid = validateText(firstNameInput, 'update-fname-error');
    const isLastNameValid = validateText(lastNameInput, 'update-lname-error');
    const isEmailValid = validateEmail(emailInput, 'update-email-error');
    const isNumberValid = validateNumber(numberInput, 'update-number-error');
    const isPasswordValid = validatePassword(passwordInput, 'update-password-error');

    if (!isFirstNameValid || !isLastNameValid || !isEmailValid || !isNumberValid || !isPasswordValid) {
      event.preventDefault();
    }
  });
};
