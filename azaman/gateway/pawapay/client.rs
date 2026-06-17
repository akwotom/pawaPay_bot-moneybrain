/*
    Copyright 2026 Son of Binary
    The pawaPay_bot Project
    The moneybrain microservice

    This module is concerned with the task of directly making requests on pawaPay's gateway
*/

use path_clean::*;
use std::str::FromStr;

const HTTP_CLIENT: std::sync::LazyLock<reqwest::Client> =
    std::sync::LazyLock::new(|| reqwest::Client::new());

/// This method makes an authenticated request to pawaPay
pub(crate) async fn make_request(
    method: &str,
    path: &str,
    body: Option<String>,
) -> Result<serde_json::Value, String> {
    //
    let api_token = std::env::var("PAWAPAY_API_KEY").unwrap();

    let api_url = std::env::var("PAWAPAY_API_URL").unwrap();

    let api_url = regex::Regex::new(r"(?i)/+$").unwrap().replace(api_url.as_str(), "");

    let inner_path = &format!("/{path}");

    let inner_path = std::path::Path::new(inner_path);

    let inner_path = inner_path.clean();

    let inner_path = inner_path.to_str().unwrap();

    let url = format!("{api_url}{inner_path}");

    let mut req = HTTP_CLIENT.request(reqwest::Method::from_str(method).unwrap(), url.clone());

    if let Option::Some(body) = &body {
        req = req.header("Content-Type", "application/json");
        req = req.body(body.clone());
    }

    req = req.header("Authorization", format!("Bearer {api_token}"));

    let response = match req.send().await {
        Ok(value) => value,
        Err(err) => return Result::Err(err.to_string()),
    };

    let status = response.status();

    let body_bytes = response.bytes().await;

    let content_str = String::from_utf8(body_bytes.unwrap().to_vec()).unwrap();

    let error_response = Result::Err(
        format!(
            "Unexpected error from server at {}\n\nStatus: {}\nContent:\n{}\nOur payload:\n{body:?}\n",
            url,
            status.as_u16(),
            content_str,
        )
        .to_string(),
    );

    if status.is_success() {
        println!(
            "pawaPay says:\n{}\nFor url {}\nbody\n{:?}",
            content_str.as_str(),
            url.clone(),
            body
        );

        Result::Ok(serde_json::from_str(content_str.as_str()).unwrap())
    } else {
        error_response
    }
}
