import { initAuthModals } from "./ui/authModalState.js";
import { initProfileModal } from "./ui/profileModalState.js";
import { initBuildingModal } from "./ui/buildingModalState.js"
import { authValidation } from "./validation/authForms.js";
import { buildingValidation } from "./validation/buildingForm.js";
import { profileValidation } from "./validation/profileForms.js";

initAuthModals();
initBuildingModal();
initProfileModal();
authValidation();
profileValidation();
buildingValidation();
