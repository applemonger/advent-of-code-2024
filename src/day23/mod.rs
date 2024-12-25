use std::collections::{HashMap, HashSet};

use aocd::*;

fn read_graph(input: &str) -> HashMap<&str, HashSet<&str>> {
    let mut graph = HashMap::<&str, HashSet<&str>>::new();
    input.lines().for_each(|line| {
        let ips: Vec<&str> = line.split('-').collect();
        graph.entry(ips[0]).or_default().insert(ips[1]);
        graph.entry(ips[1]).or_default().insert(ips[0]);
    });
    graph
}

#[aocd(2024, 23)]
pub fn solution1() {
    let data = input!();
    let graph = read_graph(&data);
    let mut clusters = HashSet::<(&str, &str, &str)>::new();
    for (node, adjacent) in graph.iter() {
        for a in adjacent.iter() {
            for b in adjacent.iter() {
                if a != b && graph[a].contains(b) {
                    let mut cluster = [node, a, b];
                    cluster.sort();
                    if node.starts_with('t') || a.starts_with('t') || b.starts_with('t') {
                        clusters.insert((cluster[0], cluster[1], cluster[2]));
                    }
                }
            }
        }
    }
    submit!(1, clusters.len());
}

#[aocd(2024, 23)]
pub fn solution2() {
    let data = input!();
    let graph = read_graph(&data);
    let mut largest = HashSet::new();
    let mut best = 0;
    for node in graph.keys() {
        let mut cluster = HashSet::new();
        cluster.insert(node);
        for other in graph.keys().filter(|n| *n != node) {
            if cluster.iter().all(|member| graph[other].contains(*member)) {
                cluster.insert(other);
            }
        }
        if cluster.len() > best {
            largest = cluster;
            best = largest.len();
        }
    }
    let mut largest: Vec<String> = largest.iter().map(|s| s.to_string()).collect();
    largest.sort();
    submit!(2, largest.join(","));
}
