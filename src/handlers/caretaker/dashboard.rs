use std::{collections::HashMap, sync::Arc, time::SystemTime};

use uuid::Uuid;

use crate::{
    entities::{
        maintenance::RequestPanelRow,
        user::{Role, User},
    },
    error::AppError,
    repositories::{building_repo, maintenance_repo, user_repo},
    server::{auth, form, request::Request, response::Response},
    state::AppState,
    templates::engine,
};

const DASH_HTML: &str = include_str!("../../templates/views/caretaker/dashboard.html");

pub fn show(req: &Request, state: &Arc<AppState>) -> Result<Response, AppError> {
    let sess = auth::require_role(req, &state.sessions, Role::Caretaker)?;

    let buildings = building_repo::find_assigned_buildings(&state.db, &sess.user_id)?;

    let selected_building: Option<Uuid> = req.query.get("building_id").and_then(|v| v.parse().ok());

    let active_building = selected_building
        .or_else(|| buildings.first().map(|(id, _)| *id))
        .ok_or(AppError::BadRequest("no building assigned".into()))?;

    let overview_metrics =
        maintenance_repo::dash_overview_row(&state.db, &sess.user_id, &active_building)?;

    let requests = maintenance_repo::request_panel_row(&state.db, &sess.user_id, &active_building)?;

    let (pending, inprogress) = request_panel(requests);

    let user: User = user_repo::find_by_id(&state.db, &sess.user_id)?
        .ok_or(AppError::BadRequest("user not found".into()))?;

    let (first_name, last_name) = user
        .name
        .split_once(" ")
        .ok_or(AppError::BadRequest("user name not found".into()))?;

    let building_selector = building_selector(&buildings, &active_building);

    let mut ctx = HashMap::new();

    ctx.insert("profile_fname", first_name.to_string());
    ctx.insert("profile_lname", last_name.to_string());
    ctx.insert("profile_email", user.email.clone());
    ctx.insert("profile_number", user.number.clone());

    ctx.insert("caretaker_name", sess.name);

    ctx.insert("building_selector", building_selector);

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
    maintenance_repo::to_inprogress(&state.db, &id)?;
    Ok(Response::redirect("/caretaker"))
}

pub fn resolve(req: &Request, state: &Arc<AppState>) -> Result<Response, AppError> {
    let f = form::parse(&req.body);
    let id: Uuid = f
        .get("request_id")
        .and_then(|v| v.parse().ok())
        .ok_or(AppError::BadRequest("request_id missing".into()))?;

    maintenance_repo::to_resolved(&state.db, &id)?;
    Ok(Response::redirect("/caretaker"))
}

fn request_panel(r: Vec<RequestPanelRow>) -> (String, String) {
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
                timestamp = time_ago(r.created_at),
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
                timestamp = time_ago(r.created_at),
                request_id = r.id,
            )
        })
        .collect();

    (pending_html, inprogress_html)
}

fn building_selector(buildings: &[(Uuid, String)], active: &Uuid) -> String {
    if buildings.len() <= 1 {
        return buildings
            .first()
            .map(|(_, name)| format!(r#"<span class="building-label">{name}</span>"#))
            .unwrap_or_default();
    }

    let options: String = buildings
        .iter()
        .map(|(id, name)| {
            let selected = if id == active { " selected" } else { "" };
            format!(r#"<option value="{id}"{selected}>{name}</option>"#)
        })
        .collect();

    format!(
        r#"<select class="building-select"
            onchange="location.href='/caretaker?building_id='+this.value">
            {options}
        </select>"#
    )
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
