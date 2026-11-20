use arcadia_framework::http::{
    ExceptionFilter, ExecutionContext, Guard, HttpException, Interceptor, JsonExceptionFilter,
    Middleware, Pipe, RequestPipe, RouteMetadata, Validate, ValidationPipe,
};
use axum::{
    body::{Body, to_bytes},
    http::{Request, StatusCode},
    response::IntoResponse,
};

#[derive(Default)]
struct AllowGuard;

impl Guard for AllowGuard {
    fn can_activate(&self, request: &Request<Body>) -> Result<(), HttpException> {
        (request.method() == "GET").then_some(()).ok_or_else(|| HttpException::forbidden("method"))
    }
}

#[derive(Debug)]
struct ValidValue(bool);

impl Validate for ValidValue {
    fn validate(&self) -> Result<(), String> {
        self.0.then_some(()).ok_or_else(|| "invalid".to_owned())
    }
}

#[derive(Default)]
struct TestRequestPipe;

impl RequestPipe for TestRequestPipe {
    fn transform_request(&self, request: &mut Request<Body>) -> Result<(), HttpException> {
        request.headers_mut().insert("x-piped", "true".parse().unwrap());
        Ok(())
    }
}

#[test]
fn request_pipe_applies_transformations() {
    let mut request = Request::new(Body::empty());
    TestRequestPipe.transform_request(&mut request).unwrap();
    assert_eq!(request.headers()["x-piped"], "true");
}

#[test]
fn guard_interceptor_and_middleware_default_contracts_work() {
    let request = Request::new(Body::empty());
    AllowGuard.can_activate(&request).unwrap();
    let metadata = RouteMetadata::new("Controller", "handler");
    let context = ExecutionContext::for_test(&request, &metadata);
    AllowGuard.can_activate_context(&context).unwrap();

    struct Empty;
    impl Interceptor for Empty {}
    impl Middleware for Empty {}
    let mut request = Request::new(Body::empty());
    let mut response = StatusCode::OK.into_response();
    Interceptor::before(&Empty, &request).unwrap();
    Empty.before_context(&context).unwrap();
    Interceptor::after(&Empty, &mut response).unwrap();
    Middleware::before(&Empty, &mut request).unwrap();
    Middleware::after(&Empty, &mut response).unwrap();
}

#[test]
fn validation_pipe_and_json_filter_return_expected_results() {
    assert!(ValidationPipe::<ValidValue>::default().transform(ValidValue(true)).is_ok());
    assert_eq!(
        ValidationPipe::<ValidValue>::default().transform(ValidValue(false)).unwrap_err().status(),
        StatusCode::BAD_REQUEST
    );

    let response = JsonExceptionFilter.catch(HttpException::bad_request("bad"));
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn json_exception_filter_preserves_json_body() {
    let response = JsonExceptionFilter.catch(HttpException::forbidden("denied"));
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();

    assert_eq!(body, r#"{"error":"forbidden","message":"denied"}"#);
}
