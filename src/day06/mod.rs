use std::collections::{HashMap, HashSet};

use aocd::*;

#[derive(Clone)]
struct Grid {
    values: HashMap<(i32, i32), char>,
    start: (i32, i32),
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
        let start = values
            .iter()
            .find_map(|(&pos, &c)| (c == '^').then_some(pos))
            .unwrap();
        Grid { values, start }
    }

    fn get(&self, xy: (i32, i32)) -> Option<char> {
        self.values.get(&xy).copied()
    }

    fn set(&mut self, xy: (i32, i32), c: char) {
        if let Some(value) = self.values.get_mut(&xy) {
            *value = c;
        }
    }

    fn patrol(&self) -> Option<HashSet<(i32, i32)>> {
        let mut position = self.start;
        let mut direction = (0, -1);
        let mut heading = (position.0 + direction.0, position.1 + direction.1);
        let mut path = HashSet::<((i32, i32), (i32, i32))>::new();
        while self.get(heading).is_some() {
            path.insert((position, direction));
            heading = (position.0 + direction.0, position.1 + direction.1);
            while let Some('#') = self.get(heading) {
                direction = (-direction.1, direction.0);
                heading = (position.0 + direction.0, position.1 + direction.1);
            }
            position = heading;
            if path.contains(&(position, direction)) {
                return None;
            }
        }
        let positions: HashSet<(i32, i32)> = path.iter().map(|(pos, _)| *pos).collect();
        Some(positions)
    }
}

#[aocd(2024, 6)]
pub fn solution1() {
    let data = input!();
    let map = Grid::new(data);
    let path = map.patrol().unwrap();
    submit!(1, path.len());
}

#[aocd(2024, 6)]
pub fn solution2() {
    let data = input!();
    let mut map = Grid::new(data);
    let mut path = map.patrol().unwrap();
    path.remove(&map.start);
    let obstacles: HashSet<(i32, i32)> = path
        .into_iter()
        .filter(|&pos| {
            map.set(pos, '#');
            let escape = map.patrol();
            map.set(pos, '.');
            escape.is_none()
        })
        .collect();
    submit!(2, obstacles.len());
}
