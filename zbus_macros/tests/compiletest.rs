#[rustversion::attr(before(1.56), ignore)]
#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/**/*.rs");
}
