use pipe_op::pipe;

fn add(a: usize, b: usize) -> Result<usize, Box<dyn std::error::Error>> {
    Ok(a + b)
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let e = 10;
    assert_eq!(pipe!(e, add(10)?), 20);
    Ok(())
}
