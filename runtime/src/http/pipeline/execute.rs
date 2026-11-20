use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Request, State},
    middleware::Next,
    response::Response,
};

use super::super::{ExecutionContext, HttpException, RouteMetadata};
use super::HttpPipeline;

pub(super) async fn execute(
    State(pipeline): State<Arc<HttpPipeline>>, mut request: Request<Body>, next: Next,
) -> Response {
    for middleware in &pipeline.middleware {
        if let Err(exception) = middleware.before(&mut request) {
            return pipeline.filter.catch(exception);
        }
    }
    if let Err(exception) = authorize(&pipeline, &request) {
        return pipeline.filter.catch(exception);
    }
    for pipe in &pipeline.pipes {
        if let Err(exception) = pipe.transform_request(&mut request) {
            return pipeline.filter.catch(exception);
        }
    }
    finish(&pipeline, next.run(request).await)
}

fn authorize(pipeline: &HttpPipeline, request: &Request<Body>) -> Result<(), HttpException> {
    let metadata = request.extensions().get::<RouteMetadata>().cloned().unwrap_or_default();
    let context = ExecutionContext::new(request, &metadata);
    for guard in &pipeline.guards {
        guard.can_activate_context(&context)?;
    }
    for interceptor in &pipeline.interceptors {
        interceptor.before_context(&context)?;
    }
    Ok(())
}

fn finish(pipeline: &HttpPipeline, mut response: Response) -> Response {
    if let Some(exception) = response.extensions_mut().remove::<HttpException>() {
        return pipeline.filter.catch(exception);
    }
    for interceptor in pipeline.interceptors.iter().rev() {
        if let Err(exception) = interceptor.after(&mut response) {
            return pipeline.filter.catch(exception);
        }
    }
    for middleware in pipeline.middleware.iter().rev() {
        if let Err(exception) = middleware.after(&mut response) {
            return pipeline.filter.catch(exception);
        }
    }
    response
}
