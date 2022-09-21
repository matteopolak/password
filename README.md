## Password 🔑

A lightweight and featureful password generator.

## Usage

| Option                      | Default                   | Description                                            |
| --------------------------- | ------------------------- | ------------------------------------------------------ |
| `-a`, `--all`               | ❌                        | Whether to use every option                            |
| `-c`, `--chars` \<number\>  | ❌                        | List of characters to use; overrides all other options |
| `-d`, `--digits`            | ✔️                        | Whether to use digits                                  |
| `-h`, `--help`              |                           | Print help information                                 |
| `-l`, `--length` \<number\> | 12                        | Length of the password                                 |
| `-o`, `--lower`             | ✔️                        | Whether to use lowercase letters                       |
| `-s`, `--special`           | ❌                        | Whether to use special characters                      |
| `-u`, `--upper`             | ✔️                        | Whether to use uppercase letters                       |
| `-V`, `--version`           | Print version information |

## Install

See the [Rust install guide](https://www.rust-lang.org/tools/install) for [Rust nightly](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html),
to [build from source with cargo](https://doc.rust-lang.org/cargo/commands/cargo-build.html), and to [run the unit tests](https://doc.rust-lang.org/cargo/commands/cargo-test.html).

```
$ cargo +nightly build --release
```

### Pre-built Binaries

Binaries are released on major releases for Windows platforms and can be located [in the releases tab](https://github.com/matteopolak/baerscript/releases).

## License

[MIT](./LICENSE)
