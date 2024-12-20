use crate::utils::{read_grid, XY};
use aocd::*;
use std::collections::HashMap;

const INF: i32 = i32::MAX / 2;

fn h(a: XY, b: XY) -> i32 {
    (b.x - a.x).abs() + (b.y - a.y).abs()
}

fn reconstruct_path(prev: &HashMap<XY, XY>, mut current: XY) -> Vec<XY> {
    let mut total_path = vec![current];
    while let Some(prior) = prev.get(&current) {
        current = *prior;
        total_path.push(current);
    }
    total_path.reverse();
    total_path
}

fn a_star(start: XY, goal: XY, map: &HashMap<XY, char>) -> Vec<XY> {
    let mut open = vec![start];
    let mut prev = HashMap::<XY, XY>::new();
    let mut g_score = HashMap::<XY, i32>::new();
    g_score.insert(start, 0);
    let mut f_score = HashMap::<XY, i32>::new();
    f_score.insert(start, h(start, goal));
    while !open.is_empty() {
        open.sort_by_key(|node| -f_score.get(node).unwrap_or(&INF));
        let current = open.pop().unwrap();
        if current == goal {
            return reconstruct_path(&prev, current);
        }
        'search: for neighbor in current.neighbors() {
            if map.get(&neighbor) == Some(&'#') || map.get(&neighbor).is_none() {
                continue 'search;
            }
            let alt = g_score.get(&current).unwrap_or(&INF) + 1;
            if alt < *g_score.get(&neighbor).unwrap_or(&INF) {
                prev.insert(neighbor, current);
                g_score.insert(neighbor, alt);
                f_score.insert(neighbor, alt + h(neighbor, goal));
                if !open.contains(&neighbor) {
                    open.push(neighbor);
                }
            }
        }
    }
    Vec::new()
}

fn find_char(grid: &HashMap<XY, char>, c: char) -> Option<XY> {
    grid.iter().find(|&(_, &v)| v == c).map(|(k, _)| k).copied()
}

fn solution(input: String, cheat_time: usize) -> usize {
    let grid = read_grid(input.as_str());
    let start = find_char(&grid, 'S').unwrap();
    let end = find_char(&grid, 'E').unwrap();
    let path = a_star(start, end, &grid);
    let mut cheats = 0;
    for (i, &a) in path.iter().enumerate() {
        for j in (i+1)..path.len() {
            let b = path[j];
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
