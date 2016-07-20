// stdlib imports
use std::ffi::CString;

// crate imports
extern crate libc;
use libc::{c_char, c_int, c_uint};

// use internals
use duk_structs::{duk_context, duk_function_list_entry};
//

extern {
    fn duk_concat(ctx: *mut duk_context, count: c_int);
    fn duk_push_array(ctx: *mut duk_context) -> c_int;
    fn duk_push_int(ctx: *mut duk_context, val: c_int);
    fn duk_push_object(ctx: *mut duk_context) -> c_int;
    fn duk_push_string(ctx: *mut duk_context, string: *const c_char) -> c_char;
    fn duk_put_function_list(ctx: *mut duk_context, obj_index: c_int, funcs: *const duk_function_list_entry);
    fn duk_put_prop_index(ctx: *mut duk_context, obj_index: c_int, arr_index: c_uint) -> bool;
    fn duk_put_prop_string(ctx: *mut duk_context, obj_index: c_int, key: *const c_char) -> bool;
    fn duk_put_global_string(ctx: *mut duk_context, key: *const c_char) -> bool;
}

// duk_concat
pub fn concat(ctx: *mut duk_context, count: i32) {
    unsafe {
        duk_concat(ctx, count)
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

// duk_put_global_string
pub fn put_global_string<T: Into<Vec<u8>>>(ctx: *mut duk_context, key: T) -> bool {
    let cstring_key = CString::new(key).unwrap();
    unsafe {
        duk_put_global_string(ctx, cstring_key.as_ptr())
    }
}
