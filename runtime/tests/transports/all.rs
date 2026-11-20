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
    assert!(
        router
            .dispatch(RpcRequest { pattern: String::from("missing"), payload: json!({}) })
            .await
            .is_none()
    );
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

#[tokio::test]
async fn graphql_router_accepts_default_variables() {
    let response = graphql_router(Schema)
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"query":"{ hello }","variables":{}}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
}

#[test]
fn transport_messages_round_trip_through_serde() {
    let request = RpcRequest { pattern: String::from("ping"), payload: json!({"ok": true}) };
    let encoded = serde_json::to_string(&request).unwrap();
    let decoded: RpcRequest = serde_json::from_str(&encoded).unwrap();
    assert_eq!(decoded.pattern, "ping");
    assert_eq!(decoded.payload["ok"], true);

    let response = arcadia_framework::transports::RpcResponse { payload: json!("pong") };
    let decoded: arcadia_framework::transports::RpcResponse =
        serde_json::from_str(&serde_json::to_string(&response).unwrap()).unwrap();
    assert_eq!(decoded.payload, json!("pong"));
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
    gateway.on_error(&context, String::from("ignored"));

    assert_eq!(*events.lock().unwrap(), ["connect", "message", "disconnect"]);
    assert!(!context.container().contains::<u32>());
    assert_eq!(context.hub().connected(), 0);
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
    assert_eq!(hub.broadcast("missing", Message::Text("ignored".into())), 0);

    let dropped = hub.register(9);
    drop(dropped);
    hub.join("closed", 9);
    assert_eq!(hub.broadcast("closed", Message::Text("ignored".into())), 0);
}

#[test]
fn websocket_hub_closes_all_registered_connections() {
    let hub = RoomHub::new();
    let mut receiver = hub.register(1);
    let mut second_receiver = hub.register(2);

    assert_eq!(hub.close_all(), 2);
    assert!(matches!(receiver.try_recv(), Ok(Message::Close(None))));
    assert!(matches!(second_receiver.try_recv(), Ok(Message::Close(None))));
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
    assert_eq!(server.close_all(), 0);
    let context = WebSocketContext::new_for_test(8);
    let message = Message::Text(String::from("hello").into());
    let interceptor = Interceptor;
    interceptor.before(&context, &message);
    interceptor.after(&context, &message);
    assert!(Allow.can_activate(&context, &message));
    assert_eq!(Pipe.transform(message.clone()).unwrap(), message);
    assert!(matches!(Filter.catch(&context, String::from("error")), Some(Message::Text(_))));
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
    let binary = arcadia_framework::transports::JsonMessage::<serde_json::Value>::parse(
        Message::Binary(br#"{"value":43}"#.to_vec().into()),
    )
    .unwrap();
    assert_eq!(binary.0["value"], 43);
    assert!(
        arcadia_framework::transports::JsonMessage::<serde_json::Value>::parse(Message::Text(
            "invalid".into()
        ),)
        .is_err()
    );
}
