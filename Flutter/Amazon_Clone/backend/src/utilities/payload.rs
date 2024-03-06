use axum::{
  http::{HeaderMap, StatusCode},
  response::IntoResponse,
  response::Json,
  response::Response,
};
use log::error;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub type ErrPayload = Payload<()>;

#[derive(Deserialize, Serialize)]
pub struct Payload<T> {
  #[serde(skip)]
  pub status_code: StatusCode,
  pub error: bool,
  pub message: String,
  pub headers: Option<Vec<(String, String)>>,
  pub data: Option<T>,
}

impl<T> Payload<T> {
  pub fn ok(message: &str, data: Option<T>) -> Self {
    Self {
      status_code: StatusCode::OK,
      error: false,
      message: message.to_string(),
      headers: None,
      data,
    }
  }

  pub fn err(code: StatusCode, message: &str) -> Self {
    Self {
      status_code: code,
      error: true,
      message: message.to_string(),
      headers: None,
      data: None,
    }
  }

  pub fn sww_err<N: Debug>(err: N) -> Self {
    error!("{:?}", err);
    Self {
      status_code: StatusCode::INTERNAL_SERVER_ERROR,
      error: true,
      message: "Something went wrong.".to_string(),
      headers: None,
      data: None,
    }
  }
}

impl<T> IntoResponse for Payload<T> {
  fn into_response(self) -> axum::response::Response {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    Response::builder()
      .status(self.status_code)
      .body(Json(&self).into_response().into_body())
      .unwrap()
  }
}
