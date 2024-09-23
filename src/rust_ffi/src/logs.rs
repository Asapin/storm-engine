use std::fs::File;

use chrono::Utc;
use simplelog::{CombinedLogger, Config, ConfigBuilder, SharedLogger, WriteLogger};

pub fn init() {
    let config = ConfigBuilder::default().set_time_format_rfc3339().build();
    CombinedLogger::init(vec![file_logger(config)]).expect("Couldn't initialize logger");
    log::trace!("Trace level log...");
    log::debug!("Debug level log...");
    log::info!("Info level log...");
    log::warn!("Warn level log...");
    log::error!("Error level log...");
}

fn file_logger(config: Config) -> Box<dyn SharedLogger> {
    let current_date = Utc::now().format("%Y_%m_%d_%H_%M_%S").to_string();
    let log_file_name = format!("{}.log", current_date);
    WriteLogger::new(
        log::LevelFilter::Debug,
        config,
        File::create(log_file_name).expect("Couldn't create log file"),
    )
}
