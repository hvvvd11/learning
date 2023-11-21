use axum::http::StatusCode;
use chrono::Duration;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};

use crate::utilities::payload::ErrPayload;

#[derive(Serialize, Deserialize)]
struct Claims {
  exp: usize,
}

pub fn create_token(secret: &str) -> Result<String, ErrPayload> {
  // add at least an hour for this timestamp
  let now = chrono::Utc::now();
  let expires_at = Duration::hours(1);
  let expires_at = now + expires_at;
  let exp = expires_at.timestamp() as usize;
  let claims = Claims { exp };
  let token_header = Header::default();
  let key = EncodingKey::from_secret(secret.as_bytes());

  match encode(&token_header, &claims, &key) {
    Ok(result) => Ok(result),
    Err(err) => Err(ErrPayload::internal_server_error(err)),
  }
}

pub fn validate_token(secret: &str, token: &str) -> Result<bool, ErrPayload> {
  let key = DecodingKey::from_secret(secret.as_bytes());
  let validation = Validation::new(jsonwebtoken::Algorithm::HS256);
  decode::<Claims>(token, &key, &validation)
    .map_err(|error| match error.kind() {
      jsonwebtoken::errors::ErrorKind::InvalidToken
      | jsonwebtoken::errors::ErrorKind::InvalidSignature
      | jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
        ErrPayload::new(StatusCode::UNAUTHORIZED, "not authenticated!")
      }
      _ => {
        eprintln!("Error validating token: {:?}", error);
        ErrPayload::new(StatusCode::INTERNAL_SERVER_ERROR, "Error validating token")
      }
    })
    .map(|_claim| true)
}

pub fn create_refresh_token() -> String {
  return rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(64)
    .map(char::from)
    .collect();
}
