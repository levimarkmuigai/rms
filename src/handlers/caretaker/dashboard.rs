use std::{collections::HashMap, sync::Arc};

use crate::{
    entities::user::Role,
    error::AppError,
    server::{auth, request::Request, response::Response},
    services::caretaker::dashboard_services,
    state::AppState,
    templates::engine,
};

const DASH_HTML: &str = include_str!("../../templates/views/caretaker/dashboard.html");

pub fn show(req: &Request, state: &Arc<AppState>) -> Result<Response, AppError> {
    let sess = auth::require_role(req, &state.sessions, Role::Caretaker)?;
    let overview_metrics = dashboard_services::dash_overview(&state.db, &sess.user_id)?;

    let mut ctx = HashMap::new();
    ctx.insert("caretaker_name", sess.name);
    ctx.insert("pending_count", overview_metrics.0.to_string());
    ctx.insert("inprogress_count", overview_metrics.1.to_string());
    ctx.insert("resolved_count", overview_metrics.2.to_string());
    Ok(Response::html(200, engine::render(DASH_HTML, &ctx)))
}
