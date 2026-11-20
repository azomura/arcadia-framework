use axum::{body::Body, http::Request};

use super::RouteMetadata;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TransportType {
    Http,
    WebSocket,
    Rpc,
    Graphql,
}

pub struct ExecutionContext<'a> {
    request: &'a Request<Body>,
    metadata: &'a RouteMetadata,
    transport: TransportType,
}

impl<'a> ExecutionContext<'a> {
    pub fn for_test(request: &'a Request<Body>, metadata: &'a RouteMetadata) -> Self {
        Self::new(request, metadata)
    }

    pub(crate) fn new(request: &'a Request<Body>, metadata: &'a RouteMetadata) -> Self {
        Self { request, metadata, transport: TransportType::Http }
    }

    pub fn request(&self) -> &'a Request<Body> {
        self.request
    }

    pub fn metadata(&self) -> &'a RouteMetadata {
        self.metadata
    }

    pub fn transport(&self) -> TransportType {
        self.transport
    }

    pub fn is_http(&self) -> bool {
        self.transport == TransportType::Http
    }

    pub fn class_name(&self) -> &'static str {
        self.metadata.controller()
    }

    pub fn handler_name(&self) -> &'static str {
        self.metadata.handler()
    }

    pub fn switch_to_http(&self) -> &Request<Body> {
        self.request
    }
}
