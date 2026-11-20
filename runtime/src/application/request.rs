use std::{marker::PhantomData, sync::Arc};

use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
    middleware::Next,
    response::Response,
};

use super::{Container, ProviderToken};

pub async fn context(mut request: axum::extract::Request, next: Next) -> Response {
    let Some(root) = request.extensions().get::<Container>().cloned() else {
        return next.run(request).await;
    };
    request.extensions_mut().insert(root.request_scope());
    next.run(request).await
}

pub struct Injected<T>(pub Arc<T>);

pub struct RequestContainer(pub Container);

impl<S> FromRequestParts<S> for RequestContainer
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Container>()
            .cloned()
            .map(Self)
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl<T, S> FromRequestParts<S> for Injected<T>
where
    T: Send + Sync + 'static,
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let Some(container) = parts.extensions.get::<Container>() else {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        };
        Ok(Self(container.resolve::<T>()))
    }
}

pub struct InjectedToken<T, P>(pub Arc<T>, PhantomData<P>);

impl<T, P> InjectedToken<T, P> {
    pub fn resolve(container: &Container, token: ProviderToken<T>) -> Self
    where
        T: Send + Sync + 'static,
    {
        Self(container.resolve_token(token), PhantomData)
    }
}
