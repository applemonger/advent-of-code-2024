use aocd::*;
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
struct Grid {
    values: HashMap<(i32, i32), char>,
    nodes: HashMap<char, HashSet<(i32, i32)>>,
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
        let mut grid = Grid {
            values,
            nodes: HashMap::new(),
        };
        grid.find_nodes();
        grid
    }

    fn get(&self, xy: (i32, i32)) -> Option<char> {
        self.values.get(&xy).copied()
    }

    fn find_nodes(&mut self) {
        for (&position, &c) in self.values.iter() {
            if c != '.' {
                self.nodes.entry(c).or_default().insert(position);
            }
        }
    }

    fn find_anti_nodes(&self) -> HashSet<(i32, i32)> {
        let mut antinodes = HashSet::<(i32, i32)>::new();
        for (_, nodes) in self.nodes.iter() {
            for (x, y) in nodes.iter() {
                for (u, v) in nodes.iter() {
                    if (x, y) != (u, v) {
                        let diff = (u - x, v - y);
                        let antinode = (x - diff.0, y - diff.1);
                        if self.get(antinode).is_some() {
                            antinodes.insert(antinode);
                        }
                    }
                }
            }
        }
        antinodes
    }

    fn find_harmonic_anti_nodes(&self) -> HashSet<(i32, i32)> {
        let mut antinodes = HashSet::<(i32, i32)>::new();
        for (_, nodes) in self.nodes.iter() {
            for (x, y) in nodes.iter() {
                for (u, v) in nodes.iter() {
                    if (x, y) != (u, v) {
                        let diff = (u - x, v - y);
                        let mut antinode_1 = (x - diff.0, y - diff.1);
                        let mut antinode_2 = (x + diff.0, y + diff.1);
                        while self.get(antinode_1).is_some() {
                            antinodes.insert(antinode_1);
                            antinode_1 = (antinode_1.0 - diff.0, antinode_1.1 - diff.1);
                        }
                        while self.get(antinode_2).is_some() {
                            antinodes.insert(antinode_2);
                            antinode_2 = (antinode_2.0 + diff.0, antinode_2.1 + diff.1);
                        }
                    }
                }
            }
        }
        antinodes
    }
}

#[aocd(2024, 8)]
pub fn solution1() {
    let data = input!();
    let grid = Grid::new(data);
    let antinodes = grid.find_anti_nodes();
    submit!(1, antinodes.len());
}

#[aocd(2024, 8)]
pub fn solution2() {
    let data = input!();
    let grid = Grid::new(data);
    let antinodes = grid.find_harmonic_anti_nodes();
    submit!(2, antinodes.len());
}
