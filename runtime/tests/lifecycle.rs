use std::sync::atomic::{AtomicUsize, Ordering};

use arcadia_framework::{
    ApplicationFactory, Container, Injectable, Lifecycle, Module, ModuleContext,
};

static MODULE_INIT: AtomicUsize = AtomicUsize::new(0);
static APPLICATION_BOOTSTRAP: AtomicUsize = AtomicUsize::new(0);

struct Hook;

impl Injectable for Hook {
    fn create(_: &Container) -> Self {
        Self
    }
}

impl Lifecycle for Hook {
    fn on_module_init(&self) {
        MODULE_INIT.fetch_add(1, Ordering::SeqCst);
    }

    fn on_application_bootstrap(&self) {
        APPLICATION_BOOTSTRAP.fetch_add(1, Ordering::SeqCst);
    }
}

struct TestModule;

impl Module for TestModule {
    fn register(self, context: &mut ModuleContext) {
        context.provide::<Hook>();
        context.lifecycle::<Hook>();
    }
}

#[test]
fn application_bootstraps_registered_lifecycle_hooks() {
    MODULE_INIT.store(0, Ordering::SeqCst);
    APPLICATION_BOOTSTRAP.store(0, Ordering::SeqCst);

    let _application = ApplicationFactory::create(TestModule);

    assert_eq!(MODULE_INIT.load(Ordering::SeqCst), 1);
    assert_eq!(APPLICATION_BOOTSTRAP.load(Ordering::SeqCst), 1);
}
