use std::sync::Arc;

use super::{Container, ProviderToken};

#[derive(Clone)]
pub struct ModuleRef {
    container: Container,
}

impl ModuleRef {
    pub(crate) fn new(container: Container) -> Self {
        Self { container }
    }
    pub fn resolve<T: Send + Sync + 'static>(&self) -> Arc<T> {
        self.container.resolve::<T>()
    }
    pub fn try_resolve<T: Send + Sync + 'static>(&self) -> Option<Arc<T>> {
        self.container.try_resolve::<T>()
    }
    pub fn resolve_token<T: Send + Sync + 'static>(&self, token: ProviderToken<T>) -> Arc<T> {
        self.container.resolve_token(token)
    }
}
