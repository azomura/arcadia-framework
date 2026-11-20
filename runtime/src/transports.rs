use std::{
    collections::HashMap,
    future::Future,
    pin::Pin,
    sync::{
        Arc, Mutex,
        atomic::{AtomicU64, Ordering},
    },
};

use axum::{
    Json, Router,
    extract::{
        WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    routing::post,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

pub type TransportFuture = Pin<Box<dyn Future<Output = Value> + Send>>;
type RpcHandler = Arc<dyn Fn(Value) -> TransportFuture + Send + Sync>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RpcRequest {
    pub pattern: String,
    pub payload: Value,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RpcResponse {
    pub payload: Value,
}

#[derive(Clone, Default)]
pub struct MicroserviceRouter {
    handlers: Arc<Mutex<HashMap<String, RpcHandler>>>,
}

impl MicroserviceRouter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register<F, Fut>(&self, pattern: impl Into<String>, handler: F)
    where
        F: Fn(Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Value> + Send + 'static,
    {
        self.handlers
            .lock()
            .unwrap()
            .insert(pattern.into(), Arc::new(move |payload| Box::pin(handler(payload))));
    }

    pub async fn dispatch(&self, request: RpcRequest) -> Option<RpcResponse> {
        let handler = self.handlers.lock().unwrap().get(&request.pattern).cloned()?;
        Some(RpcResponse { payload: handler(request.payload).await })
    }
}

pub trait WebSocketGateway: Clone + Send + Sync + 'static {
    fn on_connect(&self, _: &WebSocketContext) {}

    fn on_message(&self, message: Message)
    -> Pin<Box<dyn Future<Output = Option<Message>> + Send>>;

    fn on_disconnect(&self, _: &WebSocketContext) {}

    fn on_error(&self, _: &WebSocketContext, _: String) {}
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WebSocketContext {
    connection_id: u64,
}

impl WebSocketContext {
    pub fn connection_id(&self) -> u64 {
        self.connection_id
    }
    pub fn new_for_test(connection_id: u64) -> Self {
        Self { connection_id }
    }
}

static NEXT_CONNECTION_ID: AtomicU64 = AtomicU64::new(1);

pub fn websocket_router<G: WebSocketGateway>(gateway: G) -> Router {
    Router::new().route(
        "/",
        axum::routing::get(move |upgrade: WebSocketUpgrade| {
            let gateway = gateway.clone();
            async move {
                let context = WebSocketContext {
                    connection_id: NEXT_CONNECTION_ID.fetch_add(1, Ordering::Relaxed),
                };
                upgrade.on_upgrade(move |socket| websocket_session(socket, gateway, context))
            }
        }),
    )
}

async fn websocket_session<G: WebSocketGateway>(
    mut socket: WebSocket, gateway: G, context: WebSocketContext,
) {
    gateway.on_connect(&context);
    while let Some(Ok(message)) = socket.recv().await {
        if let Some(response) = gateway.on_message(message).await
            && socket.send(response).await.is_err()
        {
            gateway.on_error(&context, String::from("failed to send websocket message"));
            break;
        }
    }
    gateway.on_disconnect(&context);
}

#[derive(Clone, Debug, Deserialize)]
pub struct GraphqlRequest {
    pub query: String,
    #[serde(default)]
    pub variables: Value,
}

pub trait GraphqlSchema: Clone + Send + Sync + 'static {
    fn execute(&self, request: GraphqlRequest) -> Pin<Box<dyn Future<Output = Value> + Send>>;
}

pub fn graphql_router<S: GraphqlSchema>(schema: S) -> Router {
    Router::new().route(
        "/",
        post(move |Json(request): Json<GraphqlRequest>| {
            let schema = schema.clone();
            async move { Json(json!({ "data": schema.execute(request).await })) }
        }),
    )
}
