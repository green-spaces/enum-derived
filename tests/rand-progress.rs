#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/rand/01-builds.rs");
    t.pass("tests/rand/02-rand_function_exists.rs");
    t.pass("tests/rand/03-returns_every_variant.rs");
    t.pass("tests/rand/04-hygiene.rs");
    t.compile_fail("tests/rand/05-fail-for-struct.rs");
    t.compile_fail("tests/rand/06-fail-for-empty-enum.rs");
    t.compile_fail("tests/rand/07-fail-for-tuple-enum.rs");
}