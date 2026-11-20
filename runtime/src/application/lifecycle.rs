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

#[cfg(test)]
mod tests {
    use axum::response::IntoResponse;

    use super::{not_found, shutdown};

    #[tokio::test]
    async fn not_found_returns_json_status() {
        let response = not_found().await.into_response();
        assert_eq!(response.status(), axum::http::StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn shutdown_waits_for_a_signal() {
        tokio::select! {
            _ = shutdown() => panic!("shutdown signal arrived unexpectedly"),
            _ = tokio::task::yield_now() => {},
        }
    }
}
