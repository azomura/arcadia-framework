use std::sync::Arc;

use super::{Container, Injectable, ProviderToken};

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
    pub fn get<T: Send + Sync + 'static>(&self) -> Arc<T> {
        self.resolve::<T>()
    }
    pub fn contains<T: Send + Sync + 'static>(&self) -> bool {
        self.container.contains::<T>() || self.container.contains_async::<T>()
    }
    pub fn try_resolve<T: Send + Sync + 'static>(&self) -> Option<Arc<T>> {
        self.container.try_resolve::<T>()
    }
    pub fn resolve_token<T: Send + Sync + 'static>(&self, token: ProviderToken<T>) -> Arc<T> {
        self.container.resolve_token(token)
    }
    pub async fn resolve_async<T: Send + Sync + 'static>(&self) -> Arc<T> {
        self.container.resolve_async::<T>().await
    }
    pub fn create<T: Injectable>(&self) -> T {
        T::create(&self.container)
    }
    pub fn request_scope(&self) -> Self {
        Self::new(self.container.request_scope())
    }
}
