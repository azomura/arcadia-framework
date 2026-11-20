use arcadia_framework::{Container, DiscoveryService, Injectable, ModuleContext, Scope};

struct Service;

impl Injectable for Service {
    fn create(_: &Container) -> Self {
        Self
    }
}

#[test]
fn discovery_lists_registered_providers_and_tokens() {
    let mut context = ModuleContext::new();
    context.provide::<Service>();
    let token = arcadia_framework::ProviderToken::<String>::new("config");
    context.value(token, String::from("dev"));
    let discovery: DiscoveryService = context.discovery();
    let providers = discovery.providers();

    assert!(
        providers
            .iter()
            .any(|provider| provider.scope == Scope::Singleton && provider.token.is_none())
    );
    assert!(discovery.find("config").is_some());
}
