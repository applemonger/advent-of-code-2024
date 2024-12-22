use crate::utils::{xy, XY};
use aocd::*;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};

const INF: i32 = i32::MAX / 2;

#[derive(Clone)]
struct Pad {
    keys: HashMap<XY, char>,
    ptr: XY,
}

impl Pad {
    fn new(data: &[(XY, char)]) -> Pad {
        let keys: HashMap<XY, char> = data.iter().cloned().collect();
        let (&ptr, _) = keys.clone().iter().find(|&(_, &v)| v == 'A').unwrap();
        Pad { keys, ptr }
    }

    fn dijkstra(&self, start: XY, goal: XY) -> (HashMap<XY, i32>, HashMap<XY, HashSet<XY>>) {
        let mut open = vec![start];
        let mut prev = HashMap::<XY, HashSet<XY>>::new();
        let mut dist = HashMap::<XY, i32>::new();
        dist.insert(start, 0);
        'search: while !open.is_empty() {
            open.sort_by_key(|node| -dist.get(node).unwrap_or(&INF));
            let current = open.pop().unwrap();
            if current == goal {
                continue 'search;
            }
            'neighbors: for neighbor in current.neighbors() {
                if self.keys.get(&neighbor).is_none() {
                    continue 'neighbors;
                }
                let alt = dist.get(&current).unwrap_or(&INF) + 1;
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

    fn paths(&self, c: char) -> Vec<Vec<XY>> {
        let goal = self.pos(&c);
        let (_, prev) = self.dijkstra(self.ptr, goal);
        let mut paths = Vec::new();
        search(self.ptr, goal, &prev, Vec::new(), &mut paths);
        paths
    }

    fn pos(&self, c: &char) -> XY {
        let keys = self.keys.clone();
        let (pos, _) = keys.iter().find(|&(_, v)| v == c).unwrap();
        *pos
    }

    fn instruct(&mut self, desired: &str) -> Vec<String> {
        let mut possible = Vec::new();
        for c in desired.chars() {
            let mut sub = Vec::new();
            for path in self.paths(c) {
                let mut s: String = path.windows(2).map(|p| to_char(&(p[1] - p[0]))).collect();
                s.push('A');
                sub.push(s);
            }
            possible.push(sub);
            self.ptr = self.pos(&c);
        }
        let total: Vec<String> = possible
            .iter()
            .cloned()
            .multi_cartesian_product()
            .map(|v| v.join(""))
            .collect();
        total
    }
}

fn to_char(point: &XY) -> char {
    match point {
        XY { x: 1, y: 0 } => '>',
        XY { x: -1, y: 0 } => '<',
        XY { x: 0, y: 1 } => 'v',
        XY { x: 0, y: -1 } => '^',
        _ => panic!("Cannot convert {point:?}!"),
    }
}

fn search(
    start: XY,
    goal: XY,
    prev: &HashMap<XY, HashSet<XY>>,
    mut path: Vec<XY>,
    paths: &mut Vec<Vec<XY>>,
) {
    path.push(goal);
    if let Some(priors) = prev.get(&goal) {
        for prior in priors {
            search(start, *prior, prev, path.clone(), paths);
        }
    }
    if goal == start {
        path.reverse();
        paths.push(path);
    }
}

// fn to_xy(c: &char) -> XY {
//     match c {
//         '>' => XY { x: 1, y: 0 },
//         '<' => XY { x: -1, y: 0 },
//         '^' => XY { x: 0, y: -1 },
//         'v' => XY { x: 0, y: 1 },
//         'A' => XY { x: 0, y: 0 },
//         _ => panic!("Cannot convert {c:?}!")
//     }
// }

fn numeric(s: &str) -> usize {
    let re = Regex::new(r"(\d+)").unwrap();
    let cap = re.captures(s).unwrap();
    cap.get(1).unwrap().as_str().parse().unwrap()
}

#[aocd(2024, 21, "src/day21/test.txt")]
pub fn solution1() {
    let data = input!();
    let mut keypad = Pad::new(&[
        (xy(0, 0), '7'),
        (xy(1, 0), '8'),
        (xy(2, 0), '9'),
        (xy(0, 1), '4'),
        (xy(1, 1), '5'),
        (xy(2, 1), '6'),
        (xy(0, 2), '1'),
        (xy(1, 2), '2'),
        (xy(2, 2), '3'),
        (xy(1, 3), '0'),
        (xy(2, 3), 'A'),
    ]);
    let mut dpad_1 = Pad::new(&[
        (xy(1, 0), '^'),
        (xy(2, 0), 'A'),
        (xy(0, 1), '<'),
        (xy(1, 1), 'v'),
        (xy(2, 1), '>'),
    ]);
    let mut dpad_2 = dpad_1.clone();
    let mut complexity = 0;
    for code in data.lines() {
        let mut best = usize::MAX;
        for i in keypad.instruct(code) {
            for j in dpad_1.instruct(&i) {
                for k in dpad_2.instruct(&j) {
                    best = best.min(k.len());
                }
            }
        }
        complexity += best * numeric(code);
    }
    submit!(1, complexity);
}

//#[aocd(2024, 21)]
pub fn solution2() {}
