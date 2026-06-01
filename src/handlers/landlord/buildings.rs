use std::{collections::HashMap, sync::Arc};

use uuid::Uuid;

use crate::{
    entities::user::Role,
    error::AppError,
    handlers::landlord::utils,
    repositories::{activity_repo, building_repo, user_repo},
    server::{auth, form, request::Request, response::Response},
    services::landlord::building_service,
    state::AppState,
    templates::engine,
};

const BUILDINGS_HTML: &str = include_str!("../../templates/views/landlord/buildings.html");

fn current_month_year() -> String {
    chrono::Utc::now().format("%Y-%m").to_string()
}

pub fn show(req: &Request, state: &Arc<AppState>) -> Result<Response, AppError> {
    let sess = auth::require_role(req, &state.sessions, Role::Landlord)?;
    let month_year = current_month_year();
    let table_data = building_repo::building_table_rows(&state.db, &sess.user_id, &month_year)?;

    let selected_id: Option<Uuid> = req.query.get("id").and_then(|v| v.parse().ok());

    let active_id = selected_id.or_else(|| table_data.first().map(|b| b.id));

    let building_header: String = active_id
        .and_then(|id| table_data.iter().find(|b| b.id == id))
        .map(|b| {
            format!(
                r#"<div class="building-info-bar">
            <div class="info-group">
            <span class="info-label">location: </span>
            <span class="info-value">{city}, {location}</span>
            </div>
            <div class="info-group">
            <span class="info-label">owner: </span>
            <span class="info-value">{owner}</span>
            </div>
            </div>"#,
                city = b.city,
                location = b.location,
                owner = b.owner,
            )
        })
        .unwrap_or_default();

    let buildings_table: String = table_data
        .iter()
        .map(|b| {
            let active = if Some(b.id) == active_id {
                " active-row"
            } else {
                ""
            };
            format!(
                r#"<tr class="{active}">
        <td><a href="/landlord/buildings?id={id}" class="row-link">{name}</a></td>
        <td>{collected}</td>
        <td>{occupied}</td>
        <td>{vacant}</td>
        <td class="row-actions">
        <button class="open-assign-caretaker" id="open-assign-caretaker" data-id="{id}">assign caretaker</button>
        <button class="open-add-unit" id="open-add-unit" data-id="{id}">add unit</button>
        <form action="/delete-building" method="POST"
        onsubmit="return confirm('permanently delete this building?');">
        <input type="hidden" name="building_id" value="{id}">
        <button type="submit">delete building</button>
        </form>
        </td>
        </tr>"#,
                active = active,
                id = b.id,
                name = b.name,
                collected = utils::kes(b.collected),
                occupied = b.occupied,
                vacant = b.vacant,
            )
        })
        .collect();

    let buildings_count = table_data.len();

    let unit_form = active_id
        .map(|b_id| add_unit_form(&b_id))
        .unwrap_or_default();

    let assign_html = if let Some(b_id) = active_id {
        let caretaker_options = user_repo::find_available_caretakers(&state.db)?;
        assign_caretaker_form(caretaker_options, b_id)
    } else {
        String::new()
    };

    let mut ctx: HashMap<&str, String> = HashMap::new();
    ctx.insert(
        "buildings_count",
        format!(
            "{buildings_count} building{}",
            if buildings_count == 1 { "" } else { "s" }
        ),
    );
    ctx.insert("building_header", building_header);
    ctx.insert("buildings_table", buildings_table);
    ctx.insert("building_form_html", add_building_form());
    ctx.insert("unit_form_html", unit_form);
    ctx.insert("assign_form", assign_html);

    Ok(Response::html(200, engine::render(BUILDINGS_HTML, &ctx)))
}

pub fn add(req: &Request, state: &Arc<AppState>) -> Result<Response, AppError> {
    let sess = auth::require_role(req, &state.sessions, Role::Landlord)?;
    let f = form::parse(&req.body);
    let name = f.get("name").cloned().unwrap_or_default();
    let city = f.get("city").cloned().unwrap_or_default();
    let location = f.get("location").cloned().unwrap_or_default();
    let owner = f.get("owner").cloned().unwrap_or_default();

    building_service::add(&state.db, &sess.user_id, name, city, location, owner)?;
    tracing::info!(user_id = %sess.user_id, "building added");
    Ok(Response::redirect("/landlord/buildings"))
}

pub fn delete(req: &Request, state: &Arc<AppState>) -> Result<Response, AppError> {
    let sess = auth::require_role(req, &state.sessions, Role::Landlord)?;
    let f = form::parse(&req.body);
    let building_id = f
        .get("building_id")
        .and_then(|v| v.parse::<Uuid>().ok())
        .ok_or_else(|| AppError::BadRequest("invalid building_id".into()))?;

    building_service::remove(&state.db, &sess.user_id, &building_id)?;

    activity_repo::insert(&state.db, &sess.user_id, "removed a building")?;
    tracing::info!(user_id = %sess.user_id, %building_id, "building deleted");

    Ok(Response::redirect("/buildings"))
}

pub fn assign(req: &Request, state: &Arc<AppState>) -> Result<Response, AppError> {
    let f = form::parse(&req.body);
    let building_id: Uuid = f
        .get("building_id")
        .and_then(|v| v.parse().ok())
        .ok_or(AppError::BadRequest("building_id is missing".into()))?;
    let caretaker_id: Uuid = f
        .get("caretaker_id")
        .and_then(|v| v.parse().ok())
        .ok_or(AppError::BadRequest("caretaker_id is missing".into()))?;

    building_service::assign(&state.db, &caretaker_id, &building_id)?;

    Ok(Response::redirect("/landlord/buildings"))
}

fn add_building_form() -> String {
    r#"<form action="/landlord/buildings" method="POST" id="add-building-form">
      <p class="modal-title">building details</p>
      <div class="input-container">
        <label for="building-name">name</label>
        <input type="text" id="building-name" name="name">
        <span class="error-message" id="name-error"></span>
      </div>
      <div class="input-row">
        <div class="input-container">
          <label for="building-city">city</label>
          <input type="text" id="building-city" name="city">
        </div>
        <div class="input-container">
          <label for="building-location">location</label>
          <input type="text" id="building-location" name="location">
        </div>
      </div>
      <div class="input-container">
        <label for="building-owner">owner</label>
        <input type="text" id="building-owner" name="owner">
      </div>
      <button type="submit" class="form-button">add building</button>
    </form>"#
        .into()
}

fn add_unit_form(building_id: &Uuid) -> String {
    format!(
        "
    <form action=\"/landlord/units\" method=\"POST\" id=\"add-unit-form\">
        <fieldset class=\"form-group\">
          <legend> Unit Details</legend>
          <input type=\"hidden\" name=\"building-id\" value=\"{}\">
          <div class=\"input-container\">
            <label for=\"unit-number\">Unit Number</label>
            <input type=\"text\" name=\"unit-number\" id=\"unit-number\">
            <span class=\"error-message\" id=\"unit-number-error\"></span>
          </div>
          <div class=\"input-container\">
            <label for=\"rent-amount\">Rent Amount</label>
            <input type=\"text\" name=\"rent-amount\" id=\"rent-amount\">
            <span class=\"error-message\" id=\"rent-amount-error\"></span>
          </div>
        </fieldset>
        <button type=\"submit\" class=\"form-button\">Add Unit</button>
      </form>
    ",
        building_id
    )
}

fn assign_caretaker_form(caretakers: Vec<(Uuid, String)>, building_id: Uuid) -> String {
    let caretaker_options: String = caretakers
        .iter()
        .map(|(id, email)| format!(r#"<option value="{id}">{email}</option>"#))
        .collect();

    format!(
        r#"
    <form action="/landlord/building/assign" method="POST" class="inline-form">
    <fieldset class="form-group">
    <legend>Assign Caretaker</legend>
    <div class="input-container">
    <input type="hidden" name="building_id" value="{building_id}">
    </div>
    <div class="input-container">
    <label for="caretaker">caretaker</label>
    <select id="caretaker" name="caretaker_id">
    <option value="" disabled selected>select caretaker</option>
    {caretaker_options}
    </select>
    </div>
    </fieldset>
    <button type="submit" class="form-button">Assign</button>
    </form>
    "#
    )
}
