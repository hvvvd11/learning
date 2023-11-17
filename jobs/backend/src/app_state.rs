use crate::utilities::auth::token_wrapper::TokenWrapper;
use axum::extract::FromRef;
use sqlx::PgPool;

#[derive(Clone, FromRef)]
pub struct AppState {
  pub db_pool: PgPool,
  pub jwt_secret: TokenWrapper,
}
