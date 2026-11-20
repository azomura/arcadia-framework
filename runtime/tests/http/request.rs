use arcadia_framework::application::request::{RequestExtension, context};
use arcadia_framework::{Container, Injected, InjectedToken, ProviderToken, RequestContainer};
use axum::{
    Extension, Router, body::Body, extract::FromRequestParts, http::Request, middleware,
    routing::get,
};
use tower::ServiceExt;

async fn extract_string(RequestExtension(value): RequestExtension<String>) -> String {
    value
}

async fn extract_container(RequestContainer(container): RequestContainer) -> String {
    container.resolve::<u32>().to_string()
}

async fn extract_injected(Injected(value): Injected<u32>) -> String {
    value.to_string()
}

#[tokio::test]
async fn request_extractors_read_extensions_and_container_values() {
    let container = Container::new();
    container.insert(42_u32);
    let token = ProviderToken::<String>::new("answer");
    container.insert_token(token.clone(), "forty-two".to_owned());

    let string_router =
        Router::new().route("/", get(extract_string)).layer(Extension("hello".to_owned()));
    let response = string_router
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), 200);

    let injected_router =
        Router::new().route("/", get(extract_injected)).layer(Extension(container.clone()));
    let response = injected_router
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), 200);

    let token_value = InjectedToken::<String, ()>::resolve(&container, token);
    assert_eq!(token_value.0.as_str(), "forty-two");
}

#[tokio::test]
async fn request_context_creates_request_container_scope() {
    let container = Container::new();
    container.insert(7_u32);
    let router = Router::new()
        .route("/", get(extract_container))
        .layer(middleware::from_fn(context))
        .layer(Extension(container));
    let response =
        router.oneshot(Request::builder().uri("/").body(Body::empty()).unwrap()).await.unwrap();

    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn request_extractors_reject_missing_extensions() {
    let request = Request::new(Body::empty());
    let (mut parts, _) = request.into_parts();
    let missing_string =
        <RequestExtension<String> as FromRequestParts<()>>::from_request_parts(&mut parts, &())
            .await;
    assert!(missing_string.is_err());

    let missing_container =
        <RequestContainer as FromRequestParts<()>>::from_request_parts(&mut parts, &()).await;
    assert!(missing_container.is_err());

    let missing_injected =
        <Injected<u32> as FromRequestParts<()>>::from_request_parts(&mut parts, &()).await;
    assert!(missing_injected.is_err());
}
