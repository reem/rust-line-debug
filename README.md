# Inspect [![Build Status](https://secure.travis-ci.org/reem/rust-inspect.png?branch=master)](https://travis-ci.org/reem/rust-inspect)

A library designed to for inspecting values, and meta information like the
filename, and line number, and expression. Inspection of a value also includes
reflecting on it's type.

These functions and macros are intended for use while developing, and are not
advised to be used in releases.

## Example

```rust
#[macro_use(inspect)]
extern crate inspect;

fn main() {
    let a = 7u32;
    let b = 4u64;
    inspect!(a, b, a as u64 + b);
    // examples/readme.rs - 7: a = [u32] 7, b = [u64] 4, a as u64 + b = [u64] 11,
}
```

## Install

Add:

```toml
[dependencies]
inspect = "*"
```

to your `Cargo.toml`.

## License

MIT
