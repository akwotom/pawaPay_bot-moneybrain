/**
 * Copyright 2026 Son of Binary
 * The pawaPay_bot Project
 * The moneybrain microservice
 *
 * This module handles http requests that are intended to be webhook callbacks, such as those coming from Telegram.
 */
use crate::server::routers::webhook::{
    pawapay_webhook::handle_pawapay_webhook, telegram_webhook::handle_telegram_webhook,
};

pub fn webhook_router() -> axum::Router {
    axum::Router::new()
        .route("/telegram", axum::routing::any(handle_telegram_webhook))
        .route(
            "/payment/pawaPay",
            axum::routing::post(handle_pawapay_webhook),
        )
}
