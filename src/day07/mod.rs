use aocd::*;
use itertools::{repeat_n, Itertools};
use rayon::prelude::*;
use regex::Regex;

#[derive(Debug)]
struct Equation {
    values: Vec<i64>,
    total: i64,
}

impl From<&str> for Equation {
    fn from(value: &str) -> Self {
        let re = Regex::new(r"(.+): (.+)").unwrap();
        let caps = re.captures(value).unwrap();
        let total: i64 = caps[1].parse().unwrap();
        let values: Vec<i64> = caps[2].split(' ').map(|n| n.parse().unwrap()).collect();
        Equation { values, total }
    }
}

impl Equation {
    fn solve(&self, operators: Vec<u8>) -> i64 {
        self.values
            .iter()
            .enumerate()
            .skip(1)
            .fold(self.values[0], |acc, (idx, &num)| {
                match operators[idx - 1] {
                    0 => acc * num,
                    1 => acc + num,
                    _ => format!("{}{}", acc, num).parse().unwrap()
                }
            })
    }

    fn solvable(&self, n_operators: u8) -> bool {
        repeat_n(0..n_operators, self.values.len() - 1)
            .multi_cartesian_product()
            .any(|combo| self.solve(combo) == self.total)
    }
}

#[aocd(2024, 7)]
pub fn solution1() {
    let data: Vec<Equation> = input!().lines().map(Equation::from).collect();
    let score: i64 = data
        .par_iter()
        .filter_map(|eq| eq.solvable(2).then_some(eq.total))
        .sum();
    submit!(1, score);
}

#[aocd(2024, 7)]
pub fn solution2() {
    let data: Vec<Equation> = input!().lines().map(Equation::from).collect();
    let score: i64 = data
        .par_iter()
        .filter_map(|eq| eq.solvable(3).then_some(eq.total))
        .sum();
    submit!(2, score);
}
