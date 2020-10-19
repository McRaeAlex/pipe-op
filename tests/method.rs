use pipe_op::pipe;

struct Adder {
    value: usize,
}

impl Adder {
    fn add(&self, num: usize) -> usize {
        self.value + num
    }
}

fn main() {
    let s = Adder { value: 10 };
    assert_eq!(11, pipe!(1, s.add()));
}
