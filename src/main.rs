// stdlib imports
use std::{env, fs};
use std::ffi::CString;

// module imports
extern crate libc;
use libc::{c_char, c_int};

extern crate getopts;
use getopts::Options;
//

// Intentionally has the same name as the C struct
#[allow(non_camel_case_types)]
enum duk_context {}

extern {
    fn rust_duk_create_heap_default() -> *mut duk_context;
    fn rust_duk_peval_file(ctx: *mut duk_context, path: *const c_char) -> c_int;
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

    // process arguments
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

    // let output = matches.opt_str("o");
    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };

    let js_path = match fs::canonicalize(input) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    let c_js_path = CString::new(js_path.to_str().unwrap()).unwrap();

    let context: *mut duk_context;
    unsafe {
        context = rust_duk_create_heap_default();
        rust_duk_peval_file(context, c_js_path.as_ptr());
        duk_destroy_heap(context);
    }
}
