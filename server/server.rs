/**
 * Copyright 2026 Son of Binary
 * The pawaPay_bot Project
 * The moneybrain microservice
 *
 * This module is responsible for managing the HTTP server
 */
use axum::{Router, response::IntoResponse};
use std::{env, str::FromStr};

use crate::server::routers::webhook::*;

pub async fn start_server() {
    let port = match env::var("HTTP_PORT") {
        Result::Ok(v) => v,
        Result::Err(_) => env::var("PORT").unwrap(),
    };

    println!("Will be binding to port {port}\n");

    let router: Router = axum::Router::new();

    let router =
        router
            .route(
                "/example/{id}",
                axum::routing::get(
                    |axum::extract::Path(id): axum::extract::Path<String>| async move {
                        return format!("Hey there {}", id);
                    },
                ),
            )
            .nest("/webhook", webhook_router())
            .nest("/api", crate::server::routers::api::api_router())
            .layer(axum::middleware::from_fn(
                |req: axum::http::request::Request<axum::body::Body>,
                 next: axum::middleware::Next| async {
                    let method = req.method();

                    if method == axum::http::Method::from_str("OPTIONS").unwrap() {
                        return (axum::http::StatusCode::OK, "Thank you!").into_response();
                    }
                    next.run(req).await
                },
            ))
            .layer(axum::middleware::from_fn(
                |req: axum::http::Request<axum::body::Body>, next: axum::middleware::Next| async {
                    //

                    let mut resp = next.run(req).await;

                    let headers = resp.headers_mut();

                    headers.append(
                        "Access-Control-Allow-Origin",
                        axum::http::HeaderValue::from_str("*").unwrap(),
                    );
                    headers.append(
                        "Access-Control-Allow-Headers",
                        axum::http::HeaderValue::from_str("*").unwrap(),
                    );

                    resp
                },
            ))
            // Every other request must go through the panic_handler_middleware
            .layer(axum::middleware::from_fn(
                crate::server::error::panic_handler_middleware,
            ));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();

    println!("The server's port is {}\n", port);

    axum::serve(listener, router).await.unwrap();
}
