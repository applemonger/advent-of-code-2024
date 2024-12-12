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
        adjacent.iter().map(|&pos| self.offset(pos)).collect()
    }

    fn offset(&self, offset: (i32, i32)) -> Garden {
        Garden::new(self.x + offset.0, self.y + offset.1)
    }

    fn sides(&self, region: &Region) -> HashSet<Side> {
        let mut sides = HashSet::<Side>::new();
        for direction in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
            let neighbor = self.offset(direction);
            if !region.gardens.contains(&neighbor) {
                sides.insert(Side { position: *self, direction });
            }
        }
        sides
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct Side {
    position: Garden,
    direction: (i32, i32)
}

impl Side {
    fn vertical(&self) -> bool {
        self.direction.1 != 0
    }

    fn is_connected(&self, other: &Side, sides: &HashSet<Side>) -> bool {
        if self.position == other.position {
            return false;
        } else if self.position.x == other.position.x && self.direction.0 != 0 {
            let y_min = self.position.y.min(other.position.y) + 1;
            let y_max = self.position.y.max(other.position.y);
            for y in y_min..y_max {
                let garden = Garden::new(self.position.x, y);
                let side = Side { position: garden, direction: self.direction };
                if !sides.contains(&side) {
                    return false;
                }
            }
        } else if self.position.y == other.position.y && self.direction.1 != 0 {
            let x_min = self.position.x.min(other.position.x) + 1;
            let x_max = self.position.x.max(other.position.x);
            for x in x_min..x_max {
                let garden = Garden::new(x, self.position.y);
                let side = Side { position: garden, direction: self.direction };
                if !sides.contains(&side) {
                    return false;
                }
            }
        } else {
            return false;
        }
        self.direction == other.direction
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

    fn discounted_cost(&self) -> usize {
        let mut sides: HashSet<Side> = self.gardens.iter().flat_map(|garden| garden.sides(&self)).collect();
        //println!("{} {}", self.plant, sides.len());
        let all_sides = sides.clone();
        let mut count = 0;
        while let Some(side) = sides.iter().next().copied() {
            //println!("{} {:?}", self.plant, side);
            for other in all_sides.clone().iter().copied() {
                if side.is_connected(&other, &all_sides) {
                    //println!("-> {:?}", other);
                    sides.remove(&other);
                }
            }
            sides.remove(&side);
            count += 1;
        }
        count * self.gardens.len()
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

#[aocd(2024, 12)]
pub fn solution2() {
    let data = input!();
    let regions = Grid::new(data).to_regions();
    let score = regions.iter().map(|region| region.discounted_cost()).sum::<usize>();
    submit!(2, score);
}
