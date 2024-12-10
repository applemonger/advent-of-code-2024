use std::collections::HashSet;

use aocd::*;

#[derive(Clone)]
struct Grid {
    values: Vec<Option<usize>>,
    width: i32,
    height: i32,
}

impl Grid {
    fn new(input: String) -> Grid {
        let values: Vec<Option<usize>> = input
            .lines()
            .flat_map(|s| s.chars())
            .map(|c| c.to_digit(10).map(|x| x as usize))
            .collect();
        let width = input.lines().next().unwrap().len() as i32;
        let height = values.len() as i32 / width;
        Grid {
            values,
            width,
            height,
        }
    }

    fn get(&self, xy: (i32, i32)) -> Option<usize> {
        if (0..self.width).contains(&xy.0) && (0..self.height).contains(&xy.1) {
            let index = xy.1 * self.width + xy.0;
            *self.values.get(index as usize).unwrap()
        } else {
            None
        }
    }

    fn score(&self, mut nines: HashSet<(i32, i32)>, start: (i32, i32)) -> HashSet<(i32, i32)> {
        let current = self.get(start).unwrap();
        for (u, v) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
            let (x, y) = (start.0 + u, start.1 + v);
            match self.get((x, y)) {
                Some(num) => {
                    if num == current + 1 {
                        if num == 9 {
                            nines.insert((x, y));
                        } else {
                            nines = self.score(nines, (x, y));
                        }
                    }
                }
                None => {}
            }
        }
        nines
    }

    fn rating(&self, mut score: i32, start: (i32, i32)) -> i32 {
        let current = self.get(start).unwrap();
        for (u, v) in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
            let (x, y) = (start.0 + u, start.1 + v);
            match self.get((x, y)) {
                Some(num) => {
                    if num == current + 1 {
                        if num == 9 {
                            score += 1;
                        } else {
                            score += self.rating(0, (x, y));
                        }
                    }
                }
                None => {}
            }
        }
        score
    }
}

#[aocd(2024, 10)]
pub fn solution1() {
    let data = input!();
    let grid = Grid::new(data);
    let mut score = 0;
    for x in 0..grid.width {
        for y in 0..grid.height {
            if let Some(0) = grid.get((x, y)) {
                let mut nines: HashSet<(i32, i32)> = HashSet::<(i32, i32)>::new();
                nines = grid.score(nines, (x, y));
                score += nines.len();
            }
        }
    }
    submit!(1, score);
}

#[aocd(2024, 10)]
pub fn solution2() {
    let data = input!();
    let grid = Grid::new(data);
    let mut rating = 0;
    for x in 0..grid.width {
        for y in 0..grid.height {
            if let Some(0) = grid.get((x, y)) {
                rating += grid.rating(0, (x, y));
            }
        }
    }
    submit!(2, rating);
}
