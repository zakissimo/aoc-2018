use std::collections::HashMap;
use std::fs;
use std::io::Error;

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("./src/input")?;
    let box_ids = input.split_whitespace();

    let (mut two, mut three) = (0, 0);

    for box_id in box_ids {
        let mut letter_freq = HashMap::<char, usize>::new();
        for c in box_id.chars() {
            *letter_freq.entry(c).or_insert(0) += 1;
        }
        if letter_freq.values().any(|&count| count == 2) {
            two += 1;
        }
        if letter_freq.values().any(|&count| count == 3) {
            three += 1;
        }
    }

    println!("Part one: {}", two * three);


    Ok(())
}
