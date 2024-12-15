use aocd::*;
use std::collections::{HashMap, HashSet};

struct Map {
    robot: (i32, i32),
    map: HashMap<(i32, i32), char>,
}

impl From<&str> for Map {
    fn from(input: &str) -> Map {
        let data: Vec<&str> = input.split("\n\n").collect();
        let mut map: HashMap<(i32, i32), char> = data[0]
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|&(_, c)| c != '.')
                    .flat_map(move |(x, c)| {
                        let (x, y) = (x as i32, y as i32);
                        match c {
                            '@' => vec![((x * 2, y), c)],
                            'O' => vec![((x * 2, y), '['), ((2 * x + 1, y), ']')],
                            _ => vec![((x * 2, y), c), ((2 * x + 1, y), c)],
                        }
                    })
            })
            .collect();
        let mut robot = (0, 0);
        if let Some((&position, _)) = map.iter().find(|&(_, v)| *v == '@') {
            robot = position;
        }
        map.remove(&robot);
        Map { robot, map }
    }
}

impl Map {
    fn get(&self, position: (i32, i32)) -> char {
        *self.map.get(&position).unwrap_or(&'.')
    }

    fn can_move(&self, box_group: &HashSet<(i32, i32)>, movement: (i32, i32)) -> bool {
        !box_group
            .iter()
            .any(|pos| self.map.get(&(pos.0 + movement.0, pos.1 + movement.1)) == Some(&'#'))
    }

    fn move_boxes(&mut self, box_group: HashSet<(i32, i32)>, movement: (i32, i32)) {
        let inserts: HashMap<(i32, i32), char> = box_group
            .iter()
            .map(|pos| {
                (
                    (pos.0 + movement.0, pos.1 + movement.1),
                    *self.map.get(pos).unwrap(),
                )
            })
            .collect();
        self.map.retain(|k, _| !box_group.contains(k));
        for pos in box_group {
            self.map.remove(&pos);
        }
        self.map.extend(inserts);
    }

    fn get_box_group(
        &self,
        position: (i32, i32),
        movement: (i32, i32),
        mut current: HashSet<(i32, i32)>,
    ) -> HashSet<(i32, i32)> {
        current.insert(position);
        let adjacent = match self.get(position) {
            '[' => (position.0 + 1, position.1),
            ']' => (position.0 - 1, position.1),
            _ => unreachable!(),
        };
        if !current.contains(&adjacent) {
            current = self.get_box_group(adjacent, movement, current);
        }
        let next = (position.0 + movement.0, position.1 + movement.1);
        if !current.contains(&next) && (self.get(next) == '[' || self.get(next) == ']') {
            current = self.get_box_group(next, movement, current);
        }
        current
    }

    fn process_moves(&mut self, moves: Vec<(i32, i32)>) {
        for (x, y) in moves {
            let next = (self.robot.0 + x, self.robot.1 + y);
            if self.get(next) == '.' {
                self.robot = next;
            } else if self.get(next) == '[' || self.get(next) == ']' {
                let boxes = self.get_box_group(next, (x, y), HashSet::new());
                if self.can_move(&boxes, (x, y)) {
                    self.robot = next;
                    self.move_boxes(boxes, (x, y));
                }
            }
        }
    }

    fn gps(&self) -> i32 {
        self.map
            .iter()
            .filter(|(_, &tile)| tile == '[')
            .map(|(&pos, _)| pos.1 * 100 + pos.0)
            .sum()
    }

    fn display(&self) {
        let y_max = self.map.keys().map(|k| k.1).max().unwrap();
        let x_max = self.map.keys().map(|k| k.0).max().unwrap();
        for y in 0..=y_max {
            let mut row = String::new();
            for x in 0..=x_max {
                if (x, y) == self.robot {
                    row.push('@');
                } else if let Some(&tile) = self.map.get(&(x, y)) {
                    row.push(tile);
                } else {
                    row.push(' ');
                }
            }
            println!("{}", row);
        }
        println!();
    }
}

fn read_moves(input: &str) -> Vec<(i32, i32)> {
    let data: Vec<&str> = input.split("\n\n").collect();
    data[1]
        .lines()
        .flat_map(|line| line.chars())
        .map(|c| match c {
            '^' => (0, -1),
            'v' => (0, 1),
            '<' => (-1, 0),
            '>' => (1, 0),
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
    map.display();
    submit!(2, map.gps());
}
