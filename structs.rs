/*
   Copyright 2026 Son of Binary
   The pawaPay_bot Project
   The moneybrain microservice

   This module contains necessary type definitions for data types that are used throughout the entire project.
*/

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Amount {
    pub value: i32,
    pub currency_code: String,
}
