use crate::utils::{xy, Grid, XY};
use aocd::*;
use std::collections::{HashMap, HashSet};

struct Map {
    robot: XY,
    map: HashMap<XY, char>,
}

impl From<&str> for Map {
    fn from(input: &str) -> Map {
        let data: Vec<&str> = input.split("\n\n").collect();
        let mut grid = Grid::from(data[0]);
        grid.data.retain(|_, v| *v != '.');
        let mut map = HashMap::<XY, char>::new();
        let mut robot = XY::default();
        for (pos, &c) in grid.data.iter() {
            if c == 'O' {
                map.insert(xy(pos.x * 2, pos.y), '[');
                map.insert(xy(pos.x * 2 + 1, pos.y), ']');
            } else if c == '#' {
                map.insert(xy(pos.x * 2, pos.y), c);
                map.insert(xy(pos.x * 2 + 1, pos.y), c);
            } else if c == '@' {
                robot = xy(pos.x * 2, pos.y);
            }
        }
        Map { robot, map }
    }
}

impl Map {
    fn get(&self, position: XY) -> char {
        *self.map.get(&position).unwrap_or(&'.')
    }

    fn move_boxes(&mut self, box_group: HashSet<XY>, movement: XY) {
        let inserts: HashMap<XY, char> = box_group
            .iter()
            .map(|&pos| (pos + movement, self.get(pos)))
            .collect();
        self.map.retain(|k, _| !box_group.contains(k));
        self.map.extend(inserts);
    }

    fn get_box_group(&self, pos: XY, movement: XY, mut current: HashSet<XY>) -> HashSet<XY> {
        current.insert(pos);
        let adjacent = match self.get(pos) {
            '[' => pos + xy(1, 0),
            ']' => pos + xy(-1, 0),
            _ => unreachable!(),
        };
        if !current.contains(&adjacent) {
            current = self.get_box_group(adjacent, movement, current);
        }
        if self.get(pos + movement) == '[' || self.get(pos + movement) == ']' {
            current = self.get_box_group(pos + movement, movement, current);
        }
        current
    }

    fn process_moves(&mut self, moves: Vec<XY>) {
        for movement in moves {
            let next = self.robot + movement;
            if self.get(next) == '.' {
                self.robot = next;
            } else if self.get(next) == '[' || self.get(next) == ']' {
                let boxes = self.get_box_group(next, movement, HashSet::new());
                if !boxes.iter().any(|&pos| self.get(pos + movement) == '#') {
                    self.robot = next;
                    self.move_boxes(boxes, movement);
                }
            }
        }
    }

    fn gps(&self) -> i32 {
        self.map
            .iter()
            .filter(|(_, &tile)| tile == '[')
            .map(|(&pos, _)| pos.y * 100 + pos.x)
            .sum()
    }
}

fn read_moves(input: &str) -> Vec<XY> {
    let data: Vec<&str> = input.split("\n\n").collect();
    data[1]
        .lines()
        .flat_map(|line| line.chars())
        .map(|c| match c {
            '^' => xy(0, -1),
            'v' => xy(0, 1),
            '<' => xy(-1, 0),
            '>' => xy(1, 0),
            _ => panic!("unknown character"),
        })
        .collect()
}

#[aocd(2024, 15)]
pub fn solution2() {
    let data = input!();
    let mut map = Map::from(data.as_str());
    let moves = read_moves(&data);
    map.process_moves(moves);
    submit!(2, map.gps());
}
