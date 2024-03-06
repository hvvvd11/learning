use crate::{
  handlers::{login::login, register::register},
  AppState,
};
use axum::{routing::post, Router};
use tower_http::services::ServeDir;

pub fn create_router(app_state: AppState) -> Router {
  Router::new()
    .nest_service("/static", ServeDir::new("static"))
    .route("/register", post(register))
    .route("/login", post(login))
    .with_state(app_state)
}
