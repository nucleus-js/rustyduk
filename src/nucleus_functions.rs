// stdlib imports
use std::process;

// crate imports
extern crate libc;
use libc::c_int;

// use internals
use duk_structs::duk_context;

extern {
    fn duk_require_int(ctx: *mut duk_context, index: c_int) -> c_int;
}

pub extern fn exit(ctx: *mut duk_context) -> i32 {
    let code: i32;
    unsafe {
        code = duk_require_int(ctx, 0)
    }
    process::exit(code);
}
