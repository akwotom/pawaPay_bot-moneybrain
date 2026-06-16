/*
    Copyright 2026 Son of Binary
    The pawaPay_bot Project
    The moneybrain microservice

    This module contains the logic for handling situations when a transaction completes, whether for funds or transfers
*/

use crate::{
    azaman::{Transaction, TransactionType},
    cheque,
};

/// This method performs very vital tasks after a transaction completes.
pub fn on_transaction_complete(txn: &Transaction, txn_type: TransactionType) {
    if txn.service_callback.name == "cheque" {
        let txn = txn.clone();
        tokio::spawn(async move {
            cheque::on_payment_complete(&txn, txn_type).await;
        });
    }
}
