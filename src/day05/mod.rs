use std::collections::{HashMap, HashSet};

use aocd::*;

fn read_rules(input: &str) -> HashMap<i32, HashSet<i32>> {
    let mut rules = HashMap::<i32, HashSet<i32>>::new();
    input
        .lines()
        .filter(|line| line.contains('|'))
        .for_each(|line| {
            let pair: Vec<i32> = line.split('|').map(|num| num.parse().unwrap()).collect();
            let entry = rules.entry(pair[0]).or_default();
            entry.insert(pair[1]);
        });
    rules
}

fn read_updates(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .filter(|line| line.contains(','))
        .map(|line| line.split(',').map(|num| num.parse().unwrap()).collect())
        .collect()
}

fn is_valid_update(orders: &[i32], rules: &HashMap<i32, HashSet<i32>>) -> bool {
    let n = orders.len();
    orders.iter().take(n - 1).enumerate().all(|(i, order)| {
        let after: HashSet<i32> = orders[(i + 1)..].iter().cloned().collect();
        let allowed = rules.get(order).cloned().unwrap_or_default();
        after.is_subset(&allowed)
    })
}

#[aocd(2024, 5)]
pub fn solution1() {
    let data = input!();
    let rules = read_rules(&data);
    let score: i32 = read_updates(&data)
        .into_iter()
        .filter(|orders| is_valid_update(orders, &rules))
        .map(|orders| orders[orders.len() / 2])
        .sum();

    submit!(1, score);
}

#[aocd(2024, 5)]
pub fn solution2() {
    let data = input!();
    let rules = read_rules(&data);
    let score: i32 = read_updates(&data)
        .into_iter()
        .filter(|orders| !is_valid_update(orders, &rules))
        .map(|mut orders| {
            let orders_set: HashSet<i32> = orders.iter().cloned().collect();
            orders.sort_by_key(|order| {
                let allowed = rules.get(order).cloned().unwrap_or_default();
                orders_set.intersection(&allowed).count()
            });
            orders[orders.len() / 2]
        })
        .sum();

    submit!(2, score);
}
