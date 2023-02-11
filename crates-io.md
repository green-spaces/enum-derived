<!-- Enum-Derived readme rendered on crates.io -->

# Enum-Derived

Enum-Derived is a collection of derive macros that extend the functionality of enums

[![crates.io](https://img.shields.io/crates/v/enum-derived.svg)](https://crates.io/crates/enum-derived)
![Build](https://github.com/green-spaces/enum-derived/actions/workflows/build.yml/badge.svg?branch=main)
---

## Rand

Rand allows for a random variant of an enum to be generated. This can be particularly useful when testing and the specific variant isn't important.

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

Two examples where the `rand` method is only available in tests.

`Vehicle::rand()` is not available in `main()`

```rust compile_fail
#[cfg(test)]
use enum_derived::Rand;

#[cfg_attr(test, Rand)]
#[derive(Debug)]
pub enum Vehicle {
    Car,
    Truck,
    Motorbike,
}

fn main() {
    let vehicle = Vehicle::rand();
}

```

`Vehicle::rand()` is available in the `tests` module

```rust
#[cfg(test)]
use enum_derived::Rand;

#[cfg_attr(test, Rand)]
#[derive(Debug)]
pub enum Vehicle {
    Car,
    Truck,
    Motorbike,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_vehicle() {
        let vehicle = Vehicle::rand();
    }
}
```
