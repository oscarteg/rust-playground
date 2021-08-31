#[allow(unused_macros)]
// use macro_rules! <name of macro>{<Body>}
macro_rules! add {
    // macth like arm for macro
    ($a:expr,$b:expr) => {
        // macro expand to this code
        {
            // $a and $b will be templated using the value/variable provided to macro
            $a + $b
        }
    };
}

#[allow(unused_macros)]
macro_rules! add_as {
    ($a: expr, $b: expr, $typ: ty) => {
        $a as $typ + $b as $typ
    };
}

#[allow(unused_macros)]
macro_rules! add_multiple {
    ($($a: expr), *) => {
        0
        $(+$a)*
    }
}

#[test]
fn test_macro() {
    assert_eq!(add!(1, 2), 3);
    assert_eq!(add_as!(1, 2, u8), 3);
    assert_eq!(add_multiple!(1, 2, 3, 4), 10);
}
