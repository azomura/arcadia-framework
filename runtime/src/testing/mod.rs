use std::sync::Arc;

use axum::Router;

use crate::{Container, Module, ModuleContext, ProviderToken, Scope};

type Override = Box<dyn FnOnce(&Container) + Send>;

pub struct TestingModule<M> {
    module: M,
    overrides: Vec<Override>,
}

impl<M: Module> TestingModule<M> {
    pub fn new(module: M) -> Self {
        Self { module, overrides: Vec::new() }
    }

    pub fn override_provider<T: Send + Sync + 'static>(mut self, value: T) -> Self {
        self.overrides.push(Box::new(move |container| container.insert(value)));
        self
    }

    pub fn override_token<T: Send + Sync + 'static>(
        mut self, token: ProviderToken<T>, value: T,
    ) -> Self {
        self.overrides.push(Box::new(move |container| container.override_token(token, value)));
        self
    }

    pub fn override_provider_factory<T, F>(mut self, scope: Scope, factory: F) -> Self
    where
        T: Send + Sync + 'static,
        F: Fn(&Container) -> T + Send + Sync + 'static,
    {
        self.overrides.push(Box::new(move |container| {
            container.register_type_factory::<T, _>(scope, factory);
        }));
        self
    }

    pub fn override_token_factory<T, F>(
        mut self, token: ProviderToken<T>, scope: Scope, factory: F,
    ) -> Self
    where
        T: Send + Sync + 'static,
        F: Fn(&Container) -> T + Send + Sync + 'static,
    {
        self.overrides.push(Box::new(move |container| {
            container.override_token_factory(token, scope, factory);
        }));
        self
    }

    pub fn compile(self) -> CompiledModule {
        let container = Container::default();
        for apply in self.overrides {
            apply(&container);
        }
        let mut context = ModuleContext::with_container(container);
        self.module.register(&mut context);
        let (container, router) = context.finish_parts();
        container.initialize();
        CompiledModule { container, router }
    }
}

pub struct CompiledModule {
    container: Container,
    router: Router,
}

impl CompiledModule {
    pub fn resolve<T: Send + Sync + 'static>(&self) -> Arc<T> {
        self.container.resolve::<T>()
    }

    pub fn resolve_token<T: Send + Sync + 'static>(&self, token: ProviderToken<T>) -> Arc<T> {
        self.container.resolve_token(token)
    }

    pub fn router(&self) -> Router {
        self.router.clone()
    }

    pub fn request_context(&self) -> Container {
        self.container.request_scope()
    }

    pub fn close(self) {
        self.container.shutdown();
    }
}
