use super::Application;
use crate::Module;

pub struct ApplicationFactory;

impl ApplicationFactory {
    pub fn create(module: impl Module) -> Application {
        let (container, router) = module.into_parts();
        Application::new(container, router)
    }
}
