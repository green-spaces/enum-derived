use enum_derived::Rand;

#[derive(Rand)]
pub enum Messages {
    Email,
    Bill(u32),
    Letter { length: usize, has_stamp: bool },
}

fn main() {}
