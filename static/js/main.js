import { initAuthModals } from "./ui/authModalState.js";
import { initProfileModal } from "./ui/profileModalState.js";
import { assignBuildingModal, initBuildingModal } from "./ui/buildingModalState.js"
import { authValidation } from "./validation/authForms.js";
import { buildingValidation } from "./validation/buildingForm.js";
import { profileValidation } from "./validation/profileForms.js";
import { addUnitModal, assignUnitModal } from "./ui/unitModalState.js";
import { unitFormValidation } from "./validation/unitForm.js";
import { paymentModal, requestModal } from "./ui/tenantModalState.js";
import { requestFormValidation } from "./validation/requestForm.js";
import { paymentFormValidation } from "./validation/paymentForm.js";
import { noticeFormValidation } from "./validation/vacationForm.js";


initAuthModals();
initBuildingModal();
initProfileModal();
addUnitModal();
assignUnitModal();
assignBuildingModal();
requestModal();
paymentModal();

authValidation();
profileValidation();
buildingValidation();
unitFormValidation();
requestFormValidation();
paymentFormValidation();
noticeFormValidation();
