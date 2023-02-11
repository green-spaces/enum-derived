#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use enum_derived::Rand;
enum Dna {
    A,
    C,
    T,
    G,
}
impl Dna {
    fn rand() -> Self {
        use ::rand::{thread_rng, Rng};
        let mut samples = <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                ::std::mem::discriminant(&Dna::A),
                ::std::mem::discriminant(&Dna::C),
                ::std::mem::discriminant(&Dna::T),
                ::std::mem::discriminant(&Dna::G),
            ]),
        );
        let mut rng = thread_rng();
        let idx: usize = rng.gen_range(0..samples.len());
        let sample_disc = samples.swap_remove(idx);
        let vari = match sample_disc {
            x if x == ::std::mem::discriminant(&Dna::A) => Dna::A,
            x if x == ::std::mem::discriminant(&Dna::C) => Dna::C,
            x if x == ::std::mem::discriminant(&Dna::T) => Dna::T,
            x if x == ::std::mem::discriminant(&Dna::G) => Dna::G,
            _ => ::core::panicking::panic_fmt(format_args!("Unreachable")),
        };
        vari
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Dna {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                Dna::A => "A",
                Dna::C => "C",
                Dna::T => "T",
                Dna::G => "G",
            },
        )
    }
}
fn main() {
    let mut bases = Vec::new();
    for _ in 0..4 {
        bases.push(Dna::rand());
    }
    {
        ::std::io::_print(format_args!("random bases: {0:?}\n", bases));
    };
}
