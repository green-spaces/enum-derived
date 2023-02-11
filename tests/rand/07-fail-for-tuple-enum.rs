use enum_derived::Rand;

#[derive(Rand)]
pub enum Messages {
    Email,
    Letter(String),
    Bil,
}

fn main() {}

