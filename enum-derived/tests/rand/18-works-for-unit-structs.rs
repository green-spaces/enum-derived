use enum_derived::Rand;

#[derive(Rand)]
pub struct Hello;

fn main() {
    let _r = Hello::rand();
}
