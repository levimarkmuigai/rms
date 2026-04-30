use std::sync::Arc;

use crate::{
    entities::user::Role,
    error::AppError,
    server::{auth, form, request::Request, response::Response},
    services::user_service,
    state::AppState,
};

pub fn update_profile(req: &Request, state: &Arc<AppState>) -> Result<Response, AppError> {
    let sess = auth::require_any_role(req, &state.sessions)?;
    let f = form::parse(&req.body);

    let first = f.get("first-name").cloned().unwrap_or_default();
    let last = f.get("last-name").cloned().unwrap_or_default();
    let email = f.get("email").cloned().unwrap_or_default();
    let number = f.get("number").cloned().unwrap_or_default();
    let name = format!("{first} {last}");

    user_service::update(&state.db, &sess.user_id, name, email, number)?;

    let redirect = match sess.role {
        Role::Landlord => "/landlord",
        Role::Caretaker => "/caretaker",
        Role::Tenant => "/tenant",
    };

    Ok(Response::redirect(redirect))
}

