use std::fmt::Debug;

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Payload<T> {
  pub error: bool,
  pub message: String,
  pub data: Option<T>,
}

impl<T> Payload<T> {
  pub fn new(message: impl Into<String>, data: T) -> Json<Payload<T>> {
    Json(Payload {
      error: false,
      message: message.into(),
      data: Some(data),
    })
  }
}

#[derive(Debug)]
pub struct ErrPayload {
  code: StatusCode,
  message: String,
}

impl ErrPayload {
  pub fn new(code: StatusCode, message: impl Into<String>) -> Self {
    Self {
      code,
      message: message.into(),
    }
  }

  pub fn internal_server_error<T: Debug>(err: T) -> Self {
    eprintln!("Error: {:?}", err);
    Self {
      code: StatusCode::INTERNAL_SERVER_ERROR,
      message: "Something went wrong..".to_owned(),
    }
  }
}

impl IntoResponse for ErrPayload {
  fn into_response(self) -> axum::response::Response {
    (
      self.code,
      Json(ErrorResponse {
        error: true,
        message: self.message.clone(),
      }),
    )
      .into_response()
  }
}

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
  error: bool,
  message: String,
}
