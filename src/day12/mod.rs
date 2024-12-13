use aocd::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Region {
    plant: char,
    gardens: HashSet<(i32, i32)>,
}

impl Region {
    fn has(&self, point: (i32, i32)) -> bool {
        self.gardens.contains(&point)
    }

    fn cost(&self) -> usize {
        let perimeter: usize = self.gardens.iter().map(|garden| self.sides(garden)).sum();
        perimeter * self.gardens.len()
    }

    fn cost2(&self) -> usize {
        let perimeter: usize = self.gardens.iter().map(|garden| self.corners(garden)).sum();
        perimeter * self.gardens.len()
    }

    fn sides(&self, point: &(i32, i32)) -> usize {
        [(-1, 0), (1, 0), (0, 1), (0, -1)]
            .iter()
            .map(|(x, y)| !self.has((point.0 + x, point.1 + y)) as usize)
            .sum()
    }

    fn corners(&self, point: &(i32, i32)) -> usize {
        let mut corners = 0;
        for xy in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
            let a = (point.0 + xy.0, point.1 + xy.1);
            let b = (point.0 - xy.1, point.1 + xy.0);
            let c = (point.0 + xy.0 - xy.1, point.1 + xy.1 + xy.0);
            let outer = !self.has(a) && !self.has(b);
            let inner = self.has(a) && self.has(b) && !self.has(c);
            corners += (inner || outer) as usize;
        }
        corners
    }

    fn build(&mut self, current: (i32, i32), gardens: &mut HashMap<(i32, i32), char>) {
        self.gardens.insert(current);
        gardens.remove(&current);
        for offset in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
            let neighbor = (current.0 + offset.0, current.1 + offset.1);
            if gardens.get(&neighbor) == Some(&self.plant) && !self.has(neighbor) {
                self.build(neighbor, gardens);
            }
        }
    }
}

fn regions(input: String) -> Vec<Region> {
    let mut values: HashMap<(i32, i32), char> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x as i32, y as i32), c))
        })
        .collect();

    let mut regions = Vec::<Region>::new();
    while let Some((&garden, &plant)) = values.iter().next() {
        let gardens = HashSet::new();
        let mut region = Region { plant, gardens };
        region.build(garden, &mut values);
        regions.push(region);
    }
    regions
}

#[aocd(2024, 12)]
pub fn solution1() {
    let data = input!();
    let regions = regions(data);
    let score: usize = regions.iter().map(|region| region.cost()).sum();
    submit!(1, score);
}

#[aocd(2024, 12)]
pub fn solution2() {
    let data = input!();
    let regions = regions(data);
    let score: usize = regions.iter().map(|region| region.cost2()).sum();
    submit!(2, score);
}
