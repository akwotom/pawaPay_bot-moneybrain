/*
    Copyright 2026 Son of Binary
    The pawaPay_bot Project
    The moneybrain microservice

    This module contains features concerning management of cheques, thatare done on Telegram.
    For example, updating the original message, once a user has paid, or liquidated.
*/

use crate::{
    cheque::Cheque,
    telegram::{self, TG_STATE},
};

impl Cheque {
    pub fn on_status_change(&self) {
        // If liquidated, let's update the original message
        let inline_message_id = (&self.telegram).inline_message_id.clone();

        let liquidated = self.liquidated;
        let funded = self.funded;

        tokio::spawn(async move {
            if liquidated {
                // Update message on Telegram
                match telegram::api::send_request(
                    "/editMessageReplyMarkup",
                    Option::Some(
                        serde_json::json!({
                            "inline_message_id": inline_message_id,
                            "reply_markup": {
                                "inline_keyboard": []
                            }
                        })
                        .to_string(),
                    ),
                )
                .await
                {
                    Result::Err(e) => {
                        println!("Could not update Telegram message\n{e}\n");
                        return;
                    }
                    Result::Ok(_) => {
                        return;
                    }
                };
            } else {
                if funded {
                    // Update the message to give the user the possibility of liquidating.
                    let bot_name = unsafe { TG_STATE.bot_name };
                    let url = format!("t.me/{}/webui?", bot_name);

                    match telegram::api::send_request(
                        "/editMessageReplyMarkup",
                        Option::Some(
                            serde_json::json!({
                                "inline_message_id": inline_message_id,
                                "reply_markup": {
                                    "inline_keyboard": [
                                        [
                                            {
                                                "text": "Collect Funds",
                                                "url": url,
                                            }
                                        ]
                                    ]
                                }
                            })
                            .to_string(),
                        ),
                    )
                    .await
                    {
                        Result::Err(e) => {
                            println!("Could not update Telegram message\n{e}\n");
                            return;
                        }
                        Result::Ok(_) => {
                            return;
                        }
                    };
                }
            }
        });
    }
}
