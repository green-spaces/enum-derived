use enum_derived::Rand;

#[derive(Rand)]
struct BoolWrapper {
    _works: bool,
    #[custom_rand(always_false)]
    _is_rand: bool,
}

fn always_false() -> bool {
    false
}

#[derive(Rand)]
enum Sample {
    Wrapper(BoolWrapper),
    NotWrapped,
}

fn main() {
    let _s = Sample::rand();
}
