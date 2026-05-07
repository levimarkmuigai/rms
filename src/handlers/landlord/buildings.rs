use std::{collections::HashMap, sync::Arc};

use uuid::Uuid;

use crate::{
    entities::user::Role,
    error::AppError,
    handlers::landlord::utils,
    server::{auth, form, request::Request, response::Response},
    services::landlord::{building_service, dashboard_service},
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
    let cards = dashboard_service::building_cards(&state.db, &sess.user_id, &month_year)?;

    let selected_id: Option<Uuid> = req.query.get("id").and_then(|v| v.parse().ok());

    let active_id = selected_id.or_else(|| cards.first().map(|b| b.id));

    let list_html: String = if cards.is_empty() {
        "<p class=\"empty-list\">no buildings added yet. <a href=\"#\" id=\"open-add-modal\">add one →</a></p>".into()
    } else {
        cards
            .iter()
            .map(|b| {
                let active = if Some(b.id) == active_id {
                    " active-item"
                } else {
                    ""
                };
                format!(
                    "<a href=\"/landlord/buildings?id={id}\" class=\"list-item{active}\">
                <span class=\"b-name\">{name}</span>
                </a>",
                    id = b.id,
                    name = b.name,
                )
            })
            .collect()
    };

    let detail_html: String = match active_id.and_then(|id| cards.iter().find(|b| b.id == id)) {
        None => "<p class=\"empty-detail\">select a building to see details.</p>".into(),
        Some(b) => format!(
            "<div class=\"detail-header\">
            <h2 class=\"detail-title\">{name}</h2>
            <div class=\"detail-actions\">
            <button id=\"open-add-unit\">+ add unit</button>
            <form action=\"/delete-building\" method=\"POST\" class=\"inline-form\"
            onsubmit=\"return confirm('permanently delete this building?');\">
            <input type=\"hidden\" name=\"building_id\" value=\"{id}\">
            <button type=\"submit\" class=\"danger-btn\">delete property</button>
            </form>
            </div>
            </div>
            <div class=\"b-stat-grid\">
            <div class=\"b-stat-box\">
            <span class=\"stat-label\">collected this month</span>
            <span class=\"stat-value\">{collected}</span>
            </div>
            <div class=\"b-stat-box\">
            <span class=\"stat-label\">occupied</span>
            <span class=\"stat-value\">{occupied}</span>
            <span class=\"stat-label\">vacant</span>
            <span class=\"stat-context\">{vacant}</span>
            </div>
            </div>",
            name = b.name,
            id = b.id,
            collected = utils::kes(b.collected),
            occupied = b.is_occupied,
            vacant = b.vacant,
        ),
    };

    let buildings_count = cards.len();

    let unit_form = active_id
        .map(|b_id| add_unit_form(&b_id))
        .unwrap_or_default();
    let mut ctx: HashMap<&str, String> = HashMap::new();
    ctx.insert(
        "buildings_count",
        format!(
            "{buildings_count} building{}",
            if buildings_count == 1 { "" } else { "s" }
        ),
    );
    ctx.insert("buildings_list", list_html);
    ctx.insert("detail", detail_html);
    ctx.insert("building_form_html", add_building_form());
    ctx.insert("unit_form_html", unit_form);

    Ok(Response::html(200, engine::render(BUILDINGS_HTML, &ctx)))
}

pub fn add(req: &Request, state: &Arc<AppState>) -> Result<Response, AppError> {
    let sess = auth::require_role(req, &state.sessions, Role::Landlord)?;
    let f = form::parse(&req.body);
    let name = f.get("name").cloned().unwrap_or_default();

    building_service::add(&state.db, &sess.user_id, name)?;
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
    tracing::info!(user_id = %sess.user_id, %building_id, "building deleted");

    Ok(Response::redirect("/buildings"))
}

fn add_building_form() -> String {
    r#"<form action="/landlord/buildings" method="POST" id="add-building-form">
    <fieldset class="form-group">
    <legend> Building Details</legend>
    <div class="input-container">
    <label for="building-name">Building Name</label>
    <input type="text" id="building-name" name="name">
    <span class="error-message" id="name-error"></span>
    </div>
    </fieldset>
    <button type="submit" class="form-button">Add Buildings</button>
    </form>
    "#
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
