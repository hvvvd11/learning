use crate::handlers::auth::{
  email_verification::{confirm_email_verification_code, send_email_confirmation_code_again},
  refresh_token::refresh_token,
};
use axum::{http::Method, routing::post, Router};
use tower_http::cors::{Any, CorsLayer};

use crate::{
  app_state::AppState,
  handlers::auth::{login::login, register::register},
};

pub fn create_router(app_state: AppState) -> Router {
  let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::PUT, Method::POST])
    .allow_origin(Any)
    .allow_headers(Any)
    .expose_headers(Any);

  Router::new()
    .route(
      "/auth/send_email_confirmation_code_again",
      post(send_email_confirmation_code_again),
    )
    .route("/auth/refresh_token", post(refresh_token))
    .route("/auth/confirm_email", post(confirm_email_verification_code))
    .route("/auth/register", post(register))
    .route("/auth/login", post(login))
    // .route_layer(middleware::from_fn_with_state(app_state.clone(), require_authentication))
    .with_state(app_state)
    .layer(cors)
}
