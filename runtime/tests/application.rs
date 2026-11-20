use std::sync::Arc;

use arcadia_framework::{Container, Injectable, ProviderToken, Scope};

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
