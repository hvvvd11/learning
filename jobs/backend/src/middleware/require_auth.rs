// use crate::utilities::auth::token_wrapper::TokenWrapper;
// use crate::utilities::{auth::jwt::validate_token, payload::ErrPayload};
// use axum::http::StatusCode;
// use axum::{
//   extract::State,
//   http::{HeaderMap, Request},
//   middleware::Next,
//   response::Response,
// };
// use sqlx::PgPool;
//
// FINNISH FUNCTION
// сейчас она поидее куда-то отправляет юзера
// надо разобраться, но пока я ее коменчу

// pub async fn require_authentication<T>(
//   State(db): State<PgPool>,
//   State(token_secret): State<TokenWrapper>,
//   headers: HeaderMap,
//   mut request: Request<T>,
//   next: Next<T>,
// ) -> Result<Response, ErrPayload> {
//   let header_token = if let Some(token) = headers.get("token") {
//     token.to_str().map_err(|error| {
//       eprintln!("Error extracting token from headers: {:?}", error);
//       ErrPayload::new(StatusCode::BAD_REQUEST, "Error reading token")
//     })?
//   } else {
//     return Err(ErrPayload::new(StatusCode::UNAUTHORIZED, "not authenticated!"));
//   };
//
//   validate_token(&token_secret.0, header_token)?;
//
//   let user = find_by_token(&db, header_token).await?;
//
//   request.extensions_mut().insert(user);
//
//   Ok(next.run(request).await)
// }
