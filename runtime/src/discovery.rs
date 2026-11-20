use crate::{Container, Scope};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderDescriptor {
    pub name: &'static str,
    pub scope: Scope,
    pub token: Option<&'static str>,
}

#[derive(Clone)]
pub struct DiscoveryService {
    container: Container,
}

impl DiscoveryService {
    pub(crate) fn new(container: Container) -> Self {
        Self { container }
    }

    pub fn providers(&self) -> Vec<ProviderDescriptor> {
        self.container.provider_descriptors()
    }

    pub fn find(&self, name: &str) -> Option<ProviderDescriptor> {
        self.providers().into_iter().find(|provider| provider.name == name)
    }
}
