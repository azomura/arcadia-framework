use std::{
    any::{Any, TypeId},
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex, RwLock},
};

use super::{Injectable, Scope};

mod imports;
mod resolution;
mod tokens;

pub(super) use resolution::resolve_entry;
pub use tokens::ProviderToken;

pub(super) type Provider = Arc<dyn Any + Send + Sync>;
pub(super) type Factory = Arc<dyn Fn(&Container) -> Provider + Send + Sync>;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub(super) enum ProviderKey {
    Type(TypeId),
    Token(&'static str),
}

pub(super) struct Entry {
    pub(super) name: &'static str,
    pub(super) factory: Factory,
    pub(super) scope: Scope,
    pub(super) singleton: Mutex<Option<Provider>>,
}

#[derive(Default)]
pub(super) struct Registry {
    pub(super) entries: RwLock<HashMap<TypeId, Arc<Entry>>>,
    pub(super) tokens: RwLock<HashMap<&'static str, Arc<Entry>>>,
    pub(super) exports: RwLock<HashSet<ProviderKey>>,
    pub(super) hooks: RwLock<Vec<Arc<dyn super::Lifecycle>>>,
    pub(super) overrides: RwLock<HashSet<ProviderKey>>,
}

#[derive(Clone, Default)]
pub struct Container {
    pub(super) registry: Arc<Registry>,
    requests: Arc<Mutex<HashMap<ProviderKey, Provider>>>,
}

impl Container {
    pub fn contains<T: Send + Sync + 'static>(&self) -> bool {
        self.registry.entries.read().unwrap().contains_key(&TypeId::of::<T>())
    }

    pub fn register<T: Injectable + Send + Sync + 'static>(&self, scope: Scope) {
        self.registry.entries.write().unwrap().entry(TypeId::of::<T>()).or_insert_with(|| {
            let factory: Factory = Arc::new(|container| Arc::new(T::create(container)));
            Arc::new(Entry {
                name: std::any::type_name::<T>(),
                factory,
                scope,
                singleton: Mutex::new(None),
            })
        });
    }

    pub fn register_type_factory<T, F>(&self, scope: Scope, factory: F)
    where
        T: Send + Sync + 'static,
        F: Fn(&Container) -> T + Send + Sync + 'static,
    {
        let factory: Factory = Arc::new(move |container| Arc::new(factory(container)));
        let name = std::any::type_name::<T>();
        let entry = Entry { name, factory, scope, singleton: Mutex::new(None) };
        self.registry.entries.write().unwrap().insert(TypeId::of::<T>(), Arc::new(entry));
    }

    pub fn insert<T: Send + Sync + 'static>(&self, provider: T) {
        let value: Provider = Arc::new(provider);
        let entry = Entry {
            name: std::any::type_name::<T>(),
            factory: Arc::new(|_| unreachable!("inserted providers are initialized")),
            scope: Scope::Singleton,
            singleton: Mutex::new(Some(value)),
        };
        self.registry.entries.write().unwrap().insert(TypeId::of::<T>(), Arc::new(entry));
    }

    pub fn resolve<T: Send + Sync + 'static>(&self) -> Arc<T> {
        let id = TypeId::of::<T>();
        let entry =
            self.registry.entries.read().unwrap().get(&id).cloned().unwrap_or_else(|| {
                panic!("provider {} is not registered", std::any::type_name::<T>())
            });
        let value = resolve_entry(self, ProviderKey::Type(id), &entry);
        value.downcast::<T>().unwrap_or_else(|_| unreachable!("provider type changed"))
    }

    pub fn request_scope(&self) -> Self {
        Self { registry: self.registry.clone(), requests: Arc::default() }
    }
}
