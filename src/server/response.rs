use std::{collections::HashMap, io::Write, net::TcpStream};

use crate::error::AppError;

pub struct Response {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Response {
    pub fn html(status: u16, body: String) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".into(), "text/html; charset=utf-8".into());
        headers.insert("Content-Length".into(), body.len().to_string());
        headers.insert("Connection".into(), "close".into());
        Self {
            status,
            headers,
            body: body.into_bytes(),
        }
    }

    pub fn redirect(location: &str) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Location".into(), location.to_string());
        headers.insert("Content-Length".into(), "0".into());
        headers.insert("Connection".into(), "close".into());

        Self {
            status: 302,
            headers,
            body: vec![],
        }
    }

    pub fn redirect_with_cookie(location: &str, cookie: &str) -> Self {
        let mut res = Self::redirect(location);
        res.headers.insert("Set-Cookie".into(), cookie.to_string());
        res
    }
}

pub fn write(stream: &mut TcpStream, res: Response) -> Result<(), AppError> {
    let reason = reason(res.status);
    stream.write_all(format!("HTTP/1.1 {} {}\r\n", res.status, reason).as_bytes())?;
    for (k, v) in &res.headers {
        stream.write_all(format!("{k}: {v}\r\n").as_bytes())?;
    }
    stream.write_all(b"\r\n")?;
    stream.write_all(&res.body)?;
    stream.flush().map_err(AppError::Io)
}

fn reason(code: u16) -> &'static str {
    match code {
        200 => "OK",
        400 => "Bad Request",
        404 => "Not Found",
        500 => "Internal Server Error",
        _ => "Unknown",
    }
}
