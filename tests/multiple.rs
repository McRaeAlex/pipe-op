use pipe_op::pipe;

fn add(a: usize, b: usize) -> usize {
    a + b
}

fn main() {
    assert_eq!(pipe!(1, add(10), add(10)), 21);
}
