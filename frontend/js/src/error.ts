

export function setError(error: Error) {
  let errorMessage = document.querySelector('.error');
  if (!errorMessage) {
    throw new Error(".error not found");
  }
  errorMessage.innerHTML = error.toString();
}
