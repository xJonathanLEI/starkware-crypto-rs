<p align="center">
  <h1 align="center">starkware-crypto-rs</h1>
</p>

**Rust FFI bindings for StarkWare's [crypto-cpp](https://github.com/starkware-libs/crypto-cpp) library**

[![linting-badge](https://github.com/xJonathanLEI/starkware-crypto-rs/actions/workflows/lint.yaml/badge.svg?branch=master)](https://github.com/xJonathanLEI/starkware-crypto-rs/actions/workflows/lint.yaml)
[![tests-unix-badge](https://github.com/xJonathanLEI/starkware-crypto-rs/actions/workflows/test_unix.yaml/badge.svg?branch=master)](https://github.com/xJonathanLEI/starkware-crypto-rs/actions/workflows/test_unix.yaml)
[![tests-windows-badge](https://github.com/xJonathanLEI/starkware-crypto-rs/actions/workflows/test_windows.yaml/badge.svg?branch=master)](https://github.com/xJonathanLEI/starkware-crypto-rs/actions/workflows/test_windows.yaml)
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

On the author's machine with _Intel(R) Core(TM) i7-8700K CPU @ 3.70GHz_ running _Ubuntu 20.04.2 LTS_:

```log
ecdsa_get_public_key    time:   [4.1132 ms 4.1567 ms 4.2157 ms]
ecdsa_sign              time:   [185.54 ms 186.31 ms 187.20 ms]
ecdsa_verify            time:   [1.5582 ms 1.5708 ms 1.5853 ms]
pedersen_hash           time:   [293.27 us 294.74 us 296.33 us]
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](./LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
