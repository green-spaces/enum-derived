#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

use rand::{
    distributions::{Distribution, Standard},
    thread_rng, Rng,
};

/// Derive [Rand] for any enum or struct
pub use enum_derived_macro::Rand;

/// Generate a random version of the implementor
pub trait Rand: Sized {
    fn rand() -> Self {
        Self::rand_ext(&mut thread_rng())
    }

    fn rand_ext<R: Rng>(rng: &mut R) -> Self;
}

impl<S> Rand for S
where
    Standard: Distribution<S>,
{
    fn rand() -> Self {
        thread_rng().gen()
    }

    fn rand_ext<R: Rng>(rng: &mut R) -> Self {
        rng.gen()
    }
}
