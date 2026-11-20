use arcadia_framework::{Container, Injectable, Module, ModuleContext, TestingModule};

struct Service(String);

impl Injectable for Service {
    fn create(_: &Container) -> Self {
        Self(String::from("default"))
    }
}

struct TestModule;

impl Module for TestModule {
    fn register(self, context: &mut ModuleContext) {
        context.provide::<Service>();
    }

    fn openapi() -> utoipa::openapi::OpenApi {
        utoipa::openapi::OpenApi::default()
    }
}

#[test]
fn testing_module_supports_overrides_and_resolution() {
    let module =
        TestingModule::new(TestModule).override_provider(Service(String::from("mock"))).compile();

    assert_eq!(module.get::<Service>().0, "mock");
    assert!(module.module_ref().contains::<Service>());
}

#[tokio::test]
async fn testing_module_supports_async_compile() {
    let module = TestingModule::new(TestModule).compile_async().await;

    assert_eq!(module.resolve_async::<Service>().await.0, "default");
}
