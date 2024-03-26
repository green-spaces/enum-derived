/// Test that variants can have different weights applied to them
use std::collections::HashMap;
use enum_derived::Rand;

#[derive(Rand)]
pub enum RandomWeights {
    // Default weight is one
    Default,
    #[weight(0)]
    Zero,
    #[weight(1)]
    One,
    #[weight(2)]
    Two,
    #[weight(3)]
    Three,
    #[custom_rand(|_rng| RandomWeights::Four)]
    #[weight(4)]
    Four,
}

fn main() {
    let mut seen_variants: HashMap<_, i32> = HashMap::new();

    for _ in 0..11000 {
        let rt = RandomWeights::rand();
        let counts = seen_variants.entry(std::mem::discriminant(&rt)).or_default();
        *counts += 1;
    }

    let zero_count = seen_variants.get(&std::mem::discriminant(&RandomWeights::Zero));
    assert!(zero_count.is_none());

    let default_count = seen_variants.get(&std::mem::discriminant(&RandomWeights::Default)).unwrap();
    let one_count = seen_variants.get(&std::mem::discriminant(&RandomWeights::One)).unwrap();
    let two_count = seen_variants.get(&std::mem::discriminant(&RandomWeights::Two)).unwrap();
    let three_count = seen_variants.get(&std::mem::discriminant(&RandomWeights::Three)).unwrap();
    let four_count = seen_variants.get(&std::mem::discriminant(&RandomWeights::Four)).unwrap();

    // Check that the counts are ordered properly    
    assert!(default_count < two_count);
    assert!(one_count < two_count);
    assert!(two_count < three_count);
    assert!(three_count < four_count);
}
