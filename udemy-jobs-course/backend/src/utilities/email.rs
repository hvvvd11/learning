use crate::queries::email_verification_codes::{save_email_verification_code_model, EmailVerificationCode};
use axum::http::StatusCode;
use dotenvy_macro::dotenv;
use lettre::{
  message::{header, MultiPart, SinglePart},
  transport::smtp::authentication::Credentials,
  Message, SmtpTransport, Transport,
};
use maud::{html, PreEscaped};
use rand::{distributions::Alphanumeric, Rng};
use sqlx::PgPool;

use super::payload::ErrPayload;

pub fn create_an_email_confirmation_code() -> i32 {
  let mut rng = rand::thread_rng();
  let random_number: i32 = rng.gen_range(100_000..=999_999);

  return random_number;
}

pub fn create_a_tmp_auth_token_for_an_authorized_users() -> String {
  return rand::thread_rng().sample_iter(&Alphanumeric).take(32).map(char::from).collect();
}

pub async fn build_html_email(recipient: String, html: PreEscaped<String>) -> Result<Message, ErrPayload> {
  let email = Message::builder()
    .from("NoBody <nobody@domain.tld>".parse().unwrap())
    .to(
      recipient
        .parse()
        .map_err(|_| ErrPayload::new(StatusCode::UNAUTHORIZED, "invalid email recipient"))?,
    )
    .subject("Email verification")
    .multipart(
      MultiPart::alternative() // This is composed of two parts.
        .singlepart(
          SinglePart::builder()
            .header(header::ContentType::TEXT_HTML)
            .body(html.into_string()),
        ),
    )
    .map_err(|err| {
      eprintln!("Error building email: {:?}", err);
      ErrPayload::new(StatusCode::INTERNAL_SERVER_ERROR, "failed building email")
    });

  email
}

pub async fn send_email_confirmation_code(user_email: String, confirmation_code: i32) -> Result<(), ErrPayload> {
  let html = html! {
      head {
          title { "Email confirmation" }
          style type="text/css" {
              "h2, h4 { font-family: Arial, Helvetica, sans-serif; }"
          }
      }
      div style="display: flex; flex-direction: column; align-items: center;" {
          p { "Confirmation code is - " }
          p { (confirmation_code) }
      }
  };

  let email = build_html_email(user_email, html).await?;

  let creds = Credentials::new(
    dotenv!("MAILING_SERVICE_USERNAME").to_owned(),
    dotenv!("MAILING_SERVICE_PASSWORD").to_owned(),
  );

  let mailer = SmtpTransport::starttls_relay("smtp.gmail.com").unwrap().credentials(creds).build();

  match mailer.send(&email) {
    Ok(_) => Ok(()),
    Err(err) => {
      eprintln!("failed sending email: {}", err);
      Err(ErrPayload::new(StatusCode::INTERNAL_SERVER_ERROR, "Failed sending email"))
    }
  }
}

pub async fn send_and_create_email_confirmation_code(db_pool: &PgPool, email: String, user_id: i32) -> Result<String, ErrPayload> {
  let confirmation_code = create_an_email_confirmation_code();

  let email_verification_code_model = EmailVerificationCode {
    login_tmp_token: create_a_tmp_auth_token_for_an_authorized_users(),
    verification_code: confirmation_code,
    user_id,
    ..Default::default()
  };

  let email_verification_code_model = save_email_verification_code_model(&db_pool, &email_verification_code_model).await?;

  match send_email_confirmation_code(email, confirmation_code).await {
    Ok(_) => Ok(email_verification_code_model.login_tmp_token),
    Err(_) => Err(ErrPayload::new(StatusCode::NOT_ACCEPTABLE, "This email can not be proceeded")),
  }
}
