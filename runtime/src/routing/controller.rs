use axum::Router;
use utoipa::openapi::OpenApi;

use crate::Container;

pub trait Controller: Clone + Send + Sync + 'static {
    fn routes(self) -> Router;
    fn routes_with_container(self, _container: Container) -> Router
    where
        Self: Sized,
    {
        self.routes()
    }
    fn openapi() -> OpenApi;
}
