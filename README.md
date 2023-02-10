[![crates.io](https://img.shields.io/crates/v/enum-derived.svg)](https://crates.io/crates/enum-derived)
![Build](https://github.com/green-spaces/enum-derived/actions/workflows/build.yml/badge.svg?branch=main)

# Enum-Derived

Enum-Derived adds new functionality to unit-like enums

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
