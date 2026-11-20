use std::{
    marker::PhantomData,
    sync::{Arc, Mutex},
};

use super::{Container, Entry, Factory, Provider, ProviderKey, Scope, resolve_entry};

#[derive(Clone, Copy)]
pub struct ProviderToken<T> {
    name: &'static str,
    marker: PhantomData<fn() -> T>,
}

impl<T> ProviderToken<T> {
    pub const fn new(name: &'static str) -> Self {
        Self { name, marker: PhantomData }
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }
}

impl Container {
    pub fn register_factory<T, F>(&self, token: ProviderToken<T>, scope: Scope, factory: F)
    where
        T: Send + Sync + 'static,
        F: Fn(&Container) -> T + Send + Sync + 'static,
    {
        let factory: Factory = Arc::new(move |container| Arc::new(factory(container)));
        let entry = Entry { name: token.name, factory, scope, singleton: Mutex::new(None) };
        self.registry.tokens.write().unwrap().entry(token.name).or_insert(Arc::new(entry));
    }

    pub fn insert_token<T: Send + Sync + 'static>(&self, token: ProviderToken<T>, value: T) {
        let provider: Provider = Arc::new(value);
        let entry = Entry {
            name: token.name,
            factory: Arc::new(|_| unreachable!("inserted providers are initialized")),
            scope: Scope::Singleton,
            singleton: Mutex::new(Some(provider)),
        };
        self.registry.tokens.write().unwrap().entry(token.name).or_insert(Arc::new(entry));
    }

    pub fn override_token<T: Send + Sync + 'static>(&self, token: ProviderToken<T>, value: T) {
        let name = token.name;
        self.registry.tokens.write().unwrap().remove(name);
        self.insert_token(token, value);
        self.registry.overrides.write().unwrap().insert(ProviderKey::Token(name));
    }

    pub fn override_token_factory<T, F>(&self, token: ProviderToken<T>, scope: Scope, factory: F)
    where
        T: Send + Sync + 'static,
        F: Fn(&Container) -> T + Send + Sync + 'static,
    {
        let name = token.name;
        self.registry.tokens.write().unwrap().remove(name);
        self.register_factory(token, scope, factory);
        self.registry.overrides.write().unwrap().insert(ProviderKey::Token(name));
    }

    pub fn resolve_token<T: Send + Sync + 'static>(&self, token: ProviderToken<T>) -> Arc<T> {
        let entry = self
            .registry
            .tokens
            .read()
            .unwrap()
            .get(token.name)
            .cloned()
            .unwrap_or_else(|| panic!("provider token {} is not registered", token.name));
        resolve_entry(self, ProviderKey::Token(token.name), &entry)
            .downcast::<T>()
            .unwrap_or_else(|_| unreachable!("provider token type changed"))
    }
}
