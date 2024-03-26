use enum_derived::Rand;

#[derive(Rand)]
pub enum Messages {
    Email,
    Letter {
        length: usize,
        #[custom_rand(stamped)]
        has_stamp: bool,
    },
    Bil,
}

fn stamped(_rng: &mut impl rand::Rng) -> bool {
    true
}

fn main() {
    for _ in 0..10000 {
        let r = Messages::rand();
        if let Messages::Letter { has_stamp, .. } = r {
            assert_eq!(has_stamp, stamped())
        }
    }
}
