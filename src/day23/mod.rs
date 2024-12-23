use std::collections::{HashMap, HashSet};

use aocd::*;

#[aocd(2024, 23)]
pub fn solution1() {
    let data = input!();
    let mut graph = HashMap::<&str, HashSet<&str>>::new();
    data.lines().for_each(|line| {
        let ips: Vec<&str> = line.split('-').collect();
        graph.entry(ips[0]).or_default().insert(ips[1]);
        graph.entry(ips[1]).or_default().insert(ips[0]);
    });
    let mut clusters = HashSet::<(&str, &str, &str)>::new();
    for (node, adjacent) in graph.iter() {
        for a in adjacent.iter() {
            for b in adjacent.iter() {
                if a != b {
                    if graph.get(a).unwrap().contains(b) {
                        let mut cluster = vec![node, a, b];
                        cluster.sort();
                        if node.starts_with('t') || a.starts_with('t') || b.starts_with('t') {
                            clusters.insert((cluster[0], cluster[1], cluster[2]));
                        }
                    }
                }
            }
        }
    }
    submit!(1, clusters.len());
}



#[aocd(2024, 23)]
pub fn solution2() {
    
}