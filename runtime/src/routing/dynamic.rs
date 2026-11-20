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

pub struct ForwardRef<F>(Option<F>);

impl<F> ForwardRef<F> {
    pub fn new(factory: F) -> Self {
        Self(Some(factory))
    }
}

impl<F, M> Module for ForwardRef<F>
where
    F: FnOnce() -> M + Send + 'static,
    M: Module,
{
    fn register(mut self, context: &mut ModuleContext) {
        (self.0.take().expect("forward reference already consumed"))().register(context);
    }

    fn openapi() -> OpenApi {
        M::openapi()
    }
}
