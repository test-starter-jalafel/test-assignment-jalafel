/// What should the type of _function be?
pub fn map(input: Vec<i32>, _function: impl Fn(i32) -> i32) -> Vec<i32> {
    input.into_iter().map(_function).collect()
}
