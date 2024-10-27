#[test]
fn test_simple() {
    macrotest::expand("tests/macro/simple/*.rs");
}

#[test]
fn test_nested() {
    macrotest::expand("tests/macro/nested/*.rs");
}
