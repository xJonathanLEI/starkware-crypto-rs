<p align="center">
  <h1 align="center">starkware-crypto-rs</h1>
</p>

**Rust FFI bindings for StarkWare's [crypto-cpp](https://github.com/starkware-libs/crypto-cpp) library**

[![linting-badge](https://github.com/xJonathanLEI/starkware-crypto-rs/actions/workflows/lint.yaml/badge.svg?branch=master)](https://github.com/xJonathanLEI/starkware-crypto-rs/actions/workflows/lint.yaml)
[![tests-badge](https://github.com/xJonathanLEI/starkware-crypto-rs/actions/workflows/test.yaml/badge.svg?branch=master)](https://github.com/xJonathanLEI/starkware-crypto-rs/actions/workflows/test.yaml)
[![crates-badge](https://img.shields.io/crates/v/starkware-crypto-sys.svg)](https://crates.io/crates/starkware-crypto-sys)

> _Note that currently target `x86_64-pc-windows-msvc` is [not supported](https://github.com/xJonathanLEI/starkware-crypto-rs/issues/3). If you're building on Windows, you need to [use the GNU build of Rust](https://rust-lang.github.io/rustup/installation/windows.html)._

## Adding starkware-crypto-rs to your project

To use the crate from [crates.io](https://crates.io/crates/starkware-crypto-sys), add the following to your `Cargo.toml` file:

```toml
[dependencies]
starkware-crypto-sys = "0.1"
```

## Running benchmark

To run benchmark:

```sh
$ cargo bench
```

On the author's machine, the results are:

```log
pedersen_hash           time:   [267.08 us 270.36 us 274.40 us]
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](./LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
