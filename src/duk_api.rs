// stdlib imports
use std::ffi::CString;

// crate imports
extern crate libc;
use libc::{c_char, c_int, c_uint};

// use internals
use duk_structs::duk_context;
use utils;
//

// These lists should be kept Alphabetized!

extern "C" {
    fn duk_call(ctx: *mut duk_context, nargs: c_int);
    fn _duk_compile(ctx: *mut duk_context, flags: c_uint);
    fn duk_concat(ctx: *mut duk_context, count: c_int);
    fn _duk_error(ctx: *mut duk_context, err_code: i32, fmt: *const c_char);
    fn duk_get_prop_string(ctx: *mut duk_context, obj_index: c_int, key: *const c_char) -> bool;
    fn duk_get_string(ctx: *mut duk_context, index: c_int) -> *const c_char;
    fn duk_is_string(ctx: *mut duk_context, index: c_int) -> bool;
    fn duk_push_array(ctx: *mut duk_context) -> c_int;
    fn duk_push_c_function(ctx: *mut duk_context,
                           func: extern "C" fn(*mut duk_context) -> i32,
                           nargs: c_int)
                           -> c_int;
    fn duk_push_int(ctx: *mut duk_context, val: c_int);
    fn duk_push_lstring(ctx: *mut duk_context, string: *const c_char, len: c_int) -> *const c_char;
    fn duk_push_object(ctx: *mut duk_context) -> c_int;
    fn duk_push_null(ctx: *mut duk_context);
    fn duk_push_string(ctx: *mut duk_context, string: *const c_char) -> c_char;
    fn duk_put_global_string(ctx: *mut duk_context, key: *const c_char) -> bool;
    fn duk_put_prop_index(ctx: *mut duk_context, obj_index: c_int, arr_index: c_uint) -> bool;
    fn duk_put_prop_string(ctx: *mut duk_context, obj_index: c_int, key: *const c_char) -> bool;
    fn duk_require_int(ctx: *mut duk_context, index: c_int) -> c_int;
    fn duk_require_string(ctx: *mut duk_context, index: c_int) -> *const c_char;
    fn _duk_safe_to_string(ctx: *mut duk_context, index: c_int) -> *const c_char;
}

// duk_call
pub fn call(ctx: *mut duk_context, nargs: c_int) {
    unsafe { duk_call(ctx, nargs) }
}

// duk_compile
pub fn compile(ctx: *mut duk_context, flags: c_uint) {
    unsafe { _duk_compile(ctx, flags) }
}

// duk_concat
pub fn concat(ctx: *mut duk_context, count: i32) {
    unsafe { duk_concat(ctx, count) }
}

// duk_error
pub fn error<T: Into<Vec<u8>>>(ctx: *mut duk_context, err_code: i32, fmt: T) {
    let cstring_fmt = CString::new(fmt).unwrap();
    unsafe { _duk_error(ctx, err_code, cstring_fmt.as_ptr()) }
}

// duk_get_prop_string
pub fn get_prop_string<T: Into<Vec<u8>>>(ctx: *mut duk_context, obj_index: c_int, key: T) -> bool {
    let cstring_key = CString::new(key).unwrap();
    unsafe { duk_get_prop_string(ctx, obj_index, cstring_key.as_ptr()) }
}

// duk_get_string
pub fn get_string(ctx: *mut duk_context, index: c_int) -> String {
    let external_str: *const c_char;
    unsafe {
        external_str = duk_get_string(ctx, index);
    }
    utils::string_from_c_pointer(external_str)
}

// duk_is_string
pub fn is_string(ctx: *mut duk_context, index: c_int) -> bool {
    unsafe { duk_is_string(ctx, index) }
}

// duk_push_array
pub fn push_array(ctx: *mut duk_context) -> i32 {
    unsafe { duk_push_array(ctx) }
}

// duk_push_c_function
pub fn push_c_function(ctx: *mut duk_context,
                       func: extern "C" fn(*mut duk_context) -> i32,
                       nargs: c_int)
                       -> c_int {
    unsafe { duk_push_c_function(ctx, func, nargs) }
}

// duk_push_int
pub fn push_int(ctx: *mut duk_context, val: i32) {
    unsafe { duk_push_int(ctx, val) }
}

// duk_push_lstring
pub fn push_lstring<T: Into<Vec<u8>>>(ctx: *mut duk_context, string: T) -> *const c_char {
    let cstring_string = CString::new(string).unwrap();
    unsafe {
        duk_push_lstring(ctx,
                         cstring_string.as_ptr(),
                         cstring_string.into_bytes().len() as i32)
    }
}

// duk_push_object
pub fn push_object(ctx: *mut duk_context) -> i32 {
    unsafe { duk_push_object(ctx) }
}

// duk_push_null
pub fn push_null(ctx: *mut duk_context) {
    unsafe { duk_push_null(ctx) }
}

// duk_push_string
pub fn push_string<T: Into<Vec<u8>>>(ctx: *mut duk_context, string: T) -> c_char {
    let cstring_string = CString::new(string).unwrap();
    unsafe { duk_push_string(ctx, cstring_string.as_ptr()) }
}

// duk_put_global_string
pub fn put_global_string<T: Into<Vec<u8>>>(ctx: *mut duk_context, key: T) -> bool {
    let cstring_key = CString::new(key).unwrap();
    unsafe { duk_put_global_string(ctx, cstring_key.as_ptr()) }
}

// duk_put_prop_index
pub fn put_prop_index(ctx: *mut duk_context, obj_index: i32, arr_index: u32) -> bool {
    unsafe { duk_put_prop_index(ctx, obj_index, arr_index) }
}

// duk_put_prop_string
pub fn put_prop_string<T: Into<Vec<u8>>>(ctx: *mut duk_context, obj_index: i32, key: T) -> bool {
    let cstring_key = CString::new(key).unwrap();
    unsafe { duk_put_prop_string(ctx, obj_index, cstring_key.as_ptr()) }
}

// duk_require_int
pub fn require_int(ctx: *mut duk_context, index: i32) -> i32 {
    unsafe { duk_require_int(ctx, index) }
}

// duk_require_string
pub fn require_string(ctx: *mut duk_context, index: c_int) -> String {
    let external_str: *const c_char;
    unsafe {
        external_str = duk_require_string(ctx, index);
    }
    utils::string_from_c_pointer(external_str)
}

// duk_safe_to_string
pub fn safe_to_string(ctx: *mut duk_context, index: c_int) -> String {
    let external_str: *const c_char;
    unsafe {
        external_str = _duk_safe_to_string(ctx, index);
    }
    utils::string_from_c_pointer(external_str)
}
