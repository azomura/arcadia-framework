use arcadia_framework::{ApplicationFactory, Container, Injectable, Module, ModuleContext};

struct Service;

impl Injectable for Service {
    fn create(_: &Container) -> Self {
        Self
    }
}

struct FeatureModule;

impl Module for FeatureModule {
    fn register(self, context: &mut ModuleContext) {
        context.provide::<Service>();
        context.export::<Service>();
    }

    fn openapi() -> utoipa::openapi::OpenApi {
        utoipa::openapi::OpenApi::default()
    }
}

struct RootModule;

impl Module for RootModule {
    fn register(self, context: &mut ModuleContext) {
        context.forward_ref(|| FeatureModule);
    }

    fn openapi() -> utoipa::openapi::OpenApi {
        utoipa::openapi::OpenApi::default()
    }
}

#[test]
fn module_ref_resolves_forward_ref_module_provider() {
    let application = ApplicationFactory::create(RootModule);

    assert!(application.container().contains::<Service>());
    let _service = application.container().resolve::<Service>();
}

#[test]
fn module_ref_resolves_registered_provider() {
    let mut context = ModuleContext::new();
    context.provide::<Service>();
    let module_ref = context.module_ref();

    assert!(module_ref.try_resolve::<Service>().is_some());
}
