use backend::logger::logger_run;
use backend::run;

#[tokio::main]
async fn main() {
  println!("Server is running!");
  logger_run().await;

  run().await;
}
