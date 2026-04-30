use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read},
    net::TcpStream,
};

use crate::error::AppError;

pub struct Request {
    pub method: String,
    pub path: String,
    pub query: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

pub fn parse(stream: &mut TcpStream) -> Result<Request, AppError> {
    let mut reader = BufReader::new(stream);

    let mut request_line = String::new();
    reader.read_line(&mut request_line)?;

    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or("GET").to_string();
    let raw_path = parts.next().unwrap_or("/").to_string();
    let (path, query) = split_path(&raw_path);

    let mut headers = HashMap::new();
    loop {
        let mut line = String::new();
        reader.read_line(&mut line)?;
        if line.trim().is_empty() {
            break;
        }
        if let Some((k, v)) = line.trim().split_once(':') {
            headers.insert(k.trim().to_lowercase(), v.trim().to_string());
        }
    }

    let body = match headers.get("content-length").and_then(|v| v.parse().ok()) {
        Some(len) => {
            let mut buf = vec![0u8; len];
            reader.read_exact(&mut buf)?;
            buf
        }
        None => vec![],
    };

    Ok(Request {
        method,
        path,
        query,
        headers,
        body,
    })
}

fn split_path(raw: &str) -> (String, HashMap<String, String>) {
    match raw.split_once('?') {
        None => (raw.to_string(), HashMap::new()),
        Some((path, qs)) => {
            let query = qs
                .split('&')
                .filter_map(|p| {
                    p.split_once('=')
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                })
                .collect();
            (path.to_string(), query)
        }
    }
}
