use std::sync::Arc;

use uuid::Uuid;

use crate::{
    entities::user::Role,
    error::AppError,
    server::{auth, form, request::Request, response::Response},
    services::unit_services,
    state::AppState,
};

pub fn add(req: &Request, state: &Arc<AppState>) -> Result<Response, AppError> {
    let sess = auth::require_role(req, &state.sessions, Role::Landlord)?;
    let f = form::parse(&req.body);

    let unit_number = f.get("unit-number").cloned().unwrap_or_default();
    let rent_amount: i32 = f
        .get("rent-amount")
        .and_then(|v| v.parse().ok())
        .ok_or_else(|| AppError::BadRequest("missing rent_amount".into()))?;
    let building_id: Uuid = f
        .get("building-id")
        .and_then(|v| v.parse().ok())
        .ok_or_else(|| AppError::BadRequest("missing building_id".into()))?;

    unit_services::add(&state.db, &building_id, &unit_number, rent_amount)?;

    tracing::info!(user_id = %sess.user_id, %building_id, "unit added");

    Ok(Response::redirect("/landlord/buildings"))
}
