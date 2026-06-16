use path_clean::PathClean;
/**
 * Copyright 2026 Son of Binary
 * The pawaPay_bot Project
 * The moneybrain microservice
 *
 * This module is concerned with the task of making API calls to Telegram.
 */
use reqwest;

const HTTP_CLIENT: std::sync::LazyLock<reqwest::Client> =
    std::sync::LazyLock::new(|| reqwest::Client::new());

/// This method sends a direct request to the server.
pub(crate) async fn send_request(
    path: &str,
    body: Option<String>,
) -> Result<serde_json::Value, String> {
    let api_token = std::env::var("TELEGRAM_API_KEY").unwrap();
    //

    let inner_path = &format!("/bot{}/{}", api_token, path);

    let inner_path = std::path::Path::new(inner_path);

    let inner_path = inner_path.clean();

    let inner_path = inner_path.to_str().unwrap();

    // let inner_path = inner_path.to_str().unwrap();

    let url = format!("https://api.telegram.org{}", inner_path);

    let mut req = HTTP_CLIENT.post(url.clone());

    if let Option::Some(body) = &body {
        req = req.header("Content-Type", "application/json");
        req = req.body(body.clone());
    }

    let response = match req.send().await {
        Ok(value) => value,
        Err(err) => return Result::Err(err.to_string()),
    };

    let status = response.status();

    let body_bytes = response.bytes().await;

    let content_str = String::from_utf8(body_bytes.unwrap().to_vec()).unwrap();

    let error_response = Result::Err(
        format!(
            "Unexpected error from server at {}\n\nStatus: {}\nContent:\n{}\nOUr payload:\n{:?}\n",
            url,
            status.as_u16(),
            content_str,
            body,
        )
        .to_string(),
    );

    if status.is_success() {
        Result::Ok(serde_json::from_str(content_str.as_str()).unwrap())
    } else {
        error_response
    }
}
