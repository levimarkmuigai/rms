use std::net::TcpStream;

use crate::{
    error::AppError,
    server::{response::Response, router::Router},
};

pub mod auth;
pub mod form;
pub mod request;
pub mod response;
pub mod router;
pub mod static_files;

pub fn handle_connection(mut stream: TcpStream, router: &Router) -> Result<(), AppError> {
    let req = request::parse(&mut stream)?;

    tracing::debug!(method = %req.method, path = %req.path, "request");

    let res = router.dispatch(&req).unwrap_or_else(|e| match e {
        AppError::Unauthorized => Response::redirect("/"),
        AppError::NotFound(_) => Response::html(404, "<h1>404 Not Found</h1>".into()),
        AppError::BadRequest(m) => Response::html(400, format!("<h1>Bad Request: {m}</h1>")),
        _ => Response::html(500, "<h1>Internal Server Error</h1>".into()),
    });

    tracing::debug!(status = res.status, "response");

    response::write(&mut stream, res)
}
