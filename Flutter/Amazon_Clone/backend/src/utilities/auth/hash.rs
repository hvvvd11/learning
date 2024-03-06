use axum::http::StatusCode;
use bcrypt::{hash, verify};

use crate::utilities::payload::{ErrPayloadResponse, Payload};

const COST: u32 = 4;

pub async fn hash_password(password: &str) -> Result<String, ErrPayloadResponse> {
  match hash(password, COST) {
    Ok(hash_result) => Ok(hash_result),
    Err(err) => Err(Payload::sww_err(err)),
  }
}

pub async fn verify_password(password: &str, hash: &str) -> Result<(), ErrPayloadResponse> {
  match verify(password, hash) {
    Ok(true) => Ok(()),
    Ok(false) => Err(Payload::err(
      StatusCode::UNAUTHORIZED,
      "Невірний логін або пароль",
    )),
    Err(err) => Err(Payload::sww_err(err)),
  }
}
