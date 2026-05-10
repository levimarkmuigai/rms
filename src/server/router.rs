use std::{collections::HashMap, sync::Arc};

use crate::{
    error::AppError,
    server::{request::Request, response::Response, static_files},
    state::AppState,
};

pub type Handler = fn(&Request, &Arc<AppState>) -> Result<Response, AppError>;

pub struct Router {
    routes: HashMap<(&'static str, &'static str), Handler>,
    pub state: Arc<AppState>,
}

impl Router {
    pub fn dispatch(&self, req: &Request) -> Result<Response, AppError> {
        if req.path.starts_with("/static/") {
            return static_files::serve(&req.path);
        }
        self.routes
            .get(&(req.method.as_str(), req.path.as_str()))
            .copied()
            .ok_or_else(|| {
                tracing::warn!(method = %req.method, path = %req.path, "no route method");
                AppError::NotFound(req.path.clone())
            })
            .and_then(|h| h(req, &self.state))
    }
}

pub fn build(state: Arc<AppState>) -> Router {
    let mut routes: HashMap<(&'static str, &'static str), Handler> = HashMap::new();
    routes.insert(("POST", "/login"), crate::handlers::auth::login_submit);
    routes.insert(("POST", "/signup"), crate::handlers::auth::signup_submit);
    routes.insert(
        ("POST", "/update_profile"),
        crate::handlers::users::update_profile,
    );
    routes.insert(("GET", "/"), crate::handlers::home::index);

    routes.insert(
        ("GET", "/landlord"),
        crate::handlers::landlord::dashboard::show,
    );
    routes.insert(
        ("GET", "/landlord/buildings"),
        crate::handlers::landlord::buildings::show,
    );
    routes.insert(
        ("GET", "/landlord/units"),
        crate::handlers::landlord::unit::show,
    );
    routes.insert(
        ("POST", "/landlord/buildings"),
        crate::handlers::landlord::buildings::add,
    );
    routes.insert(
        ("POST", "/landlord/buildings/delete"),
        crate::handlers::landlord::buildings::delete,
    );
    routes.insert(
        ("POST", "/landlord/units"),
        crate::handlers::landlord::unit::add,
    );
    routes.insert(
        ("POST", "/landlord/unit/assign"),
        crate::handlers::landlord::unit::assign_unit,
    );
    routes.insert(
        ("POST", "/landlord/unit/vacate"),
        crate::handlers::landlord::unit::vacate,
    );
    routes.insert(
        ("POST", "/landlord/building/assign"),
        crate::handlers::landlord::buildings::assign,
    );
    routes.insert(
        ("POST", "/landlord/caretaker/release"),
        crate::handlers::landlord::dashboard::release_caretaker,
    );

    routes.insert(
        ("GET", "/caretaker"),
        crate::handlers::caretaker::dashboard::show,
    );
    routes.insert(
        ("POST", "/caretaker/request/start"),
        crate::handlers::caretaker::dashboard::inprogress,
    );
    routes.insert(
        ("POST", "/caretaker/request/resolve"),
        crate::handlers::caretaker::dashboard::resolve,
    );

    routes.insert(("GET", "/tenant"), crate::handlers::tenant::dashboard::show);
    Router { routes, state }
}
