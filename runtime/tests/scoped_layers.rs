use std::sync::{Mutex, OnceLock};

use arcadia_framework::ApplicationFactory;
use arcadia_framework::http::{
    ExceptionFilter, Guard, HttpException, Interceptor, Middleware, Reflector, RequestPipe,
    RouteMetadata,
};
use arcadia_macros::{controller, injectable, module};
use axum::{
    body::Body,
    http::Request,
    response::{IntoResponse, Response},
};
use tower::ServiceExt;

static EVENTS: OnceLock<Mutex<Vec<&'static str>>> = OnceLock::new();

fn events() -> &'static Mutex<Vec<&'static str>> {
    EVENTS.get_or_init(|| Mutex::new(Vec::new()))
}
fn push(event: &'static str) {
    events().lock().unwrap().push(event);
}

#[derive(Default)]
struct ControllerMiddleware;
impl Middleware for ControllerMiddleware {
    fn before(&self, _: &mut Request<Body>) -> Result<(), HttpException> {
        push("controller_middleware");
        Ok(())
    }
    fn after(&self, _: &mut Response) -> Result<(), HttpException> {
        push("controller_middleware_after");
        Ok(())
    }
}

#[derive(Default)]
struct RouteMiddleware;
impl Middleware for RouteMiddleware {
    fn before(&self, _: &mut Request<Body>) -> Result<(), HttpException> {
        push("route_middleware");
        Ok(())
    }
    fn after(&self, _: &mut Response) -> Result<(), HttpException> {
        push("route_middleware_after");
        Ok(())
    }
}

#[derive(Default)]
struct ControllerGuard;
impl Guard for ControllerGuard {
    fn can_activate(&self, _: &Request<Body>) -> Result<(), HttpException> {
        push("controller_guard");
        Ok(())
    }
}

#[derive(Default)]
struct RouteGuard;
impl Guard for RouteGuard {
    fn can_activate(&self, _: &Request<Body>) -> Result<(), HttpException> {
        push("route_guard");
        Ok(())
    }
}

#[derive(Default)]
struct ControllerPipe;
impl RequestPipe for ControllerPipe {
    fn transform_request(&self, _: &mut Request<Body>) -> Result<(), HttpException> {
        push("controller_pipe");
        Ok(())
    }
}

#[derive(Default)]
struct RoutePipe;
impl RequestPipe for RoutePipe {
    fn transform_request(&self, _: &mut Request<Body>) -> Result<(), HttpException> {
        push("route_pipe");
        Ok(())
    }
}

#[derive(Default)]
struct ControllerInterceptor;
impl Interceptor for ControllerInterceptor {
    fn before(&self, _: &Request<Body>) -> Result<(), HttpException> {
        push("controller_interceptor");
        Ok(())
    }
    fn after(&self, _: &mut Response) -> Result<(), HttpException> {
        push("controller_interceptor_after");
        Ok(())
    }
}

#[derive(Default)]
struct RouteInterceptor;
impl Interceptor for RouteInterceptor {
    fn before(&self, _: &Request<Body>) -> Result<(), HttpException> {
        push("route_interceptor");
        Ok(())
    }
    fn after(&self, _: &mut Response) -> Result<(), HttpException> {
        push("route_interceptor_after");
        Ok(())
    }
}

#[derive(Default)]
struct ControllerFilter;
impl ExceptionFilter for ControllerFilter {
    fn catch(&self, exception: HttpException) -> Response {
        push("controller_filter");
        exception.into_response()
    }
}

#[derive(Default)]
struct RouteFilter;
impl ExceptionFilter for RouteFilter {
    fn catch(&self, exception: HttpException) -> Response {
        push("route_filter");
        exception.into_response()
    }
}

#[derive(Clone)]
struct AppController;

#[controller(
    path = "/scope",
    guards(ControllerGuard),
    interceptors(ControllerInterceptor),
    pipes(ControllerPipe),
    filters(ControllerFilter),
    middleware(ControllerMiddleware)
)]
#[injectable]
impl AppController {
    fn new() -> Self {
        Self
    }

    #[get("/")]
    #[guards(RouteGuard)]
    #[interceptors(RouteInterceptor)]
    #[pipes(RoutePipe)]
    #[filters(RouteFilter)]
    #[middleware(RouteMiddleware)]
    #[metadata(role = "admin")]
    async fn index(&self, metadata: RouteMetadata) -> &'static str {
        assert_eq!(Reflector::get(&metadata, "role"), Some("admin"));
        push("handler");
        "ok"
    }
}

#[module(controllers(AppController))]
struct AppModule {}

#[tokio::test]
async fn controller_and_route_layers_are_applied() {
    events().lock().unwrap().clear();
    let response = ApplicationFactory::create(AppModule {})
        .into_router()
        .oneshot(Request::builder().uri("/scope/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    assert_eq!(
        *events().lock().unwrap(),
        [
            "controller_middleware",
            "route_middleware",
            "controller_guard",
            "route_guard",
            "controller_interceptor",
            "route_interceptor",
            "controller_pipe",
            "route_pipe",
            "handler",
            "route_interceptor_after",
            "controller_interceptor_after",
            "route_middleware_after",
            "controller_middleware_after",
        ]
    );
}
