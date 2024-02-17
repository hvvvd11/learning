use std::{default, net::SocketAddr};

use crate::{
  queries::{tokens::save_token, users::find_user_by_email},
  utilities::{
    auth::{
      hash::verify_password,
      jwt::{create_refresh_token, create_token},
      token_wrapper::TokenWrapper,
    },
    email::send_and_create_email_confirmation_code,
    payload::{ErrPayload, Payload},
  },
};
use axum::{
  extract::{ConnectInfo, State},
  headers::UserAgent,
  http::{HeaderMap, HeaderValue, StatusCode},
  response::{IntoResponse, Response},
  Json, TypedHeader,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize)]
pub struct ResponseUser {
  pub username: String,
  pub email: String,
}

#[derive(Deserialize)]
pub struct RequestUser {
  email: String,
  password: String,
}

pub async fn login(
  State(db_pool): State<PgPool>,
  State(jwt_secret): State<TokenWrapper>,
  TypedHeader(user_agent): TypedHeader<UserAgent>,
  ConnectInfo(ip_addr): ConnectInfo<SocketAddr>,
  Json(request_user): Json<RequestUser>,
) -> Result<Response, ErrPayload> {
  let user = find_user_by_email(&db_pool, &request_user.email).await?;

  if !verify_password(&request_user.password, &user.password)? {
    return Err(ErrPayload::new(
      StatusCode::UNAUTHORIZED,
      "incorrect username and/or password",
    ));
  }

  let token = create_token(&jwt_secret.0)?;
  let refresh_tokens = create_refresh_token();

  let tokens = save_token(
    &db_pool,
    user.user_id,
    token,
    user_agent.to_string(),
    ip_addr.ip().to_string(),
    refresh_tokens,
  )
  .await?;

  let response_user = ResponseUser {
    username: user.username,
    email: user.email,
  };

  let mut headers = HeaderMap::new();

  if user.is_email_verified == true {
    headers.insert(
      "token",
      tokens
        .token
        .parse()
        .map_err(|err| ErrPayload::internal_server_error(err))
        .unwrap(),
    );
    headers.insert(
      "refresh-token",
      tokens
        .refresh_token
        .parse()
        .map_err(|err| ErrPayload::internal_server_error(err))
        .unwrap(),
    );

    Ok((headers, Payload::new("Logged in succesfully", response_user)).into_response())

  // EMAIL NOT CONFIRMED CASE
  } else {
    let login_tmp_token = send_and_create_email_confirmation_code(&db_pool, request_user.email, user.user_id).await?;
    headers.insert(
      "login_tmp_token",
      HeaderValue::from_str(&login_tmp_token).map_err(|err| ErrPayload::internal_server_error(err))?,
    );
    Ok((headers, Payload::new("EMAIL_IS_NOT_CONFIRMED", ())).into_response())
  }
}
