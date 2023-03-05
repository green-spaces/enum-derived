use std::collections::HashSet;

use enum_derived::Rand;

#[derive(Rand)]
pub struct Hello (
    u8,
    #[custom_rand(is_rand)]
    bool
);

fn is_rand() -> bool {
    false
}

fn main() {
    let mut seen_values = HashSet::new();
    for _ in 0..10000 {
        let r = Hello::rand();
        assert_eq!(r.1, false);
        seen_values.insert(r.0);
    }
    assert_eq!(seen_values.len(), u8::MAX as usize + 1);
}
