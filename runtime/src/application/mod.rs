use axum::{Router, middleware};
use std::net::SocketAddr;
use utoipa::openapi::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{http::HttpPipeline, routing::log_request};

pub mod container;
mod factory;
mod hooks;
mod injectable;
mod lifecycle;
pub mod request;
mod scope;

pub use container::{Container, ProviderToken};
pub use factory::ApplicationFactory;
pub use hooks::Lifecycle;
pub use injectable::Injectable;
pub use request::{Injected, InjectedToken, RequestContainer};
pub use scope::Scope;

use lifecycle::{not_found, shutdown};
use tokio::net::TcpListener;

pub struct Application {
    container: Container,
    router: Router,
    documentation: bool,
}

impl Application {
    pub(super) fn new(container: Container, router: Router) -> Self {
        container.initialize();
        Self {
            container,
            router: router.fallback(not_found).layer(middleware::from_fn(log_request)),
            documentation: false,
        }
    }

    pub fn documentation(mut self, enabled: bool) -> Self {
        self.documentation = enabled;
        self
    }

    pub fn swagger(mut self, document: OpenApi) -> Self {
        self.router =
            self.router.merge(SwaggerUi::new("/docs").url("/docs/openapi.json", document));
        self.documentation = true;
        self
    }

    pub fn pipeline(mut self, pipeline: HttpPipeline) -> Self {
        self.router = pipeline.apply(self.router);
        self
    }

    pub fn into_router(self) -> Router {
        self.router
    }

    pub async fn listen(self, address: SocketAddr) -> std::io::Result<()> {
        let listener = TcpListener::bind(address).await?;
        let address = listener.local_addr()?;
        let url = format!("http://{address}");

        tracing::info!(
            log_kind = "SUCCESS", address = %address,
            "Arcadia backend is listening at {url}"
        );
        if self.documentation {
            tracing::info!(log_kind = "SUCCESS", "API documentation is available at {url}/docs/");
        }

        let service = self.router.into_make_service_with_connect_info::<SocketAddr>();
        let server = axum::serve(listener, service);

        let result = server.with_graceful_shutdown(shutdown()).await;
        self.container.shutdown();
        result
    }
}
