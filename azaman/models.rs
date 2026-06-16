/*
    Copyright 2026 Son of Binary
    The pawaPay_bot Project
    The moneybrain microservice

    This module contains useful data structure related to the money collection/transfer feature.
*/

use crate::Amount;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct PaymentMethod {
    pub account_id: String,
    pub provider: String,
}

/// This data structure represents both money transfer and collecting.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Transaction {
    pub amount: Amount,
    pub id: String,
    /// Information about the service that would be called back should the transaction get completed
    pub service_callback: CallbackServiceInfo,
    /// 10 -> created, 20 -> processing, 30 -> successful
    pub status: i32,
    /// Provider-specific data, from the external payment gateway.
    pub provider_data: Option<serde_json::Value>,

    pub last_refresh_time: Option<String>,

    pub payment_method: PaymentMethod,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct CallbackServiceInfo {
    pub name: String,
    /// A unique ID for this transaction that would be used by the service
    /// to know how to find data related to this transaction
    pub id: String,
}

pub enum TransactionType {
    Fund,
    Payout,
}

impl Transaction {
    pub fn is_complete(&self) -> bool {
        return self.status >= 30 || self.status < 0;
    }
}
