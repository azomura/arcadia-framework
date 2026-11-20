use axum::{body::Body, http::Request, response::Response};

use super::{ExecutionContext, HttpResult};

pub trait Interceptor: Send + Sync {
    fn before(&self, _request: &Request<Body>) -> HttpResult<()> {
        Ok(())
    }

    fn before_context(&self, context: &ExecutionContext<'_>) -> HttpResult<()> {
        self.before(context.request())
    }

    fn after(&self, _response: &mut Response) -> HttpResult<()> {
        Ok(())
    }
}
