use aocd::*;
use itertools::{repeat_n, Itertools};
use regex::Regex;

#[derive(Debug)]
struct Equation {
    values: Vec<i64>,
    total: i64,
}

impl From<&str> for Equation {
    fn from(value: &str) -> Self {
        let re = Regex::new(r"(.+): (.+)").unwrap();
        let captures = re.captures(value).unwrap();
        let total: i64 = captures.get(1).unwrap().as_str().parse().unwrap();
        let values: Vec<i64> = captures
            .get(2)
            .unwrap()
            .as_str()
            .split(' ')
            .map(|num| num.parse().unwrap())
            .collect();
        Equation { values, total }
    }
}

impl Equation {
    fn solvable(&self, n_operators: i32) -> bool {
        let operators: Vec<i32> = (0..n_operators).collect();
        let num_values = self.values.len();
        repeat_n(operators.iter(), num_values - 1)
            .multi_cartesian_product()
            .any(|combo| {
                let first = self.values[0];
                let total: i64 =
                    self.values
                        .iter()
                        .enumerate()
                        .skip(1)
                        .fold(first, |acc, (idx, &num)| {
                            if *combo[idx - 1] == 0 {
                                acc * num
                            } else if *combo[idx - 1] == 1 {
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
        .iter()
        .filter_map(|eq| eq.solvable(2).then_some(eq.total))
        .sum();
    submit!(1, score);
}

#[aocd(2024, 7)]
pub fn solution2() {
    let data: Vec<Equation> = input!().lines().map(Equation::from).collect();
    let score: i64 = data
        .iter()
        .filter_map(|eq| eq.solvable(3).then_some(eq.total))
        .sum();
    submit!(2, score);
}
