use crate::router::create_router;
use std::net::SocketAddr;

mod handlers;
mod router;

pub async fn run() {
  let app = create_router();

  axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
    .serve(app.into_make_service_with_connect_info::<SocketAddr>())
    .await
    .unwrap();
}
