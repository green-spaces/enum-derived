#[derive(enum_derived::Rand)]
pub enum Sample {
    A,
    B,
    C,
}

fn main() {
    let _s = Sample::rand();
}

