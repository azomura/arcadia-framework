use arcadia_framework::ApplicationFactory;
use arcadia_framework::http::{HttpException, JsonBody, PathParams, Pipe, PipedJson, QueryParams};
use arcadia_macros::{controller, injectable, module};
use axum::{body::Body, http::Request};
use serde::Deserialize;
use tower::ServiceExt;

#[derive(Deserialize)]
struct ItemPath {
    id: u64,
}

#[derive(Deserialize)]
struct ItemQuery {
    page: u32,
}

#[derive(Deserialize)]
struct ItemBody {
    name: String,
}

#[derive(Default)]
struct IncrementPipe;

impl Pipe<ItemBody> for IncrementPipe {
    fn transform(&self, mut value: ItemBody) -> Result<ItemBody, HttpException> {
        value.name.push_str("-piped");
        Ok(value)
    }
}

#[derive(Clone)]
struct ItemsController;

#[controller(path = "/items")]
#[injectable]
impl ItemsController {
    fn new() -> Self {
        Self
    }

    #[get("/{id}")]
    async fn get(
        &self, PathParams(path): PathParams<ItemPath>, QueryParams(query): QueryParams<ItemQuery>,
    ) -> &'static str {
        assert_eq!(path.id, 7);
        assert_eq!(query.page, 2);
        "ok"
    }

    #[post("/")]
    async fn create(&self, JsonBody(body): JsonBody<ItemBody>) -> &'static str {
        assert_eq!(body.name, "arcadia");
        "created"
    }
}

#[module(controllers(ItemsController))]
struct ItemsModule {}

#[tokio::test]
async fn controller_supports_typed_parameter_decorators() {
    let router = ApplicationFactory::create(ItemsModule {}).into_router();
    let get_response = router
        .clone()
        .oneshot(Request::builder().uri("/items/7?page=2").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(get_response.status(), 200);

    let post_response = router
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/items/")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"name":"arcadia"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(post_response.status(), 200);

    let invalid_query = router_clone()
        .oneshot(Request::builder().uri("/items/7?page=nope").body(Body::empty()).unwrap());
    assert_eq!(invalid_query.await.unwrap().status(), 400);

    let invalid_path = router_clone()
        .oneshot(Request::builder().uri("/items/nope?page=2").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(invalid_path.status(), 400);

    let invalid_body = router_clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/items/")
                .header("content-type", "application/json")
                .body(Body::from("not-json"))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(invalid_body.status(), 400);
}

fn router_clone() -> axum::Router {
    ApplicationFactory::create(ItemsModule {}).into_router()
}

#[derive(Clone)]
struct PipedController;

#[controller(path = "/piped")]
#[injectable]
impl PipedController {
    fn new() -> Self {
        Self
    }

    #[post("/")]
    async fn create(&self, body: PipedJson<ItemBody, IncrementPipe>) -> String {
        body.into_inner().name
    }
}

#[module(controllers(PipedController))]
struct PipedModule {}

#[tokio::test]
async fn piped_json_transforms_valid_body_and_rejects_invalid_body() {
    let router = ApplicationFactory::create(PipedModule {}).into_router();
    let valid = router
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/piped/")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"name":"arcadia"}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(valid.status(), 200);

    let invalid = router
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/piped/")
                .header("content-type", "application/json")
                .body(Body::from("not-json"))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(invalid.status(), 400);
}
