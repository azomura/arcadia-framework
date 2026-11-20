use std::sync::Arc;

use arcadia_framework::{Container, Injectable, ProviderToken, Scope};

#[derive(Debug, PartialEq)]
struct Service(u32);

impl Injectable for Service {
    fn create(_: &Container) -> Self {
        Self(1)
    }
}

#[tokio::test]
async fn container_supports_type_factories_and_async_resolution() {
    let container = Container::new();
    assert!(!container.contains::<Service>());
    assert!(container.try_resolve::<Service>().is_none());

    container.register_type_factory(Scope::Singleton, |_: &Container| Service(2));
    assert!(container.contains::<Service>());
    assert_eq!(container.resolve::<Service>().0, 2);

    container.register_async_factory::<String, _, _>(|_: Container| async { "async".to_owned() });
    assert!(container.contains_async::<String>());
    let first = container.resolve_async::<String>().await;
    let second = container.resolve_async::<String>().await;
    assert_eq!(first.as_str(), "async");
    assert!(Arc::ptr_eq(&first, &second));
}

#[test]
fn container_supports_token_factories_overrides_and_exports() {
    let container = Container::new();
    let token = ProviderToken::<String>::new("message");

    container.register_factory(token.clone(), Scope::Singleton, |_: &Container| "first".to_owned());
    assert_eq!(token.name(), "message");
    assert_eq!(container.resolve_token(token.clone()).as_str(), "first");
    assert_eq!(container.try_resolve_token(token.clone()).unwrap().as_str(), "first");
    assert!(!container.is_token_exported(token.clone()));

    container.export_token(token.clone());
    assert!(container.is_token_exported(token.clone()));
    container.override_token(token.clone(), "second".to_owned());
    assert_eq!(container.resolve_token(token.clone()).as_str(), "second");

    container.override_token_factory(token.clone(), Scope::Singleton, |_: &Container| {
        "third".to_owned()
    });
    assert_eq!(container.resolve_token(token).as_str(), "third");
}

#[test]
fn token_factories_support_request_and_transient_scopes() {
    let container = Container::new();
    let request = ProviderToken::<u32>::new("request");
    let transient = ProviderToken::<u64>::new("transient");
    container.register_factory(request, Scope::Request, |_: &Container| 1);
    container.register_factory(transient, Scope::Transient, |_: &Container| 2);

    let scope = container.request_scope();
    assert!(Arc::ptr_eq(&scope.resolve_token(request), &scope.resolve_token(request)));
    assert!(!Arc::ptr_eq(&scope.resolve_token(transient), &scope.resolve_token(transient)));
}

#[test]
fn container_supports_insert_and_provider_exports() {
    let container = Container::new();
    container.insert(Service(7));
    assert_eq!(*container.resolve::<Service>(), Service(7));
    container.export::<Service>();
    assert!(container.is_exported::<Service>());
}
