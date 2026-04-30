use std::{net::TcpListener, sync::Arc};

use crate::{config::Config, state::AppState};

pub mod config;
pub mod db;
pub mod entities;
pub mod error;
pub mod handlers;
pub mod repositories;
pub mod server;
pub mod services;
pub mod state;
pub mod templates;
fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rms=debug".into()),
        )
        .init();

    let cfg = Config::default();
    let pool = db::build_pool(&cfg.database_url);
    let listener = TcpListener::bind(cfg.addr.clone()).expect("bind failed");
    let state = AppState::new(pool);
    let router = Arc::new(server::router::build(state));

    tracing::info!("listening on http://{}", cfg.addr);

    for stream in listener.incoming().flatten() {
        let router = Arc::clone(&router);
        std::thread::spawn(move || {
            if let Err(e) = server::handle_connection(stream, &router) {
                tracing::error!("{e}");
            }
        });
    }
}
