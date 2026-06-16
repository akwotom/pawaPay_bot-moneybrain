use crate::telegram;

/**
 * Copyright 2026 Son of Binary
 * The pawaPay_bot Project
 * The moneybrain microservice
 *
 * This module contains the logic for handling telegram webhooks.
 */

///
pub(crate) async fn handle_telegram_webhook(
    req: axum::http::Request<axum::body::Body>,
) -> (axum::http::StatusCode, &'static str) {
    let body_txt = String::from_utf8(
        axum::body::to_bytes(req.into_body(), usize::MAX)
            .await
            .unwrap()
            .to_vec(),
    )
    .unwrap();

    println!("A webhook request just came in\n {}", body_txt);

    tokio::spawn(async move {
        let json: Result<serde_json::Value, serde_json::Error> =
            serde_json::from_str(body_txt.as_str());

        telegram::webhook::parse_webhook(match json {
            Ok(v) => v,
            Err(err) => {
                println!(
                    "Invalid payload sent for telegram webhook\n{}\nError:\n{}\n\n",
                    body_txt, err
                );
                return;
            }
        })
        .await;
    });

    (axum::http::StatusCode::OK, "Thank you")
}
