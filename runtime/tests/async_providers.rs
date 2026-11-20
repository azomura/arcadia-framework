use arcadia_framework::{ApplicationFactory, Module, ModuleContext};

struct AsyncService(String);

struct FeatureModule;

impl Module for FeatureModule {
    fn register(self, context: &mut ModuleContext) {
        context
            .async_factory::<AsyncService, _, _>(|_| async { AsyncService(String::from("ready")) });
        context.export::<AsyncService>();
    }

    fn openapi() -> utoipa::openapi::OpenApi {
        utoipa::openapi::OpenApi::default()
    }
}

struct RootModule;

impl Module for RootModule {
    fn register(self, context: &mut ModuleContext) {
        context.import(FeatureModule);
    }

    fn openapi() -> utoipa::openapi::OpenApi {
        utoipa::openapi::OpenApi::default()
    }
}

#[tokio::test]
async fn imported_module_resolves_async_provider() {
    let application = ApplicationFactory::create(RootModule);
    let service = application.container().resolve_async::<AsyncService>().await;

    assert_eq!(service.0, "ready");
}
