use enum_derived::Rand;

#[derive(Rand)]
pub struct Hello {
    world: u8,
}

fn main() {
    let _u = Hello::rand();
}
