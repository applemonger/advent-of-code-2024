use std::collections::HashMap;

use aocd::*;

type Seq = (isize, isize, isize, isize);

fn gen(mut num: usize) -> usize {
    num ^= num * 64;
    num %= 16777216;
    num ^= num / 32;
    num %= 16777216;
    num ^= num * 2048;
    num % 16777216
}

fn seq_map(seed: usize, n: usize) -> HashMap<Seq, usize> {
    let mut prices = Vec::new();
    (0..n).fold(seed, |num, _| {
        prices.push(num as isize % 10);
        gen(num)
    });
    let diff: Vec<isize> = prices.windows(2).map(|x| x[1] - x[0]).collect();
    let mut changes = HashMap::<Seq, usize>::new();
    diff.windows(4).enumerate().for_each(|(i, diffs)| {
        let seq = (diffs[0], diffs[1], diffs[2], diffs[3]);
        changes.entry(seq).or_insert(prices[i + 4] as usize);
    });
    changes
}

#[aocd(2024, 22)]
pub fn solution1() {
    let data = input!();
    let seeds: Vec<usize> = data.lines().map(|x| x.parse().unwrap()).collect();
    let k = 2000;
    let total: usize = seeds.iter().map(|&x| (0..k).fold(x, |n, _| gen(n))).sum();
    submit!(1, total);
}

#[aocd(2024, 22)]
pub fn solution2() {
    let data = input!();
    let secrets: Vec<usize> = data.lines().map(|x| x.parse().unwrap()).collect();
    let mut totals = HashMap::<Seq, usize>::new();
    for secret in secrets {
        for (seq, price) in seq_map(secret, 2000) {
            *totals.entry(seq).or_insert(0) += price;
        }
    }
    let (_, best) = totals.iter().max_by_key(|&(_, &v)| v).unwrap();
    submit!(2, *best);
}
