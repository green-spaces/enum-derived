<!-- Enum-Derived readme rendered on crates.io -->

# Enum-Derived adds new functionality to unit-like enums

[![crates.io](https://img.shields.io/crates/v/enum-derived.svg)](https://crates.io/crates/enum-derived)
![Build](https://github.com/github/docs/actions/workflows/rust.yml/badge.svg)
---

You may be looking for:

- [API documentation](https://docs.rs/enum-derived)

## Enum-Derived in action

```rust
use enum_derived::Rand;

#[derive(Rand, Debug)]
pub enum Dna {
    A,
    C,
    T,
    G
}

fn main() {

    let base = Dna::rand();

    println!("Random Base: ${base:?}");
}
```
