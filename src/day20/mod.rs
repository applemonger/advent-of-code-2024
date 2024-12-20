use crate::utils::{read_grid, XY};
use aocd::*;
use std::collections::HashMap;

fn h(a: XY, b: XY) -> i32 {
    (b.x - a.x).abs() + (b.y - a.y).abs()
}

fn path(start: XY, goal: XY, map: &HashMap<XY, char>) -> Vec<XY> {
    let mut current = start;
    let mut open = vec![start];
    'trail: while current != goal {
        for neighbor in current.neighbors() {
            if map.get(&neighbor) != Some(&'#') && !map.get(&neighbor).is_none() {
                if !open.contains(&neighbor) {
                    open.push(neighbor);
                    current = neighbor;
                    continue 'trail;
                }
            }
        }
    }
    open
}

fn find_char(grid: &HashMap<XY, char>, c: char) -> Option<XY> {
    grid.iter().find(|&(_, &v)| v == c).map(|(k, _)| k).copied()
}

fn solution(input: String, cheat_time: usize) -> usize {
    let grid = read_grid(input.as_str());
    let start = find_char(&grid, 'S').unwrap();
    let end = find_char(&grid, 'E').unwrap();
    let path = path(start, end, &grid);
    let mut cheats = 0;
    for (i, &a) in path.iter().enumerate() {
        for (j, &b) in path.iter().enumerate().skip(i + 1) {
            let dist = h(a, b) as usize;
            if dist <= cheat_time && dist < (j - i) {
                let shave = (j - i) - dist;
                cheats += (shave >= 100) as usize;
            }
        }
    }
    cheats
}

#[aocd(2024, 20)]
pub fn solution1() {
    let data = input!();
    submit!(1, solution(data, 2));
}

#[aocd(2024, 20)]
pub fn solution2() {
    let data = input!();
    submit!(2, solution(data, 20));
}
