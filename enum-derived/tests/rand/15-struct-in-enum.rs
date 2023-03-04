use enum_derived::Rand;

#[derive(Rand)]
struct BoolWrapperNamed {
    works: bool,
    other: u64,
}

#[derive(Rand)]
struct BoolWrapperUnnamed(bool, u8);

#[derive(Rand)]
struct UnitStruct;

#[derive(Rand)]
struct BoolWrapper {
    works: bool,
}

#[derive(Rand)]
enum Sample {
    WrapperNamed(BoolWrapperNamed),
    WrapperUnnamed(BoolWrapperUnnamed),
    WrapperUnit(UnitStruct),
    NotWrapped,
}

fn main() {
    let _s = Sample::rand();
}

