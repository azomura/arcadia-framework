use std::{net::SocketAddr, time::Instant};

use axum::{
    extract::{ConnectInfo, Request},
    middleware::Next,
    response::Response,
};

pub(crate) async fn log_request(request: Request, next: Next) -> Response {
    let started = Instant::now();
    let method = request.method().clone();
    let path = request.uri().path().to_owned();

    let client = request
        .extensions()
        .get::<ConnectInfo<SocketAddr>>()
        .map(|peer| peer.0.to_string())
        .unwrap_or_else(|| "unknown".to_owned());

    let response = next.run(request).await;

    let status = response.status();
    let duration_ms = started.elapsed().as_secs_f64() * 1000.0;
    let message = format!("{method} {path} {} {duration_ms:.2} ms — {client}", status.as_u16());

    match status.as_u16() {
        500..=599 => tracing::error!("{message}"),
        400..=499 => tracing::warn!("{message}"),
        _ => tracing::info!(log_kind = "SUCCESS", "{message}"),
    }
    response
}
