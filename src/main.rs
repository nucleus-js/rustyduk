// stdlib imports
use std::{env, fs, process};
use std::ffi::CString;
use std::path::Path;

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
mod resource;
mod utils;
//

// use internals
use nucleus::duk_put_nucleus;
use duk_structs::duk_context;
use duk_api as duk;
//

extern {
    fn _duk_create_heap_default() -> *mut duk_context;
    fn _duk_peval(ctx: *mut duk_context) -> c_int;
    fn _duk_peval_file(ctx: *mut duk_context, path: *const c_char) -> c_int;
    fn duk_destroy_heap(ctx: *mut duk_context);
    fn _duk_dump_context_stderr(ctx: *mut duk_context);
}


fn print_version() {
    println!("rustyduk v0.0.0 implmenting Nucleus v0.0.0");
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
    opts.optflag("v", "version", "print the Nucleus version");
    opts.optflag("N", "no-bundle", "do not execute as a bundle");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    // --help
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    // --version
    if matches.opt_present("v") {
        print_version();
        return;
    }

    // At this point we know we'll do something.
    // Init resource to check if we are a zip or not.
    resource::init();

    // TODO(Fishrock123): Uncomment when we start working on bundling.
    // let output = matches.opt_str("o");

    // duktape setup
    let ctx: *mut duk_context;
    unsafe {
        ctx = _duk_create_heap_default();
    }

    // nucleus JS setup
    duk_put_nucleus(ctx, args);

    // eval the file and store a potential error indicator
    let err: i32;
    if matches.opt_present("N") {
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

        unsafe {
            err = _duk_peval_file(ctx, c_js_path.as_ptr());
        }

    } else {
        let mut entry_file = "main.js".to_owned();

        if !matches.free.is_empty() {
            let possible_entry = matches.free[0].clone();
            if Path::new(&possible_entry).ends_with(".js") {
                entry_file = possible_entry;
            }
        }

        duk::push_string(ctx, "nucleus.dofile('");
        duk::push_lstring(ctx, entry_file);
        duk::push_string(ctx, "')");
        duk::concat(ctx, 3);

        unsafe {
            err = _duk_peval(ctx);
        }
    }

    // if we have an error, we need to print it & the stack
    if err > 0 {
        unsafe {
            // dumps some extra stack infomation
            _duk_dump_context_stderr(ctx);
        }
        duk::get_prop_string(ctx, -1, "stack");
        let err_str = duk::safe_to_string(ctx, -1);
        println!("Uncaught {}", err_str);
        process::exit(1);
    }

    // at this point the process is exiting normally
    unsafe {
        duk_destroy_heap(ctx);
    }
}
