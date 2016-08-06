// stdlib imports
use std::{env, fs, process};
use std::ffi::CString;

// crate imports
extern crate libc;
use libc::{c_char, c_int};

extern crate getopts;
use getopts::Options;
//

// declare internal modules
mod bundler;
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

extern "C" {
    fn _duk_create_heap_default() -> *mut duk_context;
    fn _duk_peval(ctx: *mut duk_context) -> c_int;
    fn _duk_peval_file(ctx: *mut duk_context, path: *const c_char) -> c_int;
    fn duk_destroy_heap(ctx: *mut duk_context);
    fn _duk_dump_context_stderr(ctx: *mut duk_context);
}


fn print_version() {
    println!("rustyduk v0.0.0 implementing Nucleus v0.0.0");
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
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("v", "version", "print the Nucleus version");
    opts.optopt("o",
                "output",
                "create a bundle with embedded nucleus at the specified file",
                "FILE");
    opts.optflag("z", "zip-only", "create zip bundle without embedding");
    opts.optflag("N", "no-bundle", "do not execute as a bundle");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
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
    // Init resource to check if we are a zip or not, and decide
    // the base path.
    //
    // The base path is either the executable or a directory we point at.
    let base_path = match resource::init() {
        true => {
            // executable is a zip
            fs::canonicalize(&program).unwrap().to_str().unwrap().to_owned()
        }
        false => {
            // if there is no free argument (such as a filename or folder),
            // print the help
            let input = if matches.free.is_empty() {
                print_usage(&program, opts);
                return;
            } else {
                // otherwise make a copy of the argument so we can use it
                matches.free[0].clone()
            };

            let path = fs::canonicalize(input).unwrap();
            if matches.opt_present("N") {
                path.parent().unwrap().to_str().unwrap().to_owned()
            } else {
                path.to_str().unwrap().to_owned()
            }
        }
    };

    // Hard to do this all in resource, so just set it from here.
    resource::set_base(&base_path);

    // Look for an output file.
    // This means we are producing some kind of bundle rather than running a program.
    if matches.opt_present("o") {
        let output = match matches.opt_str("o") {
            Some(s) => s,
            None => {
                print!("Error: the option -o, --output requires an argument!");
                process::exit(1);
            }
        };

        // let build_type: BuildType;
        // if matches.opt_present("z") {
        //     build_type = BuildType::ZIP;
        // }

        bundler::build_zip(base_path, output, matches.opt_present("z"));
        return;
    }

    // duktape setup
    let ctx: *mut duk_context;
    unsafe {
        ctx = _duk_create_heap_default();
    }

    // nucleus JS setup
    duk_put_nucleus(ctx, args);

    // evaluating some prgram. Prepare to catch an error.
    let err: i32;

    // --no-bundle
    if matches.opt_present("N") {
        // Convert the path into a String we can pass to C
        let wanted = matches.free[0].clone().to_owned();
        let filepath = fs::canonicalize(wanted).unwrap().to_str().unwrap().to_owned();
        let c_base_path = CString::new(filepath).unwrap();

        unsafe {
            err = _duk_peval_file(ctx, c_base_path.as_ptr());
        }

    } else {
        let entry_file: String = "main.js".to_owned();

        // TODO(Fishrock123): support renaming of main.js

        // If there are free arguments (i.e. not an option or past `--`)
        // if !matches.free.is_empty() {
        //     let possible_entry = matches.free[0].clone();

        //     // If we we passed a .js file,
        //     if Path::new(&possible_entry).ends_with(".js") {
        //         entry_file = possible_entry;
        //     } else {
        resource::check_set_zip(&base_path);
        //     }
        // }

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
