/*
    Copyright 2026 Son of Binary
    The pawaPay_bot Project
    The moneybrain microservice

    This module contains the logic for creating a new cheque.
    Such a function is called from the router.
*/

use crate::cheque::{self, begin_funding::begin_funding, models::*};

pub async fn fund_new_cheque(payload: CreateChequePayload) -> String {
    let CreateChequePayload {
        telegram,
        amount,
        payment_method,
    } = payload;

    let existing = cheque::db::find_by_tg_message_id(&telegram.inline_message_id).await;

    let cheque = match existing {
        Option::None => {
            let mut nw_cheque = Cheque {
                id: "".to_string(), // This will be automatically filled in when inserted into db.
                amount: amount.clone(),
                telegram: telegram.clone(),
                azaman: Option::None,
                created_at: chrono::Utc::now().to_rfc3339(),
                liquidated: false,
                funded: false,
                liquidation_lock: Option::None,
            };

            crate::cheque::db::insert(&mut nw_cheque).await;

            nw_cheque
        }
        Option::Some(v) => v,
    };

    let mut cheque_ref = cheque.clone();

    let cheque_id = cheque.id;

    // Now, if the cheque is not yet fully funded
    if !cheque_ref.funded {
        tokio::spawn(async move {
            begin_funding(&mut cheque_ref, &payment_method).await;
        });
    }

    return cheque_id;
}
