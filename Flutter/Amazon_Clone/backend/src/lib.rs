use crate::{app_state::AppState, router::create_router};
use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use sqlx::postgres::PgPoolOptions;

mod app_state;
mod handlers;
pub mod logger;
pub mod queries;
mod router;
pub mod utilities;

pub async fn run() {
  dotenv().ok();
  let database_url = dotenv!("DATABASE_URL").to_owned();

  let db_pool = PgPoolOptions::new()
    .max_connections(10)
    .connect(&database_url)
    .await
    .expect("Failed to connect to DB ABORTING");

  let app_state = AppState { db_pool };

  let app = create_router(app_state);

  let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
  axum::serve(listener, app).await.unwrap();
}
