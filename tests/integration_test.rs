use rust_playground;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, rust_playground::add_two(2))
}
