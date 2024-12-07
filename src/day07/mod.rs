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
    fn solvable(&self, n_operators: i32) -> bool {
        repeat_n(0..n_operators, self.values.len() - 1)
            .multi_cartesian_product()
            .any(|combo| {
                let total: i64 =
                    self.values
                        .iter()
                        .enumerate()
                        .skip(1)
                        .fold(self.values[0], |acc, (idx, &num)| {
                            if combo[idx - 1] == 0 {
                                acc * num
                            } else if combo[idx - 1] == 1 {
                                acc + num
                            } else {
                                format!("{}{}", acc, num).parse().unwrap()
                            }
                        });
                total == self.total
            })
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
