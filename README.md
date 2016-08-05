# rustyduk

This is an implementation of the [nucleus-js design spec](https://github.com/nucleus-js/design) written in [Rust](https://www.rust-lang.org) and using the [duktape](http://duktape.org/) JavaScript engine.

The `rustc` compiler and the `cargo` package manager are necessary in order to build this project. The best way to install Rust on Unix is probably to use [rustup.rs](https://www.rustup.rs/).

**Note**: This repo also uses _[git submodules](https://chrisjean.com/git-submodules-adding-using-removing-and-updating/)_ to manage it's dependencies. As such, cloning this repo must be done with `git clone --depth=1 ...`.

## Building

```bash
make
```
or
```bash
cargo build
```

## Tests

Manual tests:

```bash
make test
```

Currently, each test should have output that looks similar to this.
In the future, it will use the design repo's automated tests.

```
Hello World!
nucleus:  [object Object]
cmd:  target/debug/nucleus
rawArgs:  target/debug/nucleus,--no-bundle,test.js,--,0,5,10
rawArgs[0]:  target/debug/nucleus
rawArgs[1]:  --no-bundle
rawArgs[2]:  test.js
rawArgs[3]:  --
rawArgs[4]:  0
rawArgs[5]:  5
rawArgs[6]:  10
engine:  duktape
versions:  [object Object]
versions.duktape:  v1.5.0
envkeys:  SHELL, <... etc>
envkeys(true): SHELL, <... etc>
readfile('test-error.js')  print('hi')

nucleus.exit('aaaa')

print(":/")

throw new Error('error!!')

print('noooo')

exit:  function () {"native"}
```

## License

rustyduk is licensed under [the Apache 2.0 License](LICENSE).
