// stdlib imports
use std::ffi::CString;

// crate imports
extern crate libc;
use libc::c_char;

// Intentionally has the same name as the C struct
#[allow(non_camel_case_types)]
pub enum duk_context {}

#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[repr(C)]
pub struct duk_function_list_entry {
	key: *const c_char,
	value: extern fn(*mut duk_context) -> i32,
	nargs: i32,
}
impl duk_function_list_entry {
    pub fn new(key: &str, value: extern fn(*mut duk_context) -> i32, nargs: i32) -> Self {
		let c_key = CString::new(key).unwrap();
        duk_function_list_entry { key: c_key.as_ptr(), value: value, nargs: nargs }
    }

	pub fn new_null(key: *const c_char, value: extern fn(*mut duk_context) -> i32, nargs: i32) -> Self {
        duk_function_list_entry { key: key, value: value, nargs: nargs }
    }
}
