// stdlib imports
use std::env;
use std::fs::{self, File};
use std::io::{BufReader, ErrorKind, Read};
use std::path::Path;
use std::mem;
//


// crate imports
extern crate libc;
use libc::c_int;

extern crate zip;
//

// use internals
use duk_structs::duk_context;
use duk_api as duk;
//

extern "C" {
    static _DUK_ERR_ERROR: c_int;
}

static mut initialized: bool = false;
static mut is_zip: bool = false;
static mut base_path: &'static str = "/";
static mut path_set: bool = false;

pub fn init() -> bool {
    unsafe {
        if initialized {
            panic!("resource was already initialized!");
        }
        initialized = true;
    }
    check_set_zip(&env::current_exe().unwrap().to_str().unwrap())
}

pub fn check_set_zip(path: &str) -> bool {
    let _is_zip = check_nucleus_is_zip(path);
    unsafe {
        is_zip = _is_zip;
    }
    _is_zip
}

pub fn set_base(_base_path: &str) {
    unsafe {
        if path_set {
            panic!("base_path was already set!");
        }
        path_set = true;

        base_path = mem::transmute(_base_path);
    }
}

fn check_nucleus_is_zip(bundle_path: &str) -> bool {
    let bundle_file = match File::open(&bundle_path) {
        Ok(f) => f,
        Err(_) => return false,
    };

    let reader = BufReader::new(bundle_file);
    match zip::ZipArchive::new(reader) {
        Ok(_) => {
            return true;
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
    let bundle_path = unsafe { base_path.to_string() };
    let filename = duk::require_string(ctx, 0);
    // canonicalize(ctx);

    let bundle_file = File::open(&bundle_path).unwrap();

    let reader = BufReader::new(bundle_file);
    let mut zip_archive = zip::ZipArchive::new(reader).unwrap();
    let len = zip_archive.len();

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
            println!("Failed to find bundled file {} - {:?}", filename, err);
            println!("Number of bundle files: {}", len);

            let _bundle_file = File::open(&bundle_path).unwrap();

            let _reader = BufReader::new(_bundle_file);
            let mut _zip_archive = zip::ZipArchive::new(_reader).unwrap();
            for index in 0..len {
                println!("File at index {}: {}",
                         index,
                         _zip_archive.by_index(index).unwrap().name());
            }

            let args = format!("Failed to find bundled file {} - {:?}", filename, err);
            duk::error(ctx, _DUK_ERR_ERROR, args);
            return 0;
        }
    }

    1
}

fn read_from_disk(ctx: *mut duk_context) -> i32 {
    let filename = duk::require_string(ctx, 0);

    let base = unsafe { base_path.to_string() };
    let path = Path::new(&base).join(&filename);

    let real_path = match fs::canonicalize(path) {
        Ok(m) => m,
        Err(err) => {
            match err.kind() {
                ErrorKind::NotFound => {
                    duk::push_null(ctx);
                    return 1;
                }
                _ => {
                    let args = format!("Failed to canonicalize {} - {:?}", filename, err.kind());
                    duk::error(ctx, _DUK_ERR_ERROR, args);
                    return 0;
                }
            }
        }
    };

    let mut file = match File::open(real_path) {
        Ok(m) => m,
        Err(err) => {
            match err.kind() {
                _ => {
                    let args = format!("Failed to open {} - {:?}", filename, err.kind());
                    duk::error(ctx, _DUK_ERR_ERROR, args);
                    return 0;
                }
            }
        }
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
