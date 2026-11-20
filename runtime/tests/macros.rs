use std::sync::Arc;

use arcadia_framework::ApplicationFactory;
use arcadia_macros::{controller, injectable, module};
use axum::{body::Body, http::Request};
use tower::ServiceExt;

#[derive(Clone)]
struct AppService;
struct OptionalService;

#[injectable]
impl AppService {
    fn new() -> Self {
        Self
    }
}

#[derive(Clone)]
struct AppController {
    service: Arc<AppService>,
    optional: Option<Arc<OptionalService>>,
}

#[controller(path = "/app")]
#[injectable]
impl AppController {
    fn new(service: Arc<AppService>, optional: Option<Arc<OptionalService>>) -> Self {
        Self { service, optional }
    }

    #[get("/")]
    async fn index(&self) -> &'static str {
        let _ = &self.service;
        let _ = &self.optional;
        "ok"
    }
}

#[module(controllers(AppController), providers(AppService))]
struct AppModule {}

#[tokio::test]
async fn macros_build_module_controller_and_inject_provider() {
    let response = ApplicationFactory::create(AppModule {})
        .into_router()
        .oneshot(Request::builder().uri("/app/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
}
