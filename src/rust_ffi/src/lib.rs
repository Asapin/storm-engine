use std::ffi::c_char;

mod logs;
mod util;

#[no_mangle]
pub extern "C" fn init() {
    logs::init();
}

#[no_mangle]
pub extern "C" fn trace(msg: *const c_char) {
    let msg = util::c_str_to_rust(msg);
    log::trace!("{}", msg)
}

#[no_mangle]
pub extern "C" fn debug(msg: *const c_char) {
    let msg = util::c_str_to_rust(msg);
    log::debug!("{}", msg)
}

#[no_mangle]
pub extern "C" fn info(msg: *const c_char) {
    let msg = util::c_str_to_rust(msg);
    log::info!("{}", msg)
}

#[no_mangle]
pub extern "C" fn warn(msg: *const c_char) {
    let msg = util::c_str_to_rust(msg);
    log::warn!("{}", msg)
}

#[no_mangle]
pub extern "C" fn error(msg: *const c_char) {
    let msg = util::c_str_to_rust(msg);
    log::error!("{}", msg)
}