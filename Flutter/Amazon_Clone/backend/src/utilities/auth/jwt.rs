use rand::{distributions::Alphanumeric, Rng};

pub fn create_token() -> String {
  return rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(64)
    .map(char::from)
    .collect();
}
