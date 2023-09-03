use std::collections::HashMap;
use std::fs;
use std::io::Error;

fn main() -> Result<(), Error> {
    let paths = fs::read_dir(".")?;
    for entry in paths {
        println!("{}", entry?.path().to_string_lossy());
    }

    let input = fs::read_to_string("./input")?;

    let v = input
        .split_whitespace()
        .map(|s| s.parse::<i32>().expect("Input should be i32"));

    let sum = v.clone().sum::<i32>();

    println!("Part one: {}", sum);

    let mut idx: usize = 0;
    let mut acc: i32 = 0;
    let v: Vec<i32> = v.collect();
    let mut freq_count = HashMap::<i32, usize>::new();
    let ans = loop {
        *freq_count.entry(acc).or_insert(0) += 1;
        if freq_count[&acc] == 2 {
            break acc;
        }
        acc += v[idx];
        idx = (idx + 1) % v.len();
    };

    println!("Part two: {}", ans);

    Ok(())
}
