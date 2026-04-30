use std::sync::Arc;

use crate::{
    entities::{session::Session, user::Role},
    error::AppError,
    server::{auth, form, request::Request, response::Response},
    services::user_service,
    state::AppState,
};

const INDEX_HTML: &str = include_str!("../templates/views/index.html");

pub fn home_page(_req: &Request, _state: &Arc<AppState>) -> Result<Response, AppError> {
    Ok(Response::html(200, INDEX_HTML.to_string()))
}

pub fn signup_submit(req: &Request, state: &Arc<AppState>) -> Result<Response, AppError> {
    let f = form::parse(&req.body);

    let first = f.get("first-name").cloned().unwrap_or_default();
    let last = f.get("last-name").cloned().unwrap_or_default();
    let email = f.get("email").cloned().unwrap_or_default();
    let number = f.get("number").cloned().unwrap_or_default();
    let role = f.get("role").cloned().unwrap_or_default();
    let password = f.get("password").cloned().unwrap_or_default();
    let name = format!("{first} {last}");

    if name.is_empty()
        || email.is_empty()
        || number.is_empty()
        || role.is_empty()
        || password.is_empty()
    {
        return Err(AppError::BadRequest("all fields required".into()));
    }

    user_service::signup(&state.db, name, email, number, &role, password)?;
    tracing::info!("new user registered");
    Ok(Response::redirect("/"))
}

pub fn login_submit(req: &Request, state: &Arc<AppState>) -> Result<Response, AppError> {
    let fields = form::parse(&req.body);
    let email = fields.get("email").cloned().unwrap_or_default();
    let pass = fields.get("password").cloned().unwrap_or_default();

    let user = user_service::authenticate(&state.db, &email, &pass)?;

    let token = auth::generate_token();
    let sess = Session {
        user_id: *user.id.value(),
        role: user.role.clone(),
        name: user.name.clone(),
    };

    state.sessions.lock().unwrap().insert(token.clone(), sess);

    let redirect = match user.role {
        Role::Landlord => "/landlord",
        Role::Caretaker => "/caretaker",
        Role::Tenant => "/tenant",
    };

    let cookie = format!("session={token}; HttpOnly; Path=/");
    Ok(Response::redirect_with_cookie(redirect, &cookie))
}
