use crate::utils::{xy, XY};
use aocd::*;
use std::collections::{HashMap, HashSet};

const INF: i32 = i32::MAX / 2;

fn read_data(input: &str, n: usize) -> Vec<XY> {
    let mut all: Vec<XY> = input
        .lines()
        .take(n)
        .map(|line| {
            let xy: Vec<i32> = line.split(',').map(|x| x.parse().unwrap()).collect();
            XY::new(xy[0], xy[1])
        })
        .collect();
    all.reverse();
    all
}

struct Heap {
    data: Vec<XY>,
}

impl Heap {
    fn insert(&mut self, node: XY) {
        self.data.push(node);
    }

    fn pop(&mut self, f_score: &HashMap<XY, i32>) -> Option<XY> {
        self.data
            .sort_by_key(|node| -f_score.get(node).unwrap_or(&INF));
        self.data.pop()
    }

    fn contains(&self, node: &XY) -> bool {
        self.data.contains(node)
    }
}

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
    let mut open = Heap { data: vec![start] };
    let mut prev = HashMap::<XY, XY>::new();
    let mut g_score = HashMap::<XY, i32>::new();
    g_score.insert(start, 0);
    let mut f_score = HashMap::<XY, i32>::new();
    f_score.insert(start, h(start, goal));
    while let Some(current) = open.pop(&f_score) {
        if current == goal {
            return reconstruct_path(&prev, current);
        }
        'search: for neighbor in current.neighbors() {
            if map.get(&neighbor) != Some(&'.') {
                continue 'search;
            }
            let alt = g_score.get(&current).unwrap_or(&INF) + 1;
            if alt < *g_score.get(&neighbor).unwrap_or(&INF) {
                prev.insert(neighbor, current);
                g_score.insert(neighbor, alt);
                f_score.insert(neighbor, alt + h(neighbor, goal));
                if !open.contains(&neighbor) {
                    open.insert(neighbor);
                }
            }
        }
    }
    Vec::new()
}

#[aocd(2024, 18)]
pub fn solution1() {
    let data = input!();
    let mut grid = HashMap::<XY, char>::new();
    let mut obstacles: Vec<XY> = read_data(data.as_str(), 1024);
    let dim = 70;
    for i in 0..=dim {
        for j in 0..=dim {
            grid.insert(xy(i, j), '.');
        }
    }
    while let Some(obstacle) = obstacles.pop() {
        grid.insert(obstacle, '#');
    }
    let path = a_star(xy(0, 0), xy(dim, dim), &grid);
    submit!(1, path.len() - 1);
}

#[aocd(2024, 18)]
pub fn solution2() {
    let data = input!();
    let mut grid = HashMap::<XY, char>::new();
    let mut obstacles = read_data(data.as_str(), usize::MAX);
    let dim = 70;
    for i in 0..=dim {
        for j in 0..=dim {
            grid.insert(xy(i, j), '.');
        }
    }
    let mut path: HashSet<XY> = a_star(xy(0, 0), xy(dim, dim), &grid).into_iter().collect();
    'simulate: while let Some(obstacle) = obstacles.pop() {
        grid.insert(obstacle, '#');
        if path.contains(&obstacle) {
            path = a_star(xy(0, 0), xy(dim, dim), &grid).into_iter().collect();
        }
        if path.is_empty() {
            let obstacle = format!("{},{}", obstacle.x, obstacle.y);
            submit!(2, obstacle);
            break 'simulate;
        }
    }
}
