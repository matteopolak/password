# Password 🔑

![Build Status](https://github.com/matteopolak/password/actions/workflows/build.yml/badge.svg)
![Release Status](https://github.com/matteopolak/password/actions/workflows/release.yml/badge.svg)
[![License:MIT](https://img.shields.io/badge/license-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust:Nightly](https://img.shields.io/badge/rust-nightly-blue.svg)](https://www.rust-lang.org/tools/install)

A lightweight and featureful password generator.

```powershell
Usage: password [OPTIONS]
Options:
  -a, --all              Whether to use every option
  -c, --chars <CHARS>    List of characters to use in addition to other options
  -d, --digits           Whether to use digits
  -l, --length <LENGTH>  Length of the password [default: 12]
  -o, --lower            Whether to use lowercase letters
  -n, --num <NUM>        Number of passwords to generate [default: 1]
  -s, --special          Whether to use special characters
  -u, --upper            Whether to use uppercase letters
  -h, --help             Print help information
  -V, --version          Print version information
```

## Install

See the [Rust install guide](https://www.rust-lang.org/tools/install) for [Rust nightly](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html),
to [build from source with cargo](https://doc.rust-lang.org/cargo/commands/cargo-build.html), and to [run the unit tests](https://doc.rust-lang.org/cargo/commands/cargo-test.html).

```bash
cargo +nightly build --release
```

### Pre-built Binaries

Binaries are released on major releases for Windows platforms and can be located [in the releases tab](https://github.com/matteopolak/baerscript/releases).

## License

[MIT](./LICENSE)
