use aocd::*;

struct Grid {
    values: Vec<char>,
    width: usize,
    height: usize
}

impl Grid {
    fn new(input: String) -> Grid {
        let mut width = 0;
        let mut height = 0;
        let values: Vec<char> = input
            .split('\n')
            .map(|s| {
                width = s.len();
                height += 1;
                s.chars()
            })
            .flatten()
            .collect();
        Grid {
            values,
            width,
            height
        }
    }

    fn get(&self, x: i32, y: i32) -> char {
        if !(0..self.width as i32).contains(&x) || !(0..self.height as i32).contains(&y) {
            '!'
        } else {
            let index = y as usize * self.width + x as usize;
            *self.values.get(index).unwrap()
        }
    }

    fn get_word(&self, x: i32, y: i32, u: i32, v: i32, length: i32) -> String {
        let mut word = String::new();
        for scale in 0..length {
            let x_offset = u * scale;
            let y_offset = v * scale;
            word.push(self.get(x+x_offset, y+y_offset));
        }
        word
    }
}

#[aocd(2024, 4)]
pub fn solution1() {
    let data = input!();
    let grid = Grid::new(data);
    let mut count = 0;
    for x in 0..grid.width as i32 {
        for y in 0..grid.height as i32 {
            if 'X' == grid.get(x, y) {
                for (u, v) in [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)] {
                    if String::from("XMAS") == grid.get_word(x, y, u, v, 4) {
                        count += 1;
                    }
                }
            }
        }
    }
    submit!(1, count);
}

#[aocd(2024, 4)]
pub fn solution2() {
    let data = input!();
    let grid = Grid::new(data);
    let mut count = 0;
    for y in 0..grid.height as i32 {
        for x in 0..grid.width as i32 {
            let first = grid.get_word(x, y, 1, 1, 3);
            let second = grid.get_word(x+2, y, -1, 1, 3);
            if first == String::from("MAS") || first == String::from("SAM") {
                if second == String::from("MAS") || second == String::from("SAM") {
                    count += 1;
                }
            }
        }
    }
    submit!(2, count);
}