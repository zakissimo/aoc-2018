use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn get_canvas_dimensions(path: &str) -> Result<(usize, usize), Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut width = 1;
    let mut height = 1;

    for line in reader.lines() {
        let line = line?;
        let tokens: Vec<_> = line.split_whitespace().collect();

        if let [_, _, coords, dimensions] = &tokens[..] {
            if let [x_str, y_str] = coords.split(',').collect::<Vec<_>>().as_slice() {
                if let (Ok(x), Ok(y)) = (
                    x_str.parse::<usize>(),
                    y_str[..y_str.len() - 1].parse::<usize>(),
                ) {
                    if let [w, h] = dimensions.split('x').collect::<Vec<_>>().as_slice() {
                        let w = w.parse::<usize>()?;
                        let h = h.parse::<usize>()?;
                        height = if height > (h + y) { height } else { h + y };
                        width = if width > (w + x) { width } else { w + x };
                    }
                }
            }
        }
    }
    Ok((width, height))
}

fn main() -> Result<(), Box<dyn Error>> {
    if let Ok((width, height)) = get_canvas_dimensions("./src/sample") {
        println!("width: {}, height: {}", width, height);
    }
    if let Ok((width, height)) = get_canvas_dimensions("./src/input") {
        println!("width: {}, height: {}", width, height);
    }
    Ok(())
}
