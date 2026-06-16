/**
    Copyright 2026 Son of Binary
    The pawaPay_bot Project
    The moneybrain microservice

    This module is the entry point of the moneybraina service
*/
use dotenvy::dotenv;
use path_clean::PathClean;

use crate::server::server::start_server;
mod server;
mod structs;
pub use structs::*;
mod cheque;
mod db;
mod telegram;
mod azaman;

macro_rules! read_env {
    ($keyName:expr, $errMsg:expr) => {
        match std::env::var($keyName) {
            Ok(value) => value,
            Err(_) => {
                let err = format!("{} Check the {} environment variable.", $errMsg, $keyName);
                panic!("{}", err)
            }
        }
    };
}

fn main() {
    dotenv().ok();

    println!("Hey, it works");

    let rtime = tokio::runtime::Runtime::new().unwrap();

    let own_server_url_var_name = "THIS_SERVER_URL";

    let own_server_url = match std::env::var(own_server_url_var_name) {
        Ok(value) => value,
        Err(_) => {
            let err = format!(
                "Error: No API key found for Telegram. Check the {} environment variable.",
                own_server_url_var_name
            );
            panic!("{}", err)
        }
    };
    let mut final_cb_url =
        url::Url::parse(&format!("{}/webhook/telegram", own_server_url)).unwrap();
    let normalized_path = std::path::Path::new(final_cb_url.clone().path()).clean();
    let normalized_path = normalized_path.to_str().unwrap();

    final_cb_url.set_path(normalized_path);

    let final_cb_url = final_cb_url.to_string();

    telegram::init(
        &read_env!("TELEGRAM_API_KEY", "Error: No API key found for Telegram."),
        &read_env!(
            "TELEGRAM_BOT_USER_NAME",
            "The user name for the Telegram bot is not set"
        ),
    );

    let _ = match rtime.block_on(telegram::webhook::set_webhook_url(final_cb_url)) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("\nCould not set Telegram webhook\n{}\n", e)
        }
    };

    rtime.block_on(async { start_server().await });

    println!("GoooD!");
}
