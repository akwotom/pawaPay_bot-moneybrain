/*
    Copyright 2026 Son of Binary
    The pawaPay_bot Project
    The moneybrain microservice

    This module is focused on the logic of polling transactions
*/

use crate::azaman::{Transaction, gateway, payout::db};

/// This function calls the provider to check the latest status of the transaction.
/// There's a rate limit to how transactions can be refreshed.
/// Ideally, that is once every 30s
/// The transaction will not refresh if it was recently refreshed.
pub async fn refresh_with_provider(id: String) -> Result<Option<Transaction>, String> {
    let mut txn = match db::find_by_id(id).await {
        Option::Some(v) => v,
        Option::None => return Result::Ok(Option::None),
    };

    // Now, is the transaction mature enough to be refreshed?
    let ref_txn = txn.clone();

    if txn.is_complete() {
        println!("No need to refresh this transaction\n{txn:?}");
        return Result::Ok(Option::Some(txn));
    }

    async fn do_refresh(txn: &mut Transaction) {
        // The original txn is mutated here, and here only.
        gateway::pawapay::poll_payout(txn).await;

        let txn_clone = txn.clone();
        tokio::spawn(async move {
            // Update the db in a different thread, while immediately returning.
            db::update(&txn_clone).await;
        });
    }

    let provider_data = ref_txn.provider_data.clone();

    if txn.provider_data.is_none() {
        do_refresh(&mut txn).await;
    } else {
        let last_refresh_time = provider_data.unwrap();
        let last_refresh_time = last_refresh_time.get("last_refresh_time");

        if last_refresh_time.is_none() {
            do_refresh(&mut txn).await;
        } else {
            let last_refresh_time = last_refresh_time.unwrap().to_string();
            let last_refresh_time =
                chrono::DateTime::parse_from_rfc2822(last_refresh_time.as_str());

            if last_refresh_time.is_err() {
                do_refresh(&mut txn).await;
            } else {
                let last_refresh_time = last_refresh_time.unwrap();
                let next_refresh_time = last_refresh_time + chrono::TimeDelta::seconds(30);
                if chrono::Utc::now().gt(&next_refresh_time) {
                    // This is the original condition for refreshing.
                    // We refresh when the current time is already more than the next refresh time
                    do_refresh(&mut txn).await;
                }
            }
        }
    }

    return Result::Ok(Option::Some(txn.clone()));
}
