/*
    Copyright 2026 Son of Binary
    The pawaPay_bot Project
    The moneybrain microservice

    This module contains the logic for liquidating a cheque.
    Such a function is called from the router.
*/

use crate::{
    azaman::{self},
    cheque::{Cheque, db},
    server,
};

pub async fn liquidate_cheque_by_id(
    cheque_id: String,
    payment_method: azaman::PaymentMethod,
) -> Result<Cheque, server::error::APIError> {
    let mut cheque = match db::find_by_id(&cheque_id).await {
        Option::Some(v) => v,
        Option::None => {
            return Result::Err(server::error::APIError::Custom {
                msg: format!("The transaction with id {cheque_id} was not found"),
                code: axum::http::StatusCode::NOT_FOUND,
            });
        }
    };

    if cheque.liquidated {
        return Result::Ok(cheque);
    }

    liquidate_cheque(&mut cheque, payment_method).await;

    Result::Ok(cheque)
}

async fn liquidate_cheque(cheque: &mut Cheque, payment_method: azaman::PaymentMethod) {
    if let Option::Some(payout_id) = &cheque.liquidation_lock {
        // Let's check if that lock is still valid
        match azaman::payout::find_by_id(payout_id.clone()).await {
            Option::None => {}
            Option::Some(v) => {
                if v.status > 0 {
                    return; // We cannot take the risk of creating a new transaction if the status is non-failed.
                }
            }
        };
    }

    if cheque.liquidated {
        return;
    }

    let id = &short_uuid::ShortUuid::generate().to_string();

    cheque.liquidation_lock = Option::Some(id.clone());

    db::update(&cheque).await;

    azaman::payout::exec_payout(&mut azaman::Transaction {
        amount: cheque.amount.clone(),
        id: id.clone(),
        service_callback: azaman::CallbackServiceInfo {
            id: cheque.id.to_string(),
            name: "cheque".to_string(),
        },
        status: 0,
        provider_data: Option::None,
        payment_method: payment_method.clone(),
        last_refresh_time: Option::None,
    })
    .await
    .map_err(|e| format!("Could Not liquidate cheque {}\n{e}\n", cheque.id.clone()))
    .unwrap();
}
