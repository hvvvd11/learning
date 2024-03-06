use crate::{
  queries::user::User,
  utilities::{
    auth::jwt::create_token,
    payload::{ErrPayload, Payload},
  },
};
use axum::{debug_handler, extract::State, Json};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct RequestUser {
  password: String,
  email: String,
}

#[debug_handler]
pub async fn login(
  State(db_pool): State<PgPool>,
  Json(request_user): Json<RequestUser>,
) -> Result<Payload<User>, ErrPayload> {
  let user = User::find_by_email(&db_pool, &request_user.email).await?;

  user.verify_password(&request_user.password).await?;

  let token = create_token();

  Ok()
}
