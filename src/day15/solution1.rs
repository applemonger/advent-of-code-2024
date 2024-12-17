use aocd::*;
use std::collections::HashMap;

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
                    .filter(|&(_, c)| c == '.' || c == 'O' || c == '@')
                    .map(move |(x, c)| ((x as i32, y as i32), c))
            })
            .collect();
        let mut robot = (0, 0);
        for (&position, &c) in map.iter() {
            if c == '@' {
                robot = position;
            }
        }
        map.remove(&robot);
        map.insert(robot, '.');
        Map { robot, map }
    }
}

impl Map {
    fn process_moves(&mut self, moves: Vec<(i32, i32)>) {
        for (x, y) in moves {
            let mut next = (self.robot.0 + x, self.robot.1 + y);
            if self.map.get(&next) == Some(&'.') {
                self.robot = next;
            } else {
                let mut boxes = Vec::<(i32, i32)>::new();
                while self.map.get(&next) == Some(&'O') {
                    boxes.push(next);
                    next = (next.0 + x, next.1 + y);
                }
                if self.map.get(&next) == Some(&'.') && !boxes.is_empty() {
                    self.robot = (self.robot.0 + x, self.robot.1 + y);
                    self.map.insert(self.robot, '.');
                    self.map.insert(next, 'O');
                }
            }
        }
    }

    fn gps(&self) -> i32 {
        self.map
            .iter()
            .filter(|(_, &c)| c == 'O')
            .map(|(&pos, _)| pos.1 * 100 + pos.0)
            .sum()
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
pub fn solution1() {
    let data = input!();
    let mut map = Map::from(data.as_str());
    let moves = read_moves(&data);
    map.process_moves(moves);
    submit!(1, map.gps());
}
