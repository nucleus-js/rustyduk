// stdlib imports
use std::ffi::CString;

// crate imports
extern crate libc;
use libc::{c_char, c_int, c_long, c_uint};

// use internals
use duk_structs::duk_context;
//

extern {
    fn duk_concat(ctx: *mut duk_context, count: c_int);
    fn duk_push_array(ctx: *mut duk_context) -> c_int;
    fn duk_push_int(ctx: *mut duk_context, val: c_int);
    fn duk_push_object(ctx: *mut duk_context) -> c_int;
    fn duk_push_string(ctx: *mut duk_context, string: *const c_char) -> c_char;
    fn duk_put_prop_index(ctx: *mut duk_context, obj_index: c_int, arr_index: c_uint) -> bool;
    fn duk_put_prop_string(ctx: *mut duk_context, obj_index: c_int, key: *const c_char) -> bool;
    fn duk_put_global_string(ctx: *mut duk_context, key: *const c_char) -> bool;

    static _DUK_VERSION: c_long;
}

pub fn duk_put_nucleus(ctx: *mut duk_context, args: Vec<String>) {
    // nucleus
    unsafe {
        duk_push_object(ctx);
    }

    // TODO(Fishrock123): Not sure what this is supposed to be.
    //
    // // nucleus.base
    // duk_push_string(ctx, base);
    // duk_put_prop_string(ctx, -2, "base");

    // nucleus.cmd
    let argv_0 = CString::new(args[0].clone()).unwrap();
    let cmd = CString::new("cmd").unwrap();
    unsafe {
        duk_push_string(ctx, argv_0.as_ptr());
        duk_put_prop_string(ctx, -2, cmd.as_ptr());
    }

    // nucleus.args
    // unsafe {
    //     duk_push_array(ctx);
    // }
    // for (index, arg) in args.iter().enumerate() {
    //     let c_arg = CString::new(arg).unwrap();
    //     unsafe {
    //         duk_push_string(ctx, arg);
    //         duk_put_prop_index(ctx, -2, i - argstart);
    //     }
    // }
    // unsafe {
    //     duk_put_prop_string(ctx, -2, "args");
    // }

    // nucleus.rawArgs
    unsafe {
        duk_push_array(ctx);
    }
    for (index, arg) in args.iter().enumerate() {
        let c_arg = CString::new(arg.as_str()).unwrap();
        unsafe {
            duk_push_string(ctx, c_arg.as_ptr());
            duk_put_prop_index(ctx, -2, index as u32);
        }
    }
    let rawArgs = CString::new("rawArgs").unwrap();
    unsafe {
        duk_put_prop_string(ctx, -2, rawArgs.as_ptr());
    }

    // nucleus.engine
    let duktape = CString::new("duktape").unwrap();
    let engine = CString::new("engine").unwrap();
    unsafe {
        duk_push_string(ctx, duktape.as_ptr());
        duk_put_prop_string(ctx, -2, engine.as_ptr());
    }

    // nucleus.versions
    unsafe {
        duk_push_object(ctx);
    }
    // #ifdef DUK_VERSION
    let v = CString::new("v").unwrap();
    let point = CString::new(".").unwrap();
    unsafe {
        duk_push_string(ctx, v.as_ptr());
        duk_push_int(ctx, (_DUK_VERSION / 10000) as i32);
        duk_push_string(ctx, point.as_ptr());
        duk_push_int(ctx, ((_DUK_VERSION / 100) % 100) as i32);
        duk_push_string(ctx, point.as_ptr());
        duk_push_int(ctx, (_DUK_VERSION % 100) as i32);
        duk_concat(ctx, 6);
        duk_put_prop_string(ctx, -2, duktape.as_ptr());
    }
    // #endif

    // TODO(Fishrock123): minz should not be necessary with Rust
    // #ifdef MZ_VERSION
    // duk_push_string(ctx, "v");
    // duk_push_string(ctx, MZ_VERSION);
    // duk_put_prop_string(ctx, -2, "miniz");
    // #endif
    let versions = CString::new("versions").unwrap();
    unsafe {
        duk_put_prop_string(ctx, -2, versions.as_ptr());
    }

    // duk_put_function_list(ctx, -1, nucleus_functions);

    // TODO: No linkings to libuv yet
    // // nucleus.uv
    // duv_push_module(ctx);
    // duk_put_prop_string(ctx, -2, "uv");

    let nucleus = CString::new("nucleus").unwrap();
    unsafe {
        duk_put_global_string(ctx, nucleus.as_ptr());
    }
}
