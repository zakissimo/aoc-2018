use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

struct Canvas {
    frame: Vec<Vec<char>>,
    squares: Vec<(usize, usize, usize, usize)>,
    overlaps: usize,
}

impl Canvas {
    fn new(path: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let (mut width, mut height) = (1, 1);

        let mut squares = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let tokens: Vec<_> = line.split_whitespace().collect();

            if let [_, _, coords, dimensions] = &tokens[..] {
                if let [x_str, y_str] = coords.split(',').collect::<Vec<_>>().as_slice() {
                    let (x, y) = (
                        x_str.parse::<usize>()?,
                        y_str[..y_str.len() - 1].parse::<usize>()?,
                    );
                    if let [w, h] = dimensions.split('x').collect::<Vec<_>>().as_slice() {
                        let w = w.parse::<usize>()?;
                        let h = h.parse::<usize>()?;

                        squares.push((x, y, w, h));

                        height = if height > (h + y) { height } else { h + y };
                        width = if width > (w + x) { width } else { w + x };
                    }
                }
            }
        }

        let frame = vec![vec!['.'; width]; height];
        let overlaps = 0;

        Ok(Self {
            frame,
            squares,
            overlaps,
        })
    }

    fn draw(&mut self) {
        for &(x, y, w, h) in &self.squares {
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

    fn find_pure_claim(&self) -> Option<usize> {
        let mut claim = 0;
        for &(x, y, w, h) in &self.squares {
            claim += 1;
            let mut size = 0;
            for j in y..(y + h) {
                for i in x..(x + w) {
                    if self.frame[j][i] == '#' {
                        size += 1;
                    } else {
                        break;
                    }
                }
            }
            if size == w * h {
                return Some(claim);
            }
        }
        None
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = "./src/input";

    let mut canvas = Canvas::new(path)?;

    canvas.draw();

    println!("Part one: {} overlaps", canvas.overlaps);

    if let Some(claim) = canvas.find_pure_claim() {
        println!("Part two: claim {} is pure", claim);
    }

    Ok(())
}
