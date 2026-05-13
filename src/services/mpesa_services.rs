use base64::{Engine, engine::general_purpose};

use crate::{config::Config, error::AppError};

const SANDBOX_BASE: &str = "http://sandbox.safaricom.co.ke";

pub fn get_token(consumer_key: &str, consumer_secret: &str) -> Result<String, AppError> {
    tracing::debug!("requesting mpesa token");
    let credentials = format!("{}:{}", consumer_key, consumer_secret);

    let encoded = general_purpose::STANDARD.encode(credentials);

    tracing::debug!(encoded_credentials = %encoded, "token request credentials");

    let mut response = ureq::get(&format!(
        "{}/oauth/v1/generate?grant_type=client_credentials",
        SANDBOX_BASE
    ))
    .header("Authorization", &format!("Basic {}", encoded))
    .call()
    .map_err(|e| {
        tracing::error!("token request failed: {}", e);
        AppError::Internal(e.to_string())
    })?;

    let body = response
        .body_mut()
        .read_to_string()
        .map_err(|e| AppError::Internal(e.to_string()))?;

    tracing::debug!("mpesa token response: {}", body);

    let json: serde_json::Value =
        serde_json::from_str(&body).map_err(|e| AppError::Internal(e.to_string()))?;

    json["access_token"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| AppError::Internal("missing access_token".into()))
}

pub fn stk_push(cfg: &Config, phone: &str, amount: i32, unit_id: &str) -> Result<String, AppError> {
    let token = get_token(&cfg.mpesa_conusmer_key, &cfg.mpesa_secret_key)?;

    let timestamp = chrono::Local::now().format("%Y%m%d%H%M%S").to_string();

    let raw_password = format!("{}{}{}", cfg.mpesa_shortcode, cfg.mpesa_passkey, timestamp);
    let password = general_purpose::STANDARD.encode(raw_password);

    let body = serde_json::json!({
        "BusinessShortCode": cfg.mpesa_shortcode,
        "Password": password,
        "Timestamp": timestamp,
        "TransactionType": "CustomerPatBillOnline",
        "Amount": amount,
        "PartyA": phone,
        "PartyB": cfg.mpesa_shortcode,
        "PhoneNumber": phone,
        "CallBackURL": cfg.mpesa_callback,
        "AccountReference": unit_id,
        "TransactionDesc": "Rent Payment"
    });

    tracing::debug!("stk push payload: {}", body);
    tracing::debug!("stk push token: {}", token);

    let response: serde_json::Value =
        ureq::post(&format!("{}/mpesa/stkpush/v1/processrequest", SANDBOX_BASE))
            .header("Authorization", &format!("Bearer {}", token))
            .send_json(body)
            .map_err(|e| AppError::Internal(e.to_string()))?
            .body_mut()
            .read_json()
            .map_err(|e| AppError::Internal(e.to_string()))?;

    response["CheckoutRequestID"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| AppError::Internal("stk push failed".into()))
}
