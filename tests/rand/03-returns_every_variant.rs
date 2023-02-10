use std::collections::HashSet;

use enum_derived::Rand;

#[derive(Rand, PartialEq, Eq, Hash, Debug)]
pub enum Dna {
    A,
    C,
    T,
    G
}

fn main() {
    let mut seen_variants = HashSet::new();
    for _ in 0..1000 {
        let _ = seen_variants.insert(Dna::rand());
    }

    assert!(seen_variants.contains(&Dna::A));
    assert!(seen_variants.contains(&Dna::C));
    assert!(seen_variants.contains(&Dna::T));
    assert!(seen_variants.contains(&Dna::G));
}

