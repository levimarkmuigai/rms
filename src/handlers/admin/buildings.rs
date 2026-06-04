use std::{collections::HashMap, sync::Arc};

use uuid::Uuid;

use crate::{
    entities::user::Role,
    error::AppError,
    repositories::building_repo,
    server::{auth, form, request::Request, response::Response},
    state::AppState,
    templates::engine,
};

const BUILDINGS_HTML: &str = include_str!("../../templates/views/admin/building.html");

pub fn show(req: &Request, state: &Arc<AppState>) -> Result<Response, AppError> {
    let _sess = auth::require_role(req, &state.sessions, Role::Admin)?;
    let buildings = building_repo::find_all(&state.db)?;

    let buildings_count = buildings.len();

    let table_rows: String = buildings
        .into_iter()
        .map(|b| {
            format!(
                r#"<tr>
                  <td>{name}</td>
                  <td>{owner}</td>
                  <td>
                    <form action="/admin/building/delete" method="POST">
                      <input type="hidden" name="building_id" value="{id}">
                      <button type="submit" class="danger">delete</button>
                    </form>
                  </td>
                </tr>"#,
                name = b.name,
                owner = b.owner,
                id = b.id,
            )
        })
        .collect();

    let mut ctx = HashMap::new();
    ctx.insert("buildings_count", format!("{buildings_count} buildings"));
    ctx.insert("building_rows", table_rows);

    Ok(Response::html(200, engine::render(BUILDINGS_HTML, &ctx)))
}

pub fn delete_building(req: &Request, state: &Arc<AppState>) -> Result<Response, AppError> {
    let f = form::parse(&req.body);
    let building_id: Uuid = f
        .get("building_id")
        .and_then(|v| v.parse().ok())
        .ok_or_else(|| AppError::BadRequest("invalid building_id".into()))?;

    building_repo::delete(&state.db, &building_id)?;

    Ok(Response::redirect("/admin/buildings"))
}
