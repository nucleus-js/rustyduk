// build.rs

// Bring in a dependency on an externally maintained `gcc` package which manages
// invoking the C compiler.
extern crate gcc;

fn main() {
    gcc::compile_library("libduktape.a",
        &["deps/duktape-releases/src/duktape.c",
          "src/duk_rust_link.c"]);
}
