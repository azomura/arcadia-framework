use arcadia_framework::config::{ConfigError, ConfigSection};
use arcadia_framework::http::{Guard, Guarded, HttpException};
use arcadia_framework::{Lifecycle, Module, ModuleContext};
use axum::{
    body::Body,
    extract::FromRequestParts,
    http::{Request, StatusCode},
};

#[derive(Default)]
struct NoopLifecycle;

impl Lifecycle for NoopLifecycle {}

struct EmptyModule;

impl Module for EmptyModule {
    fn register(self, _: &mut ModuleContext) {}

    fn openapi() -> utoipa::openapi::OpenApi {
        utoipa::openapi::OpenApi::default()
    }
}

#[derive(Default)]
struct AllowGuard;

impl Guard for AllowGuard {
    fn can_activate(&self, _: &Request<Body>) -> Result<(), HttpException> {
        Ok(())
    }
}

#[derive(Default)]
struct RejectGuard;

impl Guard for RejectGuard {
    fn can_activate(&self, _: &Request<Body>) -> Result<(), HttpException> {
        Err(HttpException::forbidden("rejected"))
    }
}

struct Section;

impl ConfigSection for Section {
    type Source = ();

    const FIELDS: &'static [arcadia_framework::config::ConfigField] = &[];

    fn build(_: &Self::Source) -> Result<Self, ConfigError> {
        Ok(Self)
    }
}

#[tokio::test]
async fn default_lifecycle_methods_and_guarded_extractor_are_callable() {
    let lifecycle = NoopLifecycle;
    lifecycle.on_module_init();
    lifecycle.on_application_bootstrap();
    lifecycle.before_application_shutdown();
    lifecycle.on_module_destroy();
    lifecycle.on_application_shutdown();

    let request = Request::builder().uri("/").body(Body::empty()).unwrap();
    let (mut parts, _) = request.into_parts();
    let allowed =
        <Guarded<AllowGuard> as FromRequestParts<()>>::from_request_parts(&mut parts, &()).await;
    assert!(allowed.is_ok());

    let request = Request::builder().uri("/").body(Body::empty()).unwrap();
    let (mut parts, _) = request.into_parts();
    let rejected =
        <Guarded<RejectGuard> as FromRequestParts<()>>::from_request_parts(&mut parts, &()).await;
    assert_eq!(rejected.err().unwrap().status(), StatusCode::FORBIDDEN);
}

#[test]
fn default_module_and_config_section_contracts_work() {
    assert!(EmptyModule::exports().is_empty());
    let _router = EmptyModule.into_router();
    let (_container, _router) = EmptyModule.into_parts();

    assert!(Section::load(&()).is_ok());
    assert!(Section.validate().is_ok());
}
