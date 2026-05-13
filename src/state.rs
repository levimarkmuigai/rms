use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{db::PgPool, entities::session::Session};

pub type SessionMap = Mutex<HashMap<String, Session>>;

pub struct AppState {
    pub sessions: SessionMap,
    pub db: PgPool,
}

impl AppState {
    pub fn new(db: PgPool) -> Arc<Self> {
        Arc::new(Self {
            sessions: Mutex::new(HashMap::new()),
            db,
        })
    }
}
