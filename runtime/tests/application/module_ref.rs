use std::sync::atomic::{AtomicBool, Ordering};

use arcadia_framework::{ApplicationFactory, Container, Injectable, Module, ModuleContext};

static PRIVATE_VISIBLE: AtomicBool = AtomicBool::new(false);

struct Service;
struct PrivateService;

impl Injectable for Service {
    fn create(_: &Container) -> Self {
        Self
    }
}

impl Injectable for PrivateService {
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

struct PrivateFeature;

impl Module for PrivateFeature {
    fn register(self, context: &mut ModuleContext) {
        context.provide::<PrivateService>();
        PRIVATE_VISIBLE.store(context.module_ref().contains::<PrivateService>(), Ordering::SeqCst);
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

#[test]
fn module_ref_respects_module_provider_scope() {
    let mut context = ModuleContext::new();
    context.import(PrivateFeature);

    assert!(PRIVATE_VISIBLE.load(Ordering::SeqCst));
    assert!(!context.module_ref().contains::<PrivateService>());
}
