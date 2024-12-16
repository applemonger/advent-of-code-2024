use std::collections::{HashMap, HashSet};
use aocd::*;
use crate::utils::{cardinals, read_grid, xy, XY};

const INF: i32 = i32::MAX / 2;

fn heuristic(a: XY, b: XY) -> i32 {
    (b.x - a.x).abs() + (b.y - a.y).abs()
}

fn reconstruct_path(came_from: HashMap<(XY, XY), (XY, XY)>, mut current: (XY, XY)) -> Vec<XY> {
    let mut total_path = vec![current];
    while came_from.contains_key(&current) {
        current = *came_from.get(&current).unwrap();
        total_path.push(current);
    }
    total_path.reverse();
    total_path.iter().map(|node| node.0).collect()
}

fn get_min_open(open_set: &HashSet<(XY, XY)>, f_score: &HashMap<(XY, XY), i32>) -> (XY, XY) {
    let mut open_vec: Vec<(XY, XY)> = open_set.iter().copied().collect();
    open_vec.sort_by_key(|node| -f_score.get(node).unwrap_or(&INF));
    open_vec.pop().unwrap()
}

fn a_star(start: XY, goal: XY, grid: &HashMap<XY, char>) -> Vec<XY> {
    let mut open_set = HashSet::<(XY, XY)>::new();
    open_set.insert((start, xy(1, 0)));
    let mut came_from = HashMap::<(XY, XY), (XY, XY)>::new();
    let mut g_score = HashMap::<(XY, XY), i32>::new();
    g_score.insert((start, xy(1, 0)), 0);
    let mut f_score = HashMap::<(XY, XY), i32>::new();
    f_score.insert((start, xy(1, 0)), heuristic(start, goal));
    while !open_set.is_empty() {
        let current = get_min_open(&open_set, &f_score);
        if current.0 == goal {
            return reconstruct_path(came_from, current);
        }
        open_set.remove(&current);
        'neighbors: for movement in cardinals() {
            let neighbor = (current.0 + movement, movement);
            if grid.get(&neighbor.0) == Some(&'#') {
                continue 'neighbors;
            }
            let cost = 1 + (movement != current.1) as i32 * 1000;
            let tentative_g_score = g_score.get(&current).unwrap_or(&INF) + cost;
            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&INF) {
                came_from.insert(neighbor, current);
                g_score.insert(neighbor, tentative_g_score);
                f_score.insert(neighbor, tentative_g_score + heuristic(neighbor.0, goal));
                if !open_set.contains(&neighbor) {
                    open_set.insert(neighbor);
                }
            }
        }
    }
    Vec::new()
}

fn evaluate_path(path: &Vec<XY>) -> usize {
    let mut facing = xy(1, 0);
    let mut cost = 0;
    path.windows(2)
        .for_each(|pair| {
            let movement = pair[1] - pair[0];
            if facing == movement {
                cost += 1;
            } else {
                cost += 1001;
            }
            facing = movement;
        });
    cost
}

fn find_char(grid: &HashMap<XY, char>, target: char) -> Option<XY> {
    for (&pos, &c) in grid.iter() {
        if c == target {
            return Some(pos);
        }
    }
    None
}

fn print_path(path: &Vec<XY>, grid: &HashMap<XY, char>) {
    let path: HashSet<XY> = path.iter().cloned().collect();
    let y_max = grid.keys().map(|k| k.y).max().unwrap();
    let x_max = grid.keys().map(|k| k.x).max().unwrap();
    let mut result = String::new();
    for y in 0..=y_max {
        let mut row = String::new();
        for x in 0..=x_max {
            if !path.contains(&xy(x, y)) {
                let c = *grid.get(&xy(x, y)).unwrap_or(&' ');
                row.push(c);
            } else {
                row.push('o');
            }
        }
        result += &row;
        result.push('\n');
    }
    println!("{}", result);
}

#[aocd(2024, 16)]
pub fn solution1() {
    let data = input!();
    let grid = read_grid(data.as_str());
    let start = find_char(&grid, 'S').unwrap();
    let goal = find_char(&grid, 'E').unwrap();
    let path = a_star(start, goal, &grid);
    let score = evaluate_path(&path);
    submit!(1, score);
}

#[aocd(2024, 16)]
pub fn solution2() {
    
}