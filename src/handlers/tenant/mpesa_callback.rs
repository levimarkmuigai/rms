use std::sync::Arc;

use crate::{
    error::AppError,
    repositories::payment_repo,
    server::{request::Request, response::Response},
    state::AppState,
};

pub fn callback(req: &Request, state: &Arc<AppState>) -> Result<Response, AppError> {
    let body: serde_json::Value = serde_json::from_slice(&req.body)
        .map_err(|_| AppError::BadRequest("invalid json".into()))?;

    let result_code = body["Body"]["stkCallback"]["ResultCode"]
        .as_i64()
        .unwrap_or(1);

    if result_code == 0 {
        let checkout_request_id = body["Body"]["stkCallback"]["CheckoutRequestID"]
            .as_str()
            .unwrap_or("");

        payment_repo::confirm(&state.db, checkout_request_id)?;
        tracing::info!(checkout_request_id, "payment confirmed");
    } else {
        tracing::warn!(result_code, "mpesa payment failed or cancelled");
    }

    Ok(Response::html(200, "OK".into()))
}
