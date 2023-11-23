use axum::http::Request;
use axum::middleware::{from_fn, Next};
use axum::response::Response;
use axum::Router;
use super::{auth, health};
use super::vector;
use super::oss;
use super::llm;

//api
pub fn routers() -> Router {

    Router::new()
        .nest("/api/v1",auth::auth_router())
        .nest("/api/v1",health::health())
        .nest("/api/v1",vector::vector_router())
        .nest("/api/v1",oss::oss_router())
        .nest("/api/v1",llm::llm_router())
        .layer(from_fn(cors))

}



async fn cors<B>(request: Request<B>, next: Next<B>) -> Response {
    let mut response = next.run(request).await;
    let headers = response.headers_mut();
    headers.insert("Access-Control-Request-Method", "*".parse().unwrap());
    headers.insert("Access-Control-Allow-Methods", "*".parse().unwrap());
    headers.insert("Access-Control-Allow-Headers", "*".parse().unwrap());
    headers.insert("Access-Control-Allow-Credentials", "true".parse().unwrap());
    response
}