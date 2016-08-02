// Silly duktape header linkings because Rust doesn't do it (yet?),
// and `bindgen` doesn't work good enough yet.

#include "../deps/duktape-releases/src/duktape.h"

void _duk_compile(duk_context *ctx, duk_uint_t flags) {
  duk_compile(ctx, flags);
}

duk_context *_duk_create_heap_default(void) {
  return duk_create_heap_default();
}

void _duk_error(duk_context *ctx, duk_errcode_t err_code, const char *fmt) {
  duk_error(ctx, err_code, fmt);
}

void _duk_dump_context_stderr(duk_context *ctx) {
  return duk_dump_context_stderr(ctx);
}

duk_int_t _duk_peval(duk_context *ctx) {
  return duk_peval(ctx);
}

duk_int_t _duk_peval_file(duk_context *ctx, const char *path) {
  return duk_peval_file(ctx, path);
}

const char *_duk_safe_to_string(duk_context *ctx, duk_idx_t index) {
  return duk_safe_to_string(ctx, index);
}

long _DUK_VERSION = DUK_VERSION;

int _DUK_ERR_ERROR = DUK_ERR_ERROR;
