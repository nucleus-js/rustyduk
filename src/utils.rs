// stdlib imports
use std::ffi::CStr;
use std::str;

// crate imports
extern crate libc;
use libc::c_char;

pub fn string_from_c_pointer(external_str: *const c_char) -> String {
    let c_str: &CStr = unsafe { CStr::from_ptr(external_str) };
    let buf: &[u8] = c_str.to_bytes();
    let str_slice: &str = str::from_utf8(buf).unwrap();
    str_slice.to_owned()
}
