<div align="center">
    <h1 style="font-weight: 400">graze</h1>
    <img src="https://github.com/ImajinDevon/graze/actions/workflows/rust.yml/badge.svg" alt="Rust build status badge">
    <img src="https://github.com/ImajinDevon/graze/actions/workflows/clippy-check.yml/badge.svg" alt="Clippy check status badge">
    <img src="https://github.com/ImajinDevon/graze/actions/workflows/rustfmt-check.yml/badge.svg" alt="Rustfmt check status badge">
</div>

## What is `graze`?

`graze` is a zero-boilerplate configuration library.

`graze` itself does not use [serde](https://crates.io/crates/serde) as a dependency, but can
easily be used alongside the [serde](https://crates.io/crates/serde) ecosystem.

## Functions

- `load_from_path`
- `load_or_default`
- `load_or_write_default`

## Examples

### Load a configuration using the [toml](https://crates.io/crates/toml) crate

```rust
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    message: String
}

fn main() {
    let config = graze::load_from_path("Config.toml", |c| toml::from_str(c))
        .expect("Could not load configuration");

    println!("{}", config.message);
}
```