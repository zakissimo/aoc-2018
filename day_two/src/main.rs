use std::fs;
use std::io::Error;

fn main() -> Result<(), Error> {
    let file = fs::read_to_string("./sample")?;
    println!("{}", file);
    Ok(())
}
