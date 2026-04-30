import { initAuthModals } from "./ui/authModalState.js";
import { initProfileModal } from "./ui/profileModalState.js";
import { authValidation } from "./validation/authForms.js";
import { profileValidation } from "./validation/profileForms.js";

initAuthModals();
initProfileModal();
authValidation();
profileValidation();
