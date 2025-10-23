use test_organization::add_two;

mod common;

#[test]
fn it_adds_two() {
    let result = add_two(2);
    assert_eq!(result, 4);
}