use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
};

use arcadia_framework::transports::{
    GraphqlRequest, GraphqlSchema, MicroserviceRouter, RpcRequest, WebSocketContext,
    WebSocketGateway, graphql_router, websocket_router,
};
use axum::{body::Body, extract::ws::Message, http::Request};
use serde_json::json;
use tower::ServiceExt;

#[tokio::test]
async fn microservice_router_dispatches_by_pattern() {
    let router = MicroserviceRouter::new();
    router.register("sum", |payload| async move {
        json!(payload["left"].as_i64().unwrap() + payload["right"].as_i64().unwrap())
    });
    let response = router
        .dispatch(RpcRequest {
            pattern: String::from("sum"),
            payload: json!({"left": 2, "right": 3}),
        })
        .await
        .unwrap();

    assert_eq!(response.payload, json!(5));
}

#[derive(Clone)]
struct Schema;

impl GraphqlSchema for Schema {
    fn execute(
        &self, request: GraphqlRequest,
    ) -> Pin<Box<dyn Future<Output = serde_json::Value> + Send>> {
        Box::pin(async move { json!({"query": request.query}) })
    }
}

#[tokio::test]
async fn graphql_router_executes_schema_requests() {
    let response = graphql_router(Schema)
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"query":"{ hello }"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
}

#[derive(Clone)]
struct Gateway;

impl WebSocketGateway for Gateway {
    fn on_message(
        &self, message: Message,
    ) -> Pin<Box<dyn Future<Output = Option<Message>> + Send>> {
        Box::pin(async move { Some(message) })
    }
}

#[tokio::test]
async fn websocket_gateway_registers_upgrade_route() {
    let response = websocket_router(Gateway)
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), 400);
}

#[derive(Clone)]
struct LifecycleGateway(Arc<Mutex<Vec<&'static str>>>);

impl WebSocketGateway for LifecycleGateway {
    fn on_connect(&self, context: &WebSocketContext) {
        assert!(context.connection_id() > 0);
        self.0.lock().unwrap().push("connect");
    }

    fn on_message(
        &self, message: Message,
    ) -> Pin<Box<dyn Future<Output = Option<Message>> + Send>> {
        let events = Arc::clone(&self.0);
        Box::pin(async move {
            events.lock().unwrap().push("message");
            Some(message)
        })
    }

    fn on_disconnect(&self, _: &WebSocketContext) {
        self.0.lock().unwrap().push("disconnect");
    }
}

#[tokio::test]
async fn websocket_gateway_exposes_connection_lifecycle() {
    let events = Arc::new(Mutex::new(Vec::new()));
    let gateway = LifecycleGateway(Arc::clone(&events));
    let context = WebSocketContext::new_for_test(42);

    gateway.on_connect(&context);
    gateway.on_message(Message::Text("hello".into())).await;
    gateway.on_disconnect(&context);

    assert_eq!(*events.lock().unwrap(), ["connect", "message", "disconnect"]);
}
