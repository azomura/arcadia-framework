use arcadia_framework::http::{HttpException, Reflector, RouteMetadata};
use axum::{
    body::to_bytes,
    http::{Request, StatusCode},
    response::IntoResponse,
};

#[test]
fn route_metadata_stores_route_and_custom_values() {
    let metadata = RouteMetadata::new("UsersController", "list").with("version", "v1");

    assert_eq!(metadata.controller(), "UsersController");
    assert_eq!(metadata.handler(), "list");
    assert_eq!(Reflector::get(&metadata, "version"), Some("v1"));
    assert_eq!(Reflector::get(&metadata, "missing"), None);
}

#[tokio::test]
async fn route_metadata_extracts_from_request_extensions() {
    let metadata = RouteMetadata::new("UsersController", "list");
    let mut request = Request::new(());
    request.extensions_mut().insert(metadata.clone());
    let (mut parts, _) = request.into_parts();
    let extracted =
        <RouteMetadata as axum::extract::FromRequestParts<()>>::from_request_parts(&mut parts, &())
            .await
            .unwrap();

    assert_eq!(extracted.controller(), "UsersController");
    assert_eq!(extracted.handler(), "list");
}

#[tokio::test]
async fn http_exceptions_expose_status_and_json_response() {
    let exception = HttpException::bad_request("invalid payload");
    assert_eq!(exception.status(), StatusCode::BAD_REQUEST);
    assert_eq!(exception.code(), "bad_request");
    assert_eq!(exception.message(), "invalid payload");

    let response = HttpException::unauthorized("login required").into_response();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(response.headers()["content-type"], "application/json");
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    assert_eq!(body, r#"{"error":"unauthorized","message":"login required"}"#);

    assert_eq!(HttpException::forbidden("denied").status(), StatusCode::FORBIDDEN);
}
