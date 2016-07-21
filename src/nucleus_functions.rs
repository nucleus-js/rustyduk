// stdlib imports
use std::{env, process};

// use internals
use duk_structs::duk_context;
use duk_api as duk;
//

pub extern fn exit(ctx: *mut duk_context) -> i32 {
    let code = duk::require_int(ctx, 0);
    process::exit(code);
}

pub extern fn env_keys(ctx: *mut duk_context) -> i32 {
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
