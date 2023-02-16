use adder;

mod common;

// `cargo test --test integration_test`
// or `cargo test` for all

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}