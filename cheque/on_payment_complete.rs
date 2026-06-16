/*
    Copyright 2026 Son of Binary
    The pawaPay_bot Project
    The moneybrain microservice

    This module contains the logic for vital processes that run after payments that concern cheques are done.
*/

use crate::azaman::{Transaction, TransactionType};

pub async fn on_payment_complete(txn: &Transaction, txn_type: TransactionType) {
    let cheque = crate::cheque::db::find_by_id(&txn.service_callback.id).await;

    let mut cheque = match cheque {
        Option::None => {
            println!("We received a payment complete notice for a cheque that doesn't exist.");
            return;
        }
        Option::Some(v) => v,
    };

    match txn_type {
        TransactionType::Fund => {
            // Then payment for a cheque might be complete
            if txn.status < 30 {
                return;
            }

            cheque.funded = true;

            println!(
                "Funding of the following cheque has be verified via a callback\n{cheque:?}\nThe transaction:\n{txn:?}",
            );
        }
        TransactionType::Payout => {
            if txn.status > 30 {
                cheque.liquidated = true;
                cheque.liquidation_lock = Option::None;
            } else if let Option::Some(liquidation_lock) = &cheque.liquidation_lock
                && liquidation_lock.clone() == txn.id
            {
                cheque.liquidation_lock = Option::None;
            }
        }
    };

    crate::cheque::db::update(&cheque).await;
}
