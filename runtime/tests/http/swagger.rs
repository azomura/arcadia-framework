use arcadia_framework::{ApplicationFactory, Module, ModuleContext};
use axum::{body::Body, http::Request};
use tower::ServiceExt;

struct RootModule;

impl Module for RootModule {
    fn register(self, _: &mut ModuleContext) {}

    fn openapi() -> utoipa::openapi::OpenApi {
        utoipa::openapi::OpenApi::default()
    }
}

#[tokio::test]
async fn swagger_is_opt_in() {
    let response = ApplicationFactory::create(RootModule)
        .into_router()
        .oneshot(Request::builder().uri("/docs/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), 404);

    let response = ApplicationFactory::create(RootModule)
        .swagger(RootModule::openapi())
        .into_router()
        .oneshot(Request::builder().uri("/docs/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
}
