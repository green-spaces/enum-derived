#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/rand/01-builds.rs");
    t.pass("tests/rand/02-rand_function_exists.rs");
    t.pass("tests/rand/03-returns_every_variant.rs");
    t.pass("tests/rand/04-hygiene.rs");
    t.compile_fail("tests/rand/06-fail-for-empty-enum.rs");
    t.compile_fail("tests/rand/07-fail-for-tuple-enum.rs");
    t.compile_fail("tests/rand/08-fail-for-struct-enum.rs");
    t.pass("tests/rand/09-tuple-enum-with-random-values.rs");
    t.pass("tests/rand/10-struct-enum-success.rs");
    t.pass("tests/rand/11-custom-rand-function.rs");
    t.pass("tests/rand/12-custom-weights.rs");
    t.pass("tests/rand/13-recursive-rand.rs");
    t.compile_fail("tests/rand/14-rand-trait-not-imported.rs");
    t.pass("tests/rand/15-struct-in-enum.rs");
    t.pass("tests/rand/16-works-for-struct-with-named-fields.rs");
    t.pass("tests/rand/17-works-for-struct-with-unnamed-fields.rs");
    t.pass("tests/rand/18-works-for-unit-structs.rs");
    t.pass("tests/rand/19-custom-rand-for-struct-with-named-fields.rs");
    t.pass("tests/rand/20-custom-rand-for-struct-with-unnamed-fields.rs");
    t.pass("tests/rand/21-custom-rand-for-enum-with-named-fields-field-level.rs");
    t.pass("tests/rand/22-custom-rand-struct-with-string.rs");
}
