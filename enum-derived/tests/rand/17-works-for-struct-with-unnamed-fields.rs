use std::collections::HashSet;

use enum_derived::Rand;

#[derive(Rand)]
pub struct Hello(u8);

fn main() {
    let mut seen_values = HashSet::new();
    for _ in 0..10000 {
        let r = Hello::rand();
        seen_values.insert(r.0);
    }
    assert_eq!(seen_values.len(), u8::MAX as usize + 1);
}
