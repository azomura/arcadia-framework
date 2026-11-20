use std::any::TypeId;

use super::Container;

impl Container {
    pub fn export<T: Send + Sync + 'static>(&self) {
        assert!(
            self.contains::<T>() || self.contains_async::<T>(),
            "cannot export unregistered provider {}",
            std::any::type_name::<T>()
        );
        self.registry.exports.write().unwrap().insert(super::ProviderKey::Type(TypeId::of::<T>()));
    }

    pub fn is_exported<T: Send + Sync + 'static>(&self) -> bool {
        self.registry.exports.read().unwrap().contains(&super::ProviderKey::Type(TypeId::of::<T>()))
    }

    pub fn export_token<T: Send + Sync + 'static>(&self, token: super::ProviderToken<T>) {
        assert!(self.registry.tokens.read().unwrap().contains_key(token.name()));
        self.registry.exports.write().unwrap().insert(super::ProviderKey::Token(token.name()));
    }

    pub fn is_token_exported<T>(&self, token: super::ProviderToken<T>) -> bool {
        self.registry.exports.read().unwrap().contains(&super::ProviderKey::Token(token.name()))
    }

    pub(crate) fn child(&self) -> Self {
        let child = Self::default();
        let exports = self.registry.exports.read().unwrap();
        let entries = self.registry.entries.read().unwrap();
        let async_factories = self.registry.async_factories.read().unwrap();
        let mut target = child.registry.entries.write().unwrap();
        let tokens = self.registry.tokens.read().unwrap();
        let mut target_tokens = child.registry.tokens.write().unwrap();
        let overrides = self.registry.overrides.read().unwrap();
        for key in exports.iter() {
            match key {
                super::ProviderKey::Type(id) if entries.contains_key(id) => {
                    target.insert(*id, entries[id].clone());
                }
                super::ProviderKey::Type(id) if async_factories.contains_key(id) => {
                    child
                        .registry
                        .async_factories
                        .write()
                        .unwrap()
                        .insert(*id, async_factories[id].clone());
                }
                super::ProviderKey::Token(name) if tokens.contains_key(name) => {
                    target_tokens.insert(name, tokens[name].clone());
                }
                _ => {}
            }
        }
        for key in overrides.iter() {
            if let super::ProviderKey::Token(name) = key {
                target_tokens.insert(name, tokens[name].clone());
            }
        }
        *child.registry.overrides.write().unwrap() = overrides.clone();
        drop(target_tokens);
        drop(target);
        child
    }

    pub(crate) fn import_exports(&self, child: &Self) {
        let exports = child.registry.exports.read().unwrap();
        let entries = child.registry.entries.read().unwrap();
        let async_factories = child.registry.async_factories.read().unwrap();
        let mut target = self.registry.entries.write().unwrap();
        let tokens = child.registry.tokens.read().unwrap();
        let mut target_tokens = self.registry.tokens.write().unwrap();
        for key in exports.iter() {
            match key {
                super::ProviderKey::Type(id) if entries.contains_key(id) => {
                    target.insert(*id, entries[id].clone());
                }
                super::ProviderKey::Type(id) if async_factories.contains_key(id) => {
                    self.registry
                        .async_factories
                        .write()
                        .unwrap()
                        .insert(*id, async_factories[id].clone());
                }
                super::ProviderKey::Token(name) if tokens.contains_key(name) => {
                    target_tokens.insert(name, tokens[name].clone());
                }
                _ => {}
            }
        }
        let hooks = child.registry.hooks.read().unwrap().clone();
        self.registry.hooks.write().unwrap().extend(hooks);
    }
}
