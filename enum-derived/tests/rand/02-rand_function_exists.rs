use enum_derived::Rand;

#[derive(Rand)]
pub enum Sample {
    A,
    B,
    C,
}

fn main() {
    let _s = Sample::rand();
}

