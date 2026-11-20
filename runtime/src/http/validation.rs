use std::marker::PhantomData;

use super::{HttpException, HttpResult, Pipe};

pub trait Validate {
    fn validate(&self) -> Result<(), String>;
}

pub struct ValidationPipe<T>(PhantomData<T>);

impl<T> Default for ValidationPipe<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T: Validate + Send + Sync> Pipe<T> for ValidationPipe<T> {
    fn transform(&self, value: T) -> HttpResult<T> {
        value.validate().map_err(HttpException::bad_request)?;
        Ok(value)
    }
}
