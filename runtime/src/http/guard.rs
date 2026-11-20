use std::marker::PhantomData;

use axum::{
    body::Body,
    extract::FromRequestParts,
    http::{Request, request::Parts},
};

use super::{ExecutionContext, HttpException, HttpResult};

pub trait Guard: Send + Sync {
    fn can_activate(&self, request: &Request<Body>) -> HttpResult<()>;

    fn can_activate_context(&self, context: &ExecutionContext<'_>) -> HttpResult<()> {
        self.can_activate(context.request())
    }
}

pub struct Guarded<G>(PhantomData<G>);

impl<G, S> FromRequestParts<S> for Guarded<G>
where
    G: Guard + Default,
    S: Send + Sync,
{
    type Rejection = HttpException;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let mut request = Request::builder()
            .method(parts.method.clone())
            .uri(parts.uri.clone())
            .version(parts.version)
            .body(Body::empty())
            .unwrap();
        *request.headers_mut() = parts.headers.clone();
        G::default().can_activate(&request)?;
        Ok(Self(PhantomData))
    }
}
