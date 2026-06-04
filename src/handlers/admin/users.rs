use std::{collections::HashMap, sync::Arc};

use uuid::Uuid;

use crate::{
    entities::user::Role,
    error::AppError,
    repositories::user_repo,
    server::{auth, form, request::Request, response::Response},
    state::AppState,
    templates::engine,
};

const USERS_HTML: &str = include_str!("../../templates/views/admin/users.html");

pub fn show(req: &Request, state: &Arc<AppState>) -> Result<Response, AppError> {
    let _sess = auth::require_role(req, &state.sessions, Role::Admin)?;
    let users = user_repo::find_all(&state.db)?;
    let user_count = users.len();

    let table_rows: String = users
        .into_iter()
        .map(|u| {
            format!(
                r#"<tr>
                  <td>{email}</td>
                  <td>{role}</td>
                  <td>
                    <form action="/admin/users/delete" method="POST">
                      <input type="hidden" name="user_id" value="{id}">
                      <button type="submit" class="danger">delete</button>
                    </form>
                  </td>
                </tr>"#,
                email = u.email,
                role = u.role,
                id = u.id,
            )
        })
        .collect();

    let mut ctx = HashMap::new();
    ctx.insert("user_count", format!("{user_count} users"));
    ctx.insert("user_rows", table_rows);

    Ok(Response::html(200, engine::render(USERS_HTML, &ctx)))
}

pub fn delete_user(req: &Request, state: &Arc<AppState>) -> Result<Response, AppError> {
    let f = form::parse(&req.body);

    let user_id: Uuid = f
        .get("users_id")
        .and_then(|v| v.parse().ok())
        .ok_or(AppError::BadRequest("user_id not found".into()))?;

    user_repo::delete(&state.db, &user_id)?;

    Ok(Response::redirect("/admin/users"))
}
