use enum_derived::Rand;

#[derive(Rand)]
pub enum Messages {
    Email,
    Letter {
        contents: String
    },
    Bill,
}

fn main() {
    for _ in 0..1000 {
        match Messages::rand() {
            Messages::Email => {},
            Messages::Letter{contents} => assert!(contents.len() > 0),
            Messages::Bill => {}
        }
    }
}

