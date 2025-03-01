use enum_derived::Rand;

#[derive(Rand)]
pub enum Messages {
    Email(Vec<u8>),
    Letter(String),
    Bill,
}

fn main() {
    for _ in 0..1000 {
        match Messages::rand() {
            Messages::Email(inner) => assert!(inner.len() > 0),
            Messages::Letter(inner) => assert!(inner.len() > 0),
            Messages::Bill => {}
        }
    }
}

