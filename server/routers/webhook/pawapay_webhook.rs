/*
    Copyright 2026 Son of Binary
    The pawaPay_bot Project
    The moneybrain microservice

    This module contains the logic necessary for processing pawaPay's webhooks.
*/

use axum::response::IntoResponse;

use crate::azaman;

/// This function handles webhooks that come pawaPay.
pub async fn handle_pawapay_webhook(
    axum::extract::Json(body): axum::extract::Json<serde_json::Value>,
) -> axum::response::Response {
    let fund_id = body.get("depositId");

    if let Some(id) = fund_id {
        if let Option::Some(id) = id.as_str() {
            azaman::fund::on_callback_received(id.to_string());
        }
    }

    if let Some(id) = body.get("payoutId")
        && let Some(id) = id.as_str()
    {
        azaman::payout::on_callback_received(id.to_string());
    }

    (axum::http::StatusCode::OK, "Thank you").into_response()
}
