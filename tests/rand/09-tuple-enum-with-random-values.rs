/// Test tuples whose fields can be generated by `rand::random`
// use std::collections::HashSet;

use enum_derived::Rand;

#[derive(Rand)]
pub enum RandomTypes {
    Empty,
    Integers(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize),
    Character(char),
    Boolean(bool),
    FloatingPoint(f32, f64),
    Arrays([u8; 32]),
    LongTuple((u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64)),
    Options(Option<char>),
}

fn main() {
    for _ in 0..200 {
        let _rt = RandomTypes::rand();
    }
}
