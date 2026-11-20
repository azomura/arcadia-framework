use arcadia_framework::ApplicationFactory;
use arcadia_framework::http::{JsonBody, PathParams, QueryParams};
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
}
