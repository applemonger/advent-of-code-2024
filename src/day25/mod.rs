use std::collections::HashSet;

use aocd::*;

use crate::utils::{xy, Grid, XY};

fn read_data(input: String) -> (Vec<HashSet<XY>>, Vec<HashSet<XY>>) {
    let mut keys = Vec::<HashSet<XY>>::new();
    let mut locks = Vec::<HashSet<XY>>::new();
    input.split("\n\n").for_each(|block| {
        let mut grid = Grid::from(block);
        grid.data.retain(|_, &mut v| v == '#');
        if grid.data.get(&xy(0, 0)) == Some(&'#') {
            locks.push(grid.data.keys().copied().collect());
        } else {
            keys.push(grid.data.keys().copied().collect());
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
            if key.intersection(lock).count() == 0 {
                count += 1;
            }
        }
    }
    submit!(1, count);
}

#[aocd(2024, 25)]
pub fn solution2() {}
