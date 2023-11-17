use axum::http::StatusCode;
use serde::Deserialize;
use serde::Serialize;
use sqlx::FromRow;

use sqlx::PgPool;

use crate::utilities::payload::ErrPayload;

#[derive(Debug, FromRow, Serialize, Deserialize, Default)]
pub struct Users {
  pub user_id: i32,
  pub username: String,
  pub email: String,
  pub password: String,
  pub is_admin: bool,
  pub is_email_verified: bool,
}

pub async fn find_user_by_username(pool: &PgPool, username: &str) -> Result<Users, ErrPayload> {
  match sqlx::query_as!(Users, "SELECT * FROM users WHERE username = $1", username)
    .fetch_optional(pool)
    .await
  {
    Ok(Some(user)) => Ok(user),
    Ok(None) => Err(ErrPayload::new(StatusCode::NOT_FOUND, "user with such credentials not found")),
    Err(err) => Err({
      eprintln!("error finding a username error: {}", err);
      ErrPayload::new(StatusCode::UNAUTHORIZED, "Error fingind user")
    }),
  }
}

pub async fn find_user_by_user_id(pool: &PgPool, user_id: i32) -> Result<Users, ErrPayload> {
  match sqlx::query_as!(Users, "SELECT * FROM users WHERE user_id = $1", user_id)
    .fetch_optional(pool)
    .await
  {
    Ok(Some(user)) => Ok(user),
    Ok(None) => Err(ErrPayload::new(StatusCode::NOT_FOUND, "user with such credentials not found")),
    Err(err) => Err({
      eprintln!("error finding a username error: {}", err);
      ErrPayload::new(StatusCode::UNAUTHORIZED, "Error fingind user")
    }),
  }
}

pub async fn find_user_by_email(pool: &PgPool, email: &str) -> Result<Users, ErrPayload> {
  match sqlx::query_as!(Users, "SELECT * FROM users WHERE email = $1", email)
    .fetch_optional(pool)
    .await
  {
    Ok(Some(user)) => Ok(user),
    Ok(None) => Err(ErrPayload::new(StatusCode::NOT_FOUND, "Users with such credentials not found")),
    Err(err) => Err({
      eprintln!("error finding a username error: {}", err);
      ErrPayload::new(StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong..")
    }),
  }
}

pub async fn save_user(pool: &PgPool, user: &Users) -> Result<Users, ErrPayload> {
  let user = sqlx::query_as!(
    Users,
    "
        INSERT INTO users (username, email, password, is_admin, is_email_verified, user_id)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING user_id, username, email, password, is_admin, is_email_verified
        ",
    &user.username,
    &user.email,
    &user.password,
    user.is_admin,
    user.is_email_verified,
    user.user_id,
  )
  .fetch_one(pool)
  .await;

  match user {
    Ok(user) => Ok(user),
    Err(err) => {
      if err
        .to_string()
        .contains("error returned from database: duplicate key value violates unique constraint")
      {
        eprintln!("key value error: {err}");
        return Err(ErrPayload::new(StatusCode::CONFLICT, "This username or email is taken"));
      }
      eprintln!("Error creating user: {err}");
      Err(ErrPayload::new(StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong.."))
    }
  }
}

pub async fn confirm_users_email(pool: &PgPool, email: String) -> Result<String, ErrPayload> {
  let result = sqlx::query!("UPDATE users SET is_email_verified = true WHERE email = $1", email)
    .execute(pool)
    .await;

  match result {
    Ok(_) => Ok(email),
    Err(err) => {
      eprintln!("Error setting a user email confirmation to true: {err}");
      Err(ErrPayload::new(StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong.."))
    }
  }
}
