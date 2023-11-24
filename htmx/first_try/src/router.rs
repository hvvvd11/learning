use crate::handlers::test_html;
use axum::{http::Method, routing::get, Router};
use tower_http::{
  cors::{Any, CorsLayer},
  services::ServeDir,
};

pub fn create_router() -> Router {
  let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::PUT, Method::POST])
    .allow_origin(Any)
    .allow_headers(Any)
    .expose_headers(Any);

  Router::new()
    .nest_service("/assets", ServeDir::new("assets"))
    .route("/", get(test_html))
    .layer(cors)
}
