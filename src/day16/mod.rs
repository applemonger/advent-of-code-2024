use crate::utils::{cardinals, read_grid, xy, XY};
use aocd::*;
use std::{
    collections::{HashMap, HashSet},
    usize,
};

const INF: i32 = i32::MAX / 2;

pub type Node = (XY, XY);

fn dijkstra(source: Node, map: &HashMap<XY, char>) -> (HashMap<Node, i32>, HashMap<Node, Node>) {
    let mut dist = HashMap::<Node, i32>::new();
    let mut prev = HashMap::<Node, Node>::new();
    let mut open = Vec::<Node>::new();
    for (&pos, &c) in map.iter() {
        if c != '#' {
            for direction in cardinals() {
                open.push((pos, direction));
            }
        }
    }
    dist.insert(source, 0);
    while !open.is_empty() {
        open.sort_by_key(|node| -dist.get(node).unwrap_or(&INF));
        let node = open.pop().unwrap();
        'search: for direction in cardinals() {
            let neighbor = (node.0 + direction, direction);
            if map.get(&neighbor.0) == Some(&'#') {
                continue 'search;
            }
            if open.contains(&neighbor) {
                let cost = 1 + (node.1 != direction) as i32 * 1000;
                let alt = dist.get(&node).unwrap_or(&INF) + cost;
                if alt < *dist.get(&neighbor).unwrap_or(&INF) {
                    dist.insert(neighbor, alt);
                    prev.insert(neighbor, node);
                }
            }
        }
    }
    (dist, prev)
}

fn get_path(source: Node, target: Node, prev: &HashMap<Node, Node>) -> Option<Vec<Node>> {
    let mut path = Vec::<Node>::new();
    let mut node = Some(target);
    if prev.get(&target).is_some() || target == source {
        while node.is_some() {
            path.push(node.unwrap());
            node = prev.get(&node.unwrap()).copied();
        }
    }
    path.reverse();
    if path.is_empty()
        || path.first().copied() != Some(source)
        || path.last().copied() != Some(target)
    {
        None
    } else {
        Some(path)
    }
}

fn evaluate_path(path: &[Node]) -> usize {
    path.windows(2)
        .map(|pair| {
            let turn = pair[1].1 != pair[0].1;
            1 + (turn as usize) * 1000
        })
        .sum()
}

fn find_char(grid: &HashMap<XY, char>, target: char) -> Option<XY> {
    for (&pos, &c) in grid.iter() {
        if c == target {
            return Some(pos);
        }
    }
    None
}

// fn print_path(path: &Vec<XY>, grid: &HashMap<XY, char>) {
//     let path: HashSet<XY> = path.iter().cloned().collect();
//     let y_max = grid.keys().map(|k| k.y).max().unwrap();
//     let x_max = grid.keys().map(|k| k.x).max().unwrap();
//     let mut result = String::new();
//     for y in 0..=y_max {
//         let mut row = String::new();
//         for x in 0..=x_max {
//             if !path.contains(&xy(x, y)) {
//                 let c = *grid.get(&xy(x, y)).unwrap_or(&' ');
//                 row.push(c);
//             } else {
//                 row.push('o');
//             }
//         }
//         result += &row;
//         result.push('\n');
//     }
//     println!("{}", result);
// }

#[aocd(2024, 16)]
pub fn solution1() {
    let data = input!();
    let grid = read_grid(data.as_str());
    let start = find_char(&grid, 'S').unwrap();
    let goal = find_char(&grid, 'E').unwrap();
    let (_, prev) = dijkstra((start, xy(1, 0)), &grid);
    let mut score = usize::MAX;
    for direction in cardinals() {
        let path_opt = get_path((start, xy(1, 0)), (goal, direction), &prev);
        if let Some(path) = path_opt {
            let path_score = evaluate_path(&path);
            score = score.min(path_score);
        }
    }
    submit!(1, score);
}

#[aocd(2024, 16)]
pub fn solution2() {
    let data = input!();
    let grid = read_grid(data.as_str());
    let start = find_char(&grid, 'S').unwrap();
    let goal = find_char(&grid, 'E').unwrap();
    let (_, prev) = dijkstra((start, xy(1, 0)), &grid);
    let mut score = usize::MAX;
    for direction in cardinals() {
        let path_opt = get_path((start, xy(1, 0)), (goal, direction), &prev);
        if let Some(path) = path_opt {
            let path_score = evaluate_path(&path);
            score = score.min(path_score);
        }
    }
}

fn _get_points(dist: &HashMap<(XY, XY), i32>, threshold: i32) -> HashSet<XY> {
    dist.iter()
        .filter(|(_, &score)| score <= threshold)
        .map(|(&node, _)| node.0)
        .collect()
}
