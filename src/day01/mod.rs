use std::collections::HashMap;

use aocd::*;

#[aocd(2024, 1)]
pub fn solution1() {
    let data: Vec<(i64, i64)> = input!()
        .split('\n')
        .map(|s| {
            let pair: Vec<i64> = s.split("   ").map(|n| n.parse::<i64>().unwrap()).collect();
            (pair[0], pair[1])
        })
        .collect();

    let mut first: Vec<i64> = data.iter().map(|(first, _)| *first).collect();
    first.sort();

    let mut second: Vec<i64> = data.iter().map(|(_, second)| *second).collect();
    second.sort();

    let total: i64 = first
        .iter()
        .zip(second.iter())
        .map(|(u, v)| (*u - *v).abs())
        .sum();

    submit!(1, total);
}

#[aocd(2024, 1)]
pub fn solution2() {
    let data: Vec<(i64, i64)> = input!()
        .split('\n')
        .map(|s| {
            let pair: Vec<i64> = s.split("   ").map(|n| n.parse::<i64>().unwrap()).collect();
            (pair[0], pair[1])
        })
        .collect();

    let mut counts = HashMap::<i64, u64>::new();
    for (_, second) in data.iter() {
        let entry = counts.entry(*second).or_insert(0);
        *entry += 1;
    }

    let mut score: u64 = 0;
    for (first, _) in data.iter() {
        let entry = counts.entry(*first).or_insert(0);
        score += (*first as u64) * *entry;
    }

    submit!(2, score);
}
