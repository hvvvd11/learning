use crate::utilities::payload::ErrPayload;
use chrono::NaiveDateTime;
use serde::Deserialize;

use axum::http::StatusCode;
use serde::Serialize;
use sqlx::{FromRow, PgPool};

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenResult {
  pub token: String,
  pub refresh_token: String,
}

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Tokens {
  pub token_id: i32,
  pub user_id: i32,
  pub token: String,
  pub refresh_token: String,
  pub user_agent: String,
  pub ip_address: String,
  pub created_at: NaiveDateTime,
  pub token_expiration_time: NaiveDateTime,
  pub refresh_token_expiration_time: NaiveDateTime,
}

pub async fn save_token(
  pool: &PgPool,
  user_id: i32,
  token: String,
  user_agent: String,
  ip_addr: String,
  refresh_token: String,
) -> Result<TokenResult, ErrPayload> {
  let result = sqlx::query_as!(
        TokenResult,
        "
        INSERT INTO tokens (user_id, token, user_agent, ip_address, created_at, token_expiration_time, refresh_token, refresh_token_expiration_time)
        VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP + INTERVAL '1 hour', $5, CURRENT_TIMESTAMP + INTERVAL '30 day')
        ON CONFLICT (user_id, user_agent, ip_address)
        DO UPDATE SET 
            token = EXCLUDED.token, 
            refresh_token = EXCLUDED.refresh_token, 
            created_at = CURRENT_TIMESTAMP, 
            token_expiration_time = CURRENT_TIMESTAMP + INTERVAL '1 hour',
            refresh_token_expiration_time = CURRENT_TIMESTAMP + INTERVAL '30 day'
        RETURNING token, refresh_token
        ",
        user_id,
        token,
        user_agent,
        ip_addr,
        refresh_token
    )
    .fetch_one(pool)
    .await;

  match result {
    Ok(result) => Ok(result),
    Err(err) => Err(ErrPayload::internal_server_error(err)),
  }
}

pub async fn update_token(
  pool: &PgPool,
  refresh_token: &str,
  new_token: &str,
  new_refresh_token: &str,
) -> Result<Tokens, ErrPayload> {
  let updated_row = sqlx::query_as!(
    Tokens,
    "
        UPDATE tokens
        SET 
            token = $2, 
            refresh_token = $3, 
            created_at = CURRENT_TIMESTAMP, 
            token_expiration_time = CURRENT_TIMESTAMP + INTERVAL '1 hour',
            refresh_token_expiration_time = CURRENT_TIMESTAMP + INTERVAL '30 day'
        WHERE refresh_token = $1
        RETURNING *
        ",
    refresh_token,
    new_token,
    new_refresh_token
  )
  .fetch_one(pool)
  .await;

  match updated_row {
    Ok(row) => Ok(row),
    Err(sqlx::Error::RowNotFound) => {
      // No matching record found
      Err(ErrPayload::new(StatusCode::UNAUTHORIZED, "No matching record found"))
    }
    Err(err) => Err(ErrPayload::internal_server_error(err)),
  }
}

pub async fn find_by_token(pool: &PgPool, token: &str) -> Result<Tokens, ErrPayload> {
  match sqlx::query_as!(Tokens, "SELECT * FROM tokens WHERE token = $1", token)
    .fetch_optional(pool)
    .await
  {
    Ok(Some(user)) => Ok(user),
    Ok(None) => Err(ErrPayload::new(StatusCode::NOT_FOUND, "Token not authorized")),
    Err(err) => Err({
      eprintln!("error finding a token error: {}", err);
      ErrPayload::new(StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong..")
    }),
  }
}

pub async fn find_by_refresh_token(pool: &PgPool, refresh_token: &str) -> Result<Tokens, ErrPayload> {
  match sqlx::query_as!(Tokens, "SELECT * FROM tokens WHERE refresh_token = $1", refresh_token)
    .fetch_optional(pool)
    .await
  {
    Ok(Some(user)) => Ok(user),
    Ok(None) => Err(ErrPayload::new(
      StatusCode::NOT_FOUND,
      "Refresh token is not authorized",
    )),
    Err(err) => Err({
      eprintln!("error finding a token error: {}", err);
      ErrPayload::new(StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong..")
    }),
  }
}
