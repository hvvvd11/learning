use crate::{
  queries::tokens::update_token,
  utilities::{
    auth::{
      jwt::{create_refresh_token, create_token},
      token_wrapper::TokenWrapper,
    },
    payload::{ErrPayload, Payload},
  },
};
use axum::{
  extract::State,
  http::HeaderMap,
  response::{IntoResponse, Response},
  Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct RefreshTokenRequest {
  refresh_token: String,
}

#[derive(Serialize)]
pub struct ResponseModel {
  token: String,
  refresh_token: String,
}

pub async fn refresh_token(
  State(db_pool): State<PgPool>,
  State(jwt_secret): State<TokenWrapper>,
  Json(refresh_token_request): Json<RefreshTokenRequest>,
) -> Result<Response, ErrPayload> {
  let new_refresh_token = create_refresh_token();
  let new_token = create_token(&jwt_secret.0)?;

  let token_model = update_token(
    &db_pool,
    &refresh_token_request.refresh_token,
    &new_token,
    &new_refresh_token,
  )
  .await?;

  let mut headers = HeaderMap::new();

  headers.insert(
    "token",
    token_model
      .token
      .parse()
      .map_err(|err| ErrPayload::internal_server_error(err))
      .unwrap(),
  );
  headers.insert(
    "refresh-token",
    token_model
      .refresh_token
      .parse()
      .map_err(|err| ErrPayload::internal_server_error(err))
      .unwrap(),
  );

  Ok((headers, Payload::new("Updated token succesfully", ())).into_response())
}
