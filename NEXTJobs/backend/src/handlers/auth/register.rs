use std::net::SocketAddr;

use crate::{
  queries::{
    tokens::save_token,
    users::{save_user, Users},
  },
  utilities::{
    auth::{
      hash::hash_password,
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
use rand::Rng;
use regex::Regex;
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct RequestUser {
  username: String,
  password: String,
  email: String,
}

pub async fn register(
  State(db_pool): State<PgPool>,
  State(jwt_secret): State<TokenWrapper>,
  TypedHeader(user_agent): TypedHeader<UserAgent>,
  ConnectInfo(ip_addr): ConnectInfo<SocketAddr>,
  Json(request_user): Json<RequestUser>,
) -> Result<Response, ErrPayload> {
  check_email(&request_user.email)?;

  let request_user = Users {
    username: request_user.username,
    email: request_user.email,
    password: hash_password(&request_user.password)?,
    is_admin: false,
    is_email_verified: false,
    user_id: generate_user_id(),
  };

  let user = save_user(&db_pool, &request_user).await?;
  let token = create_token(&jwt_secret.0)?;
  let refresh_token = create_refresh_token();

  save_token(
    &db_pool,
    user.user_id,
    token,
    user_agent.to_string(),
    ip_addr.ip().to_string(),
    refresh_token,
  )
  .await?;

  let mut headers = HeaderMap::new();
  let login_tmp_token = send_and_create_email_confirmation_code(&db_pool, user.email, user.user_id).await?;
  headers.insert(
    "login_tmp_token",
    HeaderValue::from_str(&login_tmp_token).map_err(|err| ErrPayload::internal_server_error(err))?,
  );

  Ok((headers, Payload::new("User registered successfully!", ())).into_response())
}

pub fn check_email(email: &str) -> Result<String, ErrPayload> {
  let email_regex =
    Regex::new(r"(?i)^[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,}$").map_err(|err| ErrPayload::internal_server_error(err))?;

  if email_regex.is_match(email) == true {
    return Ok(email.to_string());
  } else {
    return Err(ErrPayload::new(
      StatusCode::BAD_REQUEST,
      "Invalid email, try another one",
    ));
  }
}

fn generate_user_id() -> i32 {
  let mut rng = rand::thread_rng();
  let random_number: i32 = rng.gen_range(10_000_000..100_000_000);

  return random_number;
}
