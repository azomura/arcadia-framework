use utoipa::openapi::OpenApi;

use super::{Module, ModuleContext};

type Registration = Box<dyn FnOnce(&mut ModuleContext) + Send>;

pub struct DynamicModule {
    registration: Registration,
}

impl DynamicModule {
    pub fn new(register: impl FnOnce(&mut ModuleContext) + Send + 'static) -> Self {
        Self { registration: Box::new(register) }
    }
}

impl Module for DynamicModule {
    fn register(self, context: &mut ModuleContext) {
        (self.registration)(context);
    }

    fn openapi() -> OpenApi {
        OpenApi::default()
    }
}

pub struct GlobalModule<M>(pub M);

impl<M: Module> GlobalModule<M> {
    pub fn register(self, context: &mut ModuleContext) {
        self.0.register(context);
    }
}
