use axum::http::StatusCode;
use bcrypt::{hash, verify};

use crate::utilities::payload::ErrPayload;

const COST: u32 = 12;

pub fn hash_password(password: &str) -> Result<String, ErrPayload> {
  match hash(password, COST) {
    Ok(hash_result) => Ok(hash_result),
    Err(err) => Err({
      eprint!("Error hashing password: {:?}", err);
      ErrPayload::new(StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong..")
    }),
  }
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, ErrPayload> {
  match verify(password, hash) {
    Ok(true) => Ok(true),
    Ok(false) => Ok(false),
    Err(err) => Err({
      eprint!("Error verifying password: {:?}", err);
      ErrPayload::new(StatusCode::UNAUTHORIZED, "Something went wrong..")
    }),
  }
}
