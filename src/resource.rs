// stdlib imports
use std::env;
use std::fs::{File};
use std::io::{BufReader, ErrorKind, Read};
use std::process;
//


// crate imports
extern crate libc;
use libc::{c_int};

extern crate zip;
//

// use internals
use duk_structs::duk_context;
use duk_api as duk;
//

extern {
    static _DUK_ERR_ERROR: c_int;
}

static mut is_zip: bool = false;

static mut initialized: bool = false;

pub fn init() {
    unsafe {
        if initialized {
            panic!("Resource was already initialized!")
        }
        initialized = true;
    }

    let _is_zip = check_nucleus_is_zip();
    unsafe {
        is_zip = _is_zip;
    }
}

fn check_nucleus_is_zip() -> bool {
    let bundle_path = env::current_exe().unwrap();
    let bundle_file = match File::open(&bundle_path) {
        Ok(f) => { f }
        Err(err) => {
            println!("Error: could not open file or dir \"{:?}\" - {:?}", bundle_path, err.kind());
            process::exit(1);
        }
    };

    let reader = BufReader::new(bundle_file);
    match zip::ZipArchive::new(reader) {
        Ok(_) => {
            return true
        }
        Err(_) => {
            return false;
        }
    };
}

pub fn read(ctx: *mut duk_context) -> i32 {
    if unsafe { is_zip } {
        read_from_zip(ctx)
    } else {
        read_from_disk(ctx)
    }
}

// // Changes the first arg in place
// fn canonicalize(ctx: *mut duk_context) {
//   duk::require_string(ctx, 0);
//   duk::push_c_function(ctx, duv_path_join, DUK_VARARGS);
//   duk::dup(ctx, 0);
//   duk::call(ctx, 1);
//   duk::replace(ctx, 0);
// }
//
// // Changes the first arg in place
// fn resolve(ctx: *mut duk_context) {
//   duk_require_string(ctx, 0);
//   duk_push_c_function(ctx, duv_path_join, DUK_VARARGS);
//   duk_push_string(ctx, base);
//   duk_dup(ctx, 0);
//   duk_call(ctx, 2);
//   duk_replace(ctx, 0);
// }

fn read_from_zip(ctx: *mut duk_context) -> i32 {
    let bundle_path = env::current_exe().unwrap();
    let filename = duk::require_string(ctx, 0);
    // canonicalize(ctx);

    let bundle_file = File::open(&bundle_path).unwrap();

    let reader = BufReader::new(bundle_file);
    let mut zip_archive = zip::ZipArchive::new(reader).unwrap();

    let mut data = String::new();
    match zip_archive.by_name(&filename) {
        Ok(mut file) => {
            match file.read_to_string(&mut data) {
                Err(_) => {
                    duk::push_null(ctx);
                }
                _ => {
                    duk::push_lstring(ctx, data);
                }
            }
        }
        Err(err) => {
            let args = format!("Failed to find bundled file {} - {:?}", filename, err);
            duk::error(ctx, _DUK_ERR_ERROR, args);
            return 0;
        }
    }

    1
}

fn read_from_disk(ctx: *mut duk_context) -> i32 {
    let filename = duk::require_string(ctx, 0);

    let mut file = match File::open(&filename) {
        Ok(m) => { m }
        Err(err) => { match err.kind() {
            ErrorKind::NotFound => {
                duk::push_null(ctx);
                return 1;
            }
            _ => {
                let args = format!("Failed to open {} - {:?}", filename, err.kind());
                duk::error(ctx, _DUK_ERR_ERROR, args);
                return 0;
            }
        }}
    };

    let mut buf = String::new();
    match file.read_to_string(&mut buf) {
        Err(err) => {
            let args = format!("Failed to read {} - {:?}", filename, err.kind());
            duk::error(ctx, _DUK_ERR_ERROR, args);
            return 0;
        }
        _ => {}
    }

    duk::push_lstring(ctx, buf);
    1
}
