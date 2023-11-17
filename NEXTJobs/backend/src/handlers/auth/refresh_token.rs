use crate::utilities::payload::ErrPayload;
use axum::Json;
use std::net::SocketAddr;

use axum::{
  extract::{ConnectInfo, State},
  headers::UserAgent,
  TypedHeader,
};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct RefreshTokenRequest {
  refresh_token: String,
}

pub fn refresh_token(
  State(db_pool): State<PgPool>,
  Json(refresh_token_request): Json<RefreshTokenRequest>,
) -> Result<(), ErrPayload> {
  return Ok(());
}
