use std::sync::Arc;

use crate::{
    entities::user::Role,
    error::AppError,
    repositories::{activity_repo, payment_repo, unit_repo},
    server::{auth, form, request::Request, response::Response},
    services::mpesa_services,
    state::AppState,
};

pub fn initiate(req: &Request, state: &Arc<AppState>) -> Result<Response, AppError> {
    let sess = auth::require_role(req, &state.sessions, Role::Tenant)?;
    let f = form::parse(&req.body);

    let month_year = f
        .get("month_year")
        .cloned()
        .ok_or(AppError::BadRequest("missing month_year".into()))?;

    let phone = f
        .get("phone")
        .cloned()
        .ok_or(AppError::BadRequest("missing phone".into()))?;

    let unit_id = unit_repo::find_by_tenant(&state.db, &sess.user_id)?
        .ok_or(AppError::BadRequest("unit missing".into()))?;

    let unit = unit_repo::find_by_id(&state.db, &unit_id)?
        .ok_or(AppError::BadRequest("unit row missing".into()))?;

    let checkout_request_id =
        mpesa_services::stk_push(&state.cfg, &phone, unit.rent_amount, &unit.id.to_string())?;

    payment_repo::insert_pending(
        &state.db,
        &unit.id,
        unit.rent_amount,
        month_year,
        &checkout_request_id,
    )?;

    activity_repo::insert(&state.db, &sess.user_id, "initiated rent payment")?;
    Ok(Response::redirect("/tenant"))
}
