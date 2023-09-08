use std::collections::HashMap;
use std::fs;
use std::io::Error;

fn ids_match(l: &str, r: &str) -> i32 {
    let mut count = 0;
    let mut last: i32 = -1;
    for (i, c) in l.chars().enumerate() {
        if c != r
            .chars()
            .nth(i)
            .expect("Index should point to a char in the &str")
        {
            count += 1;
            last = i as i32;
        }
        if count > 1 {
            return -1;
        }
    }
    last
}

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

    let box_vec: Vec<&str> = input.split_whitespace().collect();

    let mut ans = String::new();

    for i in 1..box_vec.len() {
        for j in (0..i).rev() {
            let idx = ids_match(box_vec[i], box_vec[j]);
            if idx != -1 {
                let edited: String =
                    box_vec[i][..idx as usize].to_string() + &box_vec[i][(idx + 1) as usize..];
                ans = edited;
            }
        }
    }

    println!("Part two: {}", ans);

    Ok(())
}
