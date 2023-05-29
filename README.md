# switch-tester

A dioxus-based Rust GUI for displaying mechanical keyboard switch info on a switch tester board

## Dependencies

### Linux

* Cargo / Rust toolchain (https://rustup.rs)
* See the Dioxus [platform-specific dependencies](https://dioxuslabs.com/docs/0.3/guide/en/getting_started/desktop.html#platform-specific-dependencies)

## Run

```
$ cargo run --release
```

## Code Format

The formatting options currently use nightly-only options.

```
$ cargo +nightly fmt
```

## Code Linting

```
$ cargo clippy --all-targets -- -D warnings
```
