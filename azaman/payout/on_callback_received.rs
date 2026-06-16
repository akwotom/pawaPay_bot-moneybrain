/*
    Copyright 2026 Son of Binary
    The pawaPay_bot Project
    The moneybrain microservice

    This module contains the logic for handling callbacks from external payment provider.
*/

use crate::azaman::{TransactionType, payout::db, gateway, on_transaction_complete};

/// This method processes all incoming callbacks for funding transactions.
pub fn on_callback_received(id: String) {
    tokio::spawn(async move {
        let mut txn = match db::find_by_id(id.clone()).await {
            Option::None => {
                println!(
                    "Could not process a callback for an azaman payout transaction, because nothing was found for {id}"
                );
                return;
            }
            Option::Some(v) => v,
        };

        if txn.is_complete() {
            return;
        }

        println!("Processing callback for id {id}, which matches this record\n{txn:?}\n");
        // Now, let's refresh the transaction from the provider's side, before we process callbacks.
        // TODO: Dynamically change the payment plugin.
        gateway::pawapay::poll_fund(&mut txn).await;
        if txn.is_complete() {
            on_transaction_complete(&txn, TransactionType::Fund);
        }
    });
}
