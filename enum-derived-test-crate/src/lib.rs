//! This library exists to test that enum_derived works without requiring the crate to import the rand crate

#[derive(::enum_derived::Rand)]
pub enum SingleEnum {
    A,
}

#[derive(::enum_derived::Rand)]
pub enum DoubleRand {
    A,
    B,
}

#[derive(::enum_derived::Rand)]
pub enum MultiRand {
    A,
    B(u8),
    C { named: u16 },
}

#[cfg(test)]
mod tests {
    use enum_derived::Rand;

    use super::*;

    #[test]
    fn single() {
        let _ele = SingleEnum::rand();
    }

    #[test]
    fn double() {
        let _ele = DoubleRand::rand();
    }

    #[test]
    fn multi() {
        for _ in 0..30 {
            let _ele = MultiRand::rand();
        }
    }
}
