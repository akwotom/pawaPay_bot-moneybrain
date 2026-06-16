/**
 * Copyright 2026 Son of Binary
 * The pawaPay_bot Project
 * The moneybrain microservice
 *
 * This module is the entry point of the Telegram module.
 * The module is intended to manage all logic involved with communicating with Telegram.
 */
mod api;
mod cheque;

pub mod webhook;

// Do some necessary checks

#[derive(Clone)]
struct TelegramModuleState {
    bot_name: &'static str,
    api_key: &'static str,
}

static mut TG_STATE: std::sync::LazyLock<TelegramModuleState> =
    std::sync::LazyLock::new(|| TelegramModuleState {
        api_key: "",
        bot_name: "",
    });

pub fn init(api_key: &String, bot_name: &String) {
    unsafe {
        TG_STATE.api_key = api_key.to_string().leak();
        TG_STATE.bot_name = bot_name.to_string().leak();
    }
}
