/**
 * Copyright 2026 Son of Binary
 * The pawaPay_bot Project
 * The moneybrain microservice
 *
 * This module contains the logic for working with Telegram webhooks
 */
use crate::{
    Amount,
    telegram::{TG_STATE, api},
};

/// This method sets the callback URL of the bot, the value of [url]
pub async fn set_webhook_url(url: String) -> Result<(), String> {
    let response = api::send_request(
        "/setWebHook",
        Option::Some(
            serde_json::json!({
                "url": url
            })
            .to_string(),
        ),
    )
    .await;

    println!("Setting telegram webhook to {}\n", url);

    let resp = match response {
        Ok(value) => value,
        Err(err) => return Result::Err(err),
    };

    println!("Telegram said\n{}", resp);

    return Result::Ok(());
}

/// This function parses an incoming Telegram webhook callback, and takes appropriate action.
pub async fn parse_webhook(json: serde_json::Value) {
    macro_rules! invalid_data {
        () => {
            eprintln!("Invalid webhook callback from Telegram.");
        };
    }

    fn parse_query_txt(query_txt: String) -> Option<Amount> {
        let money_pattern =
            regex::Regex::new(r"(?i)([0-9]+(?:\.[0-9]){0,1}[0-9]*) ([a-z]{3}(?:[^a-z]|$)){0,1}")
                .unwrap();

        if let Option::Some(results) = money_pattern.captures(query_txt.as_str()) {
            //
            let amount = &results[1];

            let mut currency_code = match results.get(2) {
                Option::Some(code) => code.as_str(),
                Option::None => "XAF", // The default currency
            };

            // TODO: Remove this
            if 1 < 2 {
                currency_code = "XAF"
            };

            println!(
                "The user wants to send an amount of {}, in {} ",
                amount, currency_code
            );

            return Option::Some(Amount {
                value: match amount.parse::<i32>() {
                    Result::Ok(v) => v,
                    Result::Err(_) => return Option::None,
                },
                currency_code: currency_code.to_string(),
            });
        }
        Option::None
    }
    /// This function handles only inline query webhooks
    async fn handle_inline_query(inline_query_data: serde_json::Value) {
        let query_id = match inline_query_data.get("id") {
            Option::Some(v) => match v.as_str() {
                Option::Some(v) => v,
                Option::None => {
                    invalid_data!();
                    return;
                }
            },
            Option::None => {
                invalid_data!();
                return;
            }
        };

        let query_txt = match inline_query_data.get("query") {
            Some(v) => v,
            None => {
                invalid_data!();
                return;
            }
        };

        let query_txt = match query_txt.as_str() {
            Option::Some(value) => value,
            Option::None => {
                invalid_data!();
                return;
            }
        };

        if let Option::Some(Amount {
            currency_code,
            value,
        }) = parse_query_txt(query_txt.to_string())
        {
            println!(
                "The user wants to send an amount of {}, in {} ",
                value, currency_code
            );

            let user_name = "some user name";

            // This inline query response just gives the client the starting message which he can send to his correspondent
            let response = api::send_request(
                "/answerInlineQuery",
                Option::Some(serde_json::json!({
                    "inline_query_id": query_id,
                    "results": [
                        {
                            "type": "article",
                            "id": "1",
                            "title": format!("Sending {} {}", value, currency_code),
                            "description": "Tap here to send the money.",
                            "input_message_content": {
                                "message_text": format!("{} sent you money\n{}\n\nOnce {} completes payment, you'll get it.", user_name, query_txt, user_name),
                            },
                            "cache_time": 0,
                            "reply_markup": {
                                "inline_keyboard": [
                                    [
                                      {
                                            "text": "Pay",
                                            "callback_data": "nothing",
                                        }
                                    ],
                                ],
                            },
                            "is_personal": true,
                        }
                    ]

                }).to_string()),
            ).await;
            match response {
                Result::Ok(v) => v,
                Result::Err(e) => {
                    println!("Could not respond to Telegram user query\n{}\n", e);
                    return;
                }
            };
        }
    }

    async fn handle_query_select(value: serde_json::Value) {
        macro_rules! get_string_field {
            ($field: expr, $from: expr) => {
                match $from.get($field) {
                    Option::None => {
                        invalid_data!();
                        return;
                    }
                    Option::Some(id) => match id.as_str() {
                        Option::None => {
                            invalid_data!();
                            return;
                        }
                        Option::Some(id) => id,
                    },
                }
            };
        }
        let inline_message_id = get_string_field!("inline_message_id", value);

        let bot_name = unsafe { TG_STATE.bot_name };

        println!("The full value is {value}\n");

        let amount = match parse_query_txt(get_string_field!("query", value).to_string()) {
            Option::None => return,
            Option::Some(v) => v,
        };

        let amount_value = amount.value;
        let amount_currency_code = &(&amount).currency_code;

        println!("The amount parsed is \n{amount:?}\n");

        let btn_url = format!(
            "t.me/{bot_name}/webui/fund?startapp={amount_value}_{amount_currency_code}_{inline_message_id}",
        );

        // Now, just edit the message, update the payment button.
        api::send_request(
            "/editMessageReplyMarkup",
            Option::Some(
                serde_json::json!({
                    "inline_message_id": inline_message_id,
                    "reply_markup": {
                        "inline_keyboard": [
                            [
                                {
                                    "text": "Make Payment",
                                    "url": btn_url,
                                }
                            ]
                        ]
                    }
                })
                .to_string(),
            ),
        )
        .await
        .map_err(|e| {
            format!("Could not edit Telegram message after inline query result selection\n{e}\n")
        })
        .unwrap();
    }

    if let Option::Some(value) = json.get("inline_query") {
        handle_inline_query(value.clone()).await;
    }

    if let Option::Some(value) = json.get("chosen_inline_result") {
        handle_query_select(value.clone()).await;
    }
}
