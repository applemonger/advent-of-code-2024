use aocd::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct Garden {
    x: i32,
    y: i32,
}

impl Garden {
    fn new(x: i32, y: i32) -> Garden {
        Garden { x, y }
    }

    fn neighbors(&self) -> HashSet<Garden> {
        let adjacent = [(-1, 0), (1, 0), (0, 1), (0, -1)];
        adjacent.iter().map(|&(x, y)| self.offset(x, y)).collect()
    }

    fn offset(&self, x: i32, y: i32) -> Garden {
        Garden::new(self.x + x, self.y + y)
    }
}

#[derive(Debug)]
struct Region {
    plant: char,
    gardens: HashSet<Garden>,
}

impl Region {
    fn new(plant: char) -> Region {
        Region {
            plant,
            gardens: HashSet::new(),
        }
    }

    fn cost(&self) -> usize {
        let perimeter: usize = self
            .gardens
            .iter()
            .map(|garden| garden.neighbors().difference(&self.gardens).count())
            .sum();
        perimeter * self.gardens.len()
    }

    fn build(&mut self, current: Garden, grid: &mut Grid) {
        self.gardens.insert(current);
        grid.values.remove(&current);
        current.neighbors().iter().for_each(|&neighbor| {
            if let Some(c) = grid.get(neighbor) {
                if c == self.plant && !self.gardens.contains(&neighbor) {
                    self.build(neighbor, grid);
                }
            }
        });
    }
}

#[derive(Clone)]
struct Grid {
    values: HashMap<Garden, char>,
}

impl Grid {
    fn new(input: String) -> Grid {
        let values: HashMap<Garden, char> = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, c)| (Garden::new(x as i32, y as i32), c))
            })
            .collect();
        Grid { values }
    }

    fn get(&self, garden: Garden) -> Option<char> {
        self.values.get(&garden).copied()
    }

    fn to_regions(mut self) -> Vec<Region> {
        let mut regions = Vec::<Region>::new();
        while let Some((garden, plant)) = self.values.iter().next() {
            let mut region = Region::new(*plant);
            region.build(*garden, &mut self);
            regions.push(region);
        }
        regions
    }
}

#[aocd(2024, 12)]
pub fn solution1() {
    let data = input!();
    let regions = Grid::new(data).to_regions();
    let score = regions.iter().map(|region| region.cost()).sum::<usize>();
    submit!(1, score);
}

#[aocd(2024, 12, "src/day12/test2.txt")]
pub fn solution2() {}
