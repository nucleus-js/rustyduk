
// crate imports
extern crate libc;
use libc::c_long;

// use internals
use duk_structs::duk_context;
use duk_api as duk;
use nucleus_functions;
//

extern {
    static _DUK_VERSION: c_long;
}

pub fn duk_put_nucleus(ctx: *mut duk_context, args: Vec<String>) {
    // nucleus
    duk::push_object(ctx);

    // TODO(Fishrock123): Not sure what this is supposed to be.
    //
    // // nucleus.base
    // duk_push_string(ctx, base);
    // duk_put_prop_string(ctx, -2, "base");

    // nucleus.cmd
    duk::push_string(ctx, args[0].clone());
    duk::put_prop_string(ctx, -2, "cmd");

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
    duk::push_array(ctx);
    for (index, arg) in args.iter().enumerate() {
        duk::push_string(ctx, arg.as_str());
        duk::put_prop_index(ctx, -2, index as u32);
    }
    duk::put_prop_string(ctx, -2, "rawArgs");

    // nucleus.engine
    duk::push_string(ctx, "duktape");
    duk::put_prop_string(ctx, -2, "engine");

    // nucleus.versions
    duk::push_object(ctx);
    // #ifdef DUK_VERSION
    duk::push_string(ctx, "v");
    duk::push_int(ctx, (_DUK_VERSION / 10000) as i32);
    duk::push_string(ctx, ".");
    duk::push_int(ctx, ((_DUK_VERSION / 100) % 100) as i32);
    duk::push_string(ctx, ".");
    duk::push_int(ctx, (_DUK_VERSION % 100) as i32);
    duk::concat(ctx, 6);
    duk::put_prop_string(ctx, -2, "duktape");
    // #endif
    // TODO(Fishrock123): minz should not be necessary with Rust
    // #ifdef MZ_VERSION
    // duk_push_string(ctx, "v");
    // duk_push_string(ctx, MZ_VERSION);
    // duk_put_prop_string(ctx, -2, "miniz");
    // #endif
    duk::put_prop_string(ctx, -2, "versions");

    duk::push_c_function(ctx, nucleus_functions::exit, 1);
    duk::put_prop_string(ctx, -2, "exit");

    // TODO: No linkings to libuv yet
    // // nucleus.uv
    // duv_push_module(ctx);
    // duk_put_prop_string(ctx, -2, "uv");

    duk::put_global_string(ctx, "nucleus");
}
