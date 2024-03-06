use crate::utilities::{
  auth::hash::hash_password,
  payload::{ErrPayload, Payload},
};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};

#[derive(Debug, FromRow, Serialize, Deserialize, Default)]
pub struct User {
  pub email: String,
  pub password: String,
  pub name: String,
  pub address: String,
  pub is_admin: bool,
}

impl User {
  pub async fn find_by_email(db_pool: &sqlx::PgPool, email: &str) -> Result<User, ErrPayload> {
    match sqlx::query_as!(
      User,
      "SELECT email, password, name, address, is_admin FROM users WHERE email = $1",
      email
    )
    .fetch_one(db_pool)
    .await
    {
      Ok(user) => Ok(user),
      Err(err) => {
        if let sqlx::Error::RowNotFound = err {
          return Err(Payload::err(StatusCode::NOT_FOUND, "User not found."));
        }

        return Err(Payload::sww_err(err));
      }
    }
  }
  pub async fn save(pool: &PgPool, user: &User) -> Result<User, ErrPayloadResponse> {
    let result = sqlx::query_as!(
      User,
      "INSERT INTO users (email, password, name, address, is_admin) VALUES ($1, $2, $3, $4, $5)
         RETURNING email, password, name, address, is_admin",
      user.email,
      hash_password(&user.password).await?,
      user.name,
      user.address,
      user.is_admin,
    )
    .fetch_one(pool)
    .await;

    match result {
      Ok(user) => Ok(user),
      Err(err) => {
        if let sqlx::Error::Database(ref e) = err {
          if e.constraint() == Some("your_unique_constraint_here") {
            return Err(Payload::err(StatusCode::CONFLICT, "This email is already taken."));
          }
        }
        Err(Payload::sww_err(&err))
      }
    }
  }

  pub async fn verify_password(&self, password: &str) -> Result<(), ErrPayload> {
    match bcrypt::verify(password, &self.password) {
      Ok(true) => Ok(()),
      Ok(false) => Err(Payload::err(
        StatusCode::UNAUTHORIZED,
        "Invalid email or password.",
      )),
      Err(err) => Err(Payload::sww_err(err)),
    }
  }
}
