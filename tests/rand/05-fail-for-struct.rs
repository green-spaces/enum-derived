use enum_derived::Rand;

#[derive(Rand, PartialEq, Eq, Hash, Debug)]
pub struct Hello {
    world: String,
}

fn main() {}
