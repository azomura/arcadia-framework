use axum::{Extension, Router, middleware};

use crate::{
    Container, Controller, GlobalModule, Injectable, Lifecycle, Module, ProviderToken, Scope,
};

pub struct ModuleContext {
    container: Container,
    router: Router,
}

impl ModuleContext {
    pub fn new() -> Self {
        Self { container: Container::default(), router: Router::new() }
    }

    pub(crate) fn with_container(container: Container) -> Self {
        Self { container, router: Router::new() }
    }

    pub fn provide<T: Injectable + Send + Sync + 'static>(&mut self) {
        self.provide_scoped::<T>(Scope::Singleton);
    }

    pub fn provide_scoped<T: Injectable + Send + Sync + 'static>(&mut self, scope: Scope) {
        self.container.register::<T>(scope);
    }

    pub fn export<T: Send + Sync + 'static>(&mut self) {
        self.container.export::<T>();
    }

    pub fn value<T: Send + Sync + 'static>(&mut self, token: ProviderToken<T>, value: T) {
        self.container.insert_token(token, value);
    }

    pub fn factory<T, F>(&mut self, token: ProviderToken<T>, scope: Scope, factory: F)
    where
        T: Send + Sync + 'static,
        F: Fn(&Container) -> T + Send + Sync + 'static,
    {
        self.container.register_factory(token, scope, factory);
    }

    pub fn async_factory<T, F, Fut>(&mut self, factory: F)
    where
        T: Send + Sync + 'static,
        F: Fn(Container) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = T> + Send + 'static,
    {
        self.container.register_async_factory(factory);
    }

    pub fn export_token<T: Send + Sync + 'static>(&mut self, token: ProviderToken<T>) {
        self.container.export_token(token);
    }

    pub fn lifecycle<T: Lifecycle + 'static>(&mut self) {
        self.container.register_lifecycle::<T>();
    }

    pub fn container(&self) -> &Container {
        &self.container
    }

    pub fn controller<T: Controller + Injectable>(&mut self) {
        self.container.register::<T>(Scope::Request);
        self.router = std::mem::take(&mut self.router)
            .merge(T::create(&self.container).routes_with_container(self.container.clone()));
    }

    pub fn controller_instance<T: Controller>(&mut self, controller: T) {
        self.router = std::mem::take(&mut self.router).merge(controller.routes());
    }

    pub fn merge(&mut self, router: Router) {
        self.router = std::mem::take(&mut self.router).merge(router);
    }

    pub fn import(&mut self, module: impl crate::Module) {
        let mut child = Self { container: self.container.child(), router: Router::new() };
        module.register(&mut child);
        self.container.import_exports(&child.container);
        self.merge(child.router);
    }

    pub fn import_global(&mut self, module: GlobalModule<impl Module>) {
        module.register(self);
    }

    pub fn finish(self) -> Router {
        self.router
            .layer(middleware::from_fn(crate::application::request::context))
            .layer(Extension(self.container))
    }

    pub(crate) fn finish_parts(self) -> (Container, Router) {
        let container = self.container.clone();
        (container, self.finish())
    }
}

impl Default for ModuleContext {
    fn default() -> Self {
        Self::new()
    }
}
