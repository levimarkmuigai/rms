use std::{collections::HashMap, fs};

use crate::{error::AppError, server::response::Response};

pub fn serve(path: &str) -> Result<Response, AppError> {
    let disk_path = path.trim_start_matches('/');
    let bytes = fs::read(disk_path).map_err(|_| AppError::NotFound(path.to_string()))?;
    let mime = mime_type(path);

    let mut headers = HashMap::new();
    headers.insert("Content-Type".into(), mime.to_string());
    headers.insert("Content-Length".into(), bytes.len().to_string());
    headers.insert("Connection".into(), "close".into());
    headers.insert("Cache-Control".into(), "public, max-age=3600".into());

    Ok(Response {
        status: 200,
        headers,
        body: bytes,
    })
}

fn mime_type(path: &str) -> &'static str {
    if path.ends_with(".css") {
        "text/css"
    } else if path.ends_with(".js") {
        "text/javascript"
    } else if path.ends_with(".html") {
        "text/html; charset=utf-8"
    } else {
        "application/octet-stream"
    }
}
