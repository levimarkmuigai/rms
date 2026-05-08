use std::{collections::HashMap, sync::Arc};

use uuid::Uuid;

use crate::{
    entities::{building::Building, user::Role},
    error::AppError,
    handlers::landlord::utils,
    server::{auth, form, request::Request, response::Response},
    services::{
        landlord::{
            building_service,
            unit_service::{self, UnitStats},
        },
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

    unit_service::assign(&state.db, &tenant_id, &unit_id)?;
    Ok(Response::redirect("/landlord/units"))
}

pub fn vacate(req: &Request, state: &Arc<AppState>) -> Result<Response, AppError> {
    let f = form::parse(&req.body);
    let unit_id: Uuid = f
        .get("unit_id")
        .and_then(|v| v.parse().ok())
        .ok_or(AppError::BadRequest("unit id not found".into()))?;

    unit_service::vacate_tenant(&state.db, &unit_id)?;
    Ok(Response::redirect("/landlord/units"))
}

pub fn show(req: &Request, state: &Arc<AppState>) -> Result<Response, AppError> {
    let sess = auth::require_role(req, &state.sessions, Role::Landlord)?;
    let buildings = building_service::find_by_lanlord(&state.db, &sess.user_id)?;

    let selected_building: Option<Uuid> = req.query.get("building_id").and_then(|v| v.parse().ok());
    let active_building = selected_building.or_else(|| buildings.first().map(|b| b.id));

    let buildings_list = building_nav(&buildings, &active_building);

    let selected_unit: Option<Uuid> = req.query.get("id").and_then(|v| v.parse().ok());

    let building_stats: HashMap<Uuid, Vec<UnitStats>> = buildings
        .into_iter()
        .map(|b| {
            let unit_stats = unit_service::unit_stats(&state.db, &b.id)?;
            Ok((b.id, unit_stats))
        })
        .collect::<Result<_, AppError>>()?;
    let data_display: (usize, String, String, Option<Uuid>) = match active_building {
        None => (
            0,
            "<p class=\"empty-detail\">select a building to see details.</p>".into(),
            "-".into(),
            None,
        ),
        Some(b_id) => {
            let units = building_stats.get(&b_id).map(Vec::as_slice).unwrap_or(&[]);

            let active_unit: Option<Uuid> = selected_unit.or_else(|| units.first().map(|u| u.id));

            let unit_stats = active_unit
                .and_then(|u_id| units.iter().find(|u| u.id == u_id))
                .or(units.first());

            let list_html: String = if units.is_empty() {
                "<p class=\"empty-list\">no buildings added yet.
                    <a href=\"/landlord/buildings\" id=\"open-add-modal\">add one →</a></p>"
                    .into()
            } else {
                units.iter().map(|us| {
                   let active = if Some(us.id) == active_unit {
                       " active-item"
                   } else {
                       ""
                   };
                   let id = us.id;
                   let number = us.number.clone();
                   format!(
                   "<a href=\"/landlord/units?id={id}&building_id={b_id}\" class=\"list-item{active}\">
                   <span class=\"b-name\">{number}</span>
                   </a>"
                )
               }).collect()
            };

            let details_html = match unit_stats {
                None => "<p class=\"empty-detail\">no units for this building.</p>".into(),
                Some(u) => format!(
                    "
                <div class=\"detail-header\">
                <h2 class=\"detail-title\">{number}</h2>
                <div class=\"detail-actions\">
                <button id=\"open-assign-tenant\"> assign unit</button>
                <form action=\"/landlord/unit/vacate\" method=\"POST\">
                <input type=\"hidden\" value=\"{id}\" name=\"unit_id\">
                <button type=\"submit\">vacate tenant</button>
                </form>
                </div>
                </div>
                <div class=\"stat-grid\">
                <div class=\"stat-box\">
                <span class=\"stat-label\">rent amount</span>
                <span class=\"stat-value\">{rent}</span>
                </div>
                <div class=\"stat-box\">
                <span class=\"stat-label\">vacancy status</span>
                <span class=\"stat-value\">{status}</span>
                </div>
                </div>",
                    id = u.id,
                    number = u.number,
                    rent = utils::kes(u.rent_amount),
                    status = u.status,
                ),
            };

            let units_count = units.len();

            (units_count, list_html, details_html, active_unit)
        }
    };

    let assign_html = if let Some(id) = data_display.3 {
        let tenant_options = user_service::get_unassigned_tenant(&state.db)?;
        assign_form(tenant_options, id)
    } else {
        String::new()
    };

    let units_count = data_display.0;

    let mut ctx: HashMap<&str, String> = HashMap::new();

    ctx.insert("buildings_list", buildings_list);
    ctx.insert(
        "units_count",
        format!(
            "{units_count} unit{}",
            if units_count == 1 { "" } else { "s" }
        ),
    );
    ctx.insert("units_list", data_display.1);
    ctx.insert("details", data_display.2);
    ctx.insert("assign_units", assign_html);
    Ok(Response::html(200, engine::render(UNIT_HTML, &ctx)))
}

fn assign_form(tenant_options: Vec<(Uuid, String)>, unit_id: Uuid) -> String {
    let tenant_list: String = tenant_options
        .iter()
        .map(|(id, email)| format!(r#"<option value="{id}">{email}</option>"#))
        .collect();

    format!(
        r#"
    <form action="/landlord/unit/assign" method="POST" class="inline-form">
    <fieldset class="form-group">
    <legend>Assign Tenant</legend>
    <div class="input-container">
    <input type="hidden" name="unit_id" value="{unit_id}">
    </div>
    <div class="input-container">
    <label for="tenant">tenant</label>
    <select id="tenant" name="tenant_id">
    <option value="" disabled selected>select tenant</option>
    {tenant_list}
    </select>
    </div>
    </fieldset>
    <button type="submit" class="form-button">Assign</button>
    </form>
    "#
    )
}

fn building_nav(buildings: &[Building], active: &Option<Uuid>) -> String {
    buildings.iter().map(|b| {
        let active_class = if &Some(b.id) == active { " active-building" } else { "" };
        format!(r#"<a href="/landlord/units?building_id={id}" class="building-tab{active_class}">{name}</a>"#,
            id = b.id, name = b.name,)
    }).collect()
}
