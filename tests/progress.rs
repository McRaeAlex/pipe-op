#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.pass("tests/basic.rs");
    t.pass("tests/multiple.rs");
    t.pass("tests/many.rs");
    t.pass("tests/closure.rs");
    t.pass("tests/chaining.rs");
    t.pass("tests/method.rs");
    t.pass("tests/try.rs");
}
