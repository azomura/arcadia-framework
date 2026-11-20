use std::sync::Arc;

use arcadia_framework::{Container, Injectable, Module, ModuleContext, ProviderToken, Scope};

struct Singleton;

impl Injectable for Singleton {
    fn create(_: &Container) -> Self {
        Self
    }
}

struct RequestScoped;

impl Injectable for RequestScoped {
    fn create(_: &Container) -> Self {
        Self
    }
}

struct Transient;

impl Injectable for Transient {
    fn create(_: &Container) -> Self {
        Self
    }
}

#[test]
fn container_resolves_scoped_providers() {
    let container = Container::new();
    container.register::<Singleton>(Scope::Singleton);
    container.register::<RequestScoped>(Scope::Request);
    container.register::<Transient>(Scope::Transient);

    assert!(Arc::ptr_eq(&container.resolve::<Singleton>(), &container.resolve::<Singleton>()));

    let first_request = container.request_scope();
    let second_request = container.request_scope();
    assert!(Arc::ptr_eq(
        &first_request.resolve::<RequestScoped>(),
        &first_request.resolve::<RequestScoped>()
    ));
    assert!(!Arc::ptr_eq(
        &first_request.resolve::<RequestScoped>(),
        &second_request.resolve::<RequestScoped>()
    ));
    assert!(!Arc::ptr_eq(&container.resolve::<Transient>(), &container.resolve::<Transient>()));
}

#[test]
fn container_resolves_values_and_tokens() {
    let container = Container::new();
    let token = ProviderToken::<String>::new("greeting");
    container.insert(42_u32);
    container.insert_token(token.clone(), String::from("hello"));

    assert_eq!(*container.resolve::<u32>(), 42);
    assert_eq!(container.resolve_token(token).as_str(), "hello");
}

struct ExportedService;

impl Injectable for ExportedService {
    fn create(_: &Container) -> Self {
        Self
    }
}

struct FeatureModule;

impl Module for FeatureModule {
    fn register(self, context: &mut ModuleContext) {
        context.provide::<ExportedService>();
        context.export::<ExportedService>();
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

#[test]
fn imported_module_exports_are_available_to_root() {
    let (container, _) = RootModule.into_parts();

    assert!(container.contains::<ExportedService>());
    let _ = container.resolve::<ExportedService>();
}
