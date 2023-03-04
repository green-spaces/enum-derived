use enum_derived::Rand;

#[derive(Rand)]
struct BoolWrapper {
    works: bool,
}

#[derive(Rand)]
enum Sample {
    Wrapper(BoolWrapper),
    NotWrapped,
}

fn main() {
    let _s = Sample::rand();
}

