use aocd::*;

struct Grid {
    values: Vec<char>,
    width: i32,
    height: i32,
}

impl Grid {
    fn new(input: String) -> Grid {
        let values: Vec<char> = input.split('\n').flat_map(|s| s.chars()).collect();
        let width = input.split('\n').next().unwrap().len();
        let height = values.len() / width;
        Grid {
            values,
            width: width as i32,
            height: height as i32,
        }
    }

    fn get(&self, x: i32, y: i32) -> Option<char> {
        if !(0..self.width).contains(&x) || !(0..self.height).contains(&y) {
            None
        } else {
            let index = y * self.width + x;
            Some(self.values[index as usize])
        }
    }

    fn get_word(&self, x: i32, y: i32, u: i32, v: i32, length: i32) -> String {
        (0..length)
            .flat_map(|scale| {
                let x_offset = u * scale;
                let y_offset = v * scale;
                self.get(x + x_offset, y + y_offset)
            })
            .collect()
    }
}

#[aocd(2024, 4)]
pub fn solution1() {
    let data = input!();
    let grid = Grid::new(data);
    let mut count = 0;
    for x in 0..grid.width {
        for y in 0..grid.height {
            for u in -1..=1 {
                for v in -1..=1 {
                    if (u, v) != (0, 0) && grid.get_word(x, y, u, v, 4) == *"XMAS" {
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
    for y in 0..grid.height {
        for x in 0..grid.width {
            let first = grid.get_word(x, y, 1, 1, 3);
            let second = grid.get_word(x + 2, y, -1, 1, 3);
            if (first == *"MAS" || first == *"SAM") && (second == *"MAS" || second == *"SAM") {
                count += 1;
            }
        }
    }
    submit!(2, count);
}
