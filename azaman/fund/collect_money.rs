/*
    Copyright 2026 Son of Binary
    The pawaPay_bot Project
    The moneybrain microservice

    This module contains the logic for collecting money from the user, via a given payment method.
*/

use crate::azaman::{Transaction, fund::db, gateway};

/// This function initiates a debit of the user, based on the transaction details provided.
pub async fn collect_money(transaction: &mut Transaction) -> Result<(), String> {
    db::insert(transaction).await;

    // Now, call the plugin to actually collect the money.
    // For now, there's just one provider (pawaPay)

    gateway::pawapay::collect_money(transaction)
        .await
        .map(|_| ())?; // If the API call is successful, do nothing. If it fails, just return the error.

    // Transaction is pending
    transaction.status = 20;

    let now_string = chrono::Utc::now().to_rfc3339();

    transaction.last_refresh_time = Option::Some(now_string);

    db::update(transaction).await;

    Result::<(), String>::Ok(())
}
