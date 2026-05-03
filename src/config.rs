use std::env;

pub struct Config {
    pub addr: String,
    pub database_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            addr: env::var("RMS_ADDR").unwrap_or_else(|_| "127.0.0.1:7878".into()),
            database_url: env::var("RMS_DATABASE_URL").expect("RMS_DATABASE_URL must be set"),
        }
    }
}
