use std::net::SocketAddr;

use crate::{
  handlers::auth::login::ResponseUser,
  queries::{
    email_verification_codes::{
      find_email_confirmation_code_model_by_login_tmp_token, save_email_verification_code_model,
    },
    tokens::save_token,
    users::{confirm_users_email, find_user_by_user_id},
  },
  utilities::{
    auth::{
      jwt::{create_refresh_token, create_token},
      token_wrapper::TokenWrapper,
    },
    email::{create_an_email_confirmation_code, send_email_confirmation_code},
    payload::{ErrPayload, Payload},
  },
};
use axum::{
  extract::{ConnectInfo, State},
  headers::UserAgent,
  http::{HeaderMap, StatusCode},
  response::{IntoResponse, Response},
  Json, TypedHeader,
};

use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize, Serialize)]
pub struct SendEmailRequest {
  login_tmp_token: String,
}

pub async fn send_email_confirmation_code_again(
  State(db_pool): State<PgPool>,
  Json(request_user): Json<SendEmailRequest>,
) -> Result<Json<Payload<()>>, ErrPayload> {
  let confirmation_code = create_an_email_confirmation_code();

  let mut email_verification_code_model =
    find_email_confirmation_code_model_by_login_tmp_token(&db_pool, request_user.login_tmp_token).await?;
  email_verification_code_model.verification_code = confirmation_code;

  save_email_verification_code_model(&db_pool, &email_verification_code_model).await?;
  let user_model = find_user_by_user_id(&db_pool, email_verification_code_model.user_id).await?;

  match send_email_confirmation_code(user_model.email, confirmation_code).await {
    Ok(_) => Ok(Payload::new("Code confirmation was sent, check your email", ())),
    Err(e) => Err(e),
  }
}

#[derive(Deserialize)]
pub struct ConfirmEmailRequest {
  login_tmp_token: String,
  verification_code: i32,
}

pub async fn confirm_email_verification_code(
  State(db_pool): State<PgPool>,
  State(jwt_secret): State<TokenWrapper>,
  TypedHeader(user_agent): TypedHeader<UserAgent>,
  ConnectInfo(ip_addr): ConnectInfo<SocketAddr>,
  Json(request_payload): Json<ConfirmEmailRequest>,
) -> Result<Response, ErrPayload> {
  let token_model =
    find_email_confirmation_code_model_by_login_tmp_token(&db_pool, request_payload.login_tmp_token).await?;
  let user_model = find_user_by_user_id(&db_pool, token_model.user_id).await?;

  if token_model.verification_code == request_payload.verification_code {
    if Utc::now().naive_utc() < token_model.expiration_time {
      let user = confirm_users_email(&db_pool, user_model.email).await?;

      let response_user = ResponseUser {
        username: user.username.clone(),
        email: user.email,
      };

      let token = create_token(&jwt_secret.0)?;
      let refresh_token = create_refresh_token();
      let tokens = save_token(
        &db_pool,
        user.user_id,
        token,
        user_agent.to_string(),
        ip_addr.ip().to_string(),
        refresh_token,
      )
      .await?;

      let mut headers = HeaderMap::new();

      headers.insert(
        "token",
        tokens
          .token
          .parse()
          .map_err(|err| ErrPayload::internal_server_error(err))
          .unwrap(),
      );
      headers.insert(
        "refresh-token",
        tokens
          .refresh_token
          .parse()
          .map_err(|err| ErrPayload::internal_server_error(err))
          .unwrap(),
      );

      Ok((headers, Payload::new("Logged in succesfully", response_user)).into_response())
    } else {
      Err(ErrPayload::new(StatusCode::UNAUTHORIZED, "Token has expired."))
    }
  } else {
    Err(ErrPayload::new(StatusCode::UNAUTHORIZED, "Token is invalid"))
  }
}
