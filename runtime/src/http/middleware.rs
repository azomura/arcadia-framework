use axum::{body::Body, http::Request, response::Response};

use super::HttpResult;

pub trait Middleware: Send + Sync {
    fn before(&self, _request: &mut Request<Body>) -> HttpResult<()> {
        Ok(())
    }

    fn after(&self, _response: &mut Response) -> HttpResult<()> {
        Ok(())
    }
}
