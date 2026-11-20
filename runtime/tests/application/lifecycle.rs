use std::sync::{
    Mutex,
    atomic::{AtomicUsize, Ordering},
};

use arcadia_framework::{
    ApplicationFactory, Container, Injectable, Lifecycle, Module, ModuleContext,
};

static MODULE_INIT: AtomicUsize = AtomicUsize::new(0);
static APPLICATION_BOOTSTRAP: AtomicUsize = AtomicUsize::new(0);
static BEFORE_SHUTDOWN: AtomicUsize = AtomicUsize::new(0);
static MODULE_DESTROY: AtomicUsize = AtomicUsize::new(0);
static APPLICATION_SHUTDOWN: AtomicUsize = AtomicUsize::new(0);
static TEST_LOCK: Mutex<()> = Mutex::new(());

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

    fn before_application_shutdown(&self) {
        BEFORE_SHUTDOWN.fetch_add(1, Ordering::SeqCst);
    }

    fn on_module_destroy(&self) {
        MODULE_DESTROY.fetch_add(1, Ordering::SeqCst);
    }

    fn on_application_shutdown(&self) {
        APPLICATION_SHUTDOWN.fetch_add(1, Ordering::SeqCst);
    }
}

struct TestModule;

impl Module for TestModule {
    fn register(self, context: &mut ModuleContext) {
        context.provide::<Hook>();
        context.lifecycle::<Hook>();
    }

    fn openapi() -> utoipa::openapi::OpenApi {
        utoipa::openapi::OpenApi::default()
    }
}

#[test]
fn application_bootstraps_registered_lifecycle_hooks() {
    let _lock = TEST_LOCK.lock().unwrap();
    MODULE_INIT.store(0, Ordering::SeqCst);
    APPLICATION_BOOTSTRAP.store(0, Ordering::SeqCst);
    BEFORE_SHUTDOWN.store(0, Ordering::SeqCst);
    MODULE_DESTROY.store(0, Ordering::SeqCst);
    APPLICATION_SHUTDOWN.store(0, Ordering::SeqCst);

    let _application = ApplicationFactory::create(TestModule);

    assert_eq!(MODULE_INIT.load(Ordering::SeqCst), 1);
    assert_eq!(APPLICATION_BOOTSTRAP.load(Ordering::SeqCst), 1);
}

#[test]
fn testing_module_closes_lifecycle_hooks_in_order() {
    let _lock = TEST_LOCK.lock().unwrap();
    BEFORE_SHUTDOWN.store(0, Ordering::SeqCst);
    MODULE_DESTROY.store(0, Ordering::SeqCst);
    APPLICATION_SHUTDOWN.store(0, Ordering::SeqCst);

    arcadia_framework::TestingModule::new(TestModule).compile().close();

    assert_eq!(BEFORE_SHUTDOWN.load(Ordering::SeqCst), 1);
    assert_eq!(MODULE_DESTROY.load(Ordering::SeqCst), 1);
    assert_eq!(APPLICATION_SHUTDOWN.load(Ordering::SeqCst), 1);
}
