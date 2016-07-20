// stdlib imports
use std::ffi::CString;

// crate imports
extern crate libc;
use libc::{c_char, c_int};

// use internals
use duk_structs::duk_context;
//

extern {
    fn duk_push_object(ctx: *mut duk_context) -> c_int;
    fn duk_push_string(ctx: *mut duk_context, string: *const c_char) -> c_char;
    fn duk_put_prop_string(ctx: *mut duk_context, index: c_int, key: *const c_char) -> bool;
    fn duk_put_global_string(ctx: *mut duk_context, key: *const c_char) -> bool;
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

    // // nucleus.args
    // duk_push_array(ctx);
    // for (int i = argstart; i < argc; i++) {
    //   duk_push_string(ctx, argv[i]);
    //   duk_put_prop_index(ctx, -2, i - argstart);
    // }
    // duk_put_prop_string(ctx, -2, "args");
    //
    // // nucleus.rawArgs
    // duk_push_array(ctx);
    // for (int i = 0; i < argc; i++) {
    //   duk_push_string(ctx, argv[i]);
    //   duk_put_prop_index(ctx, -2, i);
    // }
    // duk_put_prop_string(ctx, -2, "rawArgs");
    //
    // // nucleus.engine
    // duk_push_string(ctx, "duktape");
    // duk_put_prop_string(ctx, -2, "engine");
    //
    // // nucleus.versions
    // duk_push_object(ctx);
    // #ifdef DUK_VERSION
    // duk_push_string(ctx, "v");
    // duk_push_int(ctx, DUK_VERSION / 10000);
    // duk_push_string(ctx, ".");
    // duk_push_int(ctx, (DUK_VERSION / 100) % 100);
    // duk_push_string(ctx, ".");
    // duk_push_int(ctx, DUK_VERSION % 100);
    // duk_concat(ctx, 6);
    // duk_put_prop_string(ctx, -2, "duktape");
    // #endif
    // #ifdef MZ_VERSION
    // duk_push_string(ctx, "v");
    // duk_push_string(ctx, MZ_VERSION);
    // duk_put_prop_string(ctx, -2, "miniz");
    // #endif
    // duk_put_prop_string(ctx, -2, "versions");
    //
    // duk_put_function_list(ctx, -1, nucleus_functions);
    //
    // // nucleus.uv
    // duv_push_module(ctx);
    // duk_put_prop_string(ctx, -2, "uv");

    let nucleus = CString::new("nucleus").unwrap();
    unsafe {
        duk_put_global_string(ctx, nucleus.as_ptr());
    }
}
