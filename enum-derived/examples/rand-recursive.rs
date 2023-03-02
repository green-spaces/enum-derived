/// Test that variants recursive derives applied to them
use enum_derived::Rand;

#[derive(Rand)]
pub enum Nested {
    A,
    B,
}

#[derive(Rand)]
pub enum TopLevel {
    Base,
    Nest(Nested),
}

fn main() {
    let _rt = TopLevel::rand();
}
