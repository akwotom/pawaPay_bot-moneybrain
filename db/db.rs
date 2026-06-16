/*
    Copyright 2026 Son of Binary
    The pawaPay_bot Project
    The moneybrain microservice

    This module contains the logic for reading and writing to the database.
*/

pub async fn get_connection() -> mongodb::Database {
    let uri_env_name = "MONGO_DB_URL";

    let uri = std::env::var(uri_env_name)
        .map_err(|e| {
            format!(
                "Could not connect to the database, because the {uri_env_name} was not set.\n{e}\n"
            )
        })
        .unwrap();

    mongodb::Client::with_uri_str(uri)
        .await
        .map_err(|e| {
            format!("Could not connect to the database, because of the following error:\n{e}\n")
        })
        .unwrap()
        .database("moneybrain")
}
