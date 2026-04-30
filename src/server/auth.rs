use crate::{
    entities::{session::Session, user::Role},
    error::AppError,
    server::request::Request,
    state::SessionMap,
};

fn extract_token(req: &Request) -> Option<String> {
    req.headers.get("cookie")?.split(';').find_map(|pair| {
        let (k, v) = pair.trim().split_once('=')?;
        (k.trim() == "session").then(|| v.trim().to_string())
    })
}

pub fn require_role(
    req: &Request,
    store: &SessionMap,
    required: Role,
) -> Result<Session, AppError> {
    let token = extract_token(req).ok_or(AppError::Unauthorized)?;
    let map = store.lock().unwrap();
    let sess = map.get(&token).cloned().ok_or(AppError::Unauthorized)?;

    if std::mem::discriminant(&sess.role) != std::mem::discriminant(&required) {
        return Err(AppError::Unauthorized);
    }

    Ok(sess)
}

pub fn require_any_role(req: &Request, store: &SessionMap) -> Result<Session, AppError> {
    let token = extract_token(req).ok_or(AppError::Unauthorized)?;
    let map = store.lock().unwrap();
    map.get(&token).cloned().ok_or(AppError::Unauthorized)
}

pub fn generate_token() -> String {
    use std::fs::File;
    use std::io::Read;
    let mut f = File::open("/dev/urandom").expect("/dev/urandom unavailable");
    let mut buf = [0u8; 32];
    f.read_exact(&mut buf).expect("urandom read failed");
    buf.iter().map(|b| format!("{b:02x}")).collect()
}
