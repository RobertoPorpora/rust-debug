mod debug;
mod example;

fn main() -> Result<(), i32> {
    example::test();
    return Ok(());
    // return Err(-1);
}
