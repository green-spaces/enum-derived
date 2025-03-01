use enum_derived::Rand;

#[derive(Rand)]
pub struct HasString {
    my_string: String,
}

fn stamped() -> bool {
    true
}

fn main() {
    let mut total_length = 0;
    for _ in 0..10000 {
        let r = HasString::rand();
        total_length += r.my_string.len();
    }
    assert!(total_length > 0);
}
