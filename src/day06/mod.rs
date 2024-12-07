use std::collections::HashSet;

use aocd::*;

#[derive(Clone)]
struct Grid {
    values: Vec<char>,
    start: (i32, i32),
    width: i32,
    height: i32,
}

impl Grid {
    fn new(input: String) -> Grid {
        let values: Vec<char> = input.lines().flat_map(|s| s.chars()).collect();
        let guard = values.iter().position(|c| *c == '^').unwrap() as i32;
        let width = input.lines().next().unwrap().len() as i32;
        let height = values.len() as i32 / width;
        Grid {
            values,
            start: (guard % width, guard / width),
            width,
            height,
        }
    }

    fn get(&self, xy: (i32, i32)) -> Option<char> {
        let index = xy.1 * self.width + xy.0;
        if index >= 0 {
            let value = *self.values.get(index as usize)?;
            Some(value)
        } else {
            None
        }
    }

    fn set(&mut self, xy: (i32, i32), c: char) {
        if (0..self.width).contains(&xy.0) && (0..self.height).contains(&xy.1) {
            let index = xy.1 * self.width + xy.0;
            self.values[index as usize] = c;
        }
    }
}

fn patrol(map: &Grid) -> Option<HashSet<(i32, i32)>> {
    let mut position = map.start;
    let mut direction = (0, -1);
    let mut heading = (position.0 + direction.0, position.1 + direction.1);
    let mut path = HashSet::<((i32, i32), (i32, i32))>::new();
    while map.get(heading).is_some() {
        path.insert((position, direction));
        heading = (position.0 + direction.0, position.1 + direction.1);
        while let Some('#') = map.get(heading) {
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

#[aocd(2024, 6)]
pub fn solution1() {
    let data = input!();
    let map = Grid::new(data);
    let path = patrol(&map).unwrap();
    submit!(1, path.len());
}

#[aocd(2024, 6)]
pub fn solution2() {
    let data = input!();
    let mut map = Grid::new(data);
    let mut path = patrol(&map).unwrap();
    path.remove(&map.start);
    let obstacles: HashSet<(i32, i32)> = path
        .into_iter()
        .filter(|&pos| {
            map.set(pos, '#');
            let escape = patrol(&map);
            map.set(pos, '.');
            escape.is_none()
        })
        .collect();
    submit!(2, obstacles.len());
}
