
use pipe_op::pipe;

fn add(a: usize, b: usize) -> usize {
    a + b
}

fn main() {
    let a = 10;
    assert_eq!(pipe!(1, add(a)), 11);
}