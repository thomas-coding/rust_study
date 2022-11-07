use myadd;

mod common;

#[test]
fn test_add() {
    common::setup();
    let result = myadd::add(4, 10);
    assert_eq!(result, 14);
}

#[test]
fn test_add_three() {
    let result = myadd::add_three(4);
    assert_eq!(result, 7);
}
