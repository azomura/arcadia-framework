use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};

#[derive(Clone, Default)]
pub struct RouteMetadata {
    controller: &'static str,
    handler: &'static str,
    values: Arc<HashMap<&'static str, &'static str>>,
}

impl RouteMetadata {
    pub fn new(controller: &'static str, handler: &'static str) -> Self {
        Self { controller, handler, values: Arc::default() }
    }

    pub fn with(mut self, key: &'static str, value: &'static str) -> Self {
        Arc::make_mut(&mut self.values).insert(key, value);
        self
    }

    pub fn controller(&self) -> &'static str {
        self.controller
    }

    pub fn handler(&self) -> &'static str {
        self.handler
    }
}

impl<S> FromRequestParts<S> for RouteMetadata
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        parts.extensions.get::<Self>().cloned().ok_or(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

pub struct Reflector;

impl Reflector {
    pub fn get<'a>(metadata: &'a RouteMetadata, key: &str) -> Option<&'a str> {
        metadata.values.get(key).copied()
    }
}
