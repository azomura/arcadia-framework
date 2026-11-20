use std::sync::{Arc, Mutex};

use arcadia_framework::http::{
    ExceptionFilter, Guard, HttpException, HttpPipeline, Interceptor, Middleware, RequestPipe,
};
use axum::{
    Router,
    body::{Body, to_bytes},
    http::Request,
    response::IntoResponse,
    routing::get,
};
use tower::ServiceExt;

#[derive(Clone)]
struct Trace(Arc<Mutex<Vec<&'static str>>>);

impl Middleware for Trace {
    fn before(&self, _: &mut Request<Body>) -> Result<(), arcadia_framework::HttpException> {
        self.0.lock().unwrap().push("middleware");
        Ok(())
    }

    fn after(
        &self, _: &mut axum::response::Response,
    ) -> Result<(), arcadia_framework::HttpException> {
        self.0.lock().unwrap().push("middleware_after");
        Ok(())
    }
}

impl Guard for Trace {
    fn can_activate(&self, _: &Request<Body>) -> Result<(), arcadia_framework::HttpException> {
        self.0.lock().unwrap().push("guard");
        Ok(())
    }
}

impl Interceptor for Trace {
    fn before(&self, _: &Request<Body>) -> Result<(), arcadia_framework::HttpException> {
        self.0.lock().unwrap().push("interceptor");
        Ok(())
    }

    fn after(
        &self, _: &mut axum::response::Response,
    ) -> Result<(), arcadia_framework::HttpException> {
        self.0.lock().unwrap().push("interceptor_after");
        Ok(())
    }
}

impl RequestPipe for Trace {
    fn transform_request(
        &self, _: &mut Request<Body>,
    ) -> Result<(), arcadia_framework::HttpException> {
        self.0.lock().unwrap().push("pipe");
        Ok(())
    }
}

#[tokio::test]
async fn pipeline_matches_nest_request_order() {
    let trace = Arc::new(Mutex::new(Vec::new()));
    let pipeline = HttpPipeline::new()
        .middleware(Trace(Arc::clone(&trace)))
        .guard(Trace(Arc::clone(&trace)))
        .interceptor(Trace(Arc::clone(&trace)))
        .pipe(Trace(Arc::clone(&trace)));
    let router = pipeline.apply(Router::new().route("/", get(|| async { "ok" })));

    let response =
        router.oneshot(Request::builder().uri("/").body(Body::empty()).unwrap()).await.unwrap();

    assert_eq!(response.status(), 200);
    assert_eq!(
        *trace.lock().unwrap(),
        ["middleware", "guard", "interceptor", "pipe", "interceptor_after", "middleware_after"]
    );
}

struct RejectGuard;

impl Guard for RejectGuard {
    fn can_activate(&self, _: &Request<Body>) -> Result<(), HttpException> {
        Err(HttpException::new(axum::http::StatusCode::IM_A_TEAPOT, "teapot", "rejected"))
    }
}

struct CustomFilter;

impl ExceptionFilter for CustomFilter {
    fn catch(&self, exception: HttpException) -> axum::response::Response {
        (exception.status(), "filtered").into_response()
    }
}

#[tokio::test]
async fn pipeline_default_and_custom_filter_handle_rejected_requests() {
    let _default = HttpPipeline::default();
    let router = HttpPipeline::new()
        .guard(RejectGuard)
        .filter(CustomFilter)
        .apply(Router::new().route("/", get(|| async { "ok" })));
    let response =
        router.oneshot(Request::builder().uri("/").body(Body::empty()).unwrap()).await.unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::IM_A_TEAPOT);
    assert_eq!(to_bytes(response.into_body(), usize::MAX).await.unwrap(), "filtered");
}
