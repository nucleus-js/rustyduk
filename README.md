# rustyduk

This is an implementation of the [nucleus-js design spec](https://github.com/nucleus-js/design) written in [Rust](https://www.rust-lang.org) and using the [duktape](http://duktape.org/) JavaScript engine.

The `rustc` compiler and the `cargo` package manager are necessary in order to build this project. The best way to install Rust on Unix is probably to use [rustup.rs](https://www.rustup.rs/).

**Note**: This repo also uses _[git submodules](https://chrisjean.com/git-submodules-adding-using-removing-and-updating/)_ to manage it's dependencies. As such, cloning this repo must be done with `git clone --depth=1 ...`.

## Building

```bash
cargo build
```

## Tests

```bash
./target/debug/rusty-duktape test.js
```

Currently, the test output should look like this. In the future, it will use the design repo's automated tests.

```
Hello World!
nucleus:  [object Object]
cmd:  ./target/debug/rusty-duktape
rawArgs:  ./target/debug/rusty-duktape,test.js
rawArgs[0]:  ./target/debug/rusty-duktape
rawArgs[1]:  test.js
engine:  duktape
versions:  [object Object]
versions.duktape:  v1.5.0
exit:  function () {"native"}
```

## License

rustyduk is licensed under [the Apache 2.0 License](LICENSE).
