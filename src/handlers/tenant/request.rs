use std::sync::Arc;

use uuid::Uuid;

use crate::{
    entities::user::Role,
    error::AppError,
    repositories::{maintenance_repo, unit_repo},
    server::{auth, form, request::Request, response::Response},
    state::AppState,
};

pub fn submit(req: &Request, state: &Arc<AppState>) -> Result<Response, AppError> {
    let sess = auth::require_role(req, &state.sessions, Role::Tenant)?;
    let f = form::parse(&req.body);

    let unit_id: Uuid = unit_repo::find_by_tenant(&state.db, &sess.user_id)?
        .ok_or(AppError::BadRequest("no unit assigned".into()))?;

    let desc = f
        .get("description")
        .filter(|v| !v.trim().is_empty())
        .cloned()
        .ok_or(AppError::BadRequest("description is required".into()))?;

    maintenance_repo::insert(&state.db, &unit_id, &sess.user_id, &desc)?;

    Ok(Response::redirect("/tenant"))
}
