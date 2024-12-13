use aocd::*;
use regex::Regex;

#[derive(Default, Debug)]
struct Machine {
    a: (f64, f64),
    b: (f64, f64),
    prize: (f64, f64),
    upper: f64,
    extra: f64,
}

impl From<&str> for Machine {
    fn from(s: &str) -> Self {
        let re = Regex::new(r"[X|Y][\+|=](\d+)").unwrap();
        let caps: Vec<f64> = re.captures_iter(s).map(|c| c[1].parse().unwrap()).collect();
        Machine {
            a: (caps[0], caps[1]),
            b: (caps[2], caps[3]),
            prize: (caps[4], caps[5]),
            ..Default::default()
        }
    }
}

impl Machine {
    fn solve(&self) -> Option<f64> {
        let (xa, ya) = self.a;
        let (xb, yb) = self.b;
        let (xp, yp) = (self.prize.0 + self.extra, self.prize.1 + self.extra);
        let b = (xp * ya - yp * xa) / (xb * ya - yb * xa);
        let a = (xp - b * xb) / xa;
        let out_of_bounds = self.out_of_bounds(a) || self.out_of_bounds(b);
        let within_epsilon = self.within_epsilon(a) && self.within_epsilon(b);
        (!out_of_bounds && within_epsilon).then_some(a * 3.0 + b)
    }

    fn out_of_bounds(&self, num: f64) -> bool {
        num < 0. || num > self.upper
    }

    fn within_epsilon(&self, num: f64) -> bool {
        num > num.round() - 1e-3 && num < num.round() + 1e-3
    }
}

#[aocd(2024, 13)]
pub fn solution1() {
    let data = input!();
    let mut machines: Vec<Machine> = data.split("\n\n").map(Machine::from).collect();
    machines.iter_mut().for_each(|machine| machine.upper = 100.);
    let tokens: f64 = machines.iter().filter_map(|machine| machine.solve()).sum();
    submit!(1, tokens);
}

#[aocd(2024, 13)]
pub fn solution2() {
    let data = input!();
    let mut machines: Vec<Machine> = data.split("\n\n").map(Machine::from).collect();
    machines
        .iter_mut()
        .for_each(|machine| machine.upper = f64::MAX);
    machines
        .iter_mut()
        .for_each(|machine| machine.extra = 10000000000000.);
    let tokens: f64 = machines.iter().filter_map(|machine| machine.solve()).sum();
    submit!(2, tokens);
}
