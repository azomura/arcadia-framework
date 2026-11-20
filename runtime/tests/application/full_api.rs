use std::sync::Arc;

use arcadia_framework::http::HttpPipeline;
use arcadia_framework::{
    ApplicationFactory, Container, Injectable, Module, ModuleContext, ProviderToken, Scope,
    TestingModule,
};

#[derive(Debug, PartialEq)]
struct Service(String);

impl Injectable for Service {
    fn create(_: &Container) -> Self {
        Self(String::from("created"))
    }
}

const MESSAGE: ProviderToken<String> = ProviderToken::new("message");

struct TestModule;

impl Module for TestModule {
    fn register(self, context: &mut ModuleContext) {
        context.provide::<Service>();
        context.value(MESSAGE, String::from("module"));
    }

    fn openapi() -> utoipa::openapi::OpenApi {
        utoipa::openapi::OpenApi::default()
    }
}

#[test]
fn lazy_resolves_provider_and_clones_container_handle() {
    let container = Container::new();
    container.insert(Service(String::from("lazy")));
    let lazy = container.lazy::<Service>();
    let clone = lazy.clone();

    assert_eq!(lazy.get().0, "lazy");
    assert!(Arc::ptr_eq(&lazy.get(), &clone.get()));
}

#[tokio::test]
async fn testing_module_exposes_all_compiled_module_operations() {
    let compiled = TestingModule::new(TestModule)
        .override_provider(Service(String::from("override")))
        .override_token(MESSAGE, String::from("token override"))
        .override_provider_factory::<u32, _>(Scope::Singleton, |_| 7)
        .override_token_factory(MESSAGE, Scope::Singleton, |_| String::from("factory"))
        .compile_async()
        .await;

    assert_eq!(compiled.get::<Service>().0, "override");
    assert_eq!(compiled.resolve::<Service>().0, "override");
    assert_eq!(*compiled.resolve_async::<Service>().await, Service(String::from("override")));
    assert_eq!(compiled.create::<Service>().0, "created");
    assert_eq!(compiled.resolve_token(MESSAGE).as_str(), "factory");
    assert!(compiled.module_ref().contains::<Service>());
    assert_eq!(*compiled.request_context().resolve::<Service>(), Service(String::from("override")));
    let _router = compiled.router();
    compiled.close();
}

#[tokio::test]
async fn application_factory_supports_sync_and_async_bootstrap() {
    let application = ApplicationFactory::create(TestModule);
    assert!(application.container().contains::<Service>());
    let application = ApplicationFactory::create_async(TestModule).await;
    assert!(application.container().contains::<Service>());
}

#[tokio::test]
async fn application_listen_attempts_to_bind_requested_address() {
    let application = ApplicationFactory::create(TestModule);
    tokio::select! {
        result = application.listen("127.0.0.1:0".parse().unwrap()) => {
            let _ = result;
        }
        _ = tokio::task::yield_now() => {}
    }
}

#[test]
fn application_configures_pipeline() {
    let router = ApplicationFactory::create(TestModule)
        .documentation(false)
        .pipeline(HttpPipeline::default())
        .into_router();
    let _ = router;
}
