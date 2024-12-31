use std::{ffi::c_char, fs::File};

use chrono::Utc;
use simplelog::{CombinedLogger, Config, ConfigBuilder, SharedLogger, WriteLogger};

use crate::ffi_util;

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

#[no_mangle]
pub extern "C" fn trace(msg: *const c_char) {
    let msg = ffi_util::c_str_to_rust(msg);
    log::trace!("{}", msg)
}

#[no_mangle]
pub extern "C" fn debug(msg: *const c_char) {
    let msg = ffi_util::c_str_to_rust(msg);
    log::debug!("{}", msg)
}

#[no_mangle]
pub extern "C" fn info(msg: *const c_char) {
    let msg = ffi_util::c_str_to_rust(msg);
    log::info!("{}", msg)
}

#[no_mangle]
pub extern "C" fn warn(msg: *const c_char) {
    let msg = ffi_util::c_str_to_rust(msg);
    log::warn!("{}", msg)
}

#[no_mangle]
pub extern "C" fn error(msg: *const c_char) {
    let msg = ffi_util::c_str_to_rust(msg);
    log::error!("{}", msg)
}