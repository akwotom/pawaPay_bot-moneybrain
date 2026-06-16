/*
    Copyright 2026 Son of Binary
    The pawaPay_bot Project
    The moneybrain microservice

    This module contains the logic for polling a transaction on pawapay
*/

use crate::azaman::{Transaction, gateway::pawapay::client};

/// This method fetches the latest information about a transaction from the pawaPay
pub async fn poll_fund(txn: &mut Transaction) {
    let id = &txn.id;

    let info = client::make_request("GET", format!("/deposits/{id}").as_str(), Option::None).await;

    let info = match info {
        Err(e) => {
            eprintln!("Could not refresh transaction on pawaPay.\n{e}\n\n");
            return;
        }
        Ok(v) => v,
    };

    let info = match info.get("data") {
        Option::None => {
            eprintln!(
                "The response from pawaPay when refreshing a payment {id} was unexpected\n{info}\n"
            );
            return;
        }
        Option::Some(v) => v,
    };

    txn.provider_data = Option::Some(info.clone());

    let status = match info.get("status") {
        Option::Some(v) => v,
        Option::None => return,
    };

    if status.as_str().unwrap().to_uppercase() == "COMPLETED" {
        txn.status = 30;
    }
}

/// This method fetches the latest information about a transaction from the pawaPay
pub async fn poll_payout(txn: &mut Transaction) {
    let id = &txn.id;

    let info = client::make_request("GET", format!("/payouts/{id}").as_str(), Option::None).await;

    let info = match info {
        Err(e) => {
            eprintln!("Could not refresh payout {id} on pawaPay.\n{e}\n\n");
            return;
        }
        Ok(v) => v,
    };

    let info = match info.get("data") {
        Option::None => {
            eprintln!(
                "The response from pawaPay when refreshing a payout {id} was unexpected\n{info}\n"
            );
            return;
        }
        Option::Some(v) => v,
    };

    txn.provider_data = Option::Some(info.clone());

    let status = match info.get("status") {
        Option::Some(v) => v,
        Option::None => return,
    };

    if status.as_str().unwrap().to_uppercase() == "COMPLETED" {
        txn.status = 30;
    }
}
