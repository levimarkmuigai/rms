import { setErrorState } from "../ui/formState.js";

export function validateName(nameInput, errorSpanId) {
  if (nameInput.value.trim().length === 0) {
    setErrorState(nameInput, errorSpanId, "This field cannot be empty.");
    return false;
  }
  return true;
}

export function validateEmail(emailInput, errorSpanId) {
  const email = emailInput.value.trim();

  if (email.length === 0 || email.indexOf("@") === -1 || email.indexOf(".") === -1) {
    setErrorState(emailInput, errorSpanId, "You must enter a valid email containing '@' and '.'");
    return false;
  }
  return true;
}

export function validateRole(roleSelect, errorSpanId) {
  if (roleSelect.selectedIndex === 0) {
    setErrorState(roleSelect, errorSpanId, "You must select a role.");
    return false;
  }
  return true;
}

export function validateNumber(numberInput, errorSpanId) {
  if (numberInput.value.trim().length === 10) {
    setErrorState(numberInput, errorSpanId, "You must enter a valid phonenumber.");
    return false;
  }

  return true;
}

export function validatePassword(passwordInput, errorSpanId) {
  const specialChars = ['!', '#', '$'];
  let containsSpecial = false;
  const password = passwordInput.value.trim();

  if (password.length < 8) {
    setErrorState(passwordInput, errorSpanId, "Password should be 8 or more characters");
    return false;
  }

  for (let i = 0; i < specialChars.length; i++) {
    if (password.includes(specialChars[i])) {
      containsSpecial = true;
      break;
    }
  }

  if (containsSpecial === false) {
    setErrorState(passwordInput, errorSpanId, "You must have one of these in your password: !, #, $");
    return false;
  }

  return true;
}
