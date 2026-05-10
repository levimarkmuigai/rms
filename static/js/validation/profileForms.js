export const profileValidation = () => {
  const firstNameInput = document.getElementById('update-fname');
  const lastNameInput = document.getElementById('update-lname');
  const emailInput = document.getElementById('update-email');
  const numberInput = document.getElementById('update-number');
  const passwordInput = document.getElementById('update-password');
  const profileForm = document.getElementById('profile-form');

  // all or nothing — if any element is missing, the form isn't on this page
  if (!firstNameInput || !lastNameInput || !emailInput || !numberInput || !passwordInput || !profileForm) return;

  firstNameInput.addEventListener('change', () => validateText(firstNameInput, 'update-fname-error'));
  lastNameInput.addEventListener('change', () => validateText(lastNameInput, 'update-lname-error'));
  emailInput.addEventListener('change', () => validateEmail(emailInput, 'update-email-error'));
  numberInput.addEventListener('change', () => validateNumber(numberInput, 'update-number-error'));
  passwordInput.addEventListener('change', () => validatePassword(passwordInput, 'update-password-error'));

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
