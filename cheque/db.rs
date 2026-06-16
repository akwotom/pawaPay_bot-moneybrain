/*
    Copyright 2026 Son of Binary
    The pawaPay_bot Project
    The moneybrain microservice

    This module contains the logic for reading and writing from the cheques database
*/

use crate::cheque::Cheque;

async fn collection() -> mongodb::Collection<Cheque> {
    crate::db::get_connection().await.collection("cheques")
}

/// This function adds a new item in the cheques database
pub(crate) async fn insert(item: &mut Cheque) {
    item.id = short_uuid::ShortUuid::generate().to_string();
    collection()
        .await
        .insert_one(item)
        .await
        .expect("Could not add item to cheques db.");
}

pub(crate) async fn update(item: &Cheque) {
    let Cheque { id, .. } = item;

    collection()
        .await
        .replace_one(
            mongodb::bson::doc! {
                "id": id,
            },
            item,
        )
        .upsert(true)
        .await
        .map(|_| ()) // We're not bothered about the update results.
        .unwrap() // But if there's an error, let the entire process just fail.
}

/// This function finds a cheque, by using the id of the message for which it was created
pub(crate) async fn find_by_tg_message_id(id: &String) -> Option<Cheque> {
    collection()
        .await
        .find_one(mongodb::bson::doc! {
            "telegram.inline_message_id": id
        })
        .await
        .unwrap()
}

/// This function finds a cheque, by using its id.
pub(crate) async fn find_by_id(id: &String) -> Option<Cheque> {
    collection()
        .await
        .find_one(mongodb::bson::doc! {
            "id": id
        })
        .await
        .unwrap()
}
