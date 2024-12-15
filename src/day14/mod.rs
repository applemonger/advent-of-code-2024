use core::f64;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fmt::Display,
    io,
};

use aocd::*;
use regex::Regex;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Robot {
    start: (i32, i32),
    position: (i32, i32),
    velocity: (i32, i32),
}

impl From<&str> for Robot {
    fn from(value: &str) -> Self {
        let re = Regex::new(r"(-*\d+)").unwrap();
        let caps: Vec<i32> = re
            .captures_iter(value)
            .map(|n| n[1].parse().unwrap())
            .collect();
        Robot {
            start: (caps[0], caps[1]),
            position: (caps[0], caps[1]),
            velocity: (caps[2], caps[3]),
        }
    }
}

impl Robot {
    fn go(&mut self, t: i32, x_limit: i32, y_limit: i32) {
        let x = self.position.0 + self.velocity.0 * t;
        let y = self.position.1 + self.velocity.1 * t;
        let x = (x % x_limit + x_limit) % x_limit;
        let y = (y % y_limit + y_limit) % y_limit;
        self.position = (x, y);
    }

    fn quadrant(&self, x_limit: i32, y_limit: i32) -> Option<(usize, usize)> {
        let x = match self.position.0.cmp(&(x_limit / 2)) {
            Ordering::Greater => 1,
            Ordering::Less => 0,
            Ordering::Equal => return None,
        };
        let y = match self.position.1.cmp(&(y_limit / 2)) {
            Ordering::Greater => 1,
            Ordering::Less => 0,
            Ordering::Equal => return None,
        };
        Some((x, y))
    }

    fn is_start(&self) -> bool {
        self.position == self.start
    }
}

struct Robots {
    robots: Vec<Robot>,
    limits: (i32, i32),
}

impl From<&str> for Robots {
    fn from(value: &str) -> Self {
        Robots {
            robots: value.lines().map(Robot::from).collect(),
            limits: (0, 0),
        }
    }
}

impl Display for Robots {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut grid = HashMap::<(i32, i32), char>::new();
        self.robots.iter().for_each(|robot| {
            grid.insert(robot.position, '#');
        });
        let mut result = String::new();
        for y in 0..self.limits.1 {
            let mut row = String::new();
            for x in 0..self.limits.0 {
                row.push(*grid.get(&(x, y)).unwrap_or(&' '));
            }
            result += &row;
            result.push('\n');
        }
        write!(f, "{}", result)
    }
}

impl Robots {
    fn set_limits(&mut self, x_limit: i32, y_limit: i32) {
        self.limits = (x_limit, y_limit);
    }

    fn forward(&mut self, t: i32) {
        self.robots
            .iter_mut()
            .for_each(|robot| robot.go(t, self.limits.0, self.limits.1));
    }

    fn quadrants(&self) -> HashMap<(usize, usize), i32> {
        let mut quadrants = HashMap::<(usize, usize), i32>::new();
        self.robots
            .iter()
            .filter_map(|robot: &Robot| robot.quadrant(self.limits.0, self.limits.1))
            .for_each(|quadrant| {
                *quadrants.entry(quadrant).or_default() += 1;
            });
        quadrants
    }

    fn regions(&self) -> Vec<Region> {
        let mut points: HashSet<(i32, i32)> = self.robots.iter().map(|robot| robot.position).collect();
        let mut regions = Vec::<Region>::new();
        while let Some(&garden) = points.iter().next() {
            let mut region = Region::default();
            region.build(garden, &mut points);
            regions.push(region);
        }
        regions
    }
}

#[derive(Default)]
struct Region {
    points: HashSet<(i32, i32)>
}

impl Region {
    fn build(&mut self, current: (i32, i32), points: &mut HashSet<(i32, i32)>) {
        self.points.insert(current);
        points.remove(&current);
        for offset in [(-1, 0), (1, 0), (0, 1), (0, -1)] {
            let neighbor = (current.0 + offset.0, current.1 + offset.1);
            if points.contains(&neighbor) && !self.points.contains(&neighbor) {
                self.build(neighbor, points);
            }
        }
    }
}

#[aocd(2024, 14)]
pub fn solution1() {
    let data = input!();
    let mut robots = Robots::from(data.as_str());
    robots.set_limits(101, 103);
    robots.forward(100);
    println!("{}", robots);
    let safety: i32 = robots.quadrants().values().copied().product();
    submit!(1, safety);
}

#[aocd(2024, 14)]
pub fn solution2() {
    let data = input!();
    let mut robots = Robots::from(data.as_str());
    robots.set_limits(101, 103);

    // Get periodicity
    let mut period = 0;
    loop {
        robots.forward(1);
        period += 1;
        if robots.robots.iter().all(|robot| robot.is_start()) {
            break;
        }
    }

    // Get number of regions within each step
    let mut regions = Vec::new();
    for i in 0..period {
        robots.forward(1);
        let n: usize = robots.regions().len();
        regions.push(n as f64);
    }

    // Look for outliers
    let mean: f64 = regions.iter().sum::<f64>() / regions.len() as f64;
    let sd: f64 = (regions.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / regions.len() as f64).sqrt();
    let outliers: Vec<usize> = regions
        .iter()
        .enumerate()
        .filter(|&(_, x)| ((*x - mean).abs() / sd) > 10.)
        .map(|(i, _)| i + 1)
        .collect();

    // Examine
    for i in outliers {
        let data = input!();
        let mut robots = Robots::from(data.as_str());
        robots.set_limits(101, 103);
        robots.forward(i as i32);
        println!("{}", robots);
        println!("{}", i);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("error");
    }
}
