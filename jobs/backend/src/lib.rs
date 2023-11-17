use std::net::SocketAddr;

use app_state::AppState;
use router::create_router;

pub mod app_state;
mod handlers;
mod middleware;
mod queries;
mod router;
pub mod utilities;
pub async fn run(app_state: AppState) {
  let app = create_router(app_state);

  axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
    .serve(app.into_make_service_with_connect_info::<SocketAddr>())
    .await
    .unwrap();
}
