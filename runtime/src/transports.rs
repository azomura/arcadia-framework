use std::{
    collections::{HashMap, HashSet},
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
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::{Value, json};
use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};

use crate::Container;

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

#[derive(Clone, Default)]
pub struct RoomHub {
    connections: Arc<Mutex<HashMap<u64, UnboundedSender<Message>>>>,
    rooms: Arc<Mutex<HashMap<String, HashSet<u64>>>>,
}

impl RoomHub {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn register(&self, connection_id: u64) -> tokio::sync::mpsc::UnboundedReceiver<Message> {
        let (sender, receiver) = unbounded_channel();
        self.connections.lock().unwrap().insert(connection_id, sender);
        receiver
    }
    pub fn unregister(&self, connection_id: u64) {
        self.connections.lock().unwrap().remove(&connection_id);
        self.rooms.lock().unwrap().values_mut().for_each(|connections| {
            connections.remove(&connection_id);
        });
    }
    pub fn connected(&self) -> usize {
        self.connections.lock().unwrap().len()
    }
    pub fn join(&self, room: impl Into<String>, connection_id: u64) {
        self.rooms.lock().unwrap().entry(room.into()).or_default().insert(connection_id);
    }
    pub fn leave(&self, room: &str, connection_id: u64) {
        if let Some(connections) = self.rooms.lock().unwrap().get_mut(room) {
            connections.remove(&connection_id);
        }
    }
    pub fn broadcast(&self, room: &str, message: Message) -> usize {
        let ids = self.rooms.lock().unwrap().get(room).cloned().unwrap_or_default();
        let connections = self.connections.lock().unwrap();
        ids.into_iter()
            .filter(|id| connections.get(id).is_some_and(|tx| tx.send(message.clone()).is_ok()))
            .count()
    }
    pub fn close_all(&self) -> usize {
        let connections = self.connections.lock().unwrap();
        connections.values().filter(|tx| tx.send(Message::Close(None)).is_ok()).count()
    }
}

#[derive(Clone)]
pub struct WebSocketContext {
    connection_id: u64,
    container: Container,
    hub: RoomHub,
}

impl WebSocketContext {
    pub fn connection_id(&self) -> u64 {
        self.connection_id
    }
    pub fn container(&self) -> &Container {
        &self.container
    }
    pub fn hub(&self) -> &RoomHub {
        &self.hub
    }
    pub fn new_for_test(connection_id: u64) -> Self {
        Self { connection_id, container: Container::new(), hub: RoomHub::new() }
    }
}

pub trait WebSocketGateway: Clone + Send + Sync + 'static {
    fn on_connect(&self, _: &WebSocketContext) {}
    fn on_message(&self, message: Message)
    -> Pin<Box<dyn Future<Output = Option<Message>> + Send>>;
    fn on_disconnect(&self, _: &WebSocketContext) {}
    fn on_error(&self, _: &WebSocketContext, _: String) {}
}

pub trait WebSocketGuard: Send + Sync {
    fn can_activate(&self, context: &WebSocketContext, message: &Message) -> bool;
}
pub trait WebSocketPipe: Send + Sync {
    fn transform(&self, message: Message) -> Result<Message, String>;
}
pub trait WebSocketInterceptor: Send + Sync {
    fn before(&self, _: &WebSocketContext, _: &Message) {}
    fn after(&self, _: &WebSocketContext, _: &Message) {}
}
pub trait WebSocketFilter: Send + Sync {
    fn catch(&self, context: &WebSocketContext, error: String) -> Option<Message>;
}

pub struct WebSocketServer<G> {
    gateway: G,
    guards: Vec<Arc<dyn WebSocketGuard>>,
    pipes: Vec<Arc<dyn WebSocketPipe>>,
    interceptors: Vec<Arc<dyn WebSocketInterceptor>>,
    filter: Option<Arc<dyn WebSocketFilter>>,
    hub: RoomHub,
    container: Container,
}

impl<G: WebSocketGateway> WebSocketServer<G> {
    pub fn new(gateway: G) -> Self {
        Self {
            gateway,
            guards: Vec::new(),
            pipes: Vec::new(),
            interceptors: Vec::new(),
            filter: None,
            hub: RoomHub::new(),
            container: Container::new(),
        }
    }
    pub fn guard(mut self, guard: impl WebSocketGuard + 'static) -> Self {
        self.guards.push(Arc::new(guard));
        self
    }
    pub fn pipe(mut self, pipe: impl WebSocketPipe + 'static) -> Self {
        self.pipes.push(Arc::new(pipe));
        self
    }
    pub fn interceptor(mut self, interceptor: impl WebSocketInterceptor + 'static) -> Self {
        self.interceptors.push(Arc::new(interceptor));
        self
    }
    pub fn filter(mut self, filter: impl WebSocketFilter + 'static) -> Self {
        self.filter = Some(Arc::new(filter));
        self
    }
    pub fn hub(&self) -> RoomHub {
        self.hub.clone()
    }
    pub fn close_all(&self) -> usize {
        self.hub.close_all()
    }
    pub fn router(self) -> Router {
        websocket_router_with(self)
    }
}

pub fn websocket_router<G: WebSocketGateway>(gateway: G) -> Router {
    WebSocketServer::new(gateway).router()
}
pub fn websocket_router_with<G: WebSocketGateway>(server: WebSocketServer<G>) -> Router {
    let server = Arc::new(server);
    Router::new().route(
        "/",
        axum::routing::get(move |upgrade: WebSocketUpgrade| {
            let server = Arc::clone(&server);
            async move {
                let id = NEXT_CONNECTION_ID.fetch_add(1, Ordering::Relaxed);
                upgrade.on_upgrade(move |socket| websocket_session(socket, server, id))
            }
        }),
    )
}

static NEXT_CONNECTION_ID: AtomicU64 = AtomicU64::new(1);

async fn websocket_session<G: WebSocketGateway>(
    mut socket: WebSocket, server: Arc<WebSocketServer<G>>, id: u64,
) {
    let (sender, mut receiver) = unbounded_channel();
    server.hub.connections.lock().unwrap().insert(id, sender);
    let context = WebSocketContext {
        connection_id: id,
        container: server.container.request_scope(),
        hub: server.hub.clone(),
    };
    server.gateway.on_connect(&context);
    loop {
        tokio::select! {
            Some(Ok(message)) = socket.recv() => {
                if matches!(message, Message::Close(_)) { break; }
                if let Message::Ping(payload) = &message {
                    if socket.send(Message::Pong(payload.clone())).await.is_err() { break; }
                    continue;
                }
                if !server.guards.iter().all(|guard| guard.can_activate(&context, &message)) {
                    let error = String::from("websocket guard rejected message");
                    server.gateway.on_error(&context, error.clone());
                    if let Some(filter) = &server.filter
                        && let Some(response) = filter.catch(&context, error)
                        && socket.send(response).await.is_err()
                    {
                        break;
                    }
                    continue;
                }
                let mut message = Some(message);
                let mut valid = true;
                for pipe in &server.pipes {
                    let value = message.take().unwrap();
                    match pipe.transform(value) {
                        Ok(value) => message = Some(value),
                        Err(error) => {
                            valid = false;
                            server.gateway.on_error(&context, error.clone());
                            if let Some(filter) = &server.filter
                                && let Some(response) = filter.catch(&context, error)
                                && socket.send(response).await.is_err()
                            {
                                break;
                            }
                            break;
                        }
                    }
                }
                if !valid { continue; }
                let message = message.unwrap();
                for interceptor in &server.interceptors { interceptor.before(&context, &message); }
                if let Some(response) = server.gateway.on_message(message).await {
                    for interceptor in server.interceptors.iter().rev() { interceptor.after(&context, &response); }
                    if socket.send(response).await.is_err() { server.gateway.on_error(&context, String::from("failed to send websocket message")); break; }
                }
            }
            Some(message) = receiver.recv() => {
                let close = matches!(message, Message::Close(_));
                if socket.send(message).await.is_err() || close { break; }
            }
            else => break,
        }
    }
    server.hub.unregister(id);
    server.gateway.on_disconnect(&context);
}

pub struct JsonMessage<T>(pub T);

impl<T: DeserializeOwned> JsonMessage<T> {
    pub fn parse(message: Message) -> Result<Self, String> {
        match message {
            Message::Text(text) => serde_json::from_str(&text).map(Self).map_err(|e| e.to_string()),
            Message::Binary(bytes) => {
                serde_json::from_slice(&bytes).map(Self).map_err(|e| e.to_string())
            }
            _ => Err(String::from("message is not JSON text or binary")),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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
