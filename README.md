# Some To Err

A single trait crate that provides a method for converting an `Option` into a `Result` by treating `Some` values as `Err` and `None` values as `Ok`.

## Usage

Add this to your crate by:
```bash
cargo add some-to-err
```

Or add this to your `Cargo.toml`:
```toml
[dependencies]
some-to-err = "0.0.2"
```

and then:
```rust
use some_to_err::SomeToErr;

{
    let some: Option<&str> = Some("Error");
    let result = some.some_to_err(42);
    assert_eq!(result, Err("Error"));
}

{
    let none: Option<&str> = None;
    let result = none.some_to_err(42);
    assert_eq!(result, Ok(42));
}
```

## License
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)
