/*
    Copyright 2026 Son of Binary
    Theh pawaPay_bot Project
    The moneybrain microservice

    This module contains the logic for funding a cheque
*/

use crate::{
    azaman::{self, CallbackServiceInfo, PaymentMethod, Transaction},
    cheque::Cheque,
};

/// This method begins the process of allowing the user pay for this cheque (fund it).
pub(crate) async fn begin_funding(cheque: &mut Cheque, payment_method: &PaymentMethod) {
    // Here' we have to create a Collect transaction with the provider

    let Cheque { id, amount, .. } = cheque.clone();

    azaman::fund::collect_money(&mut Transaction {
        amount: amount.clone(),
        id: id.clone(),
        service_callback: CallbackServiceInfo {
            id: id.to_string(),
            name: "cheque".to_string(),
        },
        status: 0,
        provider_data: Option::None,
        payment_method: payment_method.clone(),
        last_refresh_time: Option::None,
    })
    .await
    .unwrap();

    // Now, the process of collecting money will begin. The user can poll to check status.
}
