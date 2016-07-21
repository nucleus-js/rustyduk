// stdlib imports
use std::{env, fs};
use std::ffi::CString;

// crate imports
extern crate libc;
use libc::{c_char, c_int};

extern crate getopts;
use getopts::Options;
//

// declare internal modules
mod nucleus;
mod duk_structs;
mod duk_api;
mod nucleus_functions;
//

// use internals
use nucleus::duk_put_nucleus;
use duk_structs::duk_context;
//

extern {
    fn _duk_create_heap_default() -> *mut duk_context;
    fn _duk_peval_file(ctx: *mut duk_context, path: *const c_char) -> c_int;
    fn duk_destroy_heap(ctx: *mut duk_context);
}


fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    // setup, args gathering
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    // process options
    let mut opts = Options::new();
    opts.optopt("o", "output", "set output file name", "FILE");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    // --help
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    // TODO(Fishrock123): Uncomment when we start working on bundling.
    // let output = matches.opt_str("o");

    // if there is no free argument (such as a filename or folder),
    // print the help
    let input = if matches.free.is_empty() {
        print_usage(&program, opts);
        return;
    } else {
        // otherwise make a copy of the arument so we can use it
        matches.free[0].clone()
    };

    // Get the realpath
    let js_path = match fs::canonicalize(input) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    // Convert the path::Path into a String we can pass to C
    let c_js_path = CString::new(js_path.to_str().unwrap()).unwrap();

    // duktape setup
    let ctx: *mut duk_context;
    unsafe {
        ctx = _duk_create_heap_default();
    }

    // nucleus JS setup
    duk_put_nucleus(ctx, args);

    unsafe {
        _duk_peval_file(ctx, c_js_path.as_ptr());
        duk_destroy_heap(ctx);
    }
}
