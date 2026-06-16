/*
    Copyright 2026 Son of Binary
    The pawaPay_bot Project
    The moneybrain microservice

    This module contain necessary data structures concerning the cheque feature.
*/

use crate::{Amount, azaman::PaymentMethod};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Cheque {
    pub(crate) id: String,
    pub(crate) amount: Amount,
    pub(crate) telegram: TelegramChequePayload,
    pub(crate) created_at: String,
    pub(crate) azaman: Option<AzamanChequePayload>,
    pub(crate) funded: bool,
    pub(crate) liquidated: bool,
    pub(crate) liquidation_lock: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct CreateChequePayload {
    pub(crate) amount: Amount,
    pub(crate) telegram: TelegramChequePayload,
    pub(crate) payment_method: PaymentMethod,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub(crate) struct TelegramChequePayload {
    pub(crate) inline_message_id: String,
    // pub(crate) author_user_id: String,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub(crate) struct AzamanChequePayload {
    pub collect_id: String,
    pub transfer_id: String,
}
