export function setErrorState(inputElement, errorSpanId, errorMessage) {
  const errorSpan = document.getElementById(errorSpanId);

  if (errorSpan) {
    errorSpan.innerText = errorMessage;
    inputElement.style.borderColor = "red";
  }
}

export function clearErrorState(inputElement, errorSpanId) {
  const errorSpan = document.getElementById(errorSpanId);

  if (errorSpan) {
    errorSpan.innerText = "";
    inputElement.style.borderColor = "initial";
  }
}
