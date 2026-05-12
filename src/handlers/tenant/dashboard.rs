use std::{collections::HashMap, sync::Arc, time::SystemTime};

use uuid::Uuid;

use crate::{
    entities::user::Role,
    error::AppError,
    repositories::{maintenance_repo, unit_repo},
    server::{auth, form, request::Request, response::Response},
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
    let payment_form_html = payment_form();

    let mut ctx = HashMap::new();
    ctx.insert("tenant_name", sess.name);
    ctx.insert("unit_name", header_data.0);
    ctx.insert("apartment_name", header_data.1);
    ctx.insert("rent_amount", format!("KES {}", header_data.2));

    ctx.insert("request_card", request_html);
    ctx.insert("payment_card", payments_html);

    ctx.insert("payment_form", payment_form_html);
    Ok(Response::html(200, engine::render(DASH_HTML, &ctx)))
}

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

fn payment_form() -> String {
    let month_year = chrono::Utc::now().format("%Y-%m").to_string();
    let current_month = format_month_year(month_year);
    format!(
        r#"
     <form action="/tenant/payment/initiate" method="POST">
     <input type="hidden" name="month_year" value="{current_month}">
     <p class="modal-title">initiate payment</p>
     <div class="input-container">
     <label for="phone">phone</label>
     <input type="text" id="phone" name="phone">
     <span id="phone-error" class="error-message"></span>
     </div>
     <button type="submit" class="form-button">initiate</button> 
     </form>
        "#
    )
}

fn format_month_year(s: String) -> String {
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
    let parts: Vec<&str> = s.split('-').collect();
    match (parts.first(), parts.get(1)) {
        (Some(y), Some(m)) => {
            let idx: usize = m.parse::<usize>().unwrap_or(1).saturating_sub(1);
            format!("{} {y}", months.get(idx).unwrap_or(&""))
        }
        _ => s.to_string(),
    }
}
