use enum_derived::Rand;

#[derive(Rand)]
pub enum Messages {
    Email,
    Letter {
        contents: String
    },
    Bil,
}

fn main() {}

