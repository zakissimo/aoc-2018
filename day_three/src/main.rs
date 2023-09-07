use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

struct Canvas {
    frame: Vec<Vec<char>>,
    squares_start: Vec<(usize, usize)>,
    squares_dimensions: Vec<(usize, usize)>,
    width: usize,
    height: usize,
    overlaps: usize,
}

impl Canvas {
    fn new(path: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut width = 1;
        let mut height = 1;

        let mut squares_start = Vec::<(usize, usize)>::new();
        let mut squares_dimensions = Vec::<(usize, usize)>::new();

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

                            squares_start.push((x, y));
                            squares_dimensions.push((w, h));

                            height = if height > (h + y) { height } else { h + y };
                            width = if width > (w + x) { width } else { w + x };
                        }
                    }
                }
            }
        }

        let mut frame: Vec<Vec<char>> = Vec::with_capacity(height);
        for _ in 0..height {
            frame.push(std::iter::repeat('.').take(width).collect());
        }
        let overlaps = 0;

        Ok(Self {
            frame,
            squares_start,
            squares_dimensions,
            width,
            height,
            overlaps,
        })
    }

    fn draw(&mut self) {

        for _ in 0..self.height {
            self.frame
                .push(std::iter::repeat('.').take(self.width).collect());
        }

        for k in 0..self.squares_dimensions.len() {

            let x = self.squares_start[k].0;
            let y = self.squares_start[k].1;
            let w = self.squares_dimensions[k].0;
            let h = self.squares_dimensions[k].1;

            for j in y..(y + h) {
                for i in x..(x + w) {
                    if self.frame[j][i] == '.' {
                        self.frame[j][i] = '#';
                    } else if self.frame[j][i] == '#' {
                        self.frame[j][i] = '*';
                        self.overlaps += 1;
                    }
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = "./src/input";

    let mut canvas = Canvas::new(path)?;

    canvas.draw();

    println!("Part one: {} overlaps", canvas.overlaps);

    Ok(())
}
