use std::collections::HashSet;

use enum_derived::Rand;

#[derive(Rand, PartialEq, Eq, Hash, Debug)]
pub enum Sample {
    A,
    B,
    C,
}

// Module used in Rand that we can try to interfere with
mod rand {
    fn thread_rng() {
        panic!("thread_rng is from the wrong module");
    }

    mod seq {
        pub trait SliceRandom {}

    }
}

fn main() {
    let mut seen_variants = HashSet::new();
    for _ in 0..1000 {
        seen_variants.insert(Sample::rand());
    }

    assert!(seen_variants.contains(&Sample::A));
    assert!(seen_variants.contains(&Sample::B));
    assert!(seen_variants.contains(&Sample::C));
}
