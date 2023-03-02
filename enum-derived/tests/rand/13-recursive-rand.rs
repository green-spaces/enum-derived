/// Test that variants recursive derives applied to them
use enum_derived::Rand;

#[derive(Rand)]
pub enum NestedExtension {
    Base,
    Empty,
}

#[derive(Rand)]
pub enum NestedPath {
    Base,
    Extension(NestedExtension),
}

#[derive(Rand)]
pub enum TopLevel {
    Base,
    Path(NestedPath)
}

fn main() {
    let _rt = TopLevel::rand();
}
