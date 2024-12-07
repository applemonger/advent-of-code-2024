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

fn get_bit_at(input: u32, n: u8) -> bool {
    if n < 32 {
        input & (1 << n) != 0
    } else {
        false
    }
}

impl Equation {
    fn solvable(&self) -> bool {
        let num_operators = self.values.len() as u32;
        let combinations: u32 = 2_u32.pow(num_operators);
        (0..combinations).any(|i| {
            let first = self.values[0];
            let total: i64 =
                self.values
                    .iter()
                    .enumerate()
                    .skip(1)
                    .fold(first, |acc, (idx, &num)| {
                        if get_bit_at(i, idx as u8) {
                            acc * num
                        } else {
                            acc + num
                        }
                    });
            total == self.total
        })
    }

    fn solvable2(&self) -> bool {
        let operators = [1, 2, 3];
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
                            if *combo[idx - 1] == 1 {
                                acc * num
                            } else if *combo[idx - 1] == 2 {
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
        .filter(|equation| equation.solvable())
        .map(|equation| equation.total)
        .sum();
    submit!(1, score);
}

#[aocd(2024, 7)]
pub fn solution2() {
    let data: Vec<Equation> = input!().lines().map(Equation::from).collect();
    let score: i64 = data
        .iter()
        .filter(|equation| equation.solvable2())
        .map(|equation| equation.total)
        .sum();
    submit!(2, score);
}
