/*
    Copyright 2026 Son of Binary
    The pawaPay_bot Project
    The moneybrain microservice

    This module contains the logic for reading and writing from the collect transactions database
*/

use crate::azaman::Transaction;

async fn collection() -> mongodb::Collection<Transaction> {
    crate::db::get_connection().await.collection("funds")
}

/// This function adds a new item in the funds database
pub(crate) async fn insert(item: &mut Transaction) {
    item.id = format!("{}", mongodb::bson::uuid::Uuid::new().to_string(),);

    collection()
        .await
        .insert_one(item)
        .await
        .expect("Could not add item to funds db.");
}

pub(crate) async fn update(item: &Transaction) {
    let Transaction { id, .. } = item;

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

/// This method finds a transaction by id
pub(crate) async fn find_by_id(id: String) -> Option<Transaction> {
    collection()
        .await
        .find_one(mongodb::bson::doc! {
            "id": id,
        })
        .await
        .unwrap()
}

/// This method finds a transaction by id
pub async fn find_by_service_id(id: &String) -> Option<Transaction> {
    collection()
        .await
        .find_one(mongodb::bson::doc! {
            "service_callback.id": id,
        })
        .sort(mongodb::bson::doc! {
            "_id": -1,
        })
        .await
        .unwrap()
}
