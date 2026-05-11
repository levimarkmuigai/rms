use std::{collections::HashMap, sync::Arc, time::SystemTime};

use uuid::Uuid;

use crate::{
    entities::user::Role,
    error::AppError,
    server::{auth, form, request::Request, response::Response},
    services::caretaker::dashboard_services::{self, PanelRequests},
    state::AppState,
    templates::engine,
};

const DASH_HTML: &str = include_str!("../../templates/views/caretaker/dashboard.html");

pub fn show(req: &Request, state: &Arc<AppState>) -> Result<Response, AppError> {
    let sess = auth::require_role(req, &state.sessions, Role::Caretaker)?;
    let overview_metrics = dashboard_services::dash_overview(&state.db, &sess.user_id)?;
    let requests = dashboard_services::request_panel(&state.db, &sess.user_id)?;

    let (pending, inprogress) = request_panel(requests);
    let mut ctx = HashMap::new();
    ctx.insert("caretaker_name", sess.name);
    ctx.insert("pending_count", overview_metrics.0.to_string());
    ctx.insert("inprogress_count", overview_metrics.1.to_string());
    ctx.insert("resolved_count", overview_metrics.2.to_string());

    ctx.insert("pending_card", pending);
    ctx.insert("inprogress_card", inprogress);
    Ok(Response::html(200, engine::render(DASH_HTML, &ctx)))
}

pub fn inprogress(req: &Request, state: &Arc<AppState>) -> Result<Response, AppError> {
    let f = form::parse(&req.body);
    let id: Uuid = f
        .get("request_id")
        .and_then(|v| v.parse().ok())
        .ok_or(AppError::BadRequest("request_id missing".into()))?;
    dashboard_services::to_inprogress(&state.db, &id)?;
    Ok(Response::redirect("/caretaker"))
}

pub fn resolve(req: &Request, state: &Arc<AppState>) -> Result<Response, AppError> {
    let f = form::parse(&req.body);
    let id: Uuid = f
        .get("request_id")
        .and_then(|v| v.parse().ok())
        .ok_or(AppError::BadRequest("request_id missing".into()))?;

    dashboard_services::to_resolved(&state.db, &id)?;
    Ok(Response::redirect("/caretaker"))
}

fn request_panel(r: Vec<PanelRequests>) -> (String, String) {
    let pending_requests: Vec<_> = r.iter().filter(|r| r.status == "pending").collect();
    let inprogress_requests: Vec<_> = r.iter().filter(|r| r.status == "in_progress").collect();

    let pending_html: String = pending_requests
        .into_iter()
        .map(|r| {
            format!(
                r#"
            <div class="request-card-body">
            <span class="request-desc">{desc}</span>
            <span class="req-unit">{unit}</span>
            <span class="req-timestamp">{timestamp}</span>
            </div>
            <form action="/caretaker/request/start" method="POST">
            <input type="hidden" name="request_id" value="{request_id}">
            <button type="submit">start</button>
            </form>
            "#,
                desc = r.desc,
                unit = r.unit,
                timestamp = time_ago(r.timestamp),
                request_id = r.id,
            )
        })
        .collect();

    let inprogress_html: String = inprogress_requests
        .into_iter()
        .map(|r| {
            format!(
                r#"
        <div class="request-card-body">
        <span class="request-desc">{desc}</span>
        <span class="req-unit">{unit}</span>
        <span class="req-timestamp">{timestamp}</span>
        </div>
        <form action="/caretaker/request/resolve" method="POST">
        <input type="hidden" name="request_id" value="{request_id}">
        <button type="submit">resolve</button>
        </form>
        "#,
                desc = r.desc,
                unit = r.unit,
                timestamp = time_ago(r.timestamp),
                request_id = r.id,
            )
        })
        .collect();

    (pending_html, inprogress_html)
}

fn time_ago(t: SystemTime) -> String {
    let elapsed = SystemTime::now().duration_since(t).unwrap_or_default();

    let secs = elapsed.as_secs();
    let mins = secs / 60;
    let hours = mins / 60;
    let days = hours / 24;

    match (hours, days) {
        (0, _) => "just now".into(),
        (h, 0) => format!("{}h ago", h),
        (_, 1) => "yesterday".into(),
        (_, d) => format!("{} days ago", d),
    }
}
