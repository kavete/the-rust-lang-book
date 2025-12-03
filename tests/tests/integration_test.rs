use ::tests::add_two;

#[test]

fn adds_two() {
    let result = add_two(5);
    assert_eq!(result, 7);
}
