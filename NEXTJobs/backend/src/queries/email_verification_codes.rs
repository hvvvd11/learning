use axum::http::StatusCode;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::PgPool;

use crate::utilities::payload::ErrPayload;

#[derive(Serialize, Deserialize, FromRow, Debug, Default)]
pub struct EmailVerificationCode {
  pub user_id: i32,
  pub verification_code: i32,
  pub expiration_time: NaiveDateTime,
  pub login_tmp_token: String,
}

pub async fn save_email_verification_code_model(
  pool: &PgPool,
  email_code_model: &EmailVerificationCode,
) -> Result<EmailVerificationCode, ErrPayload> {
  let result = sqlx::query_as!(
    EmailVerificationCode,
    r#"
        INSERT INTO email_verification_codes (user_id, verification_code, login_tmp_token)
        VALUES ($1, $2, $3)
        RETURNING user_id, verification_code, expiration_time, login_tmp_token
    "#,
    email_code_model.user_id,
    email_code_model.verification_code,
    email_code_model.login_tmp_token,
  )
  .fetch_one(pool)
  .await;

  match result {
    Ok(code) => Ok(code),
    Err(err) => Err(ErrPayload::internal_server_error(err)),
  }
}
pub async fn find_email_confirmation_code_model_by_user_id(
  pool: &PgPool,
  user_id: i32,
) -> Result<EmailVerificationCode, ErrPayload> {
  match sqlx::query_as!(
    EmailVerificationCode,
    "SELECT * FROM email_verification_codes WHERE user_id = $1",
    user_id
  )
  .fetch_optional(pool)
  .await
  {
    Ok(Some(user)) => Ok(user),
    Ok(None) => Err(ErrPayload::new(
      StatusCode::NOT_FOUND,
      "User with such email is not found",
    )),
    Err(err) => Err(ErrPayload::internal_server_error(err)),
  }
}

pub async fn find_email_confirmation_code_model_by_login_tmp_token(
  pool: &PgPool,
  login_tmp_token: String,
) -> Result<EmailVerificationCode, ErrPayload> {
  match sqlx::query_as!(
    EmailVerificationCode,
    "SELECT * FROM email_verification_codes WHERE login_tmp_token = $1",
    login_tmp_token
  )
  .fetch_optional(pool)
  .await
  {
    Ok(Some(eccm)) => Ok(eccm),
    Ok(None) => Err(ErrPayload::new(
      StatusCode::NOT_FOUND,
      "User with such tmp_token is not found",
    )),
    Err(err) => Err(ErrPayload::internal_server_error(err)),
  }
}
