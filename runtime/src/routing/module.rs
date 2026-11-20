use super::ModuleContext;

pub trait Module {
    fn register(self, context: &mut ModuleContext);

    fn into_router(self) -> axum::Router
    where
        Self: Sized,
    {
        let mut context = ModuleContext::new();
        self.register(&mut context);
        context.finish()
    }

    fn into_parts(self) -> (crate::Container, axum::Router)
    where
        Self: Sized,
    {
        let mut context = ModuleContext::new();
        self.register(&mut context);
        context.finish_parts()
    }
}
