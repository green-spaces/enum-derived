<!-- Enum-Derived readme rendered on crates.io -->

# Enum-Derived

Use Enum-Derived's Rand macro to generate random variants of your enums. All fields are populated with independant random value.

Need custom constraints applied to a variant? Use the `#[custom_rand(your_function)]` attribute to override the default behaviour or extend suport to custom types.

Need some variants to be generated more ofter? Use the `#[weight(VARIANT_WEIGHT)]` to change the distribution.

[![crates.io](https://img.shields.io/crates/v/enum-derived.svg)](https://crates.io/crates/enum-derived)
![Build](https://github.com/green-spaces/enum-derived/actions/workflows/build.yml/badge.svg?branch=main)
---

## Rand

Rand allows for a random variant of an enum to be generated.

The `rand` crates `rand::random` method must have an implementation for each type used in a variant to be able to generate the enum. Unsupported variants can us the `#[custom_rand(your_function)]` to extend the functionaliy.

```rust
use enum_derived::Rand;

#[derive(Rand)]
pub enum Example {
    Empty,
    Boolean(bool),
    NamedFields {
        hello: u8,
        world: bool,
    },
    /// Use a custome random function for unsupprorted types (String)
    #[custom_rand(rand_string)]
    RandString(String),
    /// Occurs three times as often (default weight is one)
    #[weight(3)]
    OverWeighted,
    /// Not randomly generated because it's weight is zero
    #[weight(0)]
    NeverOccurs,
    Integers(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize),
    Character(char),
    FloatingPoint(f32, f64),
    MaximumArrayLength([u8; 32]),
    LongTuple((u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64)),
    Options(Option<char>),
    /// Uses a custom rand function and weighting
    #[custom_rand(rand_vec)]
    #[weight(2)]
    OverWightedRandVec(Vec<u8>),
}

fn rand_string() -> Example {
    let unique_str = format!("{:?}", std::time::SystemTime::now());
    Example::RandString(unique_str)
}

fn rand_vec() -> Example {
    Example::OverWightedRandVec(vec![1,2,3,4,5])
}

fn main() {
    let example = Example::rand();
}
```
