use std::collections::HashSet;

use aocd::*;

#[derive(Clone)]
struct Grid {
    values: Vec<char>,
    width: i32,
    height: i32,
}

impl Grid {
    fn new(input: String) -> Grid {
        let values: Vec<char> = input.lines().flat_map(|s| s.chars()).collect();
        let width = input.lines().next().unwrap().len();
        let height = values.len() / width;
        Grid {
            values,
            width: width as i32,
            height: height as i32,
        }
    }

    fn get(&self, xy: (i32, i32)) -> Option<char> {
        if !(0..self.width).contains(&xy.0) || !(0..self.height).contains(&xy.1) {
            None
        } else {
            let index = xy.1 * self.width + xy.0;
            Some(self.values[index as usize])
        }
    }

    fn set(&mut self, xy: (i32, i32), c: char) {
        if (0..self.width).contains(&xy.0) && (0..self.height).contains(&xy.1) {
            let index = xy.1 * self.width + xy.0;
            self.values[index as usize] = c;
        }
    }

    fn get_guard_position(&self) -> Option<(i32, i32)> {
        for x in 0..self.width {
            for y in 0..self.height {
                if let Some('^') = self.get((x, y)) {
                    return Some((x, y));
                }
            }
        }
        None
    }
}

#[aocd(2024, 6)]
pub fn solution1() {
    let data = input!();
    let map = Grid::new(data);
    let mut position = map.get_guard_position().unwrap();
    let mut direction = (0, -1);
    let mut path = HashSet::<((i32, i32), (i32, i32))>::new();
    'patrol: loop {
        path.insert((position, direction));
        let mut heading = (position.0 + direction.0, position.1 + direction.1);
        while let Some('#') = map.get(heading) {
            direction = (-direction.1, direction.0);
            heading = (position.0 + direction.0, position.1 + direction.1);
        }
        if map.get(heading).is_none() {
            break 'patrol;
        }
        position = (position.0 + direction.0, position.1 + direction.1);
    }
    
    let positions: HashSet<(i32, i32)> = path.iter().map(|(pos, _)| *pos).collect();

    submit!(1, positions.len());
} 

#[aocd(2024, 6)]
pub fn solution2() {
    let data = input!();
    let map = Grid::new(data);
    let mut position = map.get_guard_position().unwrap();
    let start = position;
    let mut direction = (0, -1);
    let mut path = HashSet::<((i32, i32), (i32, i32))>::new();
    'patrol: loop {
        path.insert((position, direction));
        let mut heading = (position.0 + direction.0, position.1 + direction.1);
        while let Some('#') = map.get(heading) {
            direction = (-direction.1, direction.0);
            heading = (position.0 + direction.0, position.1 + direction.1);
        }
        if map.get(heading).is_none() {
            break 'patrol;
        }
        position = (position.0 + direction.0, position.1 + direction.1);
    }
    
    let positions: HashSet<(i32, i32)> = path.iter().map(|(pos, _)| *pos).collect();

    let mut obstacles = HashSet::<(i32, i32)>::new();
    for obstacle in positions.into_iter() {
        if obstacle != start {
            let mut map_copy = map.clone();
            map_copy.set(obstacle, '#');
            let mut position = map_copy.get_guard_position().unwrap();
            let mut direction = (0, -1);
            let mut path = HashSet::<((i32, i32), (i32, i32))>::new();
            'simulation: loop {
                path.insert((position, direction));
                let mut heading = (position.0 + direction.0, position.1 + direction.1);
                while let Some('#') = map_copy.get(heading) {
                    direction = (-direction.1, direction.0);
                    heading = (position.0 + direction.0, position.1 + direction.1);
                }
                if map_copy.get(heading).is_none() {
                    break 'simulation;
                }
                position = (position.0 + direction.0, position.1 + direction.1);
                if path.contains(&(heading, direction)) {
                    obstacles.insert(obstacle);
                    break 'simulation;
                }
            }
        }
    }

    submit!(2, obstacles.len());
}