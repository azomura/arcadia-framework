use std::sync::Arc;

use arcadia_framework::{
    Container, Controller, DynamicModule, GlobalModule, Injectable, Lifecycle, Module,
    ModuleContext, ProviderToken, Scope, forward_ref,
};
use axum::{Router, body::Body, http::Request, routing::get};
use tower::ServiceExt;

#[derive(Debug, PartialEq)]
struct Service(u32);

impl Injectable for Service {
    fn create(_: &Container) -> Self {
        Self(1)
    }
}

#[derive(Clone)]
struct SimpleController;

impl Injectable for SimpleController {
    fn create(_: &Container) -> Self {
        Self
    }
}

#[derive(Clone)]
struct InstanceController;

impl Controller for InstanceController {
    fn routes(self) -> Router {
        Router::new().route("/instance", get(|| async { "instance" }))
    }

    fn openapi() -> utoipa::openapi::OpenApi {
        utoipa::openapi::OpenApi::default()
    }
}

impl Controller for SimpleController {
    fn routes(self) -> Router {
        Router::new().route("/controller", get(|| async { "controller" }))
    }

    fn openapi() -> utoipa::openapi::OpenApi {
        utoipa::openapi::OpenApi::default()
    }
}

#[derive(Default)]
struct Hooks;

impl Injectable for Hooks {
    fn create(_: &Container) -> Self {
        Self
    }
}

impl Lifecycle for Hooks {}

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

#[tokio::test]
async fn module_context_registers_values_factories_and_routes() {
    let mut context = ModuleContext::new();
    let token = ProviderToken::<String>::new("message");
    context.provide_scoped::<Service>(Scope::Transient);
    context.value(token.clone(), String::from("value"));
    context.factory(token, Scope::Singleton, |_| String::from("factory"));
    context.async_factory::<u64, _, _>(|_: Container| async { 64 });
    context.provide::<Hooks>();
    context.lifecycle::<Hooks>();
    context.controller::<SimpleController>();
    context.controller_instance(InstanceController);
    context.merge(Router::new().route("/merged", get(|| async { "merged" })));

    assert!(context.container().contains::<Service>());
    assert!(context.module_ref().contains::<Service>());
    assert!(context.discovery().find(std::any::type_name::<Service>()).is_some());
    assert_eq!(*context.module_ref().resolve_async::<u64>().await, 64);

    let router = context.finish();
    for path in ["/controller", "/instance", "/merged"] {
        let response = router
            .clone()
            .oneshot(Request::builder().uri(path).body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), 200);
    }
}

#[test]
fn module_context_imports_modules_and_global_modules() {
    let mut context = ModuleContext::new();
    let token = ProviderToken::<String>::new("global");
    context.value(token.clone(), String::from("global"));
    context.import(FeatureModule);
    context.import_global(GlobalModule(FeatureModule));
    context.forward_ref(|| FeatureModule);
    context.export_token(token);
}

#[test]
fn dynamic_and_forward_modules_register_their_callbacks() {
    let dynamic = DynamicModule::new(|context: &mut ModuleContext| {
        context.value(ProviderToken::<u32>::new("dynamic"), 7);
    });
    let (container, _) = dynamic.into_parts();
    assert_eq!(*container.resolve_token(ProviderToken::<u32>::new("dynamic")), 7);

    fn create_feature() -> FeatureModule {
        FeatureModule
    }
    let module = forward_ref(create_feature as fn() -> FeatureModule);
    assert!(
        <arcadia_framework::ForwardRef<fn() -> FeatureModule> as Module>::openapi()
            .paths
            .paths
            .is_empty()
    );
    let (container, _) = module.into_parts();
    assert!(container.contains::<Service>());
}

#[test]
fn module_context_supports_default_and_exported_handles() {
    let mut context = ModuleContext::default();
    context.provide::<Service>();
    context.export::<Service>();
    let container = context.container().clone();
    assert!(container.contains::<Service>());
    assert!(Arc::ptr_eq(&container.resolve::<Service>(), &container.resolve::<Service>()));
}
