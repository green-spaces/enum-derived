use enum_derived::Rand;

#[derive(Rand)]
pub enum Messages {
    Email,
    Letter {
        length: usize,
        has_stamp: bool,
    },
    Bil,
}

fn main() {}

