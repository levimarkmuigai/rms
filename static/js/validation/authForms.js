import { validateEmail, validateNumber, validatePassword, validateSelect, validateText } from "./utils/validator.js";

export const authValidation = () => {
  const firstNameInput = document.getElementById('first-name');
  if (!firstNameInput) return;
  const lastNameInput = document.getElementById('last-name');
  const emailInput = document.getElementById('email');
  const numberInput = document.getElementById('number');
  const roleSelect = document.getElementById('role');
  const passwordInput = document.getElementById('signup-password');

  firstNameInput.addEventListener('input', () => validateText(firstNameInput, "fname-error"));
  lastNameInput.addEventListener('input', () => validateText(lastNameInput, "lname-error"));
  emailInput.addEventListener('input', () => validateEmail(emailInput, "email-error"));
  numberInput.addEventListener('input', () => validateNumber(numberInput, "number-error"));
  roleSelect.addEventListener('input', () => validateSelect(roleSelect, "role-error"));
  passwordInput.addEventListener('input', () => validatePassword(passwordInput, "password-error"));

  const signupForm = document.getElementById('signup-form');

  signupForm.addEventListener('submit', (event) => {
    const isFirstNameValid = validateText(firstNameInput, "fname-error");
    const isLastNameValid = validateText(lastNameInput, "lname-error");
    const isEmailValid = validateEmail(emailInput, "email-error");
    const isNumberValid = validateNumber(numberInput, "number-error");
    const isRoleValid = validateSelect(roleSelect, "role-error");
    const isPasswordValid = validatePassword(passwordInput, "password-error");

    if (!isFirstNameValid || !isLastNameValid || !isEmailValid || !isNumberValid || !isRoleValid || !isPasswordValid) event.preventDefault();
  });
};
