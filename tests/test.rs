#[test]
fn route() {
    let t = trybuild::TestCases::new();
    t.pass("tests/route/valid_examples/*.rs");
    t.compile_fail("tests/route/invalid_examples/*.rs");
}
