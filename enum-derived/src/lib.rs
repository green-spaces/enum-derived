#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

use rand::distributions::{Distribution, Standard};

/// Derive [Rand] for any enum or struct
pub use enum_derived_macro::Rand;

/// Generate a random version of the implementor
pub trait Rand {
    fn rand() -> Self;
}

impl<S> Rand for S
where
    Standard: Distribution<S>,
{
    fn rand() -> S {
        ::rand::random::<S>()
    }
}
