use std::collections::HashMap;

use aocd::*;

#[derive(Clone)]
struct Grid {
    values: HashMap<(i32, i32), usize>,
}

impl Grid {
    fn new(input: String) -> Grid {
        let values: HashMap<(i32, i32), usize> = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| {
                    let value = c.to_digit(10)?;
                    Some(((x as i32, y as i32), value as usize))
                })
            })
            .collect();
        Grid { values }
    }

    fn get(&self, xy: (i32, i32)) -> Option<usize> {
        self.values.get(&xy).copied()
    }

    fn score(
        &self,
        mut nines: HashMap<(i32, i32), i32>,
        start: (i32, i32),
    ) -> HashMap<(i32, i32), i32> {
        let current = self.get(start).unwrap();
        for (u, v) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
            let (x, y) = (start.0 + u, start.1 + v);
            if let Some(num) = self.get((x, y)) {
                if num == current + 1 {
                    if num == 9 {
                        *nines.entry((x, y)).or_default() += 1;
                    } else {
                        nines = self.score(nines, (x, y));
                    }
                }
            }
        }
        nines
    }
}

#[aocd(2024, 10)]
pub fn solution1() {
    let data = input!();
    let grid = Grid::new(data);
    let mut score = 0;
    for (&position, &value) in grid.values.iter() {
        if value == 0 {
            let nines = grid.score(HashMap::new(), position);
            score += nines.len();
        }
    }
    submit!(1, score);
}

#[aocd(2024, 10)]
pub fn solution2() {
    let data = input!();
    let grid = Grid::new(data);
    let mut rating = 0;
    for (&position, &value) in grid.values.iter() {
        if value == 0 {
            let nines = grid.score(HashMap::new(), position);
            rating += nines.values().copied().sum::<i32>();
        }
    }
    submit!(2, rating);
}
