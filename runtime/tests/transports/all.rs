use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
};

use arcadia_framework::transports::{
    GraphqlRequest, GraphqlSchema, MicroserviceRouter, RoomHub, RpcRequest, WebSocketContext,
    WebSocketFilter, WebSocketGateway, WebSocketGuard, WebSocketInterceptor, WebSocketPipe,
    WebSocketServer, graphql_router, websocket_router,
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

#[test]
fn websocket_hub_manages_rooms_and_broadcasts() {
    let hub = RoomHub::new();
    let mut receiver = hub.register(7);
    hub.join("chat", 7);

    assert_eq!(hub.connected(), 1);
    assert_eq!(hub.broadcast("chat", Message::Text("hello".into())), 1);
    assert!(matches!(receiver.try_recv(), Ok(Message::Text(message)) if message == "hello"));

    hub.leave("chat", 7);
    assert_eq!(hub.broadcast("chat", Message::Text("ignored".into())), 0);
    hub.unregister(7);
    assert_eq!(hub.connected(), 0);
}

#[test]
fn websocket_server_components_compose() {
    struct Allow;
    impl WebSocketGuard for Allow {
        fn can_activate(&self, _: &WebSocketContext, _: &Message) -> bool {
            true
        }
    }
    struct Pipe;
    impl WebSocketPipe for Pipe {
        fn transform(&self, message: Message) -> Result<Message, String> {
            Ok(message)
        }
    }
    struct Interceptor;
    impl WebSocketInterceptor for Interceptor {}
    struct Filter;
    impl WebSocketFilter for Filter {
        fn catch(&self, _: &WebSocketContext, error: String) -> Option<Message> {
            Some(Message::Text(error.into()))
        }
    }

    let server = WebSocketServer::new(Gateway)
        .guard(Allow)
        .pipe(Pipe)
        .interceptor(Interceptor)
        .filter(Filter);
    assert_eq!(server.hub().connected(), 0);
    let _router = server.router();
}

#[test]
fn websocket_json_message_parses_text_and_rejects_control_frames() {
    let parsed = arcadia_framework::transports::JsonMessage::<serde_json::Value>::parse(
        Message::Text(r#"{"value":42}"#.into()),
    )
    .unwrap();
    assert_eq!(parsed.0["value"], 42);
    assert!(
        arcadia_framework::transports::JsonMessage::<serde_json::Value>::parse(Message::Ping(
            Vec::new().into()
        ),)
        .is_err()
    );
}
