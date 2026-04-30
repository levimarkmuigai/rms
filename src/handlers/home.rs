use std::sync::Arc;

use crate::{
    error::AppError,
    server::{request::Request, response::Response},
    state::AppState,
};

const INDEX_HTML: &str = include_str!("../templates/views/index.html");

pub fn index(_req: &Request, _state: &Arc<AppState>) -> Result<Response, AppError> {
    Ok(Response::html(200, INDEX_HTML.to_string()))
}
