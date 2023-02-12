use enum_derived::Rand;

#[derive(Rand)]
pub enum Messages {
    Email(Vec<u8>),
    Letter(String),
    Bill,
}

fn main() {}

