use std::sync::Arc;

use super::Container;

pub trait Lifecycle: Send + Sync {
    fn on_module_init(&self) {}
    fn on_application_bootstrap(&self) {}
    fn before_application_shutdown(&self) {}
    fn on_module_destroy(&self) {}
    fn on_application_shutdown(&self) {}
}

impl Container {
    pub fn register_lifecycle<T: Lifecycle + 'static>(&self) {
        let provider = self.resolve::<T>();
        self.registry.hooks.write().unwrap().push(provider);
    }

    pub(crate) fn initialize(&self) {
        for hook in self.hooks() {
            hook.on_module_init();
        }
        for hook in self.hooks() {
            hook.on_application_bootstrap();
        }
    }

    pub(crate) fn shutdown(&self) {
        for hook in self.hooks().into_iter().rev() {
            hook.before_application_shutdown();
            hook.on_module_destroy();
            hook.on_application_shutdown();
        }
    }

    fn hooks(&self) -> Vec<Arc<dyn Lifecycle>> {
        self.registry.hooks.read().unwrap().clone()
    }
}
