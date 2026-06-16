/**
 * Copyright 2026 Son of Binary
 * The pawaPay_bot Project
 * The moneybrain microservice
 *
 * This module is the entry point for the webhook_handler module.
 * webhook_handler specializes in handling all webhooks, especially those from Telegram.
 */
mod router;
pub use router::*;
mod pawapay_webhook;
mod telegram_webhook;
