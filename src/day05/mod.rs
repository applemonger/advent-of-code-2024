use std::collections::{HashMap, HashSet};

use aocd::*;

#[aocd(2024, 5)]
pub fn solution1() {
    let mut rules = HashMap::<i32, HashSet<i32>>::new();
    input!()
        .lines()
        .flat_map(|c| c.contains("|").then_some(c.to_string()))
        .for_each(|s| {
            let pair: Vec<i32> = s.split('|').map(|num| num.parse::<i32>().unwrap()).collect();
            let entry = rules.entry(pair[0]).or_default();
            entry.insert(pair[1]);
        });

    let updates: Vec<Vec<i32>> = input!()
        .lines()
        .flat_map(|c| c.contains(",").then_some(c.to_string()))
        .map(|s| {
            s.split(',')
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let mut score = 0;
    updates
        .iter()
        .for_each(|orders| {
            let n = orders.len();
            let mut valid = true;
            for (i, order) in orders.iter().enumerate() {
                if i != (n - 1) {
                    let rest = &orders[(i+1)..n];
                    let rest: HashSet<i32> = rest.iter().cloned().collect();
                    let allowed = rules.get(order).unwrap_or(&HashSet::new()).clone();
                    if !rest.is_subset(&allowed) {
                        valid = false;
                    }
                }
            }
            if valid {
                let middle_order = orders[n / 2];
                score += middle_order;
            }
        });

    submit!(1, score);
}

#[aocd(2024, 5)]
pub fn solution2() {
    let mut rules = HashMap::<i32, HashSet<i32>>::new();
    input!()
        .lines()
        .flat_map(|c| c.contains("|").then_some(c.to_string()))
        .for_each(|s| {
            let pair: Vec<i32> = s.split('|').map(|num| num.parse::<i32>().unwrap()).collect();
            let entry = rules.entry(pair[0]).or_default();
            entry.insert(pair[1]);
        });

    let updates: Vec<Vec<i32>> = input!()
        .lines()
        .flat_map(|c| c.contains(",").then_some(c.to_string()))
        .map(|s| {
            s.split(',')
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let mut score = 0;
    updates
        .iter()
        .for_each(|orders| {
            let n = orders.len();
            let mut valid = true;
            for (i, order) in orders.iter().enumerate() {
                if i != (n - 1) {
                    let rest = &orders[(i+1)..n];
                    let rest: HashSet<i32> = rest.iter().cloned().collect();
                    let allowed = rules.get(order).unwrap_or(&HashSet::new()).clone();
                    if !rest.is_subset(&allowed) {
                        valid = false;
                    }
                }
            }
            if !valid {
                let mut ordered_update = Vec::<i32>::new();
                for _ in 0..n {
                    ordered_update.push(0);
                }
                for (i, order) in orders.iter().enumerate() {
                    let allowed = rules.get(order).unwrap_or(&HashSet::new()).clone();
                    let mut rest = orders.clone();
                    rest.remove(i);
                    let rest: HashSet<i32> = rest.iter().cloned().collect();
                    let index = n - rest.intersection(&allowed).count() - 1;
                    ordered_update[index] = *order;
                }
                let middle_order = ordered_update[n / 2];
                score += middle_order;
            }
        });

    submit!(2, score);
}