/**
 * Copyright 2026 Son of Binary
 * The pawaPay_bot Project
 * The moneybrain microservice
 *
 * This module defines necessary errors and an error handler that makes it easier to provide smooth HTTP responses.
 */
use axum::response::IntoResponse;
use futures::FutureExt;
use std::panic::AssertUnwindSafe;

/// This function catches all error that occur when serving a user's HTTP request.
pub(crate) async fn panic_handler_middleware(
    req: axum::http::Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> axum::response::Response {
    let result = AssertUnwindSafe(next.run(req)).catch_unwind();

    match result.await {
        Ok(value) => value,
        Err(_) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Server Error",
        )
            .into_response(),
    }
}

#[derive(Debug)]
pub enum APIError {
    Custom {
        msg: String,
        code: axum::http::StatusCode,
    },

    BadRequest {
        msg: String,
    },
}

impl axum::response::IntoResponse for APIError {
    fn into_response(self) -> axum::response::Response {
        fn convert(item: APIError) -> (axum::http::StatusCode, String) {
            match item {
                APIError::Custom { msg, code } => {
                    let json_string = serde_json::json!({
                    "success": false,
                    "msg": msg,
                    })
                    .to_string();

                    (code, serde_json::to_string(&json_string).unwrap())
                }

                APIError::BadRequest { msg } => convert(APIError::Custom {
                    msg,
                    code: axum::http::StatusCode::BAD_REQUEST,
                }),
            }
        }
        convert(self).into_response()
    }
}
