use backend::app_state::AppState;
use backend::run;
use backend::utilities::auth::token_wrapper::TokenWrapper;
use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
  //DB
  dotenv().ok();
  let database_url = dotenv!("DATABASE_URL").to_owned();
  let jwt_secret = dotenv!("JWT_SECRET").to_owned();

  let db_pool = PgPoolOptions::new()
    .max_connections(10)
    .connect(&database_url)
    .await
    .expect("Failed to connect to DB ABORTING");

  let app_state = AppState {
    db_pool,
    jwt_secret: TokenWrapper(jwt_secret),
  };

  run(app_state).await;
}
