use enum_derived::Rand;

#[derive(Rand, PartialEq, Eq, Hash, Debug)]
pub enum TupleLikeEnum {
    A,
    B(u64),
    C,
}

fn main() {}
