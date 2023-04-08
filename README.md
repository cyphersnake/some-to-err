# Some To Err
[![Crates.io](https://img.shields.io/crates/v/some-to-err.svg)](https://crates.io/crates/some-to-err)
[![Crates.io](https://img.shields.io/crates/d/some-to-err.svg)](https://crates.io/crates/some-to-err)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://github.com/cyphersnake/some-to-err/actions/workflows/rust.yml/badge.svg)](https://github.com/cyphersnake/some-to-err/actions)
[![Documentation](https://docs.rs/some-to-err/badge.svg)](https://docs.rs/some-to-err)

This crate offers a pair of traits for effortlessly transforming `Option` into `Result`, elegantly converting `Some` values into `Err` while gracefully handling `None` values as `Ok`. Unleash the full potential of Rust's error handling capabilities with these versatile traits.

## Usage

```rust
use some_to_err::ErrOr;

{
    let some: Option<&str> = Some("Error");
    let result = some.err_or(42);
    assert_eq!(result, Err("Error"));
}

{
    let none: Option<&str> = None;
    let result = none.err_or(42);
    assert_eq!(result, Ok(42));
}
```

```rust
use some_to_err::ErrOrElse;
{
    let input: Option<&str> = None;
    let result = input.err_or_else(|| "Ok");
    assert_eq!(result, Ok("Ok"));
}

{
    let input = Some("Error");
    let result = input.err_or_else(|| "Ok");
    assert_eq!(result, Err("Error"));
}
```
