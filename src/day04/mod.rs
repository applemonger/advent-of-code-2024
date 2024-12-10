use aocd::*;
use std::collections::HashMap;

struct Grid {
    values: HashMap<(i32, i32), char>,
}

impl Grid {
    fn new(input: String) -> Grid {
        let values: HashMap<(i32, i32), char> = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, c)| ((x as i32, y as i32), c))
            })
            .collect();
        Grid { values }
    }

    fn get(&self, x: i32, y: i32) -> Option<char> {
        self.values.get(&(x, y)).copied()
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
    for (x, y) in grid.values.keys() {
        for u in -1..=1 {
            for v in -1..=1 {
                if (u, v) != (0, 0) && grid.get_word(*x, *y, u, v, 4) == *"XMAS" {
                    count += 1;
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
    for (x, y) in grid.values.keys() {
        let first = grid.get_word(*x, *y, 1, 1, 3);
        let second = grid.get_word(*x + 2, *y, -1, 1, 3);
        if (first == *"MAS" || first == *"SAM") && (second == *"MAS" || second == *"SAM") {
            count += 1;
        }
    }
    submit!(2, count);
}
