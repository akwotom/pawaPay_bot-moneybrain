/*
    Copyright 2026 Son of Binary
    The pawaPay_bot Project
    The moneybrain microservice

    This module is concerned with just one job: collect the user's money, using pawaPay's payment gateway.
*/

use crate::{
    Amount,
    azaman::{PaymentMethod, Transaction, gateway::pawapay::*},
};

/// This method directly collects the user's money, via pawaPay's payment gateway.
pub async fn collect_money(transaction: &Transaction) -> Result<(), String> {
    //
    let Transaction {
        amount,
        id,
        payment_method,
        ..
    } = transaction;

    let Amount {
        value: amount_value,
        currency_code,
    } = amount;

    let PaymentMethod {
        account_id: payment_method_account_id,
        provider: payment_method_provider,
    } = payment_method;

    client::make_request(
        "POST",
        "/deposits",
        Option::Some(
            serde_json::json!({
                "depositId": id,
                "amount": format!("{amount_value}"),
                "currency": currency_code,
                "payer": {
                    "type": "MMO",
                    "accountDetails": {
                        "phoneNumber": payment_method_account_id,
                        "provider": payment_method_provider,
                    }
                }

            })
            .to_string(),
        ),
    )
    .await?; // If it fails, just return an error result

    Result::Ok(())
}
