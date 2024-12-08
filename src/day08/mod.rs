use std::collections::{HashMap, HashSet};
use aocd::*;


#[derive(Clone)]
struct Grid {
    values: Vec<char>,
    width: i32,
    height: i32,
    nodes: HashMap<char, HashSet<(i32, i32)>>
}

impl Grid {
    fn new(input: String) -> Grid {
        let values: Vec<char> = input.lines().flat_map(|s| s.chars()).collect();
        let width = input.lines().next().unwrap().len() as i32;
        let height = values.len() as i32 / width;
        let mut grid = Grid {
            values,
            width,
            height,
            nodes: HashMap::new()
        };
        grid.find_nodes();
        grid
    }

    fn get(&self, xy: (i32, i32)) -> Option<char> {
        if (0..self.width).contains(&xy.0) && (0..self.height).contains(&xy.1) {
            let index = xy.1 * self.width + xy.0;
            let value = *self.values.get(index as usize).unwrap();
            Some(value)
        } else {
            None
        }
    }

    fn find_nodes(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let c = self.get((x, y)).unwrap();
                if c != '.' {
                    self.nodes.entry(c).or_default().insert((x, y));
                }
            }
        }
    }

    fn find_anti_nodes(&self) -> HashSet<(i32, i32)> {
        let mut antinodes = HashSet::<(i32, i32)>::new();
        for (_, nodes) in self.nodes.iter() {
            for (x, y) in nodes.clone().into_iter() {
                for (u, v) in nodes.clone().into_iter() {
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
            for (x, y) in nodes.clone().into_iter() {
                for (u, v) in nodes.clone().into_iter() {
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