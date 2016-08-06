// stdlib imports
use std::{env, process};

// crate imports
extern crate libc;
use libc::c_int;

// use internals
use duk_structs::duk_context;
use duk_api as duk;
use resource;
//

extern "C" {
    static _DUK_ERR_ERROR: c_int;
}

pub extern "C" fn exit(ctx: *mut duk_context) -> i32 {
    let code = duk::require_int(ctx, 0);
    process::exit(code);
}

pub extern "C" fn env_keys(ctx: *mut duk_context) -> i32 {
    // XXX(Fishrock123): showHidden is not possible with Rust I think?

    duk::push_array(ctx);

    let mut index = 0;
    for (key, _) in env::vars() {
        duk::push_lstring(ctx, key);
        duk::put_prop_index(ctx, -2, index);
        index += 1;
    }
    1
}

pub extern "C" fn read_file(ctx: *mut duk_context) -> i32 {
    resource::read(ctx)
}

pub extern "C" fn do_file(ctx: *mut duk_context) -> i32 {
    let filename = duk::require_string(ctx, 0);

    resource::read(ctx);

    if !duk::is_string(ctx, -1) {
        duk::error(ctx, _DUK_ERR_ERROR, format!("No such file {}", filename));
        return 1;
    }

    compile(ctx, duk::get_string(ctx, -1), filename);
    duk::call(ctx, 0);
    1
}


fn compile(ctx: *mut duk_context, code: String, name: String) {
    duk::push_string(ctx, "(function(){");
    duk::push_string(ctx, code);
    duk::push_string(ctx, "})()");
    duk::concat(ctx, 3);
    duk::push_string(ctx, name);
    duk::compile(ctx, 0);
}
