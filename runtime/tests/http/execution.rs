use arcadia_framework::http::{ExecutionContext, RouteMetadata, TransportType};
use axum::{body::Body, http::Request};

#[test]
fn execution_context_exposes_http_transport_metadata() {
    let request = Request::new(Body::empty());
    let metadata = RouteMetadata::new("UsersController", "list");
    let context = ExecutionContext::for_test(&request, &metadata);

    assert_eq!(context.transport(), TransportType::Http);
    assert_eq!(context.get_type(), TransportType::Http);
    assert!(context.is_http());
    assert_eq!(context.class_name(), "UsersController");
    assert_eq!(context.handler_name(), "list");
    assert_eq!(context.get_class(), "UsersController");
    assert_eq!(context.get_handler(), "list");
    assert_eq!(context.switch_to_http().method(), "GET");
    assert_eq!(context.http().request().method(), "GET");
}
