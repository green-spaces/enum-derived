[![crates.io](https://img.shields.io/crates/v/enum-derived.svg)](https://crates.io/crates/enum-derived)
![Build](https://github.com/github/docs/actions/workflows/rust.yml/badge.svg)

# Enum-Derived

Enum-Derived adds new functionality to unit-like enums

[Build Status]: https://img.shields.io/github/actions/workflow/status/green-spaces/enum-derived/rust.yml?branch=main
[actions]: https://github.com/green-spaces/enum-derived/actions?query=branch%3Amain
[Latest Version]: https://img.shields.io/crates/v/enum-derived.svg
[crates.io]: https://crates.io/crates/enum-derived

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
