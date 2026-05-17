use std::sync::Arc;

use crate::{
    entities::user::Role,
    error::AppError,
    repositories::{notice_repo, unit_repo},
    server::{auth, form, request::Request, response::Response},
    state::AppState,
};

pub fn submit(req: &Request, state: &Arc<AppState>) -> Result<Response, AppError> {
    let sess = auth::require_role(req, &state.sessions, Role::Tenant)?;
    let f = form::parse(&req.body);

    let date = f
        .get("date")
        .cloned()
        .ok_or(AppError::BadRequest("moveout_date missing".into()))?;

    let unit_id = unit_repo::find_by_tenant(&state.db, &sess.user_id)?;

    if let Some(u_id) = unit_id {
        notice_repo::insert(&state.db, &u_id, &sess.user_id, date)?;
    }
    Ok(Response::redirect("/tenant"))
}
