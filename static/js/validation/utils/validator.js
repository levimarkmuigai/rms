import { setErrorState } from "../../ui/formState.js";

export function validateText(textInput, errorSpanId) {
  if (textInput.value.trim().length === 0) {
    setErrorState(textInput, errorSpanId, "This field cannot be empty.");
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

export function validateSelect(select, errorSpanId) {
  if (select.selectedIndex === 0) {
    setErrorState(select, errorSpanId, "You must select a field.");
    return false;
  }
  return true;
}

export function validateNumber(numberInput, errorSpanId) {
  if (numberInput.value.trim().length !== 10) {
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

export function validateDate(dateInput, errorSpanId) {
  const date = dateInput.value.trim();

  if (date.indexOf("-") === -1) {
    setErrorState(dateInput, errorSpanId, "date must be entered in the dd-mm-yyyy format.");
    return false;
  }

  const parts = date.split("-");

  if (parts.length < 3 || parts[0].length < 1 || parts[1].length < 1 || parts[2].length != 4) {
    setErrorState(dateInput, errorSpanId, "date must be entered in the dd-mm-yyyy format.");
    return false;
  }

  if (isNaN(parts[0]) || isNaN(parts[1]) || isNaN(parts[2])) {
    setErrorState(dateInput, errorSpanId, "date must be numbers");
    return false;

  }

  if (parts[0] > 31) {
    setErrorState(dateInput, errorSpanId, "date out of bounds");
    return false;
  }

  if (parts[1] > 12) {
    setErrorState(dateInput, errorSpanId, "month out of bounds");
    return false;
  }

  const today = new Date();
  today.setHours(0, 0, 0, 0);

  const targetDate = new Date(parts[2], parts[1] - 1, parts[0]);

  const milisecondsPerDay = 1000 * 60 * 60 * 24;
  const differenceInDays = (targetDate.getTime() - today.getTime()) / milisecondsPerDay;

  if (differenceInDays < 30) {
    setErrorState(dateInput, errorSpanId, "move-out date has to be more than 30 days from today");
    return false;
  }

  return true;
}
