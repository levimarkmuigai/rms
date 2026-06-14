use std::{collections::HashMap, sync::Arc};

use uuid::Uuid;

use crate::{
    entities::{notice::NoticeForm, user::Role},
    error::AppError,
    handlers::landlord::utils,
    repositories::{activity_repo, notice_repo, unit_repo},
    server::{auth, form, request::Request, response::Response},
    services::{
        landlord::{building_service, unit_service},
        user_service,
    },
    state::AppState,
    templates::engine,
};

const UNIT_HTML: &str = include_str!("../../templates/views/landlord/units.html");

pub fn add(req: &Request, state: &Arc<AppState>) -> Result<Response, AppError> {
    let sess = auth::require_role(req, &state.sessions, Role::Landlord)?;
    let f = form::parse(&req.body);

    let unit_number = f.get("unit-number").cloned().unwrap_or_default();
    let rent_amount: i32 = f
        .get("rent-amount")
        .and_then(|v| v.parse().ok())
        .ok_or_else(|| AppError::BadRequest("missing rent_amount".into()))?;
    let building_id: Uuid = f
        .get("building-id")
        .and_then(|v| v.parse().ok())
        .ok_or_else(|| AppError::BadRequest("missing building_id".into()))?;

    unit_service::add(
        &state.db,
        &Uuid::new_v4(),
        &building_id,
        &unit_number,
        rent_amount,
    )?;

    tracing::info!(user_id = %sess.user_id, %building_id, "unit added");

    activity_repo::insert(&state.db, &sess.user_id, "added a unit")?;

    Ok(Response::redirect("/landlord/buildings"))
}

pub fn assign_unit(req: &Request, state: &Arc<AppState>) -> Result<Response, AppError> {
    let f = form::parse(&req.body);
    let unit_id: Uuid = f
        .get("unit_id")
        .and_then(|v| v.parse().ok())
        .ok_or(AppError::BadRequest("unit not selected".into()))?;
    let tenant_id: Uuid = f
        .get("tenant_id")
        .and_then(|v| v.parse().ok())
        .ok_or(AppError::BadRequest("tenant not selected".into()))?;

    let building_id: Uuid = f
        .get("building_id")
        .and_then(|v| v.parse().ok())
        .ok_or(AppError::BadRequest("building_id missing".into()))?;

    tracing::debug!(unit_id = %unit_id, tenant_id = %tenant_id);

    unit_service::assign(&state.db, &tenant_id, &unit_id)?;
    Ok(Response::redirect(&format!(
        "/landlord/units?building_id={building_id}"
    )))
}

pub fn vacate(req: &Request, state: &Arc<AppState>) -> Result<Response, AppError> {
    let f = form::parse(&req.body);
    let sess = auth::require_role(req, &state.sessions, Role::Landlord)?;
    let unit_id: Uuid = f
        .get("unit_id")
        .and_then(|v| v.parse().ok())
        .ok_or(AppError::BadRequest("unit id not found".into()))?;

    let building_id: Uuid = f
        .get("building_id")
        .and_then(|v| v.parse().ok())
        .ok_or(AppError::BadRequest("building_id missing".into()))?;

    unit_service::vacate_tenant(&state.db, &unit_id)?;

    activity_repo::insert(&state.db, &sess.user_id, "vacated tenant")?;
    Ok(Response::redirect(&format!(
        "/landlord/units?building_id={building_id}"
    )))
}

pub fn show(req: &Request, state: &Arc<AppState>) -> Result<Response, AppError> {
    let sess = auth::require_role(req, &state.sessions, Role::Landlord)?;
    let buildings = building_service::find_by_lanlord(&state.db, &sess.user_id)?;

    let active_building: Uuid = req
        .query
        .get("building_id")
        .and_then(|v| v.parse().ok())
        .or_else(|| buildings.first().map(|b| b.id))
        .ok_or(AppError::NotFound("no buildings found".into()))?;

    let selected_unit: Option<Uuid> = req.query.get("id").and_then(|v| v.parse().ok());

    let units = unit_repo::unit_summary_row(&state.db, &active_building)?;

    let active_unit = selected_unit.or_else(|| units.first().map(|u| u.id));

    let selected_stats = active_unit
        .and_then(|u_id| units.iter().find(|u| u.id == u_id))
        .or(units.first());

    let unit_header = match selected_stats {
        None => "<p class=\"empty-detail\">no units for this building.</p>".into(),
        Some(u) => {
            let notice_html = notice_form(
                notice_repo::find_pending(&state.db, &u.id)?,
                active_building,
            );
            format!(
                r#"<div class="building-info-bar">
                  <div class="info-group">
                    <span class="info-label">unit</span>
                    <span class="info-value">{number}</span>
                  </div>
                  <div class="info-group">
                    <span class="info-label">rent amount</span>
                    <span class="info-value">{rent}</span>
                  </div>
                  <div class="info-group">
                    <span class="info-label">status</span>
                    <span class="info-value">{status}</span>
                  </div>
                </div>
                {notice_html}"#,
                number = u.number,
                rent = utils::kes(u.rent_amount),
                status = u.status,
            )
        }
    };

    let units_table: String = units.iter().map(|u| {
        let active = if Some(u.id) == active_unit { " active-row" } else { "" };
        let tenant_name = u.tenant_name.as_deref().unwrap_or("—");
        format!(
            r#"<tr class="{active}">
              <td><a href="/landlord/units?id={id}&building_id={b_id}" class="row-link">{number}</a></td>
              <td>{tenant_name}</td>
              <td class="row-actions">
                <button class="open-assign-tenant" data-id="{id}">assign tenant</button>
                <form action="/landlord/unit/vacate" method="POST">
                  <input type="hidden" name="unit_id" value="{id}">
                  <input type="hidden" name="building_id" value="{b_id}">
                  <button type="submit">vacate</button>
                </form>
              </td>
            </tr>"#,
            active      = active,
            id          = u.id,
            b_id        = active_building,
            number      = u.number,
        )
    }).collect();

    let units_count = units.len();

    let assign_html = match active_unit {
        Some(unit_id) => {
            let tenant_options = user_service::get_unassigned_tenant(&state.db)?;
            assign_form(tenant_options, unit_id, active_building)
        }
        None => String::new(),
    };

    let mut ctx: HashMap<&str, String> = HashMap::new();
    ctx.insert(
        "units_count",
        format!(
            "{units_count} unit{}",
            if units_count == 1 { "" } else { "s" }
        ),
    );
    ctx.insert("unit_header", unit_header);
    ctx.insert("units_table", units_table);
    ctx.insert("assign_units", assign_html);

    Ok(Response::html(200, engine::render(UNIT_HTML, &ctx)))
}

fn assign_form(tenant_options: Vec<(Uuid, String)>, unit_id: Uuid, building_id: Uuid) -> String {
    let tenant_list: String = tenant_options
        .iter()
        .map(|(id, email)| format!(r#"<option value="{id}">{email}</option>"#))
        .collect();

    format!(
        r#"<form action="/landlord/unit/assign" method="POST">
          <p class="modal-title">assign tenant</p>
          <input type="hidden" name="unit_id" value="{unit_id}">
          <input type="hidden" name="building_id" value="{building_id}">
          <div class="input-container">
            <label for="tenant">tenant</label>
            <select id="tenant" name="tenant_id">
              <option value="" disabled selected>select tenant</option>
              {tenant_list}
            </select>
          </div>
          <button type="submit" class="form-button">assign</button>
        </form>"#
    )
}

fn notice_form(notice_opt: Option<NoticeForm>, building_id: Uuid) -> String {
    notice_opt
        .map(|n| {
            format!(
                r#"<div class="notice-card">
            <div class="notice-body">
    <span class="notice-label">vacancy notice</span>
    <span class="notice-date">move-out date {date}</span>
    <span class="notice-meta">submitted {submitted_at}</span>
  </div>
  <div class="notice-actions">
    <div class="notice-btns">
      <form action="/landlord/vacancy/approve" method="POST">
        <input type="hidden" name="notice_id" value="{notice_id}">
        <input type="hidden" name="building_id" value="{building_id}">
        <button type="submit" class="notice-btn">approve</button>
      </form>
      <form id="reject-{notice_id}" action="/landlord/vacancy/reject" method="POST" class="reject-form">
        <input type="hidden" name="notice_id" value="{notice_id}">
        <input type="hidden" name="building_id" value="{building_id}">
        <button type="submit" class="notice-btn danger-btn">reject</button>
      </form>
    </div>
    <textarea
      name="reason"
      form="reject-{notice_id}"
      class="reject-reason"
      placeholder="reason for rejection"
      rows="2"></textarea>
  </div>
</div>"#,
                date = n.date,
                submitted_at = utils::time_ago(n.submitted_at),
                notice_id = n.id,
            )
        })
        .unwrap_or_default()
}

pub fn approve_notice(req: &Request, state: &Arc<AppState>) -> Result<Response, AppError> {
    let f = form::parse(&req.body);

    let notice_id: Uuid = f
        .get("notice_id")
        .and_then(|v| v.parse().ok())
        .ok_or(AppError::BadRequest("notice_id missing".into()))?;

    let building_id: Uuid = f
        .get("building_id")
        .and_then(|v| v.parse().ok())
        .ok_or(AppError::BadRequest("building_id missing".into()))?;

    notice_repo::approve(&state.db, &notice_id)?;
    Ok(Response::redirect(&format!(
        "/landlord/units?building_id={building_id}"
    )))
}

pub fn reject_notice(req: &Request, state: &Arc<AppState>) -> Result<Response, AppError> {
    let f = form::parse(&req.body);

    let notice_id: Uuid = f
        .get("notice_id")
        .and_then(|v| v.parse().ok())
        .ok_or(AppError::BadRequest("notice_id missing".into()))?;

    let reason = f.get("reason").cloned().unwrap_or_default();

    let building_id: Uuid = f
        .get("building_id")
        .and_then(|v| v.parse().ok())
        .ok_or(AppError::BadRequest("building_id missing".into()))?;

    notice_repo::reject(&state.db, &notice_id, &reason)?;
    Ok(Response::redirect(&format!(
        "/landlord/units?building_id={building_id}"
    )))
}
