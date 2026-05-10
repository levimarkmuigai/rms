use std::{collections::HashMap, sync::Arc, time::SystemTime};

use crate::{
    entities::user::Role,
    error::AppError,
    server::{auth, request::Request, response::Response},
    services::tenant::dashboard_services::{self, PaymentActivity, RequestActivity},
    state::AppState,
    templates::engine,
};

const DASH_HTML: &str = include_str!("../../templates/views/tenant/dashboard.html");

pub fn show(req: &Request, state: &Arc<AppState>) -> Result<Response, AppError> {
    let sess = auth::require_role(req, &state.sessions, Role::Tenant)?;
    let header_data = dashboard_services::header_data(&state.db, &sess.user_id)?;

    let requests = dashboard_services::request_activity(&state.db, &sess.user_id)?;
    let payments = dashboard_services::payment_activity(&state.db, &sess.user_id)?;

    let request_html = request_activity(requests);
    let payments_html = payment_activity(payments);
    let mut ctx = HashMap::new();
    ctx.insert("tenant_name", sess.name);
    ctx.insert("unit_name", header_data.0);
    ctx.insert("apartment_name", header_data.1);
    ctx.insert("rent_amount", format!("KES {}", header_data.2));

    ctx.insert("request_card", request_html);
    ctx.insert("payment_card", payments_html);
    Ok(Response::html(200, engine::render(DASH_HTML, &ctx)))
}

fn request_activity(r: Vec<RequestActivity>) -> String {
    r.into_iter()
        .map(|r| {
            format!(
                r#"
         <div class="activity-card">
         <span class="activity-desc">{desc}</span>
         <span class="activity-timestamp">{timestamp}</span>
         <span class="activity-status">{status}</span>
          </div>
        "#,
                desc = r.desc,
                timestamp = time_ago(r.timestamp),
                status = r.status
            )
        })
        .collect()
}

fn payment_activity(p: Vec<PaymentActivity>) -> String {
    p.into_iter()
        .map(|p| {
            format!(
                r#"
         <div class="activity-card">
         <span class="activity-desc">{timestamp}</span>
         <span class="activity-amount">{amount}</span>
         </div>
        "#,
                timestamp = p.month_year,
                amount = p.amount,
            )
        })
        .collect()
}

fn time_ago(t: SystemTime) -> String {
    let elapsed = SystemTime::now().duration_since(t).unwrap_or_default();

    let secs = elapsed.as_secs();
    let min = secs / 60;
    let hours = min / 60;
    let days = hours / 24;

    match (hours, days) {
        (0, _) => "just now".into(),
        (h, 0) => format!("{} hours ago", h),
        (_, 1) => "yesterday".into(),
        (_, h) => format!("{} days ago", h),
    }
}
