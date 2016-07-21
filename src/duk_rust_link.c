
// Silly duktape header linkings because Rust doesn't do it (yet?),
// and `bindgen` doesn't work good enough yet.

#include "../deps/duktape-releases/src/duktape.h"

duk_context *_duk_create_heap_default(void) {
  return duk_create_heap_default();
}

duk_int_t _duk_peval_file(duk_context *ctx, const char *path) {
  return duk_peval_file(ctx, path);
}

long _DUK_VERSION = DUK_VERSION;
