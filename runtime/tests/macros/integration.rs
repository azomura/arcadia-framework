use std::sync::Arc;

use arcadia_framework::{ApplicationFactory, Lazy, ProviderToken, RequestExtension};
use arcadia_macros::{controller, injectable, module};
use axum::{body::Body, http::Request};
use tower::ServiceExt;

const VALUE_TOKEN: ProviderToken<u32> = ProviderToken::new("value");
const FACTORY_TOKEN: ProviderToken<String> = ProviderToken::new("factory");
const REQUEST_TOKEN: ProviderToken<u64> = ProviderToken::new("request");
const TRANSIENT_TOKEN: ProviderToken<bool> = ProviderToken::new("transient");

#[derive(Clone)]
struct AppService;
struct OptionalService;
const OPTIONAL_TOKEN: ProviderToken<OptionalService> = ProviderToken::new("optional");

#[derive(Clone)]
struct CycleA {
    b: Lazy<CycleB>,
}

#[injectable]
impl CycleA {
    fn new(b: Lazy<CycleB>) -> Self {
        Self { b }
    }
}

#[derive(Clone)]
struct CycleB {
    a: Arc<CycleA>,
}

#[injectable]
impl CycleB {
    fn new(a: Arc<CycleA>) -> Self {
        Self { a }
    }
}

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
    async fn index(&self, RequestExtension(value): RequestExtension<&'static str>) -> &'static str {
        let _ = &self.service;
        let _ = &self.optional;
        let _ = &self.optional_token;
        value
    }
}

#[module(controllers(AppController), providers(AppService, CycleA, CycleB))]
struct AppModule {}

#[module(
    values(VALUE_TOKEN => 7),
    factories(FACTORY_TOKEN => |_: &arcadia_framework::Container| String::from("factory")),
    request_factories(REQUEST_TOKEN => |_: &arcadia_framework::Container| 8_u64),
    transient_factories(TRANSIENT_TOKEN => |_: &arcadia_framework::Container| true),
    token_exports(VALUE_TOKEN)
)]
struct ProviderModule {}

#[tokio::test]
async fn macros_build_module_controller_and_inject_provider() {
    let response = ApplicationFactory::create(AppModule {})
        .into_router()
        .layer(axum::Extension("ok"))
        .oneshot(Request::builder().uri("/app/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
}

#[test]
fn lazy_forward_ref_breaks_cyclic_provider_resolution() {
    let application = ApplicationFactory::create(AppModule {});
    let first = application.container().resolve::<CycleA>();
    let second = first.b.get();

    assert!(Arc::ptr_eq(&first, &second.a));
}

#[test]
fn module_macro_registers_values_factories_and_exports() {
    let application = ApplicationFactory::create(ProviderModule {});
    assert_eq!(*application.container().resolve_token(VALUE_TOKEN), 7);
    assert_eq!(application.container().resolve_token(FACTORY_TOKEN).as_str(), "factory");
    assert_eq!(*application.container().resolve_token(REQUEST_TOKEN), 8);
    assert!(*application.container().resolve_token(TRANSIENT_TOKEN));
}
