use std::{collections::HashMap, sync::Arc};

use uuid::Uuid;

use crate::{
    entities::user::Role,
    error::AppError,
    handlers::landlord::utils,
    server::{auth, form, request::Request, response::Response},
    services::landlord::{
        building_service,
        dashboard_service::{self, BuildingOverview},
    },
    state::AppState,
    templates::engine,
};

const DASHBOARD_HTML: &str = include_str!("../../templates/views/landlord/dashboard.html");

fn current_month_year() -> String {
    chrono::Utc::now().format("%Y-%m").to_string()
}

fn month_label(month_year: &str) -> String {
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let parts: Vec<&str> = month_year.split('-').collect();
    match (parts.first(), parts.get(1)) {
        (Some(y), Some(m)) => {
            let idx: usize = m.parse::<usize>().unwrap_or(1).saturating_sub(1);
            format!("{} {y}", months.get(idx).unwrap_or(&""))
        }
        _ => month_year.to_string(),
    }
}

pub fn show(req: &Request, state: &Arc<AppState>) -> Result<Response, AppError> {
    let sess = auth::require_role(req, &state.sessions, Role::Landlord)?;
    let month_year = current_month_year();
    let summary = dashboard_service::portfolio_summary(&state.db, &sess.user_id, &month_year)?;
    let overview_data = dashboard_service::building_overview(&state.db, &sess.user_id)?;

    let mut ctx: HashMap<&str, String> = HashMap::new();
    ctx.insert("landlord_name", sess.name.clone());
    ctx.insert("date_label", month_label(&month_year));
    ctx.insert("overview_rows", overview_table(overview_data));

    if summary.has_buildings {
        ctx.insert("collected_revenue", utils::kes(summary.collected_revenue));
        ctx.insert("expected_revenue", utils::kes(summary.expected_revenue));
        ctx.insert("occupancy_pct", format!("{}%", summary.occupancy_pct));
        ctx.insert(
            "vacant_units",
            format!("{} vacany units", summary.vacant_units),
        );
        ctx.insert("total_arrears", utils::kes(summary.total_arrears));
        ctx.insert(
            "arrears_context",
            format!("across {} tenants", summary.arrears_tenants),
        );
        ctx.insert("kpi_block", "".into());
        ctx.insert("empty_state", "hidden".into());
    } else {
        ctx.insert("kpi_block", "hidden".into());
        ctx.insert("empty_state", "".into());
        ctx.insert("collected_revenue", "-".into());
        ctx.insert("expected_revenue", "-".into());
        ctx.insert("occupancy_pct", "-".into());
        ctx.insert("vacant_units", "-".into());
        ctx.insert("total_arrears", "-".into());
        ctx.insert("arrears_context", "-".into());
    }

    tracing::info!(user_id = %sess.user_id, has_buildings = summary.has_buildings, "dashboard rendered");
    Ok(Response::html(200, engine::render(DASHBOARD_HTML, &ctx)))
}

pub fn release_caretaker(req: &Request, state: &Arc<AppState>) -> Result<Response, AppError> {
    let f = form::parse(&req.body);

    let caretaker_id: Uuid = f
        .get("caretaker_id")
        .and_then(|v| v.parse().ok())
        .ok_or(AppError::BadRequest("caretaker_id not found".into()))?;

    building_service::release(&state.db, &caretaker_id)?;

    Ok(Response::redirect("/landlord"))
}

fn overview_table(overviews: Vec<BuildingOverview>) -> String {
    overviews
        .into_iter()
        .map(|o| {
            let caretaker_id = match o.cartaker_id {
                None => "-".to_string(),
                Some(id) => id.to_string(),
            };
            let caretaker = match o.caretaker_name {
                None => "-".to_string(),
                Some(name) => name,
            };
            let contact = match o.caretaker_number {
                None => "-".to_string(),
                Some(contact) => contact,
            };
            let requests = match o.requests {
                None => "".to_string(),
                Some(r) => r.to_string(),
            };
            format!(
                r#"
                    <tr>
                    <td>{name}</td>
                    <td>{caretaker}</td>
                    <td>{requests}</td>
                    <td>{contact}</td>
                    <td>
                    <form action="/landlord/caretaker/release" method="POST">
                    <input type="hidden" value="{caretaker_id}" name="caretaker_id">
                    <button type="submit">release</button>
                    </form>
                    </td>
                    </tr>"#,
                name = o.name,
            )
        })
        .collect()
}
