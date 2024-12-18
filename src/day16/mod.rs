use crate::utils::{cardinals, read_grid, xy, XY};
use aocd::*;
use std::collections::{HashMap, HashSet};

const INF: i32 = i32::MAX / 2;

pub type Node = (XY, XY);

fn dijkstra(
    source: Node,
    goal: XY,
    map: &HashMap<XY, char>,
) -> (HashMap<Node, i32>, HashMap<Node, HashSet<Node>>) {
    let mut dist = HashMap::<Node, i32>::new();
    let mut prev = HashMap::<Node, HashSet<Node>>::new();
    let mut open = Vec::<Node>::new();
    for (&pos, _) in map.iter().filter(|&(_, &v)| v != '#') {
        for direction in cardinals() {
            open.push((pos, direction));
        }
    }
    dist.insert(source, 0);
    'search: while !open.is_empty() {
        open.sort_by_key(|node| -dist.get(node).unwrap_or(&INF));
        let node = open.pop().unwrap();
        if node.0 == goal {
            continue 'search;
        }
        'neighbors: for direction in cardinals() {
            let neighbor = (node.0 + direction, direction);
            if map.get(&neighbor.0) == Some(&'#') {
                continue 'neighbors;
            }
            if open.contains(&neighbor) {
                let cost = 1 + (node.1 != direction) as i32 * 1000;
                let alt = dist.get(&node).unwrap_or(&INF) + cost;
                if alt <= *dist.get(&neighbor).unwrap_or(&INF) {
                    dist.insert(neighbor, alt);
                    prev.entry(neighbor).or_default().insert(node);
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
    path.windows(2)
        .map(|pair| {
            let turn = pair[1].1 != pair[0].1;
            1 + (turn as i32) * 1000
        })
        .sum()
}

fn find_char(grid: &HashMap<XY, char>, target: char) -> Option<XY> {
    grid.iter()
        .find(|&(_, &v)| v == target)
        .map(|(k, _)| k)
        .copied()
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
