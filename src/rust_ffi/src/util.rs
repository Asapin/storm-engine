use std::ffi::{c_char, CStr};

pub fn c_str_to_rust(text: *const c_char) -> String {
    unsafe {
		assert!(!text.is_null());
		CStr::from_ptr(text)
	}.to_str().expect("Couldn't convert C chars to String").to_string()
}
