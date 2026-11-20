use std::cell::RefCell;

use super::{Container, Entry, Provider, ProviderKey};
use crate::Scope;

type ResolutionStack = Vec<(ProviderKey, &'static str)>;

thread_local! {
    static RESOLVING: RefCell<ResolutionStack> = const { RefCell::new(Vec::new()) };
}

pub(crate) fn resolve_entry(container: &Container, key: ProviderKey, entry: &Entry) -> Provider {
    let _resolution = Resolution::enter(key, entry.name);
    match entry.scope {
        Scope::Singleton => singleton(container, entry),
        Scope::Transient => (entry.factory)(container),
        Scope::Request => request(container, key, entry),
    }
}

fn singleton(container: &Container, entry: &Entry) -> Provider {
    let mut value = entry.singleton.lock().unwrap();
    value.get_or_insert_with(|| (entry.factory)(container)).clone()
}

fn request(container: &Container, key: ProviderKey, entry: &Entry) -> Provider {
    if let Some(value) = container.requests.lock().unwrap().get(&key).cloned() {
        return value;
    }
    let provider = (entry.factory)(container);
    container.requests.lock().unwrap().entry(key).or_insert(provider).clone()
}

struct Resolution;

impl Resolution {
    fn enter(key: ProviderKey, name: &'static str) -> Self {
        RESOLVING.with(|stack| {
            let mut stack = stack.borrow_mut();
            if let Some(index) = stack.iter().position(|(current, _)| *current == key) {
                let mut cycle = stack[index..].iter().map(|(_, name)| *name).collect::<Vec<_>>();
                cycle.push(name);
                panic!("circular provider dependency: {}", cycle.join(" -> "));
            }
            stack.push((key, name));
        });
        Self
    }
}

impl Drop for Resolution {
    fn drop(&mut self) {
        RESOLVING.with(|stack| {
            stack.borrow_mut().pop();
        });
    }
}
