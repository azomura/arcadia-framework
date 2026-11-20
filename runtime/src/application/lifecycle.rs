use axum::{Json, http::StatusCode};
use serde_json::{Value, json};

pub(super) async fn not_found() -> (StatusCode, Json<Value>) {
    (StatusCode::NOT_FOUND, Json(json!({ "error": "not_found" })))
}

pub(super) async fn shutdown() {
    match tokio::signal::ctrl_c().await {
        Ok(()) => tracing::info!("Shutdown requested"),
        Err(error) => tracing::error!(error = %error, "Failed to listen for Ctrl+C"),
    }
}
