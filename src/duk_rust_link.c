
// Silly duktape header linkings because Rust doesn't do it (yet?),
// and `bindgen` doesn't work good enough yet.

#include "../deps/duktape-releases/src/duktape.h"

duk_context *rust_duk_create_heap_default(void) {
  return duk_create_heap_default();
}

duk_int_t rust_duk_peval_file(duk_context *ctx, const char *path) {
  return duk_peval_file(ctx, path);
}

long _DUK_VERSION = DUK_VERSION;
