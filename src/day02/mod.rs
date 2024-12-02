use aocd::*;

struct Level {
    values: Vec<i64>
}

impl From<&str> for Level {
    fn from(s: &str) -> Self {
        Level {
            values: s.split(" ").map(|num| num.parse::<i64>().unwrap()).collect()
        }
    }
}

impl Level {
    fn is_monotonic(&self) -> bool {
        let increasing = self.values.windows(2).all(|pair| pair[1] > pair[0]);
        let decreasing = self.values.windows(2).all(|pair| pair[0] > pair[1]);
        increasing || decreasing
    }

    fn is_valid_step_size(&self) -> bool {
        self.values.windows(2).all(|pair| {
            let diff = (pair[1] - pair[0]).abs();
            diff >= 1 && diff <= 3
        })
    }

    fn is_safe(&self) -> bool {
        self.is_monotonic() && self.is_valid_step_size()
    }

    fn is_partially_safe(&self) -> bool {
        let mut valid: bool = false;
        for i in 0..self.values.len() {
            let mut values_copy = self.values.clone();
            values_copy.remove(i);
            let level = Level { values: values_copy };
            valid |= level.is_safe();
        }
        valid
    }
}

#[aocd(2024, 2)]
pub fn solution1() {
    let safe_levels: u64 = input!()
        .split("\n")
        .map(|s| Level::from(s).is_safe() as u64)
        .sum();

    submit!(1, safe_levels);
}

#[aocd(2024, 2)]
pub fn solution2() {
    let safe_levels: u64 = input!()
        .split("\n")
        .map(|s| Level::from(s).is_partially_safe() as u64)
        .sum();

    submit!(2, safe_levels);
}
