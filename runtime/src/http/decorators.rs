use axum::{
    Json,
    extract::{FromRequest, FromRequestParts, Path, Query, Request},
    http::{StatusCode, request::Parts},
};
use serde::de::DeserializeOwned;

pub struct JsonBody<T>(pub T);

impl<T, S> FromRequest<S> for JsonBody<T>
where
    T: DeserializeOwned + Send,
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request(request: Request, state: &S) -> Result<Self, Self::Rejection> {
        Json::<T>::from_request(request, state)
            .await
            .map(|Json(value)| Self(value))
            .map_err(|_| StatusCode::BAD_REQUEST)
    }
}

pub struct QueryParams<T>(pub T);

impl<T, S> FromRequestParts<S> for QueryParams<T>
where
    T: DeserializeOwned + Send,
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Query::<T>::from_request_parts(parts, state)
            .await
            .map(|Query(value)| Self(value))
            .map_err(|_| StatusCode::BAD_REQUEST)
    }
}

pub struct PathParams<T>(pub T);

impl<T, S> FromRequestParts<S> for PathParams<T>
where
    T: DeserializeOwned + Send,
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Path::<T>::from_request_parts(parts, state)
            .await
            .map(|Path(value)| Self(value))
            .map_err(|_| StatusCode::BAD_REQUEST)
    }
}
