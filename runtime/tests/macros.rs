use std::sync::Arc;

use arcadia_framework::{ApplicationFactory, ProviderToken};
use arcadia_macros::{controller, injectable, module};
use axum::{body::Body, http::Request};
use tower::ServiceExt;

#[derive(Clone)]
struct AppService;
struct OptionalService;
const OPTIONAL_TOKEN: ProviderToken<OptionalService> = ProviderToken::new("optional");

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
    optional_token: Option<Arc<OptionalService>>,
}

#[controller(path = "/app")]
#[injectable]
impl AppController {
    fn new(
        service: Arc<AppService>, optional: Option<Arc<OptionalService>>,
        #[inject(OPTIONAL_TOKEN)] optional_token: Option<Arc<OptionalService>>,
    ) -> Self {
        Self { service, optional, optional_token }
    }

    #[get("/")]
    async fn index(&self) -> &'static str {
        let _ = &self.service;
        let _ = &self.optional;
        let _ = &self.optional_token;
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
