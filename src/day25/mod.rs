use aocd::*;

use crate::utils::{xy, Grid};

#[derive(Debug)]
struct Key {
    values: [u8; 5],
}

impl Key {
    fn fits(&self, lock: &Lock) -> bool {
        self.values
            .iter()
            .zip(lock.values.iter())
            .all(|(a, b)| *a + *b < 6)
    }
}

#[derive(Debug)]
struct Lock {
    values: [u8; 5],
}

fn read_data(input: String) -> (Vec<Key>, Vec<Lock>) {
    let mut keys = Vec::new();
    let mut locks = Vec::new();
    input.split("\n\n").for_each(|block| {
        let grid = Grid::from(block);
        let mut values = [0, 0, 0, 0, 0];
        for (point, _) in grid.data.iter().filter(|(_, &v)| v == '#') {
            values[point.x as usize] += 1;
        }
        values.iter_mut().for_each(|n| *n -= 1);
        if grid.data.get(&xy(0, 0)) == Some(&'#') {
            locks.push(Lock { values });
        } else {
            keys.push(Key { values });
        }
    });
    (keys, locks)
}

#[aocd(2024, 25)]
pub fn solution1() {
    let data = input!();
    let (keys, locks) = read_data(data);
    let mut count = 0;
    for key in keys.iter() {
        for lock in locks.iter() {
            if key.fits(lock) {
                count += 1;
            }
        }
    }
    submit!(1, count);
}

#[aocd(2024, 25)]
pub fn solution2() {}
