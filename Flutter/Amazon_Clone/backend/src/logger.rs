use chrono::Local;
use simplelog::{CombinedLogger, ConfigBuilder, LevelFilter, WriteLogger};
use std::fs::OpenOptions;

pub async fn logger_run() {
  let log_dir = "logs";
  std::fs::create_dir_all(log_dir).expect("Failed to create log directory");

  let log_path = format!("{}/{}.log", log_dir, Local::now().format("%Y-%m-%d"));

  let log_file = OpenOptions::new()
    .create(true)
    .append(true)
    .open(&log_path)
    .expect("Failed to open or create log file");

  let logger_config = ConfigBuilder::new()
    .set_time_format_str("%Y-%m-%d %H:%M:%S")
    .build();

  let file_logger = WriteLogger::new(LevelFilter::Info, logger_config, log_file);

  CombinedLogger::init(vec![file_logger]).expect("Failed to initialize logger");
}
