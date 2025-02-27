use accumulate::map;

fn square(x: i32) -> i32 {
    x * x
}

#[test]
fn accumulate_empty() {
    let input = vec![];
    let expected = vec![];
    assert_eq!(map(input, square), expected);
}
