// stdlib imports
use std::process;

// use internals
use duk_structs::duk_context;
use duk_api as duk;
//

pub extern fn exit(ctx: *mut duk_context) -> i32 {
    let code = duk::require_int(ctx, 0);
    process::exit(code);
}
