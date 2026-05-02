use std::{collections::HashMap, sync::Arc};

use crate::{
    entities::user::Role,
    error::AppError,
    server::{auth, request::Request, response::Response},
    services::landlord::dashboard_service,
    state::AppState,
    templates::engine,
};

const DASHBOARD_HTML: &str = include_str!("../../templates/views/landlord/dashboard.html");

fn kes(amount: i64) -> String {
    let s = amount.to_string();
    let with_commas = s
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_default()
        .join(",");
    format!("Ksh {with_commas}")
}

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
    let requests = dashboard_service::open_requests(&state.db, &sess.user_id)?;

    let mut ctx: HashMap<&str, String> = HashMap::new();
    ctx.insert("landlord_name", sess.name.clone());
    ctx.insert("date_label", month_label(&month_year));

    if summary.has_buildings {
        ctx.insert("collected_revenue", kes(summary.collected_revenue));
        ctx.insert("collected_revenue", kes(summary.expected_revenue));
        ctx.insert("occupancy_pct", format!("{}%", summary.occupancy_pct));
        ctx.insert(
            "vacant_units",
            format!("{} vacany units", summary.vacant_units),
        );
        ctx.insert("total_arrears", kes(summary.total_arrears));
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

    let open_rows: String = if requests.is_empty() {
        "<div class=\"list-row\"><span class=\"col-main\">no open requests</span></div>".into()
    } else {
        requests
            .iter()
            .map(|r| {
                format!(
                    "<div class=\"list-row\">
                <span class=\"col-main\">{}</span>
                <span class=\"col-sub\">{}</span>
                <span class=\"col-sub\">{}</span>
                <span class=\"col-sub\">{}</span>
                </div>",
                    r.category, r.unit_label, r.status, r.age_label
                )
            })
            .collect()
    };
    ctx.insert("open_requests", open_rows);

    tracing::info!(user_id = %sess.user_id, has_buildings = summary.has_buildings, "dashboard rendered");
    Ok(Response::html(200, engine::render(DASHBOARD_HTML, &ctx)))
}
