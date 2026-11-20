use axum::{body::Body, http::Request};

use super::RouteMetadata;

pub struct ExecutionContext<'a> {
    request: &'a Request<Body>,
    metadata: &'a RouteMetadata,
}

impl<'a> ExecutionContext<'a> {
    pub(crate) fn new(request: &'a Request<Body>, metadata: &'a RouteMetadata) -> Self {
        Self { request, metadata }
    }

    pub fn request(&self) -> &'a Request<Body> {
        self.request
    }

    pub fn metadata(&self) -> &'a RouteMetadata {
        self.metadata
    }
}
