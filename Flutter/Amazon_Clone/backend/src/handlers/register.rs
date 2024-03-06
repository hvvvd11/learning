use axum::{extract::State, http::Response, Json};
use axum_macros::debug_handler;
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
  queries::user::User,
  utilities::{
    auth::jwt::create_token,
    payload::{ErrPayloadResponse, Payload, PayloadResponse},
  },
};

#[derive(Deserialize)]
pub struct RequestUser {
  name: String,
  password: String,
  email: String,
}

#[debug_handler]
pub async fn register(
  State(db_pool): State<PgPool>,
  Json(request_user): Json<RequestUser>,
) -> Result<Response<Json<Payload<User>>>, ErrPayloadResponse> {
  let user = User {
    name: request_user.name,
    password: request_user.password,
    email: request_user.email,
    ..Default::default()
  };

  let user = User::save(&db_pool, &user).await?;

  let token = create_token();

  Ok(Payload::ok_with_header(
    "User registered succesfully",
    user,
    ("Authorization", &token),
  ))
}
