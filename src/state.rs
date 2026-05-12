use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{config::Config, db::PgPool, entities::session::Session};

pub type SessionMap = Mutex<HashMap<String, Session>>;

pub struct AppState {
    pub sessions: SessionMap,
    pub db: PgPool,
    pub cfg: Config,
}

impl AppState {
    pub fn new(db: PgPool, cfg: Config) -> Arc<Self> {
        Arc::new(Self {
            sessions: Mutex::new(HashMap::new()),
            db,
            cfg,
        })
    }
}
