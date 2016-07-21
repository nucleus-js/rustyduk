// stdlib imports
use std::ffi::{CStr, CString};
use std::str;

// crate imports
extern crate libc;
use libc::{c_char, c_int, c_uint};

// use internals
use duk_structs::{duk_context, duk_function_list_entry};
//

/*
 * These lists should be kept Alphabetized!
 */

extern {
    fn duk_concat(ctx: *mut duk_context, count: c_int);
    fn duk_get_prop_string(ctx: *mut duk_context, obj_index: c_int, key: *const c_char) -> bool;
    fn duk_push_array(ctx: *mut duk_context) -> c_int;
    fn duk_push_int(ctx: *mut duk_context, val: c_int);
    fn duk_push_object(ctx: *mut duk_context) -> c_int;
    fn duk_push_string(ctx: *mut duk_context, string: *const c_char) -> c_char;
    fn duk_put_function_list(ctx: *mut duk_context, obj_index: c_int, funcs: *const duk_function_list_entry);
    fn duk_put_global_string(ctx: *mut duk_context, key: *const c_char) -> bool;
    fn duk_put_prop_index(ctx: *mut duk_context, obj_index: c_int, arr_index: c_uint) -> bool;
    fn duk_put_prop_string(ctx: *mut duk_context, obj_index: c_int, key: *const c_char) -> bool;
    fn duk_require_int(ctx: *mut duk_context, index: c_int) -> c_int;
    fn _duk_safe_to_string(ctx: *mut duk_context, index: c_int) -> *const c_char;
}

// duk_concat
pub fn concat(ctx: *mut duk_context, count: i32) {
    unsafe {
        duk_concat(ctx, count)
    }
}

// duk_get_prop_string
pub fn get_prop_string<T: Into<Vec<u8>>>(ctx: *mut duk_context, obj_index: c_int, key: T) -> bool {
    let cstring_key = CString::new(key).unwrap();
    unsafe {
        duk_get_prop_string(ctx, obj_index, cstring_key.as_ptr())
    }
}

// duk_push_array
pub fn push_array(ctx: *mut duk_context) -> i32 {
    unsafe {
        duk_push_array(ctx)
    }
}

// duk_push_int
pub fn push_int(ctx: *mut duk_context, val: i32) {
    unsafe {
        duk_push_int(ctx, val)
    }
}

// duk_push_object
pub fn push_object(ctx: *mut duk_context) -> i32 {
    unsafe {
        duk_push_object(ctx)
    }
}

// duk_push_string
pub fn push_string<T: Into<Vec<u8>>>(ctx: *mut duk_context, string: T) -> c_char {
    let cstring_string = CString::new(string).unwrap();
    unsafe {
        duk_push_string(ctx, cstring_string.as_ptr())
    }
}

// duk_put_function_list
pub fn put_function_list(ctx: *mut duk_context, obj_index: i32, funcs: *const duk_function_list_entry) {
    unsafe {
        duk_put_function_list(ctx, obj_index, funcs)
    }
}

// duk_put_global_string
pub fn put_global_string<T: Into<Vec<u8>>>(ctx: *mut duk_context, key: T) -> bool {
    let cstring_key = CString::new(key).unwrap();
    unsafe {
        duk_put_global_string(ctx, cstring_key.as_ptr())
    }
}

// duk_put_prop_index
pub fn put_prop_index(ctx: *mut duk_context, obj_index: i32, arr_index: u32) -> bool {
    unsafe {
        duk_put_prop_index(ctx, obj_index, arr_index)
    }
}

// duk_put_prop_string
pub fn put_prop_string<T: Into<Vec<u8>>>(ctx: *mut duk_context, obj_index: i32, key: T) -> bool {
    let cstring_key = CString::new(key).unwrap();
    unsafe {
        duk_put_prop_string(ctx, obj_index, cstring_key.as_ptr())
    }
}

// duk_require_int
pub fn require_int(ctx: *mut duk_context, index: i32) -> i32 {
    unsafe {
        duk_require_int(ctx, index)
    }
}

// duk_safe_to_string
pub fn safe_to_string(ctx: *mut duk_context, index: c_int) -> String {
    let external_str: *const c_char;
    unsafe {
        external_str = _duk_safe_to_string(ctx, index)
    }
    let c_str: &CStr = unsafe { CStr::from_ptr(external_str) };
    let buf: &[u8] = c_str.to_bytes();
    let str_slice: &str = str::from_utf8(buf).unwrap();
    str_slice.to_owned()
}
