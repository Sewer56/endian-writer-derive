#[test]
fn test_simple() {
    macrotest::expand("tests/macro/simple/*.rs");
}

#[test]
fn test_nested() {
    macrotest::expand("tests/macro/nested/*.rs");
}

#[test]
fn test_weird_order() {
    macrotest::expand("tests/macro/weird_order/*.rs");
}

#[test]
fn test_generic() {
    macrotest::expand("tests/macro/generic/*.rs");
}

#[test]
fn test_generic_where() {
    macrotest::expand("tests/macro/generic_where/*.rs");
}

#[test]
fn test_generic_lifetime() {
    macrotest::expand("tests/macro/generic_lifetime/*.rs");
}

#[test]
fn test_generic_const() {
    macrotest::expand("tests/macro/generic_const/*.rs");
}
