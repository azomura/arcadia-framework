use std::marker::PhantomData;

use axum::{
    Json,
    extract::{FromRequest, Request},
};
use serde::de::DeserializeOwned;

use super::{HttpException, HttpResult};

pub trait RequestPipe: Send + Sync {
    fn transform_request(&self, request: &mut Request) -> HttpResult<()>;
}

pub trait Pipe<T>: Send + Sync {
    fn transform(&self, value: T) -> HttpResult<T>;
}

pub struct PipedJson<T, P>(pub T, PhantomData<P>);

impl<T, P> PipedJson<T, P> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T, P, S> FromRequest<S> for PipedJson<T, P>
where
    T: DeserializeOwned,
    P: Pipe<T> + Default,
    S: Send + Sync,
{
    type Rejection = HttpException;

    async fn from_request(request: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(request, state)
            .await
            .map_err(|error| HttpException::bad_request(error.to_string()))?;
        Ok(Self(P::default().transform(value)?, PhantomData))
    }
}
