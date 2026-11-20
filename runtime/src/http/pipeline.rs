use std::sync::Arc;

use axum::{
    Router,
    middleware::{self},
};

use super::{ExceptionFilter, Guard, Interceptor, JsonExceptionFilter, Middleware, RequestPipe};

mod execute;

use execute::execute;

pub struct HttpPipeline {
    pub(super) guards: Vec<Arc<dyn Guard>>,
    pub(super) interceptors: Vec<Arc<dyn Interceptor>>,
    pub(super) pipes: Vec<Arc<dyn RequestPipe>>,
    pub(super) middleware: Vec<Arc<dyn Middleware>>,
    pub(super) filter: Arc<dyn ExceptionFilter>,
}

impl HttpPipeline {
    pub fn new() -> Self {
        Self {
            guards: Vec::new(),
            interceptors: Vec::new(),
            pipes: Vec::new(),
            middleware: Vec::new(),
            filter: Arc::new(JsonExceptionFilter),
        }
    }

    pub fn guard(mut self, guard: impl Guard + 'static) -> Self {
        self.guards.push(Arc::new(guard));
        self
    }

    pub fn interceptor(mut self, interceptor: impl Interceptor + 'static) -> Self {
        self.interceptors.push(Arc::new(interceptor));
        self
    }

    pub fn filter(mut self, filter: impl ExceptionFilter + 'static) -> Self {
        self.filter = Arc::new(filter);
        self
    }

    pub fn pipe(mut self, pipe: impl RequestPipe + 'static) -> Self {
        self.pipes.push(Arc::new(pipe));
        self
    }

    pub fn middleware(mut self, value: impl Middleware + 'static) -> Self {
        self.middleware.push(Arc::new(value));
        self
    }

    pub fn apply(self, router: Router) -> Router {
        router.layer(middleware::from_fn_with_state(Arc::new(self), execute))
    }
}

impl Default for HttpPipeline {
    fn default() -> Self {
        Self::new()
    }
}
