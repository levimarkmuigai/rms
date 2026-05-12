use std::env;

pub struct Config {
    pub addr: String,
    pub database_url: String,
    pub mpesa_conusmer_key: String,
    pub mpesa_secret_key: String,
    pub mpesa_shortcode: String,
    pub mpesa_passkey: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            addr: env::var("RMS_ADDR").unwrap_or_else(|_| "127.0.0.1:7878".into()),
            database_url: env::var("RMS_DATABASE_URL").expect("RMS_DATABASE_URL must be set"),
            mpesa_conusmer_key: env::var("MPESA_CONSUMER_KEY")
                .expect("MPESA_CONSUMER_KEY must be set"),
            mpesa_secret_key: env::var("MPESA_SECRET_KEY").expect("MPESA_SECRET_KEY must be set"),
            mpesa_shortcode: env::var("MPESA_SHORTCODE").expect("MPESA_SHORTCODE must be set"),
            mpesa_passkey: env::var("MPESA_PASSKEY").expect("MPESA_PASSKEY must be set"),
        }
    }
}
