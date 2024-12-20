use crate::utils::{cardinals, read_grid, xy, XY};
use aocd::*;
use std::collections::{HashMap, HashSet};

const INF: i32 = i32::MAX / 2;

pub type Node = (XY, XY);

fn dijkstra(
    start: Node,
    goal: XY,
    map: &HashMap<XY, char>,
) -> (HashMap<Node, i32>, HashMap<Node, HashSet<Node>>) {
    let mut open = vec![start];
    let mut prev = HashMap::<Node, HashSet<Node>>::new();
    let mut dist = HashMap::<Node, i32>::new();
    dist.insert(start, 0);
    'search: while !open.is_empty() {
        open.sort_by_key(|node| -dist.get(node).unwrap_or(&INF));
        let current = open.pop().unwrap();
        if current.0 == goal {
            continue 'search;
        }
        'neighbors: for movement in cardinals() {
            let neighbor = (current.0 + movement, movement);
            if map.get(&neighbor.0) == Some(&'#') {
                continue 'neighbors;
            }
            let cost = 1 + (current.1 != movement) as i32 * 1000;
            let alt = dist.get(&current).unwrap_or(&INF) + cost;
            if alt <= *dist.get(&neighbor).unwrap_or(&INF) {
                prev.entry(neighbor).or_default().insert(current);
                dist.insert(neighbor, alt);
                if !open.contains(&neighbor) {
                    open.push(neighbor);
                }
            }
        }
    }
    (dist, prev)
}

fn search(
    start: Node,
    goal: Node,
    prev: &HashMap<Node, HashSet<Node>>,
    mut path: Vec<Node>,
    paths: &mut Vec<Vec<Node>>,
) {
    path.push(goal);
    if let Some(priors) = prev.get(&goal) {
        for prior in priors {
            search(start, *prior, prev, path.clone(), paths);
        }
    }
    if goal == start {
        paths.push(path);
    }
}

fn cost(path: &[Node]) -> i32 {
    let turns = path.windows(2).filter(|n| n[1].1 != n[0].1).count();
    turns as i32 * 1000 + path.len() as i32 - 1
}

fn find_char(grid: &HashMap<XY, char>, c: char) -> Option<XY> {
    grid.iter().find(|&(_, &v)| v == c).map(|(k, _)| k).copied()
}

#[aocd(2024, 16)]
pub fn solution1() {
    let data = input!();
    let grid = read_grid(data.as_str());
    let start = find_char(&grid, 'S').unwrap();
    let goal = find_char(&grid, 'E').unwrap();
    let (dist, _) = dijkstra((start, xy(1, 0)), goal, &grid);
    let best_score = cardinals()
        .iter()
        .map(|dir| dist.get(&(goal, *dir)).unwrap_or(&INF))
        .min()
        .unwrap();
    submit!(1, best_score);
}

#[aocd(2024, 16)]
pub fn solution2() {
    let data = input!();
    let grid = read_grid(data.as_str());
    let start = (find_char(&grid, 'S').unwrap(), xy(1, 0));
    let goal = find_char(&grid, 'E').unwrap();
    let (dist, mut prev) = dijkstra(start, goal, &grid);
    let best_score = cardinals()
        .iter()
        .map(|dir| dist.get(&(goal, *dir)).unwrap_or(&INF))
        .min()
        .unwrap();
    prev.retain(|k, _| dist.get(k).unwrap_or(&INF) <= best_score);
    let mut paths = Vec::new();
    for dir in cardinals() {
        search(start, (goal, dir), &prev, Vec::new(), &mut paths)
    }
    paths.retain(|path| cost(path) == *best_score);
    let seats: HashSet<XY> = paths
        .iter()
        .flat_map(|path| path.iter().map(|node| node.0))
        .collect();
    submit!(2, seats.len());
}
