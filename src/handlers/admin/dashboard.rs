use std::{collections::HashMap, sync::Arc, time::SystemTime};

use crate::{
    error::AppError,
    repositories::{activity_repo, user_repo},
    server::{request::Request, response::Response},
    state::AppState,
    templates::engine,
};

const ADMIN_HTML: &str = include_str!("../../templates/views/admin/dashboard.html");

pub fn show(_req: &Request, state: &Arc<AppState>) -> Result<Response, AppError> {
    let logs = activity_repo::find_all(&state.db)?;
    let counts = user_repo::role_counts(&state.db)?;

    let log_rows: String = logs
        .iter()
        .map(|l| {
            format!(
                "<tr>
          <td class=\"email\">{email}</td>
          <td>{action}</td>
          <td class=\"timestamp\">{when}</td>
        </tr>",
                email = l.email,
                action = l.action,
                when = time_ago(l.created_at),
            )
        })
        .collect();

    let mut ctx: HashMap<&str, String> = HashMap::new();
    ctx.insert("landlord_count", counts.landlords.to_string());
    ctx.insert("caretaker_count", counts.caretakers.to_string());
    ctx.insert("tenant_count", counts.tenants.to_string());
    ctx.insert("log_count", logs.len().to_string());
    ctx.insert("log_rows", log_rows);

    Ok(Response::html(200, engine::render(ADMIN_HTML, &ctx)))
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
