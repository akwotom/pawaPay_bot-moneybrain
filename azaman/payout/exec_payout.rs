/*
    Copyright 2026 Son of Binary
    The pawaPay_bot Project
    The moneybrain microservice

    This module contains the logic for sending money to the user, via a given payment method.
*/

use crate::azaman::{Transaction, gateway, payout::db};

/// This function initiates a transfer to the user, based on the transaction details provided.
pub async fn exec_payout(transaction: &mut Transaction) -> Result<(), String> {
    db::insert(transaction).await;

    println!("Creating a payout for\n{transaction:?}\n");

    // Now, call the plugin to actually collect the money.
    // For now, there's just one provider (pawaPay)

    gateway::pawapay::send_money(transaction)
        .await
        .map(|_| ())?; // If the API call is successful, do nothing. If it fails, just return the error.

    // Transaction is pending
    transaction.status = 20;

    let now_string = chrono::Utc::now().to_rfc3339();

    transaction.provider_data = Option::Some(serde_json::json!({
        "last_refresh_time": now_string,
    }));

    db::update(transaction).await;

    Result::<(), String>::Ok(())
}
