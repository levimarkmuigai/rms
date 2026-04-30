export const authValidation = () => {
  const firstNameInput = document.getElementById('first-name');
  if (!firstNameInput) return;
  const lastNameInput = document.getElementById('last-name');
  const emailInput = document.getElementById('email');
  const numberInput = document.getElementById('number');
  const roleSelect = document.getElementById('role');
  const passwordInput = document.getElementById('signup-password');

  firstNameInput.addEventListener('change', () => validateName(firstNameInput, "fname-error"));
  lastNameInput.addEventListener('change', () => validateName(lastNameInput, "lname-error"));
  emailInput.addEventListener('change', () => validateEmail(emailInput, "email-error"));
  numberInput.addEventListener('change', () => validateNumber(numberInput, "number-error"));
  roleSelect.addEventListener('change', () => validateRole(roleSelect, "role-error"));
  passwordInput.addEventListener('change', () => validatePassword(passwordInput, "password-error"));

  const signupForm = document.getElementById('signup-form');

  signupForm.addEventListener('submit', (event) => {
    const isFirstNameValid = validateName(firstNameInput, "fname-error");
    const isLastNameValid = validateName(lastNameInput, "lname-error");
    const isEmailValid = validateEmail(emailInput, "email-error");
    const isNumberValid = validateNumber(numberInput, "number-error");
    const isRoleValid = validateRole(roleSelect, "role-error");
    const isPasswordValid = validatePassword(passwordInput, "password-error");

    if (!isFirstNameValid || !isLastNameValid || !isEmailValid || !isNumberValid || !isRoleValid || !isPasswordValid) event.preventDefault();
  });
};
