use std::sync::Arc;

use crate::{
    entities::user::Role,
    error::AppError,
    repositories::{payment_repo, unit_repo},
    server::{auth, form, request::Request, response::Response},
    state::AppState,
};

pub fn submit(req: &Request, state: &Arc<AppState>) -> Result<Response, AppError> {
    let sess = auth::require_role(req, &state.sessions, Role::Tenant)?;
    let f = form::parse(&req.body);

    let month_year = f
        .get("month_year")
        .cloned()
        .ok_or(AppError::BadRequest("month_year".into()))?;
    let payment_details = unit_repo::payment_details(&state.db, &sess.user_id)?;

    payment_repo::insert(&state.db, &payment_details.0, payment_details.1, month_year)?;
    Ok(Response::redirect("/tenant"))
}
