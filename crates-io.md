<!-- Enum-Derived readme rendered on crates.io -->

# Enum-Derived

Enum-Derived is a collection of derive macros that extend the functionality of enums

[![crates.io](https://img.shields.io/crates/v/enum-derived.svg)](https://crates.io/crates/enum-derived)
![Build](https://github.com/green-spaces/enum-derived/actions/workflows/build.yml/badge.svg?branch=main)
---

## Rand

Rand allows for a random variant of an enum to be generated. This can be particularly useful when testing and the specific variant isn't important.

The `rand` crates `rand::random` method must have an implementation  for each type used in a variant to be able to generate the enum.

```rust
use enum_derived::Rand;

#[derive(Rand)]
pub enum Example {
    Empty,
    Integers(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize),
    Character(char),
    Boolean(bool),
    FloatingPoint(f32, f64),
    MaximumArrayLength([u8; 32]),
    LongTuple((u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64)),
    Options(Option<char>),
    NamedFields {
        hello: u8,
        world: bool,
    },
    #[custom_rand(rand_string)]
    RandString(String),
    #[custom_rand(rand_string)]
    RandVec(Vec<u8>),
}

fn rand_string() -> Example {
    let unique_str = format!("{:?}", std::time::SystemTime::now());
    Example::RandString(unique_str)
}

fn rand_vec() -> Example {
    Example::RandVec(vec![1,2,3,4,5])
}


fn main() {
    let example = Example::rand();
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
