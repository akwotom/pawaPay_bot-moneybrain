/*
 * Copyright 2026 Son of Binary
 * The pawaPay_bot Project
 * The moneybrain microservice
 *
 * This module contains routing logic for the API features provided by this microservice.
 */

use axum::response::IntoResponse;

use crate::{
    azaman,
    cheque::{self, CreateChequePayload, fund_new_cheque},
};

pub fn api_router() -> axum::Router {
    let router = axum::Router::new();

    return router
        .route(
            "/cheque/fund-new-cheque",
            axum::routing::post(
                |axum::Json(body): axum::Json<serde_json::Value>| async move {
                    let payload: CreateChequePayload = match serde_json::from_value(body.clone()) {
                        Result::Ok(v) => v,
                        Result::Err(e) => {
                            eprintln!("The user sent an invalid request\n{body}\n{e}\n");
                            return crate::server::error::APIError::BadRequest {
                                msg: "Invalid request payload.".to_string(),
                            }
                            .into_response();
                        }
                    };

                    // Now, let's respond to the user with the information for a cheque.

                    let result = fund_new_cheque(payload).await;

                    return CreateChequeResponse { id: result }.into_response();
                },
            ),
        )
        .route(
            "/cheque/{id}/liquidate",
            axum::routing::post(
                |axum::extract::Path(id): axum::extract::Path<String>,
                 axum::extract::Json(demand): axum::extract::Json<LiquidateChequeResponse>| async {
                    cheque::liquidate_cheque_by_id(id, demand.payment_method).await.map (|ch| serde_json::json!({
                        "liquidated": ch.liquidated,
                        "amount": ch.amount,
                    }).to_string())

                },
            ),
        )
        .route(
            "/cheque/{id}/payment-states",
            axum::routing::get(
                |axum::extract::Path(id): axum::extract::Path<String>| async move {
                    // The user wants to know the payment states of the cheque
                    let cheque = cheque::find_and_poll(id.clone()).await;

                    let cheque = match cheque {
                        Option::None => {
                            return (
                                axum::http::StatusCode::NOT_FOUND,
                                format!("The transaction {id} was not found."),
                            )
                                .into_response();
                        }
                        Option::Some(v) => v,
                    };

                    (
                        axum::http::StatusCode::OK,
                        serde_json::json!({
                            "funded": cheque.funded,
                            "liquidated": cheque.liquidated,
                        })
                        .to_string(),
                    )
                        .into_response()
                },
            ),
        );
}

#[derive(serde::Deserialize, serde::Serialize)]
struct CreateChequeResponse {
    id: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct LiquidateChequeResponse {
    payment_method: azaman::PaymentMethod,
}

impl axum::response::IntoResponse for CreateChequeResponse {
    fn into_response(self) -> axum::response::Response {
        (
            axum::http::StatusCode::OK,
            serde_json::json!({
                "data": self
            })
            .to_string(),
        )
            .into_response()
    }
}
