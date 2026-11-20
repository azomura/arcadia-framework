use axum::response::Response;

use super::HttpException;

pub trait ExceptionFilter: Send + Sync {
    fn catch(&self, exception: HttpException) -> Response;
}

#[derive(Default)]
pub struct JsonExceptionFilter;

impl ExceptionFilter for JsonExceptionFilter {
    fn catch(&self, exception: HttpException) -> Response {
        exception.response()
    }
}
